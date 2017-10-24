/// C++ type: <span style='color: green;'>```QVector<int>```</span>
#[repr(C)]
pub struct VectorCInt([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_C_INT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorCInt {
  unsafe fn new_uninitialized() -> VectorCInt {
    VectorCInt(::std::mem::uninitialized())
  }
}

impl VectorCInt {
  /// C++ method: <span style='color: green;'>```QVector<int>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorCInt) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::append(const QVector<int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::append(const int& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCIntAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const int& QVector<int>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_at(self as *const ::vector::VectorCInt, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_back_const(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QVector<int>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_back(self as *mut ::vector::VectorCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_capacity(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_clear(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```const int* QVector<int>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_constData(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_constFirst(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_constLast(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::contains(const int& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_int_contains(self as *const ::vector::VectorCInt,
                                            t as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::count(const int& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCIntCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const int* QVector<int>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_data_const(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```int* QVector<int>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_data(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_int_empty(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::endsWith(const int& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_int_endsWith(self as *const ::vector::VectorCInt,
                                            t as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::libc::c_int) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::fill(const int& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::libc::c_int, ::libc::c_int)) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::fill(const int& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCInt
    where Args: overloading::VectorCIntFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const int& QVector<int>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_first_const(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QVector<int>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_first(self as *mut ::vector::VectorCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QVector<int> QVector<int>::fromList(const QList<int>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::list::ListCInt) -> ::vector::VectorCInt {
    {
      let mut object: ::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_int_fromList_to_output(list as *const ::list::ListCInt, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_front_const(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QVector<int>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_front(self as *mut ::vector::VectorCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::indexOf(const int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::indexOf(const int& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCIntIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<int>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::insert(int i, const int& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::insert(int i, int n, const int& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCIntInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<int>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_int_isEmpty(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_last_const(self as *const ::vector::VectorCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::lastIndexOf(const int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::lastIndexOf(const int& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCIntLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int& QVector<int>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_last(self as *mut ::vector::VectorCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_length(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int> QVector<int>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int> QVector<int>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorCInt
    where Args: overloading::VectorCIntMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<int>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_move(self as *mut ::vector::VectorCInt, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<int>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCInt) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<int>::QVector(const QVector<int>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<int>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::libc::c_int)) -> ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<int>::QVector(int size, const int& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorCInt
    where Args: overloading::VectorCIntNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<int> QVector<int>::operator+(const QVector<int>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorCInt) -> ::vector::VectorCInt {
    {
      let mut object: ::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_int_operator_add_to_output(self as *const ::vector::VectorCInt,
                                                            l as *const ::vector::VectorCInt,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorCInt) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::operator+=(const QVector<int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_int) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::operator+=(const int& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCInt
    where Args: overloading::VectorCIntOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::operator=(const QVector<int>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorCInt) -> &'l0 mut ::vector::VectorCInt {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVector_int_operator_assign(self as *mut ::vector::VectorCInt,
                                                   v as *const ::vector::VectorCInt)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::operator==(const QVector<int>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorCInt) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_int_operator_eq(self as *const ::vector::VectorCInt,
                                               v as *const ::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```const int& QVector<int>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_int {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_int_operator_index_const(self as *const ::vector::VectorCInt, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QVector<int>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_int_operator_index(self as *mut ::vector::VectorCInt, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::operator!=(const QVector<int>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorCInt) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_int_operator_neq(self as *const ::vector::VectorCInt,
                                                v as *const ::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorCInt) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::operator<<(const QVector<int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_int) -> &'l0 mut ::vector::VectorCInt```<br>
  /// C++ method: <span style='color: green;'>```QVector<int>& QVector<int>::operator<<(const int& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCInt
    where Args: overloading::VectorCIntOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<int>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_pop_back(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_pop_front(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::prepend(const int& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_prepend(self as *mut ::vector::VectorCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::push_back(const int& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_push_back(self as *mut ::vector::VectorCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::push_front(const int& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_push_front(self as *mut ::vector::VectorCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<int>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCIntRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<int>::removeAll(const int& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_removeAll(self as *mut ::vector::VectorCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_removeAt(self as *mut ::vector::VectorCInt, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_removeFirst(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_removeLast(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::removeOne(const int& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_int_removeOne(self as *mut ::vector::VectorCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::replace(int i, const int& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_int_replace(self as *mut ::vector::VectorCInt,
                                           i,
                                           t as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_reserve(self as *mut ::vector::VectorCInt, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_int_resize(self as *mut ::vector::VectorCInt, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_size(self as *const ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_squeeze(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<int>::startsWith(const int& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_int_startsWith(self as *const ::vector::VectorCInt,
                                              t as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<int>::swap(QVector<int>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorCInt) {
    unsafe {
      ::ffi::qt_core_c_QVector_int_swap(self as *mut ::vector::VectorCInt,
                                        other as *mut ::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_takeAt(self as *mut ::vector::VectorCInt, i) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_takeFirst(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<int>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_int_takeLast(self as *mut ::vector::VectorCInt) }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QVector<int>::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_int_toList_to_output(self as *const ::vector::VectorCInt, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<int>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<int>::value(int i, const int& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCIntValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorCInt {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<int>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_int_destructor(self as *mut ::vector::VectorCInt) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<unsigned int>```</span>
#[repr(C)]
pub struct VectorCUint([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_C_UINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorCUint {
  unsafe fn new_uninitialized() -> VectorCUint {
    VectorCUint(::std::mem::uninitialized())
  }
}

impl VectorCUint {
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorCUint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::append(const QVector<unsigned int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::append(const unsigned int& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCUintAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_at(self as *const ::vector::VectorCUint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_back_const(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned int& QVector<unsigned int>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_back(self as *mut ::vector::VectorCUint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_capacity(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_clear(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned int* QVector<unsigned int>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_constData(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_constFirst(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_constLast(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::contains(const unsigned int& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_uint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_contains(self as *const ::vector::VectorCUint,
                                                     t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_uint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::count(const unsigned int& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCUintCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const unsigned int* QVector<unsigned int>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_data_const(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int* QVector<unsigned int>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_data(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_empty(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::endsWith(const unsigned int& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_uint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_endsWith(self as *const ::vector::VectorCUint,
                                                     t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::libc::c_uint) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::fill(const unsigned int& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::libc::c_uint, ::libc::c_int)) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::fill(const unsigned int& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCUint
    where Args: overloading::VectorCUintFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_first_const(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned int& QVector<unsigned int>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_first(self as *mut ::vector::VectorCUint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_front_const(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned int& QVector<unsigned int>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_front(self as *mut ::vector::VectorCUint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_uint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::indexOf(const unsigned int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_uint, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::indexOf(const unsigned int& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCUintIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::insert(int i, const unsigned int& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::insert(int i, int n, const unsigned int& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCUintInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_isEmpty(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_last_const(self as *const ::vector::VectorCUint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_uint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::lastIndexOf(const unsigned int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_uint, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::lastIndexOf(const unsigned int& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCUintLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned int& QVector<unsigned int>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_uint {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_unsigned_int_last(self as *mut ::vector::VectorCUint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_length(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QVector<unsigned int>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QVector<unsigned int>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorCUint
    where Args: overloading::VectorCUintMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_move(self as *mut ::vector::VectorCUint, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<unsigned int>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCUint) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<unsigned int>::QVector(const QVector<unsigned int>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<unsigned int>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::libc::c_uint)) -> ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<unsigned int>::QVector(int size, const unsigned int& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorCUint
    where Args: overloading::VectorCUintNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QVector<unsigned int>::operator+(const QVector<unsigned int>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorCUint) -> ::vector::VectorCUint {
    {
      let mut object: ::vector::VectorCUint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_operator_add_to_output(self as *const ::vector::VectorCUint,
                                                                     l as *const ::vector::VectorCUint,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorCUint) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::operator+=(const QVector<unsigned int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_uint) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::operator+=(const unsigned int& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCUint
    where Args: overloading::VectorCUintOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::operator=(const QVector<unsigned int>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorCUint) -> &'l0 mut ::vector::VectorCUint {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_operator_assign(self as *mut ::vector::VectorCUint,
                                                            v as *const ::vector::VectorCUint)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::operator==(const QVector<unsigned int>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorCUint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_operator_eq(self as *const ::vector::VectorCUint,
                                                        v as *const ::vector::VectorCUint)
    }
  }

  /// C++ method: <span style='color: green;'>```const unsigned int& QVector<unsigned int>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_uint {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_operator_index_const(self as *const ::vector::VectorCUint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```unsigned int& QVector<unsigned int>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_uint {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_operator_index(self as *mut ::vector::VectorCUint, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::operator!=(const QVector<unsigned int>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorCUint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_operator_neq(self as *const ::vector::VectorCUint,
                                                         v as *const ::vector::VectorCUint)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorCUint) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::operator<<(const QVector<unsigned int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_uint) -> &'l0 mut ::vector::VectorCUint```<br>
  /// C++ method: <span style='color: green;'>```QVector<unsigned int>& QVector<unsigned int>::operator<<(const unsigned int& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCUint
    where Args: overloading::VectorCUintOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_pop_back(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_pop_front(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::prepend(const unsigned int& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_uint) {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_prepend(self as *mut ::vector::VectorCUint,
                                                    t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::push_back(const unsigned int& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_uint) {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_push_back(self as *mut ::vector::VectorCUint,
                                                      t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::push_front(const unsigned int& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_uint) {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_push_front(self as *mut ::vector::VectorCUint,
                                                       t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCUintRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::removeAll(const unsigned int& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_uint) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_removeAll(self as *mut ::vector::VectorCUint,
                                                      t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_removeAt(self as *mut ::vector::VectorCUint, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_removeFirst(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_removeLast(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::removeOne(const unsigned int& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_uint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_removeOne(self as *mut ::vector::VectorCUint,
                                                      t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::replace(int i, const unsigned int& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_uint) {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_replace(self as *mut ::vector::VectorCUint,
                                                    i,
                                                    t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_reserve(self as *mut ::vector::VectorCUint, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_resize(self as *mut ::vector::VectorCUint, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<unsigned int>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_size(self as *const ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_squeeze(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<unsigned int>::startsWith(const unsigned int& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_uint) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_startsWith(self as *const ::vector::VectorCUint,
                                                       t as *const ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<unsigned int>::swap(QVector<unsigned int>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorCUint) {
    unsafe {
      ::ffi::qt_core_c_QVector_unsigned_int_swap(self as *mut ::vector::VectorCUint,
                                                 other as *mut ::vector::VectorCUint)
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QVector<unsigned int>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_takeAt(self as *mut ::vector::VectorCUint, i) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QVector<unsigned int>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_takeFirst(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QVector<unsigned int>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_takeLast(self as *mut ::vector::VectorCUint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QVector<unsigned int>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_uint)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QVector<unsigned int>::value(int i, const unsigned int& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_uint
    where Args: overloading::VectorCUintValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorCUint {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<unsigned int>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_unsigned_int_destructor(self as *mut ::vector::VectorCUint) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QPair<double, QVariant>>```</span>
#[repr(C)]
pub struct VectorPairPairCDoubleVariant([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_PAIR_PAIR_C_DOUBLE_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorPairPairCDoubleVariant {
  unsafe fn new_uninitialized() -> VectorPairPairCDoubleVariant {
    VectorPairPairCDoubleVariant(::std::mem::uninitialized())
  }
}

impl VectorPairPairCDoubleVariant {
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::pair::PairCDoubleVariant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::append(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorPairPairCDoubleVariant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::append(const QVector<QPair<double, QVariant>>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleVariantAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_at(self as *const ::vector::VectorPairPairCDoubleVariant, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_back_const(self as *const ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>& QVector<QPair<double, QVariant>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_back(self as *mut ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_capacity(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_clear(self as *mut ::vector::VectorPairPairCDoubleVariant) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>* QVector<QPair<double, QVariant>>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::pair::PairCDoubleVariant {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_constData(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_constFirst(self as *const ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_constLast(self as *const ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::contains(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::pair::PairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_contains(self as *const ::vector::VectorPairPairCDoubleVariant,
                                                              t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::pair::PairCDoubleVariant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::count(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleVariantCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>* QVector<QPair<double, QVariant>>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::pair::PairCDoubleVariant {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_data_const(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>* QVector<QPair<double, QVariant>>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::pair::PairCDoubleVariant {
    unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_data(self as *mut ::vector::VectorPairPairCDoubleVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_empty(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::endsWith(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::pair::PairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_endsWith(self as *const ::vector::VectorPairPairCDoubleVariant,
                                                              t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::pair::PairCDoubleVariant) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::fill(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::pair::PairCDoubleVariant, ::libc::c_int)) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::fill(const QPair<double, QVariant>& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_first_const(self as *const ::vector::VectorPairPairCDoubleVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>& QVector<QPair<double, QVariant>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_first(self as *mut ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_front_const(self as *const ::vector::VectorPairPairCDoubleVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>& QVector<QPair<double, QVariant>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_front(self as *mut ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::pair::PairCDoubleVariant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::indexOf(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::pair::PairCDoubleVariant, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::indexOf(const QPair<double, QVariant>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleVariantIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::pair::PairCDoubleVariant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::insert(int i, const QPair<double, QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::pair::PairCDoubleVariant)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::insert(int i, int n, const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleVariantInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_isEmpty(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_last_const(self as *const ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::pair::PairCDoubleVariant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::lastIndexOf(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::pair::PairCDoubleVariant, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::lastIndexOf(const QPair<double, QVariant>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleVariantLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>& QVector<QPair<double, QVariant>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleVariant {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_last(self as *mut ::vector::VectorPairPairCDoubleVariant)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_length(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>> QVector<QPair<double, QVariant>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>> QVector<QPair<double, QVariant>>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorPairPairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_move(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                          from,
                                                          to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QVariant>>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorPairPairCDoubleVariant) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QVariant>>::QVector(const QVector<QPair<double, QVariant>>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QVariant>>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::pair::PairCDoubleVariant)) -> ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QVariant>>::QVector(int size, const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorPairPairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>> QVector<QPair<double, QVariant>>::operator+(const QVector<QPair<double, QVariant>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorPairPairCDoubleVariant) -> ::vector::VectorPairPairCDoubleVariant {
    {
      let mut object: ::vector::VectorPairPairCDoubleVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_add_to_output(self as *const ::vector::VectorPairPairCDoubleVariant, l as *const ::vector::VectorPairPairCDoubleVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::pair::PairCDoubleVariant) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::operator+=(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorPairPairCDoubleVariant) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::operator+=(const QVector<QPair<double, QVariant>>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::operator=(const QVector<QPair<double, QVariant>>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorPairPairCDoubleVariant)
                             -> &'l0 mut ::vector::VectorPairPairCDoubleVariant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_assign(self as *mut ::vector::VectorPairPairCDoubleVariant, v as *const ::vector::VectorPairPairCDoubleVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::operator==(const QVector<QPair<double, QVariant>>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorPairPairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_eq(self as *const ::vector::VectorPairPairCDoubleVariant, v as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QVariant>& QVector<QPair<double, QVariant>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleVariant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_index_const(self as *const ::vector::VectorPairPairCDoubleVariant, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant>& QVector<QPair<double, QVariant>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::pair::PairCDoubleVariant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_index(self as *mut ::vector::VectorPairPairCDoubleVariant, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::operator!=(const QVector<QPair<double, QVariant>>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorPairPairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_neq(self as *const ::vector::VectorPairPairCDoubleVariant, v as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::pair::PairCDoubleVariant) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::operator<<(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorPairPairCDoubleVariant) -> &'l0 mut ::vector::VectorPairPairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>& QVector<QPair<double, QVariant>>::operator<<(const QVector<QPair<double, QVariant>>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_pop_back(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_pop_front(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::prepend(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::pair::PairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_prepend(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                             t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::push_back(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::pair::PairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_push_back(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                               t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::push_front(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::pair::PairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_push_front(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                                t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleVariantRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::removeAll(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::pair::PairCDoubleVariant) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_removeAll(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                               t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_removeAt(self as *mut ::vector::VectorPairPairCDoubleVariant, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_removeFirst(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_removeLast(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::removeOne(const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::pair::PairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_removeOne(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                               t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::replace(int i, const QPair<double, QVariant>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_replace(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                             i,
                                                             t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_reserve(self as *mut ::vector::VectorPairPairCDoubleVariant, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_resize(self as *mut ::vector::VectorPairPairCDoubleVariant, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QVariant>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_size(self as *const ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_squeeze(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QVariant>>::startsWith(const QPair<double, QVariant>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::pair::PairCDoubleVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_startsWith(self as *const ::vector::VectorPairPairCDoubleVariant,
                                                                t as *const ::pair::PairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QVariant>>::swap(QVector<QPair<double, QVariant>>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorPairPairCDoubleVariant) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_swap(self as *mut ::vector::VectorPairPairCDoubleVariant,
                                                          other as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant> QVector<QPair<double, QVariant>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::pair::PairCDoubleVariant {
    {
      let mut object: ::pair::PairCDoubleVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_takeAt_to_output(self as *mut ::vector::VectorPairPairCDoubleVariant, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant> QVector<QPair<double, QVariant>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::pair::PairCDoubleVariant {
    {
      let mut object: ::pair::PairCDoubleVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_takeFirst_to_output(self as *mut ::vector::VectorPairPairCDoubleVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QVariant> QVector<QPair<double, QVariant>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::pair::PairCDoubleVariant {
    {
      let mut object: ::pair::PairCDoubleVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPair_double_QVariant_takeLast_to_output(self as *mut ::vector::VectorPairPairCDoubleVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QVariant>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::pair::PairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QVariant> QVector<QPair<double, QVariant>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::pair::PairCDoubleVariant)) -> ::pair::PairCDoubleVariant```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QVariant> QVector<QPair<double, QVariant>>::value(int i, const QPair<double, QVariant>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::pair::PairCDoubleVariant
    where Args: overloading::VectorPairPairCDoubleVariantValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorPairPairCDoubleVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QPair<double, QVariant>>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPair_double_QVariant_destructor(self as *mut ::vector::VectorPairPairCDoubleVariant)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QPointF>```</span>
#[repr(C)]
pub struct VectorPointF([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_POINT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorPointF {
  unsafe fn new_uninitialized() -> VectorPointF {
    VectorPointF(::std::mem::uninitialized())
  }
}

impl VectorPointF {
  /// C++ method: <span style='color: green;'>```QVector<QPointF>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::append(const QPointF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorPointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::append(const QVector<QPointF>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPointFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_at(self as *const ::vector::VectorPointF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_back_const(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QVector<QPointF>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_back(self as *mut ::vector::VectorPointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_capacity(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_clear(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF* QVector<QPointF>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::point_f::PointF {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_constData(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_constFirst(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_constLast(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::contains(const QPointF& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_contains(self as *const ::vector::VectorPointF,
                                                t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::point_f::PointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::count(const QPointF& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPointFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPointF* QVector<QPointF>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::point_f::PointF {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_data_const(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF* QVector<QPointF>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::point_f::PointF {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_data(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_empty(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::endsWith(const QPointF& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_endsWith(self as *const ::vector::VectorPointF,
                                                t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::point_f::PointF) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::fill(const QPointF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::point_f::PointF, ::libc::c_int)) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::fill(const QPointF& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPointF
    where Args: overloading::VectorPointFFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_first_const(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QVector<QPointF>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_first(self as *mut ::vector::VectorPointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_front_const(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QVector<QPointF>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_front(self as *mut ::vector::VectorPointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::point_f::PointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::indexOf(const QPointF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::point_f::PointF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::indexOf(const QPointF& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPointFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPointF>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::insert(int i, const QPointF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::insert(int i, int n, const QPointF& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPointFInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_isEmpty(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_last_const(self as *const ::vector::VectorPointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::point_f::PointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::lastIndexOf(const QPointF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::point_f::PointF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::lastIndexOf(const QPointF& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPointFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPointF& QVector<QPointF>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_last(self as *mut ::vector::VectorPointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_length(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF> QVector<QPointF>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF> QVector<QPointF>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorPointF
    where Args: overloading::VectorPointFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_move(self as *mut ::vector::VectorPointF, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPointF>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorPointF) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPointF>::QVector(const QVector<QPointF>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPointF>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::point_f::PointF)) -> ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPointF>::QVector(int size, const QPointF& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorPointF
    where Args: overloading::VectorPointFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QPointF> QVector<QPointF>::operator+(const QVector<QPointF>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorPointF) -> ::vector::VectorPointF {
    {
      let mut object: ::vector::VectorPointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_operator_add_to_output(self as *const ::vector::VectorPointF,
                                                                l as *const ::vector::VectorPointF,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::point_f::PointF) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::operator+=(const QPointF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorPointF) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::operator+=(const QVector<QPointF>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPointF
    where Args: overloading::VectorPointFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::operator=(const QVector<QPointF>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorPointF) -> &'l0 mut ::vector::VectorPointF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVector_QPointF_operator_assign(self as *mut ::vector::VectorPointF,
                                                       v as *const ::vector::VectorPointF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::operator==(const QVector<QPointF>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorPointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_operator_eq(self as *const ::vector::VectorPointF,
                                                   v as *const ::vector::VectorPointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QVector<QPointF>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::point_f::PointF {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QPointF_operator_index_const(self as *const ::vector::VectorPointF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QVector<QPointF>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPointF_operator_index(self as *mut ::vector::VectorPointF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::operator!=(const QVector<QPointF>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorPointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_operator_neq(self as *const ::vector::VectorPointF,
                                                    v as *const ::vector::VectorPointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::point_f::PointF) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::operator<<(const QPointF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorPointF) -> &'l0 mut ::vector::VectorPointF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPointF>& QVector<QPointF>::operator<<(const QVector<QPointF>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPointF
    where Args: overloading::VectorPointFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_pop_back(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_pop_front(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::prepend(const QPointF& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_prepend(self as *mut ::vector::VectorPointF,
                                               t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::push_back(const QPointF& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_push_back(self as *mut ::vector::VectorPointF,
                                                 t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::push_front(const QPointF& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_push_front(self as *mut ::vector::VectorPointF,
                                                  t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPointFRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::removeAll(const QPointF& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::point_f::PointF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_removeAll(self as *mut ::vector::VectorPointF,
                                                 t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_removeAt(self as *mut ::vector::VectorPointF, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_removeFirst(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_removeLast(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::removeOne(const QPointF& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_removeOne(self as *mut ::vector::VectorPointF,
                                                 t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::replace(int i, const QPointF& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_replace(self as *mut ::vector::VectorPointF,
                                               i,
                                               t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_reserve(self as *mut ::vector::VectorPointF, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_resize(self as *mut ::vector::VectorPointF, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPointF>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_size(self as *const ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_squeeze(self as *mut ::vector::VectorPointF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPointF>::startsWith(const QPointF& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_startsWith(self as *const ::vector::VectorPointF,
                                                  t as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPointF>::swap(QVector<QPointF>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorPointF) {
    unsafe {
      ::ffi::qt_core_c_QVector_QPointF_swap(self as *mut ::vector::VectorPointF,
                                            other as *mut ::vector::VectorPointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVector<QPointF>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_takeAt_to_output(self as *mut ::vector::VectorPointF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVector<QPointF>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_takeFirst_to_output(self as *mut ::vector::VectorPointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVector<QPointF>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_takeLast_to_output(self as *mut ::vector::VectorPointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QVector<QPointF>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::point_f::PointF)) -> ::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QVector<QPointF>::value(int i, const QPointF& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::point_f::PointF
    where Args: overloading::VectorPointFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorPointF {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QPointF>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QPointF_destructor(self as *mut ::vector::VectorPointF) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QStaticPlugin>```</span>
#[repr(C)]
pub struct VectorStaticPlugin([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_STATIC_PLUGIN]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorStaticPlugin {
  unsafe fn new_uninitialized() -> VectorStaticPlugin {
    VectorStaticPlugin(::std::mem::uninitialized())
  }
}

impl VectorStaticPlugin {
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::static_plugin::StaticPlugin) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::append(const QStaticPlugin& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorStaticPlugin) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::append(const QVector<QStaticPlugin>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStaticPluginAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_at(self as *const ::vector::VectorStaticPlugin, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_back_const(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin& QVector<QStaticPlugin>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::static_plugin::StaticPlugin {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_back(self as *mut ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStaticPlugin>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_capacity(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_clear(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin* QVector<QStaticPlugin>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::static_plugin::StaticPlugin {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_constData(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_constFirst(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_constLast(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStaticPlugin>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_count(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin* QVector<QStaticPlugin>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::static_plugin::StaticPlugin {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_data_const(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin* QVector<QStaticPlugin>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::static_plugin::StaticPlugin {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_data(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStaticPlugin>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_empty(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::static_plugin::StaticPlugin) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::fill(const QStaticPlugin& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::static_plugin::StaticPlugin, ::libc::c_int)) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::fill(const QStaticPlugin& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStaticPlugin
    where Args: overloading::VectorStaticPluginFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_first_const(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin& QVector<QStaticPlugin>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::static_plugin::StaticPlugin {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_first(self as *mut ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_front_const(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin& QVector<QStaticPlugin>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::static_plugin::StaticPlugin {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_front(self as *mut ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::static_plugin::StaticPlugin)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::insert(int i, const QStaticPlugin& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::static_plugin::StaticPlugin)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::insert(int i, int n, const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStaticPluginInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QStaticPlugin>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_isEmpty(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_last_const(self as *const ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin& QVector<QStaticPlugin>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::static_plugin::StaticPlugin {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_last(self as *mut ::vector::VectorStaticPlugin) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStaticPlugin>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_length(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin> QVector<QStaticPlugin>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin> QVector<QStaticPlugin>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorStaticPlugin
    where Args: overloading::VectorStaticPluginMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_move(self as *mut ::vector::VectorStaticPlugin, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStaticPlugin>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorStaticPlugin) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStaticPlugin>::QVector(const QVector<QStaticPlugin>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStaticPlugin>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::static_plugin::StaticPlugin)) -> ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStaticPlugin>::QVector(int size, const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorStaticPlugin
    where Args: overloading::VectorStaticPluginNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin> QVector<QStaticPlugin>::operator+(const QVector<QStaticPlugin>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorStaticPlugin) -> ::vector::VectorStaticPlugin {
    {
      let mut object: ::vector::VectorStaticPlugin =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_operator_add_to_output(self as *const ::vector::VectorStaticPlugin,
                                                                      l as *const ::vector::VectorStaticPlugin,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::static_plugin::StaticPlugin) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::operator+=(const QStaticPlugin& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorStaticPlugin) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::operator+=(const QVector<QStaticPlugin>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStaticPlugin
    where Args: overloading::VectorStaticPluginOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::operator=(const QVector<QStaticPlugin>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorStaticPlugin)
                             -> &'l0 mut ::vector::VectorStaticPlugin {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_operator_assign(self as *mut ::vector::VectorStaticPlugin,
                                                             v as *const ::vector::VectorStaticPlugin)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStaticPlugin& QVector<QStaticPlugin>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_operator_index_const(self as *const ::vector::VectorStaticPlugin, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin& QVector<QStaticPlugin>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::static_plugin::StaticPlugin {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_operator_index(self as *mut ::vector::VectorStaticPlugin, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::static_plugin::StaticPlugin) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::operator<<(const QStaticPlugin& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorStaticPlugin) -> &'l0 mut ::vector::VectorStaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>& QVector<QStaticPlugin>::operator<<(const QVector<QStaticPlugin>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStaticPlugin
    where Args: overloading::VectorStaticPluginOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_pop_back(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_pop_front(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::prepend(const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::static_plugin::StaticPlugin) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_prepend(self as *mut ::vector::VectorStaticPlugin,
                                                     t as *const ::static_plugin::StaticPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::push_back(const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::static_plugin::StaticPlugin) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_push_back(self as *mut ::vector::VectorStaticPlugin,
                                                       t as *const ::static_plugin::StaticPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::push_front(const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::static_plugin::StaticPlugin) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_push_front(self as *mut ::vector::VectorStaticPlugin,
                                                        t as *const ::static_plugin::StaticPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStaticPluginRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_removeAt(self as *mut ::vector::VectorStaticPlugin, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_removeFirst(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_removeLast(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::replace(int i, const QStaticPlugin& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::static_plugin::StaticPlugin) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_replace(self as *mut ::vector::VectorStaticPlugin,
                                                     i,
                                                     t as *const ::static_plugin::StaticPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_reserve(self as *mut ::vector::VectorStaticPlugin, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_resize(self as *mut ::vector::VectorStaticPlugin, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStaticPlugin>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_size(self as *const ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_squeeze(self as *mut ::vector::VectorStaticPlugin) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStaticPlugin>::swap(QVector<QStaticPlugin>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorStaticPlugin) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStaticPlugin_swap(self as *mut ::vector::VectorStaticPlugin,
                                                  other as *mut ::vector::VectorStaticPlugin)
    }
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin QVector<QStaticPlugin>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::static_plugin::StaticPlugin {
    {
      let mut object: ::static_plugin::StaticPlugin =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_takeAt_to_output(self as *mut ::vector::VectorStaticPlugin,
                                                                i,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin QVector<QStaticPlugin>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::static_plugin::StaticPlugin {
    {
      let mut object: ::static_plugin::StaticPlugin =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_takeFirst_to_output(self as *mut ::vector::VectorStaticPlugin,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStaticPlugin QVector<QStaticPlugin>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::static_plugin::StaticPlugin {
    {
      let mut object: ::static_plugin::StaticPlugin =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_takeLast_to_output(self as *mut ::vector::VectorStaticPlugin,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStaticPlugin>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::static_plugin::StaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QStaticPlugin QVector<QStaticPlugin>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::static_plugin::StaticPlugin)) -> ::static_plugin::StaticPlugin```<br>
  /// C++ method: <span style='color: green;'>```QStaticPlugin QVector<QStaticPlugin>::value(int i, const QStaticPlugin& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::static_plugin::StaticPlugin
    where Args: overloading::VectorStaticPluginValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorStaticPlugin {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QStaticPlugin>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_destructor(self as *mut ::vector::VectorStaticPlugin) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QStringRef>```</span>
#[repr(C)]
pub struct VectorStringRef([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_STRING_REF]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorStringRef {
  unsafe fn new_uninitialized() -> VectorStringRef {
    VectorStringRef(::std::mem::uninitialized())
  }
}

impl VectorStringRef {
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::string_ref::StringRef) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::append(const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorStringRef) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::append(const QVector<QStringRef>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStringRefAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_at(self as *const ::vector::VectorStringRef, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_back_const(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringRef& QVector<QStringRef>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_back(self as *mut ::vector::VectorStringRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_capacity(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_clear(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QStringRef* QVector<QStringRef>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::string_ref::StringRef {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_constData(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_constFirst(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_constLast(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::contains(const QStringRef& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::string_ref::StringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_contains(self as *const ::vector::VectorStringRef,
                                                   t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::count(const QStringRef& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorStringRefCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStringRef* QVector<QStringRef>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::string_ref::StringRef {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_data_const(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```QStringRef* QVector<QStringRef>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::string_ref::StringRef {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_data(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_empty(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::endsWith(const QStringRef& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::string_ref::StringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_endsWith(self as *const ::vector::VectorStringRef,
                                                   t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::fill(const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::string_ref::StringRef, ::libc::c_int)) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::fill(const QStringRef& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStringRef
    where Args: overloading::VectorStringRefFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_first_const(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringRef& QVector<QStringRef>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_first(self as *mut ::vector::VectorStringRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_front_const(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringRef& QVector<QStringRef>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_front(self as *mut ::vector::VectorStringRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::indexOf(const QStringRef& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::indexOf(const QStringRef& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorStringRefIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::string_ref::StringRef)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::insert(int i, const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::string_ref::StringRef)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::insert(int i, int n, const QStringRef& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStringRefInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_isEmpty(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_last_const(self as *const ::vector::VectorStringRef) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string_ref::StringRef) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::lastIndexOf(const QStringRef& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::lastIndexOf(const QStringRef& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorStringRefLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringRef& QVector<QStringRef>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStringRef_last(self as *mut ::vector::VectorStringRef) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_length(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QVector<QStringRef>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QVector<QStringRef>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorStringRef
    where Args: overloading::VectorStringRefMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_move(self as *mut ::vector::VectorStringRef, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStringRef>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorStringRef) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStringRef>::QVector(const QVector<QStringRef>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStringRef>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::string_ref::StringRef)) -> ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QStringRef>::QVector(int size, const QStringRef& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorStringRef
    where Args: overloading::VectorStringRefNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QStringRef> QVector<QStringRef>::operator+(const QVector<QStringRef>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorStringRef) -> ::vector::VectorStringRef {
    {
      let mut object: ::vector::VectorStringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_operator_add_to_output(self as *const ::vector::VectorStringRef,
                                                                   l as *const ::vector::VectorStringRef,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::operator+=(const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorStringRef) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::operator+=(const QVector<QStringRef>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStringRef
    where Args: overloading::VectorStringRefOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::operator=(const QVector<QStringRef>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorStringRef) -> &'l0 mut ::vector::VectorStringRef {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_operator_assign(self as *mut ::vector::VectorStringRef,
                                                          v as *const ::vector::VectorStringRef)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::operator==(const QVector<QStringRef>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorStringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_operator_eq(self as *const ::vector::VectorStringRef,
                                                      v as *const ::vector::VectorStringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```const QStringRef& QVector<QStringRef>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_operator_index_const(self as *const ::vector::VectorStringRef, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStringRef& QVector<QStringRef>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::string_ref::StringRef {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_operator_index(self as *mut ::vector::VectorStringRef, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::operator!=(const QVector<QStringRef>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorStringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_operator_neq(self as *const ::vector::VectorStringRef,
                                                       v as *const ::vector::VectorStringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::operator<<(const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorStringRef) -> &'l0 mut ::vector::VectorStringRef```<br>
  /// C++ method: <span style='color: green;'>```QVector<QStringRef>& QVector<QStringRef>::operator<<(const QVector<QStringRef>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorStringRef
    where Args: overloading::VectorStringRefOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_pop_back(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_pop_front(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::prepend(const QStringRef& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::string_ref::StringRef) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_prepend(self as *mut ::vector::VectorStringRef,
                                                  t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::push_back(const QStringRef& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::string_ref::StringRef) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_push_back(self as *mut ::vector::VectorStringRef,
                                                    t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::push_front(const QStringRef& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::string_ref::StringRef) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_push_front(self as *mut ::vector::VectorStringRef,
                                                     t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorStringRefRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::removeAll(const QStringRef& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::string_ref::StringRef) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_removeAll(self as *mut ::vector::VectorStringRef,
                                                    t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_removeAt(self as *mut ::vector::VectorStringRef, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_removeFirst(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_removeLast(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::removeOne(const QStringRef& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::string_ref::StringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_removeOne(self as *mut ::vector::VectorStringRef,
                                                    t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::replace(int i, const QStringRef& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::string_ref::StringRef) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_replace(self as *mut ::vector::VectorStringRef,
                                                  i,
                                                  t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_reserve(self as *mut ::vector::VectorStringRef, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_resize(self as *mut ::vector::VectorStringRef, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QStringRef>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_size(self as *const ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_squeeze(self as *mut ::vector::VectorStringRef) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QStringRef>::startsWith(const QStringRef& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::string_ref::StringRef) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_startsWith(self as *const ::vector::VectorStringRef,
                                                     t as *const ::string_ref::StringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QStringRef>::swap(QVector<QStringRef>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorStringRef) {
    unsafe {
      ::ffi::qt_core_c_QVector_QStringRef_swap(self as *mut ::vector::VectorStringRef,
                                               other as *mut ::vector::VectorStringRef)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QVector<QStringRef>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_takeAt_to_output(self as *mut ::vector::VectorStringRef, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QVector<QStringRef>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_takeFirst_to_output(self as *mut ::vector::VectorStringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringRef QVector<QStringRef>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::string_ref::StringRef {
    {
      let mut object: ::string_ref::StringRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_takeLast_to_output(self as *mut ::vector::VectorStringRef, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QStringRef>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QVector<QStringRef>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::string_ref::StringRef)) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QVector<QStringRef>::value(int i, const QStringRef& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::string_ref::StringRef
    where Args: overloading::VectorStringRefValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorStringRef {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QStringRef>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QStringRef_destructor(self as *mut ::vector::VectorStringRef) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QTimeZone::OffsetData>```</span>
#[repr(C)]
pub struct VectorTimeZoneOffsetData([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_TIME_ZONE_OFFSET_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorTimeZoneOffsetData {
  unsafe fn new_uninitialized() -> VectorTimeZoneOffsetData {
    VectorTimeZoneOffsetData(::std::mem::uninitialized())
  }
}

impl VectorTimeZoneOffsetData {
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::time_zone::OffsetData) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::append(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorTimeZoneOffsetData) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::append(const QVector<QTimeZone::OffsetData>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTimeZoneOffsetDataAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_at(self as *const ::vector::VectorTimeZoneOffsetData, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_back_const(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::time_zone::OffsetData {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_back(self as *mut ::vector::VectorTimeZoneOffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTimeZone::OffsetData>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_capacity(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_clear(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData* QVector<QTimeZone::OffsetData>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::time_zone::OffsetData {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constData(self as *const ::vector::VectorTimeZoneOffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constFirst(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constLast(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTimeZone::OffsetData>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_count(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData* QVector<QTimeZone::OffsetData>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::time_zone::OffsetData {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_data_const(self as *const ::vector::VectorTimeZoneOffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData* QVector<QTimeZone::OffsetData>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::time_zone::OffsetData {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_data(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTimeZone::OffsetData>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_empty(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::time_zone::OffsetData) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::fill(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::time_zone::OffsetData, ::libc::c_int)) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::fill(const QTimeZone::OffsetData& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTimeZoneOffsetData
    where Args: overloading::VectorTimeZoneOffsetDataFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_first_const(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::time_zone::OffsetData {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_first(self as *mut ::vector::VectorTimeZoneOffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_front_const(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::time_zone::OffsetData {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_front(self as *mut ::vector::VectorTimeZoneOffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::time_zone::OffsetData)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::insert(int i, const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::time_zone::OffsetData)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::insert(int i, int n, const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTimeZoneOffsetDataInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QTimeZone::OffsetData>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_isEmpty(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_last_const(self as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::time_zone::OffsetData {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_last(self as *mut ::vector::VectorTimeZoneOffsetData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTimeZone::OffsetData>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_length(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData> QVector<QTimeZone::OffsetData>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData> QVector<QTimeZone::OffsetData>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorTimeZoneOffsetData
    where Args: overloading::VectorTimeZoneOffsetDataMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_move(self as *mut ::vector::VectorTimeZoneOffsetData, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTimeZone::OffsetData>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorTimeZoneOffsetData) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTimeZone::OffsetData>::QVector(const QVector<QTimeZone::OffsetData>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTimeZone::OffsetData>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::time_zone::OffsetData)) -> ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTimeZone::OffsetData>::QVector(int size, const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorTimeZoneOffsetData
    where Args: overloading::VectorTimeZoneOffsetDataNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData> QVector<QTimeZone::OffsetData>::operator+(const QVector<QTimeZone::OffsetData>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorTimeZoneOffsetData) -> ::vector::VectorTimeZoneOffsetData {
    {
      let mut object: ::vector::VectorTimeZoneOffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_add_to_output(self as *const ::vector::VectorTimeZoneOffsetData, l as *const ::vector::VectorTimeZoneOffsetData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::time_zone::OffsetData) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::operator+=(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorTimeZoneOffsetData) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::operator+=(const QVector<QTimeZone::OffsetData>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTimeZoneOffsetData
    where Args: overloading::VectorTimeZoneOffsetDataOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::operator=(const QVector<QTimeZone::OffsetData>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorTimeZoneOffsetData)
                             -> &'l0 mut ::vector::VectorTimeZoneOffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_assign(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                                      v as *const ::vector::VectorTimeZoneOffsetData)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::time_zone::OffsetData {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_index_const(self as *const ::vector::VectorTimeZoneOffsetData, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData& QVector<QTimeZone::OffsetData>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::time_zone::OffsetData {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_index(self as *mut ::vector::VectorTimeZoneOffsetData, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::time_zone::OffsetData) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::operator<<(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorTimeZoneOffsetData) -> &'l0 mut ::vector::VectorTimeZoneOffsetData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>& QVector<QTimeZone::OffsetData>::operator<<(const QVector<QTimeZone::OffsetData>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTimeZoneOffsetData
    where Args: overloading::VectorTimeZoneOffsetDataOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_pop_back(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_pop_front(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::prepend(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::time_zone::OffsetData) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_prepend(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                            t as *const ::time_zone::OffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::push_back(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::time_zone::OffsetData) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_push_back(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                              t as *const ::time_zone::OffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::push_front(const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::time_zone::OffsetData) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_push_front(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                               t as *const ::time_zone::OffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTimeZoneOffsetDataRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_removeAt(self as *mut ::vector::VectorTimeZoneOffsetData, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_removeFirst(self as *mut ::vector::VectorTimeZoneOffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_removeLast(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::replace(int i, const QTimeZone::OffsetData& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::time_zone::OffsetData) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_replace(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                            i,
                                                            t as *const ::time_zone::OffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_reserve(self as *mut ::vector::VectorTimeZoneOffsetData, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_resize(self as *mut ::vector::VectorTimeZoneOffsetData, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTimeZone::OffsetData>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_size(self as *const ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_squeeze(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTimeZone::OffsetData>::swap(QVector<QTimeZone::OffsetData>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorTimeZoneOffsetData) {
    unsafe {
      ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_swap(self as *mut ::vector::VectorTimeZoneOffsetData,
                                                         other as *mut ::vector::VectorTimeZoneOffsetData)
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QVector<QTimeZone::OffsetData>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_takeAt_to_output(self as *mut ::vector::VectorTimeZoneOffsetData, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QVector<QTimeZone::OffsetData>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_takeFirst_to_output(self as *mut ::vector::VectorTimeZoneOffsetData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QVector<QTimeZone::OffsetData>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::time_zone::OffsetData {
    {
      let mut object: ::time_zone::OffsetData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_takeLast_to_output(self as *mut ::vector::VectorTimeZoneOffsetData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTimeZone::OffsetData>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::time_zone::OffsetData```<br>
  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QVector<QTimeZone::OffsetData>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::time_zone::OffsetData)) -> ::time_zone::OffsetData```<br>
  /// C++ method: <span style='color: green;'>```QTimeZone::OffsetData QVector<QTimeZone::OffsetData>::value(int i, const QTimeZone::OffsetData& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::time_zone::OffsetData
    where Args: overloading::VectorTimeZoneOffsetDataValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorTimeZoneOffsetData {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QTimeZone::OffsetData>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_destructor(self as *mut ::vector::VectorTimeZoneOffsetData) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QXmlStreamAttribute>```</span>
#[repr(C)]
pub struct VectorXmlStreamAttribute([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_XML_STREAM_ATTRIBUTE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorXmlStreamAttribute {
  unsafe fn new_uninitialized() -> VectorXmlStreamAttribute {
    VectorXmlStreamAttribute(::std::mem::uninitialized())
  }
}

impl VectorXmlStreamAttribute {
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorXmlStreamAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::append(const QVector<QXmlStreamAttribute>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::xml_stream_attribute::XmlStreamAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::append(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamAttributeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_at(self as *const ::vector::VectorXmlStreamAttribute, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_back_const(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QVector<QXmlStreamAttribute>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_back(self as *mut ::vector::VectorXmlStreamAttribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_capacity(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_clear(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute* QVector<QXmlStreamAttribute>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::xml_stream_attribute::XmlStreamAttribute {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constData(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constFirst(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constLast(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::contains(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_contains(self as *const ::vector::VectorXmlStreamAttribute,
                                                            t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::xml_stream_attribute::XmlStreamAttribute) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::count(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamAttributeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute* QVector<QXmlStreamAttribute>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::xml_stream_attribute::XmlStreamAttribute {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_data_const(self as *const ::vector::VectorXmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute* QVector<QXmlStreamAttribute>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::xml_stream_attribute::XmlStreamAttribute {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_data(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_empty(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::endsWith(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_endsWith(self as *const ::vector::VectorXmlStreamAttribute,
                                                            t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::xml_stream_attribute::XmlStreamAttribute) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::fill(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::xml_stream_attribute::XmlStreamAttribute, ::libc::c_int)) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::fill(const QXmlStreamAttribute& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_first_const(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QVector<QXmlStreamAttribute>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_first(self as *mut ::vector::VectorXmlStreamAttribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_front_const(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QVector<QXmlStreamAttribute>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_front(self as *mut ::vector::VectorXmlStreamAttribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::xml_stream_attribute::XmlStreamAttribute) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::indexOf(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::xml_stream_attribute::XmlStreamAttribute, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::indexOf(const QXmlStreamAttribute& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamAttributeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::xml_stream_attribute::XmlStreamAttribute)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::insert(int i, const QXmlStreamAttribute& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::xml_stream_attribute::XmlStreamAttribute)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::insert(int i, int n, const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamAttributeInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_isEmpty(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_last_const(self as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::xml_stream_attribute::XmlStreamAttribute) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::lastIndexOf(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::xml_stream_attribute::XmlStreamAttribute, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::lastIndexOf(const QXmlStreamAttribute& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamAttributeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QVector<QXmlStreamAttribute>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_last(self as *mut ::vector::VectorXmlStreamAttribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_length(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute> QVector<QXmlStreamAttribute>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute> QVector<QXmlStreamAttribute>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorXmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_move(self as *mut ::vector::VectorXmlStreamAttribute, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamAttribute>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorXmlStreamAttribute) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamAttribute>::QVector(const QVector<QXmlStreamAttribute>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamAttribute>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::xml_stream_attribute::XmlStreamAttribute)) -> ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamAttribute>::QVector(int size, const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorXmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute> QVector<QXmlStreamAttribute>::operator+(const QVector<QXmlStreamAttribute>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorXmlStreamAttribute) -> ::vector::VectorXmlStreamAttribute {
    {
      let mut object: ::vector::VectorXmlStreamAttribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_add_to_output(self as *const ::vector::VectorXmlStreamAttribute, l as *const ::vector::VectorXmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorXmlStreamAttribute) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::operator+=(const QVector<QXmlStreamAttribute>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::xml_stream_attribute::XmlStreamAttribute) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::operator+=(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::operator=(const QVector<QXmlStreamAttribute>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorXmlStreamAttribute)
                             -> &'l0 mut ::vector::VectorXmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_assign(self as *mut ::vector::VectorXmlStreamAttribute,
                                                                     v as *const ::vector::VectorXmlStreamAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::operator==(const QVector<QXmlStreamAttribute>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorXmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_eq(self as *const ::vector::VectorXmlStreamAttribute,
                                                               v as *const ::vector::VectorXmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamAttribute& QVector<QXmlStreamAttribute>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_index_const(self as *const ::vector::VectorXmlStreamAttribute, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute& QVector<QXmlStreamAttribute>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::xml_stream_attribute::XmlStreamAttribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_index(self as *mut ::vector::VectorXmlStreamAttribute, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::operator!=(const QVector<QXmlStreamAttribute>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorXmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_neq(self as *const ::vector::VectorXmlStreamAttribute,
                                                                v as *const ::vector::VectorXmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorXmlStreamAttribute) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::operator<<(const QVector<QXmlStreamAttribute>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::xml_stream_attribute::XmlStreamAttribute) -> &'l0 mut ::vector::VectorXmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>& QVector<QXmlStreamAttribute>::operator<<(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_pop_back(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_pop_front(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::prepend(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::xml_stream_attribute::XmlStreamAttribute) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_prepend(self as *mut ::vector::VectorXmlStreamAttribute,
                                                           t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::push_back(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::xml_stream_attribute::XmlStreamAttribute) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_push_back(self as *mut ::vector::VectorXmlStreamAttribute,
                                                             t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::push_front(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::xml_stream_attribute::XmlStreamAttribute) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_push_front(self as *mut ::vector::VectorXmlStreamAttribute,
                                                              t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamAttributeRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::removeAll(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::xml_stream_attribute::XmlStreamAttribute) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_removeAll(self as *mut ::vector::VectorXmlStreamAttribute,
                                                             t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_removeAt(self as *mut ::vector::VectorXmlStreamAttribute, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_removeFirst(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_removeLast(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::removeOne(const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_removeOne(self as *mut ::vector::VectorXmlStreamAttribute,
                                                             t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::replace(int i, const QXmlStreamAttribute& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::xml_stream_attribute::XmlStreamAttribute) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_replace(self as *mut ::vector::VectorXmlStreamAttribute,
                                                           i,
                                                           t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_reserve(self as *mut ::vector::VectorXmlStreamAttribute, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_resize(self as *mut ::vector::VectorXmlStreamAttribute, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamAttribute>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_size(self as *const ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_squeeze(self as *mut ::vector::VectorXmlStreamAttribute) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamAttribute>::startsWith(const QXmlStreamAttribute& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::xml_stream_attribute::XmlStreamAttribute) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_startsWith(self as *const ::vector::VectorXmlStreamAttribute,
                                                              t as *const ::xml_stream_attribute::XmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamAttribute>::swap(QVector<QXmlStreamAttribute>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorXmlStreamAttribute) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamAttribute_swap(self as *mut ::vector::VectorXmlStreamAttribute,
                                                        other as *mut ::vector::VectorXmlStreamAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute QVector<QXmlStreamAttribute>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::xml_stream_attribute::XmlStreamAttribute {
    {
      let mut object: ::xml_stream_attribute::XmlStreamAttribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_takeAt_to_output(self as *mut ::vector::VectorXmlStreamAttribute, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute QVector<QXmlStreamAttribute>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::xml_stream_attribute::XmlStreamAttribute {
    {
      let mut object: ::xml_stream_attribute::XmlStreamAttribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_takeFirst_to_output(self as *mut ::vector::VectorXmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute QVector<QXmlStreamAttribute>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::xml_stream_attribute::XmlStreamAttribute {
    {
      let mut object: ::xml_stream_attribute::XmlStreamAttribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_takeLast_to_output(self as *mut ::vector::VectorXmlStreamAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamAttribute>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute QVector<QXmlStreamAttribute>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::xml_stream_attribute::XmlStreamAttribute)) -> ::xml_stream_attribute::XmlStreamAttribute```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamAttribute QVector<QXmlStreamAttribute>::value(int i, const QXmlStreamAttribute& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::xml_stream_attribute::XmlStreamAttribute
    where Args: overloading::VectorXmlStreamAttributeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorXmlStreamAttribute {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QXmlStreamAttribute>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_destructor(self as *mut ::vector::VectorXmlStreamAttribute) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>```</span>
#[repr(C)]
pub struct VectorXmlStreamEntityDeclaration([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_XML_STREAM_ENTITY_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorXmlStreamEntityDeclaration {
  unsafe fn new_uninitialized() -> VectorXmlStreamEntityDeclaration {
    VectorXmlStreamEntityDeclaration(::std::mem::uninitialized())
  }
}

impl VectorXmlStreamEntityDeclaration {
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorXmlStreamEntityDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::append(const QVector<QXmlStreamEntityDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::append(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamEntityDeclarationAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_at(self as *const ::vector::VectorXmlStreamEntityDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_back_const(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_back(self as *mut ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_capacity(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_clear(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration* QVector<QXmlStreamEntityDeclaration>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constData(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constFirst(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constLast(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::contains(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_contains(self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::count(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamEntityDeclarationCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration* QVector<QXmlStreamEntityDeclaration>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_data_const(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration* QVector<QXmlStreamEntityDeclaration>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_data(self as *mut ::vector::VectorXmlStreamEntityDeclaration)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_empty(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::endsWith(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_endsWith(self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::fill(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, ::libc::c_int)) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::fill(const QXmlStreamEntityDeclaration& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_first_const(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_first(self as *mut ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_front_const(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_front(self as *mut ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::indexOf(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::xml_stream_entity_declaration::XmlStreamEntityDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::indexOf(const QXmlStreamEntityDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamEntityDeclarationIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::insert(int i, const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::insert(int i, int n, const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamEntityDeclarationInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_isEmpty(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_last_const(self as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::lastIndexOf(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::xml_stream_entity_declaration::XmlStreamEntityDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::lastIndexOf(const QXmlStreamEntityDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamEntityDeclarationLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_last(self as *mut ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_length(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration> QVector<QXmlStreamEntityDeclaration>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration> QVector<QXmlStreamEntityDeclaration>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorXmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_move(self as *mut ::vector::VectorXmlStreamEntityDeclaration, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamEntityDeclaration>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorXmlStreamEntityDeclaration) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamEntityDeclaration>::QVector(const QVector<QXmlStreamEntityDeclaration>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamEntityDeclaration>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration)) -> ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamEntityDeclaration>::QVector(int size, const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorXmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration> QVector<QXmlStreamEntityDeclaration>::operator+(const QVector<QXmlStreamEntityDeclaration>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorXmlStreamEntityDeclaration) -> ::vector::VectorXmlStreamEntityDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamEntityDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_to_output(self as *const ::vector::VectorXmlStreamEntityDeclaration, l as *const ::vector::VectorXmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorXmlStreamEntityDeclaration) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::operator+=(const QVector<QXmlStreamEntityDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::operator+=(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::operator=(const QVector<QXmlStreamEntityDeclaration>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorXmlStreamEntityDeclaration)
                             -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_assign(self as *mut ::vector::VectorXmlStreamEntityDeclaration, v as *const ::vector::VectorXmlStreamEntityDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::operator==(const QVector<QXmlStreamEntityDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorXmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_eq(self as *const ::vector::VectorXmlStreamEntityDeclaration, v as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self,
                       i: ::libc::c_int)
                       -> &'l0 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_index_const(self as *const ::vector::VectorXmlStreamEntityDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration& QVector<QXmlStreamEntityDeclaration>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_index(self as *mut ::vector::VectorXmlStreamEntityDeclaration, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::operator!=(const QVector<QXmlStreamEntityDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorXmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_neq(self as *const ::vector::VectorXmlStreamEntityDeclaration, v as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorXmlStreamEntityDeclaration) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::operator<<(const QVector<QXmlStreamEntityDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> &'l0 mut ::vector::VectorXmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>& QVector<QXmlStreamEntityDeclaration>::operator<<(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_pop_back(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_pop_front(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::prepend(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_prepend(self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::push_back(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_push_back(self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::push_front(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_push_front(self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamEntityDeclarationRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::removeAll(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_removeAll(self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_removeAt(self as *mut ::vector::VectorXmlStreamEntityDeclaration, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_removeFirst(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_removeLast(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::removeOne(const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_removeOne(self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::replace(int i, const QXmlStreamEntityDeclaration& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_replace(self as *mut ::vector::VectorXmlStreamEntityDeclaration, i, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_reserve(self as *mut ::vector::VectorXmlStreamEntityDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_resize(self as *mut ::vector::VectorXmlStreamEntityDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamEntityDeclaration>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_size(self as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_squeeze(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamEntityDeclaration>::startsWith(const QXmlStreamEntityDeclaration& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::xml_stream_entity_declaration::XmlStreamEntityDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_startsWith(self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamEntityDeclaration>::swap(QVector<QXmlStreamEntityDeclaration>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorXmlStreamEntityDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_swap(self as *mut ::vector::VectorXmlStreamEntityDeclaration, other as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration QVector<QXmlStreamEntityDeclaration>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    {
      let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_takeAt_to_output(self as *mut ::vector::VectorXmlStreamEntityDeclaration, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration QVector<QXmlStreamEntityDeclaration>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    {
      let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_takeFirst_to_output(self as *mut ::vector::VectorXmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration QVector<QXmlStreamEntityDeclaration>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    {
      let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_takeLast_to_output(self as *mut ::vector::VectorXmlStreamEntityDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamEntityDeclaration>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration QVector<QXmlStreamEntityDeclaration>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::xml_stream_entity_declaration::XmlStreamEntityDeclaration)) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamEntityDeclaration QVector<QXmlStreamEntityDeclaration>::value(int i, const QXmlStreamEntityDeclaration& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration
    where Args: overloading::VectorXmlStreamEntityDeclarationValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorXmlStreamEntityDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QXmlStreamEntityDeclaration>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_destructor(self as *mut ::vector::VectorXmlStreamEntityDeclaration) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>```</span>
#[repr(C)]
pub struct VectorXmlStreamNamespaceDeclaration([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_XML_STREAM_NAMESPACE_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorXmlStreamNamespaceDeclaration {
  unsafe fn new_uninitialized() -> VectorXmlStreamNamespaceDeclaration {
    VectorXmlStreamNamespaceDeclaration(::std::mem::uninitialized())
  }
}

impl VectorXmlStreamNamespaceDeclaration {
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorXmlStreamNamespaceDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::append(const QVector<QXmlStreamNamespaceDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::append(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNamespaceDeclarationAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self,
                 i: ::libc::c_int)
                 -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_at(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_back_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_back(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_capacity(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_clear(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration* QVector<QXmlStreamNamespaceDeclaration>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constData(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constFirst(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constLast(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::contains(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_contains(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::count(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNamespaceDeclarationCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration* QVector<QXmlStreamNamespaceDeclaration>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_data_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration* QVector<QXmlStreamNamespaceDeclaration>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_data(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_empty(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::endsWith(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_endsWith(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::fill(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, ::libc::c_int)) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::fill(const QXmlStreamNamespaceDeclaration& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_first_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_first(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_front_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_front(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::indexOf(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::indexOf(const QXmlStreamNamespaceDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNamespaceDeclarationIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::insert(int i, const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::insert(int i, int n, const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNamespaceDeclarationInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_isEmpty(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_last_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::lastIndexOf(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::lastIndexOf(const QXmlStreamNamespaceDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNamespaceDeclarationLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_last(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_length(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration> QVector<QXmlStreamNamespaceDeclaration>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration> QVector<QXmlStreamNamespaceDeclaration>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorXmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_move(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNamespaceDeclaration>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorXmlStreamNamespaceDeclaration) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNamespaceDeclaration>::QVector(const QVector<QXmlStreamNamespaceDeclaration>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNamespaceDeclaration>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration)) -> ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNamespaceDeclaration>::QVector(int size, const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorXmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration> QVector<QXmlStreamNamespaceDeclaration>::operator+(const QVector<QXmlStreamNamespaceDeclaration>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorXmlStreamNamespaceDeclaration)
                -> ::vector::VectorXmlStreamNamespaceDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_to_output(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, l as *const ::vector::VectorXmlStreamNamespaceDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorXmlStreamNamespaceDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::operator+=(const QVector<QXmlStreamNamespaceDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::operator+=(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::operator=(const QVector<QXmlStreamNamespaceDeclaration>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorXmlStreamNamespaceDeclaration)
                             -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_assign(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, v as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::operator==(const QVector<QXmlStreamNamespaceDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorXmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_eq(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, v as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self,
                       i: ::libc::c_int)
                       -> &'l0 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_index_const(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration& QVector<QXmlStreamNamespaceDeclaration>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_index(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::operator!=(const QVector<QXmlStreamNamespaceDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorXmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_neq(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, v as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorXmlStreamNamespaceDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::operator<<(const QVector<QXmlStreamNamespaceDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>& QVector<QXmlStreamNamespaceDeclaration>::operator<<(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_pop_back(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_pop_front(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::prepend(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_prepend(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::push_back(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_push_back(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::push_front(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_push_front(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNamespaceDeclarationRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::removeAll(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeAll(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeAt(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeFirst(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeLast(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::removeOne(const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_removeOne(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::replace(int i, const QXmlStreamNamespaceDeclaration& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_replace(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_reserve(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_resize(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNamespaceDeclaration>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_size(self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_squeeze(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNamespaceDeclaration>::startsWith(const QXmlStreamNamespaceDeclaration& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_startsWith(self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNamespaceDeclaration>::swap(QVector<QXmlStreamNamespaceDeclaration>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorXmlStreamNamespaceDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_swap(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, other as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration QVector<QXmlStreamNamespaceDeclaration>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    {
      let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeAt_to_output(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration QVector<QXmlStreamNamespaceDeclaration>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    {
      let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeFirst_to_output(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration QVector<QXmlStreamNamespaceDeclaration>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    {
      let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_takeLast_to_output(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNamespaceDeclaration>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration QVector<QXmlStreamNamespaceDeclaration>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration)) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamNamespaceDeclaration QVector<QXmlStreamNamespaceDeclaration>::value(int i, const QXmlStreamNamespaceDeclaration& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self,
                             args: Args)
                             -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration
    where Args: overloading::VectorXmlStreamNamespaceDeclarationValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorXmlStreamNamespaceDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QXmlStreamNamespaceDeclaration>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_destructor(self as *mut ::vector::VectorXmlStreamNamespaceDeclaration) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>```</span>
#[repr(C)]
pub struct VectorXmlStreamNotationDeclaration([u8; ::type_sizes::QT_CORE_VECTOR_VECTOR_XML_STREAM_NOTATION_DECLARATION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorXmlStreamNotationDeclaration {
  unsafe fn new_uninitialized() -> VectorXmlStreamNotationDeclaration {
    VectorXmlStreamNotationDeclaration(::std::mem::uninitialized())
  }
}

impl VectorXmlStreamNotationDeclaration {
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorXmlStreamNotationDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::append(const QVector<QXmlStreamNotationDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::append(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNotationDeclarationAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_at(self as *const ::vector::VectorXmlStreamNotationDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_back_const(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_back(self as *mut ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_capacity(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_clear(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration* QVector<QXmlStreamNotationDeclaration>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constData(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constFirst(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constLast(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::contains(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_contains(self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::count(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNotationDeclarationCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration* QVector<QXmlStreamNotationDeclaration>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_data_const(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration* QVector<QXmlStreamNotationDeclaration>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_data(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_empty(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::endsWith(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_endsWith(self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::fill(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, ::libc::c_int)) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::fill(const QXmlStreamNotationDeclaration& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_first_const(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_first(self as *mut ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_front_const(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_front(self as *mut ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::indexOf(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::xml_stream_notation_declaration::XmlStreamNotationDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::indexOf(const QXmlStreamNotationDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNotationDeclarationIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::insert(int i, const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::insert(int i, int n, const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNotationDeclarationInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_isEmpty(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_last_const(self as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::lastIndexOf(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::xml_stream_notation_declaration::XmlStreamNotationDeclaration, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::lastIndexOf(const QXmlStreamNotationDeclaration& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorXmlStreamNotationDeclarationLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_last(self as *mut ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_length(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration> QVector<QXmlStreamNotationDeclaration>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration> QVector<QXmlStreamNotationDeclaration>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorXmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_move(self as *mut ::vector::VectorXmlStreamNotationDeclaration, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNotationDeclaration>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorXmlStreamNotationDeclaration) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNotationDeclaration>::QVector(const QVector<QXmlStreamNotationDeclaration>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNotationDeclaration>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration)) -> ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QXmlStreamNotationDeclaration>::QVector(int size, const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorXmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration> QVector<QXmlStreamNotationDeclaration>::operator+(const QVector<QXmlStreamNotationDeclaration>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorXmlStreamNotationDeclaration)
                -> ::vector::VectorXmlStreamNotationDeclaration {
    {
      let mut object: ::vector::VectorXmlStreamNotationDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_to_output(self as *const ::vector::VectorXmlStreamNotationDeclaration, l as *const ::vector::VectorXmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorXmlStreamNotationDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::operator+=(const QVector<QXmlStreamNotationDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::operator+=(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::operator=(const QVector<QXmlStreamNotationDeclaration>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorXmlStreamNotationDeclaration)
                             -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_assign(self as *mut ::vector::VectorXmlStreamNotationDeclaration, v as *const ::vector::VectorXmlStreamNotationDeclaration) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::operator==(const QVector<QXmlStreamNotationDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorXmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_eq(self as *const ::vector::VectorXmlStreamNotationDeclaration, v as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```const QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self,
                       i: ::libc::c_int)
                       -> &'l0 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_index_const(self as *const ::vector::VectorXmlStreamNotationDeclaration, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration& QVector<QXmlStreamNotationDeclaration>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_index(self as *mut ::vector::VectorXmlStreamNotationDeclaration, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::operator!=(const QVector<QXmlStreamNotationDeclaration>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorXmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_neq(self as *const ::vector::VectorXmlStreamNotationDeclaration, v as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorXmlStreamNotationDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::operator<<(const QVector<QXmlStreamNotationDeclaration>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> &'l0 mut ::vector::VectorXmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>& QVector<QXmlStreamNotationDeclaration>::operator<<(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_pop_back(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_pop_front(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::prepend(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_prepend(self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::push_back(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_push_back(self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::push_front(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_push_front(self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorXmlStreamNotationDeclarationRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::removeAll(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_removeAll(self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_removeAt(self as *mut ::vector::VectorXmlStreamNotationDeclaration, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_removeFirst(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_removeLast(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::removeOne(const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_removeOne(self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::replace(int i, const QXmlStreamNotationDeclaration& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_replace(self as *mut ::vector::VectorXmlStreamNotationDeclaration, i, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_reserve(self as *mut ::vector::VectorXmlStreamNotationDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_resize(self as *mut ::vector::VectorXmlStreamNotationDeclaration, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QXmlStreamNotationDeclaration>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_size(self as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_squeeze(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QXmlStreamNotationDeclaration>::startsWith(const QXmlStreamNotationDeclaration& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::xml_stream_notation_declaration::XmlStreamNotationDeclaration) -> bool {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_startsWith(self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QXmlStreamNotationDeclaration>::swap(QVector<QXmlStreamNotationDeclaration>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorXmlStreamNotationDeclaration) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_swap(self as *mut ::vector::VectorXmlStreamNotationDeclaration, other as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration QVector<QXmlStreamNotationDeclaration>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    {
      let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_takeAt_to_output(self as *mut ::vector::VectorXmlStreamNotationDeclaration, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration QVector<QXmlStreamNotationDeclaration>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    {
      let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_takeFirst_to_output(self as *mut ::vector::VectorXmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration QVector<QXmlStreamNotationDeclaration>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    {
      let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_takeLast_to_output(self as *mut ::vector::VectorXmlStreamNotationDeclaration, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QXmlStreamNotationDeclaration>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration QVector<QXmlStreamNotationDeclaration>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::xml_stream_notation_declaration::XmlStreamNotationDeclaration)) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration```<br>
  /// C++ method: <span style='color: green;'>```QXmlStreamNotationDeclaration QVector<QXmlStreamNotationDeclaration>::value(int i, const QXmlStreamNotationDeclaration& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self,
                             args: Args)
                             -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration
    where Args: overloading::VectorXmlStreamNotationDeclarationValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorXmlStreamNotationDeclaration {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QXmlStreamNotationDeclaration>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_destructor(self as *mut ::vector::VectorXmlStreamNotationDeclaration) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorCInt::append](../struct.VectorCInt.html#method.append) method.
  pub trait VectorCIntAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> ();
  }
  impl<'largs> VectorCIntAppendArgs<'largs> for &'largs ::vector::VectorCInt {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_int_append_l(original_self as *mut ::vector::VectorCInt,
                                              l as *const ::vector::VectorCInt)
      }
    }
  }
  impl<'largs> VectorCIntAppendArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_int_append_t(original_self as *mut ::vector::VectorCInt,
                                              t as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::count](../struct.VectorCInt.html#method.count) method.
  pub trait VectorCIntCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int;
  }
  impl<'largs> VectorCIntCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_int_count_no_args(original_self as *const ::vector::VectorCInt) }
    }
  }
  impl<'largs> VectorCIntCountArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_int_count_t(original_self as *const ::vector::VectorCInt,
                                             t as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::fill](../struct.VectorCInt.html#method.fill) method.
  pub trait VectorCIntFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt;
  }
  impl<'largs> VectorCIntFillArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_fill_t(original_self as *mut ::vector::VectorCInt,
                                            t as *const ::libc::c_int)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCIntFillArgs<'largs> for (&'largs ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_fill_t_size(original_self as *mut ::vector::VectorCInt,
                                                 t as *const ::libc::c_int,
                                                 size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::index_of](../struct.VectorCInt.html#method.index_of) method.
  pub trait VectorCIntIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int;
  }
  impl<'largs> VectorCIntIndexOfArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_int_indexOf_t(original_self as *const ::vector::VectorCInt,
                                               t as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> VectorCIntIndexOfArgs<'largs> for (&'largs ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_int_indexOf_t_from(original_self as *const ::vector::VectorCInt,
                                                    t as *const ::libc::c_int,
                                                    from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::insert](../struct.VectorCInt.html#method.insert) method.
  pub trait VectorCIntInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> ();
  }
  impl<'largs> VectorCIntInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_core_c_QVector_int_insert_i_n_t(original_self as *mut ::vector::VectorCInt,
                                                  i,
                                                  n,
                                                  t as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> VectorCIntInsertArgs<'largs> for (::libc::c_int, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_int_insert_i_t(original_self as *mut ::vector::VectorCInt,
                                                i,
                                                t as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::last_index_of](../struct.VectorCInt.html#method.last_index_of) method.
  pub trait VectorCIntLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int;
  }
  impl<'largs> VectorCIntLastIndexOfArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_int_lastIndexOf_t(original_self as *const ::vector::VectorCInt,
                                                   t as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> VectorCIntLastIndexOfArgs<'largs> for (&'largs ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_int_lastIndexOf_t_from(original_self as *const ::vector::VectorCInt,
                                                        t as *const ::libc::c_int,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::mid](../struct.VectorCInt.html#method.mid) method.
  pub trait VectorCIntMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::vector::VectorCInt;
  }
  impl<'largs> VectorCIntMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::vector::VectorCInt {
      let pos = self;
      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_mid_to_output_pos(original_self as *const ::vector::VectorCInt,
                                                         pos,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorCIntMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::vector::VectorCInt {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_mid_to_output_pos_len(original_self as *const ::vector::VectorCInt,
                                                             pos,
                                                             len,
                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::new](../struct.VectorCInt.html#method.new) method.
  pub trait VectorCIntNewArgs {
    fn exec(self) -> ::vector::VectorCInt;
  }
  impl VectorCIntNewArgs for () {
    fn exec(self) -> ::vector::VectorCInt {

      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorCIntNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorCInt {
      let size = self;
      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCIntNewArgs for (::libc::c_int, &'a ::libc::c_int) {
    fn exec(self) -> ::vector::VectorCInt {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_constructor_size_t(size, t as *const ::libc::c_int, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCIntNewArgs for &'a ::vector::VectorCInt {
    fn exec(self) -> ::vector::VectorCInt {
      let v = self;
      {
        let mut object: ::vector::VectorCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_int_constructor_v(v as *const ::vector::VectorCInt, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::op_add_assign](../struct.VectorCInt.html#method.op_add_assign) method.
  pub trait VectorCIntOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt;
  }
  impl<'largs> VectorCIntOpAddAssignArgs<'largs> for &'largs ::vector::VectorCInt {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_operator_add_assign_l(original_self as *mut ::vector::VectorCInt,
                                                           l as *const ::vector::VectorCInt)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCIntOpAddAssignArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_operator_add_assign_t(original_self as *mut ::vector::VectorCInt,
                                                           t as *const ::libc::c_int)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::op_shl](../struct.VectorCInt.html#method.op_shl) method.
  pub trait VectorCIntOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt;
  }
  impl<'largs> VectorCIntOpShlArgs<'largs> for &'largs ::vector::VectorCInt {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_operator_shl_l(original_self as *mut ::vector::VectorCInt,
                                                    l as *const ::vector::VectorCInt)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCIntOpShlArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> &'largs mut ::vector::VectorCInt {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_int_operator_shl_t(original_self as *mut ::vector::VectorCInt,
                                                    t as *const ::libc::c_int)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::remove](../struct.VectorCInt.html#method.remove) method.
  pub trait VectorCIntRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> ();
  }
  impl<'largs> VectorCIntRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_int_remove_i(original_self as *mut ::vector::VectorCInt, i) }
    }
  }
  impl<'largs> VectorCIntRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCInt) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_int_remove_i_n(original_self as *mut ::vector::VectorCInt, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCInt::value](../struct.VectorCInt.html#method.value) method.
  pub trait VectorCIntValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int;
  }
  impl<'largs> VectorCIntValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_int_value_i(original_self as *const ::vector::VectorCInt, i) }
    }
  }
  impl<'largs> VectorCIntValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCInt) -> ::libc::c_int {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_int_value_i_defaultValue(original_self as *const ::vector::VectorCInt,
                                                          i,
                                                          default_value as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::append](../struct.VectorCUint.html#method.append) method.
  pub trait VectorCUintAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> ();
  }
  impl<'largs> VectorCUintAppendArgs<'largs> for &'largs ::vector::VectorCUint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_append_l(original_self as *mut ::vector::VectorCUint,
                                                       l as *const ::vector::VectorCUint)
      }
    }
  }
  impl<'largs> VectorCUintAppendArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_append_t(original_self as *mut ::vector::VectorCUint,
                                                       t as *const ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::count](../struct.VectorCUint.html#method.count) method.
  pub trait VectorCUintCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int;
  }
  impl<'largs> VectorCUintCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_count_no_args(original_self as *const ::vector::VectorCUint) }
    }
  }
  impl<'largs> VectorCUintCountArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_count_t(original_self as *const ::vector::VectorCUint,
                                                      t as *const ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::fill](../struct.VectorCUint.html#method.fill) method.
  pub trait VectorCUintFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint;
  }
  impl<'largs> VectorCUintFillArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_fill_t(original_self as *mut ::vector::VectorCUint,
                                                     t as *const ::libc::c_uint)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCUintFillArgs<'largs> for (&'largs ::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_fill_t_size(original_self as *mut ::vector::VectorCUint,
                                                          t as *const ::libc::c_uint,
                                                          size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::index_of](../struct.VectorCUint.html#method.index_of) method.
  pub trait VectorCUintIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int;
  }
  impl<'largs> VectorCUintIndexOfArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_indexOf_t(original_self as *const ::vector::VectorCUint,
                                                        t as *const ::libc::c_uint)
      }
    }
  }
  impl<'largs> VectorCUintIndexOfArgs<'largs> for (&'largs ::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_indexOf_t_from(original_self as *const ::vector::VectorCUint,
                                                             t as *const ::libc::c_uint,
                                                             from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::insert](../struct.VectorCUint.html#method.insert) method.
  pub trait VectorCUintInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> ();
  }
  impl<'largs> VectorCUintInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_insert_i_n_t(original_self as *mut ::vector::VectorCUint,
                                                           i,
                                                           n,
                                                           t as *const ::libc::c_uint)
      }
    }
  }
  impl<'largs> VectorCUintInsertArgs<'largs> for (::libc::c_int, &'largs ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_insert_i_t(original_self as *mut ::vector::VectorCUint,
                                                         i,
                                                         t as *const ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::last_index_of](../struct.VectorCUint.html#method.last_index_of) method.
  pub trait VectorCUintLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int;
  }
  impl<'largs> VectorCUintLastIndexOfArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_lastIndexOf_t(original_self as *const ::vector::VectorCUint,
                                                            t as *const ::libc::c_uint)
      }
    }
  }
  impl<'largs> VectorCUintLastIndexOfArgs<'largs> for (&'largs ::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_lastIndexOf_t_from(original_self as *const ::vector::VectorCUint,
                                                                 t as *const ::libc::c_uint,
                                                                 from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::mid](../struct.VectorCUint.html#method.mid) method.
  pub trait VectorCUintMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::vector::VectorCUint;
  }
  impl<'largs> VectorCUintMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::vector::VectorCUint {
      let pos = self;
      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_mid_to_output_pos(original_self as *const ::vector::VectorCUint,
                                                                  pos,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorCUintMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::vector::VectorCUint {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_mid_to_output_pos_len(original_self as *const ::vector::VectorCUint,
                                                                      pos,
                                                                      len,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::new](../struct.VectorCUint.html#method.new) method.
  pub trait VectorCUintNewArgs {
    fn exec(self) -> ::vector::VectorCUint;
  }
  impl VectorCUintNewArgs for () {
    fn exec(self) -> ::vector::VectorCUint {

      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorCUintNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorCUint {
      let size = self;
      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCUintNewArgs for (::libc::c_int, &'a ::libc::c_uint) {
    fn exec(self) -> ::vector::VectorCUint {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_constructor_size_t(size, t as *const ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCUintNewArgs for &'a ::vector::VectorCUint {
    fn exec(self) -> ::vector::VectorCUint {
      let v = self;
      {
        let mut object: ::vector::VectorCUint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_constructor_v(v as *const ::vector::VectorCUint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::op_add_assign](../struct.VectorCUint.html#method.op_add_assign) method.
  pub trait VectorCUintOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint;
  }
  impl<'largs> VectorCUintOpAddAssignArgs<'largs> for &'largs ::vector::VectorCUint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_operator_add_assign_l(original_self as *mut ::vector::VectorCUint,
                                                                      l as *const ::vector::VectorCUint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCUintOpAddAssignArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_operator_add_assign_t(original_self as *mut ::vector::VectorCUint,
                                                                      t as *const ::libc::c_uint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::op_shl](../struct.VectorCUint.html#method.op_shl) method.
  pub trait VectorCUintOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint;
  }
  impl<'largs> VectorCUintOpShlArgs<'largs> for &'largs ::vector::VectorCUint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_operator_shl_l(original_self as *mut ::vector::VectorCUint,
                                                               l as *const ::vector::VectorCUint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCUintOpShlArgs<'largs> for &'largs ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> &'largs mut ::vector::VectorCUint {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_unsigned_int_operator_shl_t(original_self as *mut ::vector::VectorCUint,
                                                               t as *const ::libc::c_uint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::remove](../struct.VectorCUint.html#method.remove) method.
  pub trait VectorCUintRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> ();
  }
  impl<'largs> VectorCUintRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_remove_i(original_self as *mut ::vector::VectorCUint, i) }
    }
  }
  impl<'largs> VectorCUintRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCUint) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_remove_i_n(original_self as *mut ::vector::VectorCUint, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCUint::value](../struct.VectorCUint.html#method.value) method.
  pub trait VectorCUintValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_uint;
  }
  impl<'largs> VectorCUintValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_uint {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_unsigned_int_value_i(original_self as *const ::vector::VectorCUint, i) }
    }
  }
  impl<'largs> VectorCUintValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_uint) {
    fn exec(self, original_self: &'largs ::vector::VectorCUint) -> ::libc::c_uint {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_unsigned_int_value_i_defaultValue(original_self as *const ::vector::VectorCUint,
                                                                   i,
                                                                   default_value as *const ::libc::c_uint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::append](../struct.VectorPairPairCDoubleVariant.html#method.append) method.
  pub trait VectorPairPairCDoubleVariantAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> ();
  }
  impl<'largs> VectorPairPairCDoubleVariantAppendArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleVariant {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let l = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_append_l(original_self as *mut ::vector::VectorPairPairCDoubleVariant, l as *const ::vector::VectorPairPairCDoubleVariant) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantAppendArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_append_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::count](../struct.VectorPairPairCDoubleVariant.html#method.count) method.
  pub trait VectorPairPairCDoubleVariantCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleVariantCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_count_no_args(original_self as *const ::vector::VectorPairPairCDoubleVariant) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantCountArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_count_t(original_self as *const ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::fill](../struct.VectorPairPairCDoubleVariant.html#method.fill) method.
  pub trait VectorPairPairCDoubleVariantFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant;
  }
  impl<'largs> VectorPairPairCDoubleVariantFillArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_fill_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantFillArgs<'largs> for (&'largs ::pair::PairCDoubleVariant, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_fill_t_size(original_self as *mut ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::index_of](../struct.VectorPairPairCDoubleVariant.html#method.index_of) method.
  pub trait VectorPairPairCDoubleVariantIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleVariantIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_indexOf_t(original_self as *const ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantIndexOfArgs<'largs> for (&'largs ::pair::PairCDoubleVariant, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_indexOf_t_from(original_self as *const ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::insert](../struct.VectorPairPairCDoubleVariant.html#method.insert) method.
  pub trait VectorPairPairCDoubleVariantInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> ();
  }
  impl<'largs> VectorPairPairCDoubleVariantInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::pair::PairCDoubleVariant) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_insert_i_n_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, i, n, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantInsertArgs<'largs> for (::libc::c_int, &'largs ::pair::PairCDoubleVariant) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_insert_i_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, i, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::last_index_of](../struct.VectorPairPairCDoubleVariant.html#method.last_index_of) method.
  pub trait VectorPairPairCDoubleVariantLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleVariantLastIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_lastIndexOf_t(original_self as *const ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantLastIndexOfArgs<'largs>
    for (&'largs ::pair::PairCDoubleVariant, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_lastIndexOf_t_from(original_self as *const ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::mid](../struct.VectorPairPairCDoubleVariant.html#method.mid) method.
  pub trait VectorPairPairCDoubleVariantMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorPairPairCDoubleVariant)
            -> ::vector::VectorPairPairCDoubleVariant;
  }
  impl<'largs> VectorPairPairCDoubleVariantMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorPairPairCDoubleVariant)
            -> ::vector::VectorPairPairCDoubleVariant {
      let pos = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_mid_to_output_pos(original_self as *const ::vector::VectorPairPairCDoubleVariant, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorPairPairCDoubleVariant)
            -> ::vector::VectorPairPairCDoubleVariant {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_mid_to_output_pos_len(original_self as *const ::vector::VectorPairPairCDoubleVariant, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::new](../struct.VectorPairPairCDoubleVariant.html#method.new) method.
  pub trait VectorPairPairCDoubleVariantNewArgs {
    fn exec(self) -> ::vector::VectorPairPairCDoubleVariant;
  }
  impl VectorPairPairCDoubleVariantNewArgs for () {
    fn exec(self) -> ::vector::VectorPairPairCDoubleVariant {

      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorPairPairCDoubleVariantNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorPairPairCDoubleVariant {
      let size = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPairPairCDoubleVariantNewArgs for (::libc::c_int, &'a ::pair::PairCDoubleVariant) {
    fn exec(self) -> ::vector::VectorPairPairCDoubleVariant {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_constructor_size_t(size,
                                                                            t as *const ::pair::PairCDoubleVariant,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPairPairCDoubleVariantNewArgs for &'a ::vector::VectorPairPairCDoubleVariant {
    fn exec(self) -> ::vector::VectorPairPairCDoubleVariant {
      let v = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_constructor_v(v as *const ::vector::VectorPairPairCDoubleVariant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::op_add_assign](../struct.VectorPairPairCDoubleVariant.html#method.op_add_assign) method.
  pub trait VectorPairPairCDoubleVariantOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant;
  }
  impl<'largs> VectorPairPairCDoubleVariantOpAddAssignArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleVariant {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_add_assign_l(original_self as *mut ::vector::VectorPairPairCDoubleVariant, l as *const ::vector::VectorPairPairCDoubleVariant) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantOpAddAssignArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_add_assign_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::op_shl](../struct.VectorPairPairCDoubleVariant.html#method.op_shl) method.
  pub trait VectorPairPairCDoubleVariantOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant;
  }
  impl<'largs> VectorPairPairCDoubleVariantOpShlArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleVariant {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_shl_l(original_self as *mut ::vector::VectorPairPairCDoubleVariant, l as *const ::vector::VectorPairPairCDoubleVariant) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantOpShlArgs<'largs> for &'largs ::pair::PairCDoubleVariant {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant)
            -> &'largs mut ::vector::VectorPairPairCDoubleVariant {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_operator_shl_t(original_self as *mut ::vector::VectorPairPairCDoubleVariant, t as *const ::pair::PairCDoubleVariant) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::remove](../struct.VectorPairPairCDoubleVariant.html#method.remove) method.
  pub trait VectorPairPairCDoubleVariantRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> ();
  }
  impl<'largs> VectorPairPairCDoubleVariantRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_remove_i(original_self as *mut ::vector::VectorPairPairCDoubleVariant, i) }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleVariant) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QPair_double_QVariant_remove_i_n(original_self as *mut ::vector::VectorPairPairCDoubleVariant, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleVariant::value](../struct.VectorPairPairCDoubleVariant.html#method.value) method.
  pub trait VectorPairPairCDoubleVariantValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::pair::PairCDoubleVariant;
  }
  impl<'largs> VectorPairPairCDoubleVariantValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::pair::PairCDoubleVariant {
      let i = self;
      {
        let mut object: ::pair::PairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_value_to_output_i(original_self as *const ::vector::VectorPairPairCDoubleVariant, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleVariantValueArgs<'largs> for (::libc::c_int, &'largs ::pair::PairCDoubleVariant) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleVariant) -> ::pair::PairCDoubleVariant {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::pair::PairCDoubleVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPair_double_QVariant_value_to_output_i_defaultValue(original_self as *const ::vector::VectorPairPairCDoubleVariant, i, default_value as *const ::pair::PairCDoubleVariant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::append](../struct.VectorPointF.html#method.append) method.
  pub trait VectorPointFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> ();
  }
  impl<'largs> VectorPointFAppendArgs<'largs> for &'largs ::vector::VectorPointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_append_l(original_self as *mut ::vector::VectorPointF,
                                                  l as *const ::vector::VectorPointF)
      }
    }
  }
  impl<'largs> VectorPointFAppendArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_append_t(original_self as *mut ::vector::VectorPointF,
                                                  t as *const ::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::count](../struct.VectorPointF.html#method.count) method.
  pub trait VectorPointFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int;
  }
  impl<'largs> VectorPointFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QPointF_count_no_args(original_self as *const ::vector::VectorPointF) }
    }
  }
  impl<'largs> VectorPointFCountArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_count_t(original_self as *const ::vector::VectorPointF,
                                                 t as *const ::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::fill](../struct.VectorPointF.html#method.fill) method.
  pub trait VectorPointFFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF;
  }
  impl<'largs> VectorPointFFillArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_QPointF_fill_t(original_self as *mut ::vector::VectorPointF,
                                                t as *const ::point_f::PointF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPointFFillArgs<'largs> for (&'largs ::point_f::PointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_QPointF_fill_t_size(original_self as *mut ::vector::VectorPointF,
                                                     t as *const ::point_f::PointF,
                                                     size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::index_of](../struct.VectorPointF.html#method.index_of) method.
  pub trait VectorPointFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int;
  }
  impl<'largs> VectorPointFIndexOfArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_indexOf_t(original_self as *const ::vector::VectorPointF,
                                                   t as *const ::point_f::PointF)
      }
    }
  }
  impl<'largs> VectorPointFIndexOfArgs<'largs> for (&'largs ::point_f::PointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_indexOf_t_from(original_self as *const ::vector::VectorPointF,
                                                        t as *const ::point_f::PointF,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::insert](../struct.VectorPointF.html#method.insert) method.
  pub trait VectorPointFInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> ();
  }
  impl<'largs> VectorPointFInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_insert_i_n_t(original_self as *mut ::vector::VectorPointF,
                                                      i,
                                                      n,
                                                      t as *const ::point_f::PointF)
      }
    }
  }
  impl<'largs> VectorPointFInsertArgs<'largs> for (::libc::c_int, &'largs ::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_insert_i_t(original_self as *mut ::vector::VectorPointF,
                                                    i,
                                                    t as *const ::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::last_index_of](../struct.VectorPointF.html#method.last_index_of) method.
  pub trait VectorPointFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int;
  }
  impl<'largs> VectorPointFLastIndexOfArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_lastIndexOf_t(original_self as *const ::vector::VectorPointF,
                                                       t as *const ::point_f::PointF)
      }
    }
  }
  impl<'largs> VectorPointFLastIndexOfArgs<'largs> for (&'largs ::point_f::PointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QPointF_lastIndexOf_t_from(original_self as *const ::vector::VectorPointF,
                                                            t as *const ::point_f::PointF,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::mid](../struct.VectorPointF.html#method.mid) method.
  pub trait VectorPointFMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::vector::VectorPointF;
  }
  impl<'largs> VectorPointFMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::vector::VectorPointF {
      let pos = self;
      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_mid_to_output_pos(original_self as *const ::vector::VectorPointF,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPointFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::vector::VectorPointF {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_mid_to_output_pos_len(original_self as *const ::vector::VectorPointF,
                                                                 pos,
                                                                 len,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::new](../struct.VectorPointF.html#method.new) method.
  pub trait VectorPointFNewArgs {
    fn exec(self) -> ::vector::VectorPointF;
  }
  impl VectorPointFNewArgs for () {
    fn exec(self) -> ::vector::VectorPointF {

      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorPointFNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorPointF {
      let size = self;
      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPointFNewArgs for (::libc::c_int, &'a ::point_f::PointF) {
    fn exec(self) -> ::vector::VectorPointF {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_constructor_size_t(size, t as *const ::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPointFNewArgs for &'a ::vector::VectorPointF {
    fn exec(self) -> ::vector::VectorPointF {
      let v = self;
      {
        let mut object: ::vector::VectorPointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_constructor_v(v as *const ::vector::VectorPointF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::op_add_assign](../struct.VectorPointF.html#method.op_add_assign) method.
  pub trait VectorPointFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF;
  }
  impl<'largs> VectorPointFOpAddAssignArgs<'largs> for &'largs ::vector::VectorPointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_operator_add_assign_l(original_self as *mut ::vector::VectorPointF,
                                                                 l as *const ::vector::VectorPointF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPointFOpAddAssignArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_operator_add_assign_t(original_self as *mut ::vector::VectorPointF,
                                                                 t as *const ::point_f::PointF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::op_shl](../struct.VectorPointF.html#method.op_shl) method.
  pub trait VectorPointFOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF;
  }
  impl<'largs> VectorPointFOpShlArgs<'largs> for &'largs ::vector::VectorPointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_QPointF_operator_shl_l(original_self as *mut ::vector::VectorPointF,
                                                        l as *const ::vector::VectorPointF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPointFOpShlArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> &'largs mut ::vector::VectorPointF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_QPointF_operator_shl_t(original_self as *mut ::vector::VectorPointF,
                                                        t as *const ::point_f::PointF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::remove](../struct.VectorPointF.html#method.remove) method.
  pub trait VectorPointFRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> ();
  }
  impl<'largs> VectorPointFRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QPointF_remove_i(original_self as *mut ::vector::VectorPointF, i) }
    }
  }
  impl<'largs> VectorPointFRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPointF) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QPointF_remove_i_n(original_self as *mut ::vector::VectorPointF, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPointF::value](../struct.VectorPointF.html#method.value) method.
  pub trait VectorPointFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::point_f::PointF;
  }
  impl<'largs> VectorPointFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::point_f::PointF {
      let i = self;
      {
        let mut object: ::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_value_to_output_i(original_self as *const ::vector::VectorPointF,
                                                             i,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPointFValueArgs<'largs> for (::libc::c_int, &'largs ::point_f::PointF) {
    fn exec(self, original_self: &'largs ::vector::VectorPointF) -> ::point_f::PointF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QPointF_value_to_output_i_defaultValue(original_self as *const ::vector::VectorPointF, i, default_value as *const ::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::append](../struct.VectorStaticPlugin.html#method.append) method.
  pub trait VectorStaticPluginAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> ();
  }
  impl<'largs> VectorStaticPluginAppendArgs<'largs> for &'largs ::vector::VectorStaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_append_l(original_self as *mut ::vector::VectorStaticPlugin,
                                                        l as *const ::vector::VectorStaticPlugin)
      }
    }
  }
  impl<'largs> VectorStaticPluginAppendArgs<'largs> for &'largs ::static_plugin::StaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_append_t(original_self as *mut ::vector::VectorStaticPlugin,
                                                        t as *const ::static_plugin::StaticPlugin)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::fill](../struct.VectorStaticPlugin.html#method.fill) method.
  pub trait VectorStaticPluginFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin;
  }
  impl<'largs> VectorStaticPluginFillArgs<'largs> for &'largs ::static_plugin::StaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_fill_t(original_self as *mut ::vector::VectorStaticPlugin,
                                                        t as *const ::static_plugin::StaticPlugin)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStaticPluginFillArgs<'largs> for (&'largs ::static_plugin::StaticPlugin, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_fill_t_size(original_self as *mut ::vector::VectorStaticPlugin,
                                                             t as *const ::static_plugin::StaticPlugin,
                                                             size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::insert](../struct.VectorStaticPlugin.html#method.insert) method.
  pub trait VectorStaticPluginInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> ();
  }
  impl<'largs> VectorStaticPluginInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::static_plugin::StaticPlugin) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_insert_i_n_t(original_self as *mut ::vector::VectorStaticPlugin,
                                                            i,
                                                            n,
                                                            t as *const ::static_plugin::StaticPlugin)
      }
    }
  }
  impl<'largs> VectorStaticPluginInsertArgs<'largs> for (::libc::c_int, &'largs ::static_plugin::StaticPlugin) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_insert_i_t(original_self as *mut ::vector::VectorStaticPlugin,
                                                          i,
                                                          t as *const ::static_plugin::StaticPlugin)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::mid](../struct.VectorStaticPlugin.html#method.mid) method.
  pub trait VectorStaticPluginMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::vector::VectorStaticPlugin;
  }
  impl<'largs> VectorStaticPluginMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::vector::VectorStaticPlugin {
      let pos = self;
      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_mid_to_output_pos(original_self as *const ::vector::VectorStaticPlugin, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorStaticPluginMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::vector::VectorStaticPlugin {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_mid_to_output_pos_len(original_self as *const ::vector::VectorStaticPlugin, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::new](../struct.VectorStaticPlugin.html#method.new) method.
  pub trait VectorStaticPluginNewArgs {
    fn exec(self) -> ::vector::VectorStaticPlugin;
  }
  impl VectorStaticPluginNewArgs for () {
    fn exec(self) -> ::vector::VectorStaticPlugin {

      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorStaticPluginNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorStaticPlugin {
      let size = self;
      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorStaticPluginNewArgs for (::libc::c_int, &'a ::static_plugin::StaticPlugin) {
    fn exec(self) -> ::vector::VectorStaticPlugin {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_constructor_size_t(size,
                                                                    t as *const ::static_plugin::StaticPlugin,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorStaticPluginNewArgs for &'a ::vector::VectorStaticPlugin {
    fn exec(self) -> ::vector::VectorStaticPlugin {
      let v = self;
      {
        let mut object: ::vector::VectorStaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_constructor_v(v as *const ::vector::VectorStaticPlugin, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::op_add_assign](../struct.VectorStaticPlugin.html#method.op_add_assign) method.
  pub trait VectorStaticPluginOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin;
  }
  impl<'largs> VectorStaticPluginOpAddAssignArgs<'largs> for &'largs ::vector::VectorStaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_operator_add_assign_l(original_self as *mut ::vector::VectorStaticPlugin, l as *const ::vector::VectorStaticPlugin) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStaticPluginOpAddAssignArgs<'largs> for &'largs ::static_plugin::StaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_operator_add_assign_t(original_self as *mut ::vector::VectorStaticPlugin, t as *const ::static_plugin::StaticPlugin) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::op_shl](../struct.VectorStaticPlugin.html#method.op_shl) method.
  pub trait VectorStaticPluginOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin;
  }
  impl<'largs> VectorStaticPluginOpShlArgs<'largs> for &'largs ::vector::VectorStaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_operator_shl_l(original_self as *mut ::vector::VectorStaticPlugin,
                                                                l as *const ::vector::VectorStaticPlugin)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStaticPluginOpShlArgs<'largs> for &'largs ::static_plugin::StaticPlugin {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> &'largs mut ::vector::VectorStaticPlugin {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_operator_shl_t(original_self as *mut ::vector::VectorStaticPlugin,
                                                                t as *const ::static_plugin::StaticPlugin)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::remove](../struct.VectorStaticPlugin.html#method.remove) method.
  pub trait VectorStaticPluginRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> ();
  }
  impl<'largs> VectorStaticPluginRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QStaticPlugin_remove_i(original_self as *mut ::vector::VectorStaticPlugin, i) }
    }
  }
  impl<'largs> VectorStaticPluginRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStaticPlugin) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QStaticPlugin_remove_i_n(original_self as *mut ::vector::VectorStaticPlugin, i, n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStaticPlugin::value](../struct.VectorStaticPlugin.html#method.value) method.
  pub trait VectorStaticPluginValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::static_plugin::StaticPlugin;
  }
  impl<'largs> VectorStaticPluginValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::static_plugin::StaticPlugin {
      let i = self;
      {
        let mut object: ::static_plugin::StaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_value_to_output_i(original_self as *const ::vector::VectorStaticPlugin, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorStaticPluginValueArgs<'largs> for (::libc::c_int, &'largs ::static_plugin::StaticPlugin) {
    fn exec(self, original_self: &'largs ::vector::VectorStaticPlugin) -> ::static_plugin::StaticPlugin {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::static_plugin::StaticPlugin =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStaticPlugin_value_to_output_i_defaultValue(original_self as *const ::vector::VectorStaticPlugin, i, default_value as *const ::static_plugin::StaticPlugin, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::append](../struct.VectorStringRef.html#method.append) method.
  pub trait VectorStringRefAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> ();
  }
  impl<'largs> VectorStringRefAppendArgs<'largs> for &'largs ::vector::VectorStringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_append_l(original_self as *mut ::vector::VectorStringRef,
                                                     l as *const ::vector::VectorStringRef)
      }
    }
  }
  impl<'largs> VectorStringRefAppendArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_append_t(original_self as *mut ::vector::VectorStringRef,
                                                     t as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::count](../struct.VectorStringRef.html#method.count) method.
  pub trait VectorStringRefCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int;
  }
  impl<'largs> VectorStringRefCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QStringRef_count_no_args(original_self as *const ::vector::VectorStringRef) }
    }
  }
  impl<'largs> VectorStringRefCountArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_count_t(original_self as *const ::vector::VectorStringRef,
                                                    t as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::fill](../struct.VectorStringRef.html#method.fill) method.
  pub trait VectorStringRefFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef;
  }
  impl<'largs> VectorStringRefFillArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_fill_t(original_self as *mut ::vector::VectorStringRef,
                                                   t as *const ::string_ref::StringRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStringRefFillArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_fill_t_size(original_self as *mut ::vector::VectorStringRef,
                                                          t as *const ::string_ref::StringRef,
                                                          size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::index_of](../struct.VectorStringRef.html#method.index_of) method.
  pub trait VectorStringRefIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int;
  }
  impl<'largs> VectorStringRefIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_indexOf_t(original_self as *const ::vector::VectorStringRef,
                                                      t as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> VectorStringRefIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_indexOf_t_from(original_self as *const ::vector::VectorStringRef,
                                                           t as *const ::string_ref::StringRef,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::insert](../struct.VectorStringRef.html#method.insert) method.
  pub trait VectorStringRefInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> ();
  }
  impl<'largs> VectorStringRefInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::string_ref::StringRef) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_insert_i_n_t(original_self as *mut ::vector::VectorStringRef,
                                                         i,
                                                         n,
                                                         t as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> VectorStringRefInsertArgs<'largs> for (::libc::c_int, &'largs ::string_ref::StringRef) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_insert_i_t(original_self as *mut ::vector::VectorStringRef,
                                                       i,
                                                       t as *const ::string_ref::StringRef)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::last_index_of](../struct.VectorStringRef.html#method.last_index_of) method.
  pub trait VectorStringRefLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int;
  }
  impl<'largs> VectorStringRefLastIndexOfArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_lastIndexOf_t(original_self as *const ::vector::VectorStringRef,
                                                          t as *const ::string_ref::StringRef)
      }
    }
  }
  impl<'largs> VectorStringRefLastIndexOfArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QVector_QStringRef_lastIndexOf_t_from(original_self as *const ::vector::VectorStringRef,
                                                               t as *const ::string_ref::StringRef,
                                                               from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::mid](../struct.VectorStringRef.html#method.mid) method.
  pub trait VectorStringRefMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::vector::VectorStringRef;
  }
  impl<'largs> VectorStringRefMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::vector::VectorStringRef {
      let pos = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_mid_to_output_pos(original_self as *const ::vector::VectorStringRef,
                                                                pos,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorStringRefMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::vector::VectorStringRef {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_mid_to_output_pos_len(original_self as *const ::vector::VectorStringRef, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::new](../struct.VectorStringRef.html#method.new) method.
  pub trait VectorStringRefNewArgs {
    fn exec(self) -> ::vector::VectorStringRef;
  }
  impl VectorStringRefNewArgs for () {
    fn exec(self) -> ::vector::VectorStringRef {

      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorStringRefNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorStringRef {
      let size = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorStringRefNewArgs for (::libc::c_int, &'a ::string_ref::StringRef) {
    fn exec(self) -> ::vector::VectorStringRef {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_constructor_size_t(size,
                                                                 t as *const ::string_ref::StringRef,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorStringRefNewArgs for &'a ::vector::VectorStringRef {
    fn exec(self) -> ::vector::VectorStringRef {
      let v = self;
      {
        let mut object: ::vector::VectorStringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_constructor_v(v as *const ::vector::VectorStringRef, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::op_add_assign](../struct.VectorStringRef.html#method.op_add_assign) method.
  pub trait VectorStringRefOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef;
  }
  impl<'largs> VectorStringRefOpAddAssignArgs<'largs> for &'largs ::vector::VectorStringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_operator_add_assign_l(original_self as *mut ::vector::VectorStringRef,
                                                                    l as *const ::vector::VectorStringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStringRefOpAddAssignArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_operator_add_assign_t(original_self as *mut ::vector::VectorStringRef,
                                                                    t as *const ::string_ref::StringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::op_shl](../struct.VectorStringRef.html#method.op_shl) method.
  pub trait VectorStringRefOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef;
  }
  impl<'largs> VectorStringRefOpShlArgs<'largs> for &'largs ::vector::VectorStringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_operator_shl_l(original_self as *mut ::vector::VectorStringRef,
                                                             l as *const ::vector::VectorStringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorStringRefOpShlArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> &'largs mut ::vector::VectorStringRef {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_operator_shl_t(original_self as *mut ::vector::VectorStringRef,
                                                             t as *const ::string_ref::StringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::remove](../struct.VectorStringRef.html#method.remove) method.
  pub trait VectorStringRefRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> ();
  }
  impl<'largs> VectorStringRefRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_remove_i(original_self as *mut ::vector::VectorStringRef, i) }
    }
  }
  impl<'largs> VectorStringRefRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorStringRef) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QStringRef_remove_i_n(original_self as *mut ::vector::VectorStringRef, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorStringRef::value](../struct.VectorStringRef.html#method.value) method.
  pub trait VectorStringRefValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::string_ref::StringRef;
  }
  impl<'largs> VectorStringRefValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::string_ref::StringRef {
      let i = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_value_to_output_i(original_self as *const ::vector::VectorStringRef,
                                                                i,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorStringRefValueArgs<'largs> for (::libc::c_int, &'largs ::string_ref::StringRef) {
    fn exec(self, original_self: &'largs ::vector::VectorStringRef) -> ::string_ref::StringRef {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QStringRef_value_to_output_i_defaultValue(original_self as *const ::vector::VectorStringRef, i, default_value as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::append](../struct.VectorTimeZoneOffsetData.html#method.append) method.
  pub trait VectorTimeZoneOffsetDataAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> ();
  }
  impl<'largs> VectorTimeZoneOffsetDataAppendArgs<'largs> for &'largs ::vector::VectorTimeZoneOffsetData {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_append_l(original_self as *mut ::vector::VectorTimeZoneOffsetData, l as *const ::vector::VectorTimeZoneOffsetData)
      }
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataAppendArgs<'largs> for &'largs ::time_zone::OffsetData {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_append_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, t as *const ::time_zone::OffsetData)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::fill](../struct.VectorTimeZoneOffsetData.html#method.fill) method.
  pub trait VectorTimeZoneOffsetDataFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData;
  }
  impl<'largs> VectorTimeZoneOffsetDataFillArgs<'largs> for &'largs ::time_zone::OffsetData {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_fill_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, t as *const ::time_zone::OffsetData)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataFillArgs<'largs> for (&'largs ::time_zone::OffsetData, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_fill_t_size(original_self as *mut ::vector::VectorTimeZoneOffsetData, t as *const ::time_zone::OffsetData, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::insert](../struct.VectorTimeZoneOffsetData.html#method.insert) method.
  pub trait VectorTimeZoneOffsetDataInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> ();
  }
  impl<'largs> VectorTimeZoneOffsetDataInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::time_zone::OffsetData) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_insert_i_n_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, i, n, t as *const ::time_zone::OffsetData) }
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataInsertArgs<'largs> for (::libc::c_int, &'largs ::time_zone::OffsetData) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_insert_i_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, i, t as *const ::time_zone::OffsetData) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::mid](../struct.VectorTimeZoneOffsetData.html#method.mid) method.
  pub trait VectorTimeZoneOffsetDataMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::vector::VectorTimeZoneOffsetData;
  }
  impl<'largs> VectorTimeZoneOffsetDataMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::vector::VectorTimeZoneOffsetData {
      let pos = self;
      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_mid_to_output_pos(original_self as *const ::vector::VectorTimeZoneOffsetData, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::vector::VectorTimeZoneOffsetData {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_mid_to_output_pos_len(original_self as *const ::vector::VectorTimeZoneOffsetData, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::new](../struct.VectorTimeZoneOffsetData.html#method.new) method.
  pub trait VectorTimeZoneOffsetDataNewArgs {
    fn exec(self) -> ::vector::VectorTimeZoneOffsetData;
  }
  impl VectorTimeZoneOffsetDataNewArgs for () {
    fn exec(self) -> ::vector::VectorTimeZoneOffsetData {

      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorTimeZoneOffsetDataNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorTimeZoneOffsetData {
      let size = self;
      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTimeZoneOffsetDataNewArgs for (::libc::c_int, &'a ::time_zone::OffsetData) {
    fn exec(self) -> ::vector::VectorTimeZoneOffsetData {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constructor_size_t(size,
                                                                           t as *const ::time_zone::OffsetData,
                                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTimeZoneOffsetDataNewArgs for &'a ::vector::VectorTimeZoneOffsetData {
    fn exec(self) -> ::vector::VectorTimeZoneOffsetData {
      let v = self;
      {
        let mut object: ::vector::VectorTimeZoneOffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_constructor_v(v as *const ::vector::VectorTimeZoneOffsetData,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::op_add_assign](../struct.VectorTimeZoneOffsetData.html#method.op_add_assign) method.
  pub trait VectorTimeZoneOffsetDataOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData;
  }
  impl<'largs> VectorTimeZoneOffsetDataOpAddAssignArgs<'largs> for &'largs ::vector::VectorTimeZoneOffsetData {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_add_assign_l(original_self as *mut ::vector::VectorTimeZoneOffsetData, l as *const ::vector::VectorTimeZoneOffsetData) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataOpAddAssignArgs<'largs> for &'largs ::time_zone::OffsetData {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_add_assign_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, t as *const ::time_zone::OffsetData) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::op_shl](../struct.VectorTimeZoneOffsetData.html#method.op_shl) method.
  pub trait VectorTimeZoneOffsetDataOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData;
  }
  impl<'largs> VectorTimeZoneOffsetDataOpShlArgs<'largs> for &'largs ::vector::VectorTimeZoneOffsetData {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_shl_l(original_self as *mut ::vector::VectorTimeZoneOffsetData, l as *const ::vector::VectorTimeZoneOffsetData) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataOpShlArgs<'largs> for &'largs ::time_zone::OffsetData {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTimeZoneOffsetData)
            -> &'largs mut ::vector::VectorTimeZoneOffsetData {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_operator_shl_t(original_self as *mut ::vector::VectorTimeZoneOffsetData, t as *const ::time_zone::OffsetData) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::remove](../struct.VectorTimeZoneOffsetData.html#method.remove) method.
  pub trait VectorTimeZoneOffsetDataRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> ();
  }
  impl<'largs> VectorTimeZoneOffsetDataRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_remove_i(original_self as *mut ::vector::VectorTimeZoneOffsetData, i)
      }
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTimeZoneOffsetData) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_remove_i_n(original_self as *mut ::vector::VectorTimeZoneOffsetData, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTimeZoneOffsetData::value](../struct.VectorTimeZoneOffsetData.html#method.value) method.
  pub trait VectorTimeZoneOffsetDataValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::time_zone::OffsetData;
  }
  impl<'largs> VectorTimeZoneOffsetDataValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::time_zone::OffsetData {
      let i = self;
      {
        let mut object: ::time_zone::OffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_value_to_output_i(original_self as *const ::vector::VectorTimeZoneOffsetData, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTimeZoneOffsetDataValueArgs<'largs> for (::libc::c_int, &'largs ::time_zone::OffsetData) {
    fn exec(self, original_self: &'largs ::vector::VectorTimeZoneOffsetData) -> ::time_zone::OffsetData {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::time_zone::OffsetData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QTimeZone_OffsetData_value_to_output_i_defaultValue(original_self as *const ::vector::VectorTimeZoneOffsetData, i, default_value as *const ::time_zone::OffsetData, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::append](../struct.VectorXmlStreamAttribute.html#method.append) method.
  pub trait VectorXmlStreamAttributeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> ();
  }
  impl<'largs> VectorXmlStreamAttributeAppendArgs<'largs> for &'largs ::vector::VectorXmlStreamAttribute {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_append_l(original_self as *mut ::vector::VectorXmlStreamAttribute, l as *const ::vector::VectorXmlStreamAttribute)
      }
    }
  }
  impl<'largs> VectorXmlStreamAttributeAppendArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_append_t(original_self as *mut ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::count](../struct.VectorXmlStreamAttribute.html#method.count) method.
  pub trait VectorXmlStreamAttributeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamAttributeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_count_no_args(original_self as *const ::vector::VectorXmlStreamAttribute) }
    }
  }
  impl<'largs> VectorXmlStreamAttributeCountArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_count_t(original_self as *const ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::fill](../struct.VectorXmlStreamAttribute.html#method.fill) method.
  pub trait VectorXmlStreamAttributeFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute;
  }
  impl<'largs> VectorXmlStreamAttributeFillArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_fill_t(original_self as *mut ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorXmlStreamAttributeFillArgs<'largs>
    for (&'largs ::xml_stream_attribute::XmlStreamAttribute, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_fill_t_size(original_self as *mut ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::index_of](../struct.VectorXmlStreamAttribute.html#method.index_of) method.
  pub trait VectorXmlStreamAttributeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamAttributeIndexOfArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_indexOf_t(original_self as *const ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute) }
    }
  }
  impl<'largs> VectorXmlStreamAttributeIndexOfArgs<'largs>
    for (&'largs ::xml_stream_attribute::XmlStreamAttribute, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_indexOf_t_from(original_self as *const ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::insert](../struct.VectorXmlStreamAttribute.html#method.insert) method.
  pub trait VectorXmlStreamAttributeInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> ();
  }
  impl<'largs> VectorXmlStreamAttributeInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::xml_stream_attribute::XmlStreamAttribute) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_insert_i_n_t(original_self as *mut ::vector::VectorXmlStreamAttribute, i, n, t as *const ::xml_stream_attribute::XmlStreamAttribute) }
    }
  }
  impl<'largs> VectorXmlStreamAttributeInsertArgs<'largs>
    for (::libc::c_int, &'largs ::xml_stream_attribute::XmlStreamAttribute) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_insert_i_t(original_self as *mut ::vector::VectorXmlStreamAttribute, i, t as *const ::xml_stream_attribute::XmlStreamAttribute) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::last_index_of](../struct.VectorXmlStreamAttribute.html#method.last_index_of) method.
  pub trait VectorXmlStreamAttributeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamAttributeLastIndexOfArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_lastIndexOf_t(original_self as *const ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute) }
    }
  }
  impl<'largs> VectorXmlStreamAttributeLastIndexOfArgs<'largs> for (&'largs ::xml_stream_attribute::XmlStreamAttribute,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_lastIndexOf_t_from(original_self as *const ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::mid](../struct.VectorXmlStreamAttribute.html#method.mid) method.
  pub trait VectorXmlStreamAttributeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::vector::VectorXmlStreamAttribute;
  }
  impl<'largs> VectorXmlStreamAttributeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::vector::VectorXmlStreamAttribute {
      let pos = self;
      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_mid_to_output_pos(original_self as *const ::vector::VectorXmlStreamAttribute, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamAttributeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamAttribute) -> ::vector::VectorXmlStreamAttribute {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_mid_to_output_pos_len(original_self as *const ::vector::VectorXmlStreamAttribute, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::new](../struct.VectorXmlStreamAttribute.html#method.new) method.
  pub trait VectorXmlStreamAttributeNewArgs {
    fn exec(self) -> ::vector::VectorXmlStreamAttribute;
  }
  impl VectorXmlStreamAttributeNewArgs for () {
    fn exec(self) -> ::vector::VectorXmlStreamAttribute {

      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorXmlStreamAttributeNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorXmlStreamAttribute {
      let size = self;
      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorXmlStreamAttributeNewArgs for (::libc::c_int, &'a ::xml_stream_attribute::XmlStreamAttribute) {
    fn exec(self) -> ::vector::VectorXmlStreamAttribute {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constructor_size_t(size, t as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorXmlStreamAttributeNewArgs for &'a ::vector::VectorXmlStreamAttribute {
    fn exec(self) -> ::vector::VectorXmlStreamAttribute {
      let v = self;
      {
        let mut object: ::vector::VectorXmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_constructor_v(v as *const ::vector::VectorXmlStreamAttribute,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::op_add_assign](../struct.VectorXmlStreamAttribute.html#method.op_add_assign) method.
  pub trait VectorXmlStreamAttributeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute;
  }
  impl<'largs> VectorXmlStreamAttributeOpAddAssignArgs<'largs> for &'largs ::vector::VectorXmlStreamAttribute {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_add_assign_l(original_self as *mut ::vector::VectorXmlStreamAttribute, l as *const ::vector::VectorXmlStreamAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorXmlStreamAttributeOpAddAssignArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_add_assign_t(original_self as *mut ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::op_shl](../struct.VectorXmlStreamAttribute.html#method.op_shl) method.
  pub trait VectorXmlStreamAttributeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute;
  }
  impl<'largs> VectorXmlStreamAttributeOpShlArgs<'largs> for &'largs ::vector::VectorXmlStreamAttribute {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_shl_l(original_self as *mut ::vector::VectorXmlStreamAttribute, l as *const ::vector::VectorXmlStreamAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorXmlStreamAttributeOpShlArgs<'largs> for &'largs ::xml_stream_attribute::XmlStreamAttribute {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamAttribute)
            -> &'largs mut ::vector::VectorXmlStreamAttribute {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_operator_shl_t(original_self as *mut ::vector::VectorXmlStreamAttribute, t as *const ::xml_stream_attribute::XmlStreamAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::remove](../struct.VectorXmlStreamAttribute.html#method.remove) method.
  pub trait VectorXmlStreamAttributeRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> ();
  }
  impl<'largs> VectorXmlStreamAttributeRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_core_c_QVector_QXmlStreamAttribute_remove_i(original_self as *mut ::vector::VectorXmlStreamAttribute, i)
      }
    }
  }
  impl<'largs> VectorXmlStreamAttributeRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamAttribute) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamAttribute_remove_i_n(original_self as *mut ::vector::VectorXmlStreamAttribute, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamAttribute::value](../struct.VectorXmlStreamAttribute.html#method.value) method.
  pub trait VectorXmlStreamAttributeValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamAttribute)
            -> ::xml_stream_attribute::XmlStreamAttribute;
  }
  impl<'largs> VectorXmlStreamAttributeValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamAttribute)
            -> ::xml_stream_attribute::XmlStreamAttribute {
      let i = self;
      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_value_to_output_i(original_self as *const ::vector::VectorXmlStreamAttribute, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamAttributeValueArgs<'largs>
    for (::libc::c_int, &'largs ::xml_stream_attribute::XmlStreamAttribute) {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamAttribute)
            -> ::xml_stream_attribute::XmlStreamAttribute {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::xml_stream_attribute::XmlStreamAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamAttribute_value_to_output_i_defaultValue(original_self as *const ::vector::VectorXmlStreamAttribute, i, default_value as *const ::xml_stream_attribute::XmlStreamAttribute, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::append](../struct.VectorXmlStreamEntityDeclaration.html#method.append) method.
  pub trait VectorXmlStreamEntityDeclarationAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamEntityDeclarationAppendArgs<'largs> for &'largs ::vector::VectorXmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
    let l = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_append_l(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, l as *const ::vector::VectorXmlStreamEntityDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationAppendArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_append_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::count](../struct.VectorXmlStreamEntityDeclaration.html#method.count) method.
  pub trait VectorXmlStreamEntityDeclarationCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_count_no_args(original_self as *const ::vector::VectorXmlStreamEntityDeclaration) }
    }
  }
  impl<'largs> VectorXmlStreamEntityDeclarationCountArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_count_t(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::fill](../struct.VectorXmlStreamEntityDeclaration.html#method.fill) method.
  pub trait VectorXmlStreamEntityDeclarationFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationFillArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_fill_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationFillArgs<'largs> for (&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_fill_t_size(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::index_of](../struct.VectorXmlStreamEntityDeclaration.html#method.index_of) method.
  pub trait VectorXmlStreamEntityDeclarationIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationIndexOfArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_indexOf_t(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationIndexOfArgs<'largs> for (&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_indexOf_t_from(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::insert](../struct.VectorXmlStreamEntityDeclaration.html#method.insert) method.
  pub trait VectorXmlStreamEntityDeclarationInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamEntityDeclarationInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_insert_i_n_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, i, n, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationInsertArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_insert_i_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, i, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::last_index_of](../struct.VectorXmlStreamEntityDeclaration.html#method.last_index_of) method.
  pub trait VectorXmlStreamEntityDeclarationLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationLastIndexOfArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_lastIndexOf_t(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationLastIndexOfArgs<'largs> for (&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_lastIndexOf_t_from(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::mid](../struct.VectorXmlStreamEntityDeclaration.html#method.mid) method.
  pub trait VectorXmlStreamEntityDeclarationMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration)
            -> ::vector::VectorXmlStreamEntityDeclaration;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration)
            -> ::vector::VectorXmlStreamEntityDeclaration {
      let pos = self;
      {
        let mut object: ::vector::VectorXmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_mid_to_output_pos(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamEntityDeclarationMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration)
            -> ::vector::VectorXmlStreamEntityDeclaration {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorXmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_mid_to_output_pos_len(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::new](../struct.VectorXmlStreamEntityDeclaration.html#method.new) method.
  pub trait VectorXmlStreamEntityDeclarationNewArgs {
    fn exec(self) -> ::vector::VectorXmlStreamEntityDeclaration;
  }
  impl VectorXmlStreamEntityDeclarationNewArgs for () {
    fn exec(self) -> ::vector::VectorXmlStreamEntityDeclaration {

      {
        let mut object: ::vector::VectorXmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorXmlStreamEntityDeclarationNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorXmlStreamEntityDeclaration {
      let size = self;
      {
        let mut object: ::vector::VectorXmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorXmlStreamEntityDeclarationNewArgs for (::libc::c_int,&'a ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {

  fn exec(self, ) -> ::vector::VectorXmlStreamEntityDeclaration {
    let size = self.0;
let t = self.1;
    {
let mut object: ::vector::VectorXmlStreamEntityDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_size_t(size, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object); }object
}
  }
}
  impl<'a> VectorXmlStreamEntityDeclarationNewArgs for &'a ::vector::VectorXmlStreamEntityDeclaration {
    fn exec(self) -> ::vector::VectorXmlStreamEntityDeclaration {
      let v = self;
      {
        let mut object: ::vector::VectorXmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_constructor_v(v as *const ::vector::VectorXmlStreamEntityDeclaration, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::op_add_assign](../struct.VectorXmlStreamEntityDeclaration.html#method.op_add_assign) method.
  pub trait VectorXmlStreamEntityDeclarationOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationOpAddAssignArgs<'largs> for &'largs ::vector::VectorXmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_assign_l(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, l as *const ::vector::VectorXmlStreamEntityDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationOpAddAssignArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_add_assign_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::op_shl](../struct.VectorXmlStreamEntityDeclaration.html#method.op_shl) method.
  pub trait VectorXmlStreamEntityDeclarationOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationOpShlArgs<'largs> for &'largs ::vector::VectorXmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_shl_l(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, l as *const ::vector::VectorXmlStreamEntityDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamEntityDeclarationOpShlArgs<'largs> for &'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> &'largs mut ::vector::VectorXmlStreamEntityDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_operator_shl_t(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, t as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::remove](../struct.VectorXmlStreamEntityDeclaration.html#method.remove) method.
  pub trait VectorXmlStreamEntityDeclarationRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamEntityDeclarationRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_remove_i(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, i) }
    }
  }
  impl<'largs> VectorXmlStreamEntityDeclarationRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamEntityDeclaration) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_remove_i_n(original_self as *mut ::vector::VectorXmlStreamEntityDeclaration, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamEntityDeclaration::value](../struct.VectorXmlStreamEntityDeclaration.html#method.value) method.
  pub trait VectorXmlStreamEntityDeclarationValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration)
            -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration;
  }
  impl<'largs> VectorXmlStreamEntityDeclarationValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration)
            -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
      let i = self;
      {
        let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_value_to_output_i(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamEntityDeclarationValueArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_entity_declaration::XmlStreamEntityDeclaration) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamEntityDeclaration) -> ::xml_stream_entity_declaration::XmlStreamEntityDeclaration {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::xml_stream_entity_declaration::XmlStreamEntityDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamEntityDeclaration_value_to_output_i_defaultValue(original_self as *const ::vector::VectorXmlStreamEntityDeclaration, i, default_value as *const ::xml_stream_entity_declaration::XmlStreamEntityDeclaration, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::append](../struct.VectorXmlStreamNamespaceDeclaration.html#method.append) method.
  pub trait VectorXmlStreamNamespaceDeclarationAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationAppendArgs<'largs> for &'largs ::vector::VectorXmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
    let l = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_append_l(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, l as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationAppendArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_append_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::count](../struct.VectorXmlStreamNamespaceDeclaration.html#method.count) method.
  pub trait VectorXmlStreamNamespaceDeclarationCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_count_no_args(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration) }
    }
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationCountArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_count_t(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::fill](../struct.VectorXmlStreamNamespaceDeclaration.html#method.fill) method.
  pub trait VectorXmlStreamNamespaceDeclarationFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationFillArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_fill_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationFillArgs<'largs> for (&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_fill_t_size(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::index_of](../struct.VectorXmlStreamNamespaceDeclaration.html#method.index_of) method.
  pub trait VectorXmlStreamNamespaceDeclarationIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationIndexOfArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_indexOf_t(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationIndexOfArgs<'largs> for (&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_indexOf_t_from(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::insert](../struct.VectorXmlStreamNamespaceDeclaration.html#method.insert) method.
  pub trait VectorXmlStreamNamespaceDeclarationInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_insert_i_n_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i, n, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationInsertArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_insert_i_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::last_index_of](../struct.VectorXmlStreamNamespaceDeclaration.html#method.last_index_of) method.
  pub trait VectorXmlStreamNamespaceDeclarationLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationLastIndexOfArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_lastIndexOf_t(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationLastIndexOfArgs<'largs> for (&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_lastIndexOf_t_from(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::mid](../struct.VectorXmlStreamNamespaceDeclaration.html#method.mid) method.
  pub trait VectorXmlStreamNamespaceDeclarationMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration)
            -> ::vector::VectorXmlStreamNamespaceDeclaration;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration)
            -> ::vector::VectorXmlStreamNamespaceDeclaration {
      let pos = self;
      {
        let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_mid_to_output_pos(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration)
            -> ::vector::VectorXmlStreamNamespaceDeclaration {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_mid_to_output_pos_len(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::new](../struct.VectorXmlStreamNamespaceDeclaration.html#method.new) method.
  pub trait VectorXmlStreamNamespaceDeclarationNewArgs {
    fn exec(self) -> ::vector::VectorXmlStreamNamespaceDeclaration;
  }
  impl VectorXmlStreamNamespaceDeclarationNewArgs for () {
    fn exec(self) -> ::vector::VectorXmlStreamNamespaceDeclaration {

      {
        let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorXmlStreamNamespaceDeclarationNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorXmlStreamNamespaceDeclaration {
      let size = self;
      {
        let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorXmlStreamNamespaceDeclarationNewArgs for (::libc::c_int,&'a ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {

  fn exec(self, ) -> ::vector::VectorXmlStreamNamespaceDeclaration {
    let size = self.0;
let t = self.1;
    {
let mut object: ::vector::VectorXmlStreamNamespaceDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_size_t(size, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, &mut object); }object
}
  }
}
  impl<'a> VectorXmlStreamNamespaceDeclarationNewArgs for &'a ::vector::VectorXmlStreamNamespaceDeclaration {
    fn exec(self) -> ::vector::VectorXmlStreamNamespaceDeclaration {
      let v = self;
      {
        let mut object: ::vector::VectorXmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_constructor_v(v as *const ::vector::VectorXmlStreamNamespaceDeclaration, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::op_add_assign](../struct.VectorXmlStreamNamespaceDeclaration.html#method.op_add_assign) method.
  pub trait VectorXmlStreamNamespaceDeclarationOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationOpAddAssignArgs<'largs> for &'largs ::vector::VectorXmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_assign_l(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, l as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationOpAddAssignArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_add_assign_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::op_shl](../struct.VectorXmlStreamNamespaceDeclaration.html#method.op_shl) method.
  pub trait VectorXmlStreamNamespaceDeclarationOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationOpShlArgs<'largs> for &'largs ::vector::VectorXmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_shl_l(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, l as *const ::vector::VectorXmlStreamNamespaceDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNamespaceDeclarationOpShlArgs<'largs> for &'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_operator_shl_t(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, t as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::remove](../struct.VectorXmlStreamNamespaceDeclaration.html#method.remove) method.
  pub trait VectorXmlStreamNamespaceDeclarationRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_remove_i(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i) }
    }
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNamespaceDeclaration) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_remove_i_n(original_self as *mut ::vector::VectorXmlStreamNamespaceDeclaration, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNamespaceDeclaration::value](../struct.VectorXmlStreamNamespaceDeclaration.html#method.value) method.
  pub trait VectorXmlStreamNamespaceDeclarationValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration)
            -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration;
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration)
            -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
      let i = self;
      {
        let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_value_to_output_i(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamNamespaceDeclarationValueArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNamespaceDeclaration) -> ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNamespaceDeclaration_value_to_output_i_defaultValue(original_self as *const ::vector::VectorXmlStreamNamespaceDeclaration, i, default_value as *const ::xml_stream_namespace_declaration::XmlStreamNamespaceDeclaration, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::append](../struct.VectorXmlStreamNotationDeclaration.html#method.append) method.
  pub trait VectorXmlStreamNotationDeclarationAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNotationDeclarationAppendArgs<'largs> for &'largs ::vector::VectorXmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
    let l = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_append_l(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, l as *const ::vector::VectorXmlStreamNotationDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationAppendArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_append_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::count](../struct.VectorXmlStreamNotationDeclaration.html#method.count) method.
  pub trait VectorXmlStreamNotationDeclarationCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_count_no_args(original_self as *const ::vector::VectorXmlStreamNotationDeclaration) }
    }
  }
  impl<'largs> VectorXmlStreamNotationDeclarationCountArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_count_t(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::fill](../struct.VectorXmlStreamNotationDeclaration.html#method.fill) method.
  pub trait VectorXmlStreamNotationDeclarationFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationFillArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_fill_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationFillArgs<'largs> for (&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_fill_t_size(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::index_of](../struct.VectorXmlStreamNotationDeclaration.html#method.index_of) method.
  pub trait VectorXmlStreamNotationDeclarationIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationIndexOfArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_indexOf_t(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationIndexOfArgs<'largs> for (&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_indexOf_t_from(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::insert](../struct.VectorXmlStreamNotationDeclaration.html#method.insert) method.
  pub trait VectorXmlStreamNotationDeclarationInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNotationDeclarationInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_insert_i_n_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, i, n, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationInsertArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_insert_i_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, i, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::last_index_of](../struct.VectorXmlStreamNotationDeclaration.html#method.last_index_of) method.
  pub trait VectorXmlStreamNotationDeclarationLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationLastIndexOfArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_lastIndexOf_t(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) }
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationLastIndexOfArgs<'largs> for (&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_lastIndexOf_t_from(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::mid](../struct.VectorXmlStreamNotationDeclaration.html#method.mid) method.
  pub trait VectorXmlStreamNotationDeclarationMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration)
            -> ::vector::VectorXmlStreamNotationDeclaration;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration)
            -> ::vector::VectorXmlStreamNotationDeclaration {
      let pos = self;
      {
        let mut object: ::vector::VectorXmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_mid_to_output_pos(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamNotationDeclarationMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration)
            -> ::vector::VectorXmlStreamNotationDeclaration {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorXmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_mid_to_output_pos_len(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::new](../struct.VectorXmlStreamNotationDeclaration.html#method.new) method.
  pub trait VectorXmlStreamNotationDeclarationNewArgs {
    fn exec(self) -> ::vector::VectorXmlStreamNotationDeclaration;
  }
  impl VectorXmlStreamNotationDeclarationNewArgs for () {
    fn exec(self) -> ::vector::VectorXmlStreamNotationDeclaration {

      {
        let mut object: ::vector::VectorXmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorXmlStreamNotationDeclarationNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorXmlStreamNotationDeclaration {
      let size = self;
      {
        let mut object: ::vector::VectorXmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorXmlStreamNotationDeclarationNewArgs for (::libc::c_int,&'a ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {

  fn exec(self, ) -> ::vector::VectorXmlStreamNotationDeclaration {
    let size = self.0;
let t = self.1;
    {
let mut object: ::vector::VectorXmlStreamNotationDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_size_t(size, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object); }object
}
  }
}
  impl<'a> VectorXmlStreamNotationDeclarationNewArgs for &'a ::vector::VectorXmlStreamNotationDeclaration {
    fn exec(self) -> ::vector::VectorXmlStreamNotationDeclaration {
      let v = self;
      {
        let mut object: ::vector::VectorXmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_constructor_v(v as *const ::vector::VectorXmlStreamNotationDeclaration, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::op_add_assign](../struct.VectorXmlStreamNotationDeclaration.html#method.op_add_assign) method.
  pub trait VectorXmlStreamNotationDeclarationOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationOpAddAssignArgs<'largs> for &'largs ::vector::VectorXmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_assign_l(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, l as *const ::vector::VectorXmlStreamNotationDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationOpAddAssignArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_add_assign_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::op_shl](../struct.VectorXmlStreamNotationDeclaration.html#method.op_shl) method.
  pub trait VectorXmlStreamNotationDeclarationOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration)
            -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationOpShlArgs<'largs> for &'largs ::vector::VectorXmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_shl_l(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, l as *const ::vector::VectorXmlStreamNotationDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorXmlStreamNotationDeclarationOpShlArgs<'largs> for &'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {

  fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> &'largs mut ::vector::VectorXmlStreamNotationDeclaration {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_operator_shl_t(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, t as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::remove](../struct.VectorXmlStreamNotationDeclaration.html#method.remove) method.
  pub trait VectorXmlStreamNotationDeclarationRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> ();
  }
  impl<'largs> VectorXmlStreamNotationDeclarationRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
      let i = self;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_remove_i(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, i) }
    }
  }
  impl<'largs> VectorXmlStreamNotationDeclarationRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorXmlStreamNotationDeclaration) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_remove_i_n(original_self as *mut ::vector::VectorXmlStreamNotationDeclaration, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorXmlStreamNotationDeclaration::value](../struct.VectorXmlStreamNotationDeclaration.html#method.value) method.
  pub trait VectorXmlStreamNotationDeclarationValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration)
            -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration;
  }
  impl<'largs> VectorXmlStreamNotationDeclarationValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration)
            -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
      let i = self;
      {
        let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_value_to_output_i(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorXmlStreamNotationDeclarationValueArgs<'largs> for (::libc::c_int,&'largs ::xml_stream_notation_declaration::XmlStreamNotationDeclaration) {

  fn exec(self, original_self: &'largs ::vector::VectorXmlStreamNotationDeclaration) -> ::xml_stream_notation_declaration::XmlStreamNotationDeclaration {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::xml_stream_notation_declaration::XmlStreamNotationDeclaration = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_core_c_QVector_QXmlStreamNotationDeclaration_value_to_output_i_defaultValue(original_self as *const ::vector::VectorXmlStreamNotationDeclaration, i, default_value as *const ::xml_stream_notation_declaration::XmlStreamNotationDeclaration, &mut object); }object
}
  }
}
}
