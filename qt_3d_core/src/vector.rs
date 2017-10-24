/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>```</span>
#[repr(C)]
pub struct VectorAbstractAspectMutPtr([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_ABSTRACT_ASPECT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAbstractAspectMutPtr {
  unsafe fn new_uninitialized() -> VectorAbstractAspectMutPtr {
    VectorAbstractAspectMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAbstractAspectMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::append(const QVector<Qt3DCore::QAbstractAspect*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAbstractAspectMutPtr) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_append_l(self as *mut ::vector::VectorAbstractAspectMutPtr, l as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::append(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_append_t(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_at(self as *const ::vector::VectorAbstractAspectMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_back_const(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect*& QVector<Qt3DCore::QAbstractAspect*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_aspect::AbstractAspect {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_back(self as *mut ::vector::VectorAbstractAspectMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_capacity(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_clear(self as *mut ::vector::VectorAbstractAspectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const * QVector<Qt3DCore::QAbstractAspect*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::abstract_aspect::AbstractAspect {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constData(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constFirst(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constLast(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::contains(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_aspect::AbstractAspect) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_contains(self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_count_no_args(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::count(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_aspect::AbstractAspect) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_count_t(self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const * QVector<Qt3DCore::QAbstractAspect*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::abstract_aspect::AbstractAspect {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_data_const(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect** QVector<Qt3DCore::QAbstractAspect*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::abstract_aspect::AbstractAspect {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_data(self as *mut ::vector::VectorAbstractAspectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_empty(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::endsWith(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_aspect::AbstractAspect) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_endsWith(self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::abstract_aspect::AbstractAspect) -> &'l0 mut ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::fill(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::abstract_aspect::AbstractAspect, ::libc::c_int)) -> &'l0 mut ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::fill(Qt3DCore::QAbstractAspect* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorAbstractAspectMutPtr
    where Args: overloading::VectorAbstractAspectMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_first_const(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect*& QVector<Qt3DCore::QAbstractAspect*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_first(self as *mut ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_front_const(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect*& QVector<Qt3DCore::QAbstractAspect*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_front(self as *mut ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_aspect::AbstractAspect) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::indexOf(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_aspect::AbstractAspect, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::indexOf(Qt3DCore::QAbstractAspect* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractAspectMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::abstract_aspect::AbstractAspect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::insert(int i, Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::abstract_aspect::AbstractAspect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::insert(int i, int n, Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractAspectMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_isEmpty(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_last_const(self as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_aspect::AbstractAspect) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::lastIndexOf(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_aspect::AbstractAspect, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::lastIndexOf(Qt3DCore::QAbstractAspect* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractAspectMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect*& QVector<Qt3DCore::QAbstractAspect*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_aspect::AbstractAspect {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_last(self as *mut ::vector::VectorAbstractAspectMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_length(self as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*> QVector<Qt3DCore::QAbstractAspect*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*> QVector<Qt3DCore::QAbstractAspect*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAbstractAspectMutPtr
    where Args: overloading::VectorAbstractAspectMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_move(self as *mut ::vector::VectorAbstractAspectMutPtr,
                                                                    from,
                                                                    to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QAbstractAspect*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAbstractAspectMutPtr) -> ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QAbstractAspect*>::QVector(const QVector<Qt3DCore::QAbstractAspect*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAbstractAspectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QAbstractAspect*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAbstractAspectMutPtr
    where Args: overloading::VectorAbstractAspectMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QAbstractAspect*>::QVector(int size, Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::abstract_aspect::AbstractAspect)
                           -> ::vector::VectorAbstractAspectMutPtr {
    {
      let mut object: ::vector::VectorAbstractAspectMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constructor_size_t(size, t as *const *mut ::abstract_aspect::AbstractAspect, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*> QVector<Qt3DCore::QAbstractAspect*>::operator+(const QVector<Qt3DCore::QAbstractAspect*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAbstractAspectMutPtr) -> ::vector::VectorAbstractAspectMutPtr {
    {
      let mut object: ::vector::VectorAbstractAspectMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_add_to_output(self as *const ::vector::VectorAbstractAspectMutPtr, l as *const ::vector::VectorAbstractAspectMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::operator+=(const QVector<Qt3DCore::QAbstractAspect*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAbstractAspectMutPtr)
                                 -> &'l0 mut ::vector::VectorAbstractAspectMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_add_assign_l(self as *mut ::vector::VectorAbstractAspectMutPtr, l as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::operator+=(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_aspect::AbstractAspect)
                                               -> &'l0 mut ::vector::VectorAbstractAspectMutPtr {
    let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_add_assign_t(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::operator=(const QVector<Qt3DCore::QAbstractAspect*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAbstractAspectMutPtr)
                             -> &'l0 mut ::vector::VectorAbstractAspectMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_assign(self as *mut ::vector::VectorAbstractAspectMutPtr, v as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::operator==(const QVector<Qt3DCore::QAbstractAspect*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAbstractAspectMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_eq(self as *const ::vector::VectorAbstractAspectMutPtr, v as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* const & QVector<Qt3DCore::QAbstractAspect*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_index_const(self as *const ::vector::VectorAbstractAspectMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect*& QVector<Qt3DCore::QAbstractAspect*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_aspect::AbstractAspect {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_index(self as *mut ::vector::VectorAbstractAspectMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::operator!=(const QVector<Qt3DCore::QAbstractAspect*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAbstractAspectMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_neq(self as *const ::vector::VectorAbstractAspectMutPtr, v as *const ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::operator<<(const QVector<Qt3DCore::QAbstractAspect*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAbstractAspectMutPtr)
                          -> &'l0 mut ::vector::VectorAbstractAspectMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_shl_l(self as *mut ::vector::VectorAbstractAspectMutPtr, l as *const ::vector::VectorAbstractAspectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>& QVector<Qt3DCore::QAbstractAspect*>::operator<<(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_aspect::AbstractAspect)
                                        -> &'l0 mut ::vector::VectorAbstractAspectMutPtr {
    let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_operator_shl_t(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_pop_back(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_pop_front(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::prepend(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_prepend(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::push_back(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_push_back(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::push_front(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_push_front(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QAbstractAspect*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractAspectMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::removeAll(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_removeAll(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_removeAt(self as *mut ::vector::VectorAbstractAspectMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_removeFirst(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_removeLast(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::removeOne(Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_aspect::AbstractAspect) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_removeOne(self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::replace(int i, Qt3DCore::QAbstractAspect* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_aspect::AbstractAspect) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_replace(self as *mut ::vector::VectorAbstractAspectMutPtr, i, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_reserve(self as *mut ::vector::VectorAbstractAspectMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_resize(self as *mut ::vector::VectorAbstractAspectMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QAbstractAspect*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_size(self as *const ::vector::VectorAbstractAspectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_squeeze(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QAbstractAspect*>::startsWith(Qt3DCore::QAbstractAspect* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_aspect::AbstractAspect) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_startsWith(self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QAbstractAspect*>::swap(QVector<Qt3DCore::QAbstractAspect*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAbstractAspectMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_swap(self as *mut ::vector::VectorAbstractAspectMutPtr, other as *mut ::vector::VectorAbstractAspectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* QVector<Qt3DCore::QAbstractAspect*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_aspect::AbstractAspect {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_takeAt(self as *mut ::vector::VectorAbstractAspectMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* QVector<Qt3DCore::QAbstractAspect*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_aspect::AbstractAspect {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_takeFirst(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* QVector<Qt3DCore::QAbstractAspect*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_aspect::AbstractAspect {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_takeLast(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* QVector<Qt3DCore::QAbstractAspect*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_aspect::AbstractAspect {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_value_i(self as *const ::vector::VectorAbstractAspectMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QAbstractAspect* QVector<Qt3DCore::QAbstractAspect*>::value(int i, Qt3DCore::QAbstractAspect* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_aspect::AbstractAspect)
                             -> *mut ::abstract_aspect::AbstractAspect {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_value_i_defaultValue(self as *const ::vector::VectorAbstractAspectMutPtr, i, default_value as *const *mut ::abstract_aspect::AbstractAspect)
  }
}

impl Drop for ::vector::VectorAbstractAspectMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QAbstractAspect*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_destructor(self as *mut ::vector::VectorAbstractAspectMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>```</span>
#[repr(C)]
pub struct VectorComponentMutPtr([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_COMPONENT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorComponentMutPtr {
  unsafe fn new_uninitialized() -> VectorComponentMutPtr {
    VectorComponentMutPtr(::std::mem::uninitialized())
  }
}

impl VectorComponentMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::append(const QVector<Qt3DCore::QComponent*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorComponentMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_append_l(self as *mut ::vector::VectorComponentMutPtr,
                                                                   l as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::append(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::component::Component) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_append_t(self as *mut ::vector::VectorComponentMutPtr,
                                                                 t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_at(self as *const ::vector::VectorComponentMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_back_const(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent*& QVector<Qt3DCore::QComponent*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::component::Component {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_back(self as *mut ::vector::VectorComponentMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_capacity(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_clear(self as *mut ::vector::VectorComponentMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const * QVector<Qt3DCore::QComponent*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constData(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constFirst(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constLast(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::contains(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::component::Component) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_contains(self as *const ::vector::VectorComponentMutPtr,
                                                                 t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_count_no_args(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::count(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::component::Component) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_count_t(self as *const ::vector::VectorComponentMutPtr,
                                                                t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const * QVector<Qt3DCore::QComponent*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_data_const(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent** QVector<Qt3DCore::QComponent*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::component::Component {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_data(self as *mut ::vector::VectorComponentMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_empty(self as *const ::vector::VectorComponentMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::endsWith(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::component::Component) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_endsWith(self as *const ::vector::VectorComponentMutPtr,
                                                                 t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::component::Component) -> &'l0 mut ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::fill(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::component::Component, ::libc::c_int)) -> &'l0 mut ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::fill(Qt3DCore::QComponent* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorComponentMutPtr
    where Args: overloading::VectorComponentMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_first_const(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent*& QVector<Qt3DCore::QComponent*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_first(self as *mut ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_front_const(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent*& QVector<Qt3DCore::QComponent*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_front(self as *mut ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::component::Component) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::indexOf(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::component::Component, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::indexOf(Qt3DCore::QComponent* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorComponentMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::component::Component)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::insert(int i, Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::component::Component)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::insert(int i, int n, Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorComponentMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_isEmpty(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_last_const(self as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::component::Component) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::lastIndexOf(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::component::Component, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::lastIndexOf(Qt3DCore::QComponent* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorComponentMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent*& QVector<Qt3DCore::QComponent*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::component::Component {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_last(self as *mut ::vector::VectorComponentMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_length(self as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*> QVector<Qt3DCore::QComponent*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*> QVector<Qt3DCore::QComponent*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorComponentMutPtr
    where Args: overloading::VectorComponentMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_move(self as *mut ::vector::VectorComponentMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QComponent*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorComponentMutPtr) -> ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QComponent*>::QVector(const QVector<Qt3DCore::QComponent*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorComponentMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QComponent*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorComponentMutPtr
    where Args: overloading::VectorComponentMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QComponent*>::QVector(int size, Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::component::Component) -> ::vector::VectorComponentMutPtr {
    {
      let mut object: ::vector::VectorComponentMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constructor_size_t(size,
                                                                             t as *const *mut ::component::Component,
                                                                             &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*> QVector<Qt3DCore::QComponent*>::operator+(const QVector<Qt3DCore::QComponent*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorComponentMutPtr) -> ::vector::VectorComponentMutPtr {
    {
      let mut object: ::vector::VectorComponentMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_add_to_output(self as *const ::vector::VectorComponentMutPtr, l as *const ::vector::VectorComponentMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::operator+=(const QVector<Qt3DCore::QComponent*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorComponentMutPtr)
                                 -> &'l0 mut ::vector::VectorComponentMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_add_assign_l(self as *mut ::vector::VectorComponentMutPtr, l as *const ::vector::VectorComponentMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::operator+=(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::component::Component)
                                               -> &'l0 mut ::vector::VectorComponentMutPtr {
    let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_add_assign_t(self as *mut ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::operator=(const QVector<Qt3DCore::QComponent*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorComponentMutPtr)
                             -> &'l0 mut ::vector::VectorComponentMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_assign(self as *mut ::vector::VectorComponentMutPtr, v as *const ::vector::VectorComponentMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::operator==(const QVector<Qt3DCore::QComponent*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorComponentMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_eq(self as *const ::vector::VectorComponentMutPtr,
                                                                      v as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* const & QVector<Qt3DCore::QComponent*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_index_const(self as *const ::vector::VectorComponentMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent*& QVector<Qt3DCore::QComponent*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_index(self as *mut ::vector::VectorComponentMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::operator!=(const QVector<Qt3DCore::QComponent*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorComponentMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_neq(self as *const ::vector::VectorComponentMutPtr,
                                                                       v as *const ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::operator<<(const QVector<Qt3DCore::QComponent*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorComponentMutPtr)
                          -> &'l0 mut ::vector::VectorComponentMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_shl_l(self as *mut ::vector::VectorComponentMutPtr, l as *const ::vector::VectorComponentMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>& QVector<Qt3DCore::QComponent*>::operator<<(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::component::Component)
                                        -> &'l0 mut ::vector::VectorComponentMutPtr {
    let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_operator_shl_t(self as *mut ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_pop_back(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_pop_front(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::prepend(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::component::Component) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_prepend(self as *mut ::vector::VectorComponentMutPtr,
                                                                t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::push_back(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::component::Component) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_push_back(self as *mut ::vector::VectorComponentMutPtr,
                                                                  t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::push_front(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::component::Component) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_push_front(self as *mut ::vector::VectorComponentMutPtr,
                                                                   t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QComponent*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorComponentMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::removeAll(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::component::Component) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_removeAll(self as *mut ::vector::VectorComponentMutPtr,
                                                                  t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_removeAt(self as *mut ::vector::VectorComponentMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_removeFirst(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_removeLast(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::removeOne(Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::component::Component) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_removeOne(self as *mut ::vector::VectorComponentMutPtr,
                                                                  t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::replace(int i, Qt3DCore::QComponent* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::component::Component) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_replace(self as *mut ::vector::VectorComponentMutPtr,
                                                                i,
                                                                t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_reserve(self as *mut ::vector::VectorComponentMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_resize(self as *mut ::vector::VectorComponentMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QComponent*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_size(self as *const ::vector::VectorComponentMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_squeeze(self as *mut ::vector::VectorComponentMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QComponent*>::startsWith(Qt3DCore::QComponent* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::component::Component) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_startsWith(self as *const ::vector::VectorComponentMutPtr,
                                                                   t as *const *mut ::component::Component)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QComponent*>::swap(QVector<Qt3DCore::QComponent*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorComponentMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_swap(self as *mut ::vector::VectorComponentMutPtr,
                                                               other as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* QVector<Qt3DCore::QComponent*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_takeAt(self as *mut ::vector::VectorComponentMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* QVector<Qt3DCore::QComponent*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_takeFirst(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* QVector<Qt3DCore::QComponent*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_takeLast(self as *mut ::vector::VectorComponentMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* QVector<Qt3DCore::QComponent*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::component::Component {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_value_i(self as *const ::vector::VectorComponentMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QComponent* QVector<Qt3DCore::QComponent*>::value(int i, Qt3DCore::QComponent* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::component::Component)
                             -> *mut ::component::Component {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_value_i_defaultValue(self as *const ::vector::VectorComponentMutPtr, i, default_value as *const *mut ::component::Component)
  }
}

impl Drop for ::vector::VectorComponentMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QComponent*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_destructor(self as *mut ::vector::VectorComponentMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>```</span>
#[repr(C)]
pub struct VectorEntityMutPtr([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_ENTITY_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorEntityMutPtr {
  unsafe fn new_uninitialized() -> VectorEntityMutPtr {
    VectorEntityMutPtr(::std::mem::uninitialized())
  }
}

impl VectorEntityMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::append(const QVector<Qt3DCore::QEntity*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorEntityMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_append_l(self as *mut ::vector::VectorEntityMutPtr,
                                                                l as *const ::vector::VectorEntityMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::append(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_append_t(self as *mut ::vector::VectorEntityMutPtr,
                                                              t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_at(self as *const ::vector::VectorEntityMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_back_const(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity*& QVector<Qt3DCore::QEntity*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_back(self as *mut ::vector::VectorEntityMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_capacity(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_clear(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const * QVector<Qt3DCore::QEntity*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constData(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constFirst(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constLast(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::contains(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::entity::Entity) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_contains(self as *const ::vector::VectorEntityMutPtr,
                                                              t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_count_no_args(self as *const ::vector::VectorEntityMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::count(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::entity::Entity) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_count_t(self as *const ::vector::VectorEntityMutPtr,
                                                             t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const * QVector<Qt3DCore::QEntity*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_data_const(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity** QVector<Qt3DCore::QEntity*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_data(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_empty(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::endsWith(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::entity::Entity) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_endsWith(self as *const ::vector::VectorEntityMutPtr,
                                                              t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::entity::Entity) -> &'l0 mut ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::fill(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::entity::Entity, ::libc::c_int)) -> &'l0 mut ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::fill(Qt3DCore::QEntity* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorEntityMutPtr
    where Args: overloading::VectorEntityMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_first_const(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity*& QVector<Qt3DCore::QEntity*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_first(self as *mut ::vector::VectorEntityMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_front_const(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity*& QVector<Qt3DCore::QEntity*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_front(self as *mut ::vector::VectorEntityMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::entity::Entity) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::indexOf(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::entity::Entity, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::indexOf(Qt3DCore::QEntity* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorEntityMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::entity::Entity)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::insert(int i, Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::entity::Entity)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::insert(int i, int n, Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorEntityMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_isEmpty(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_last_const(self as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::entity::Entity) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::lastIndexOf(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::entity::Entity, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::lastIndexOf(Qt3DCore::QEntity* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorEntityMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity*& QVector<Qt3DCore::QEntity*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::entity::Entity {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_last(self as *mut ::vector::VectorEntityMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_length(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*> QVector<Qt3DCore::QEntity*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*> QVector<Qt3DCore::QEntity*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorEntityMutPtr
    where Args: overloading::VectorEntityMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_move(self as *mut ::vector::VectorEntityMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QEntity*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorEntityMutPtr) -> ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QEntity*>::QVector(const QVector<Qt3DCore::QEntity*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorEntityMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QEntity*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorEntityMutPtr
    where Args: overloading::VectorEntityMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QEntity*>::QVector(int size, Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::entity::Entity) -> ::vector::VectorEntityMutPtr {
    {
      let mut object: ::vector::VectorEntityMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constructor_size_t(size,
                                                                          t as *const *mut ::entity::Entity,
                                                                          &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*> QVector<Qt3DCore::QEntity*>::operator+(const QVector<Qt3DCore::QEntity*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorEntityMutPtr) -> ::vector::VectorEntityMutPtr {
    {
      let mut object: ::vector::VectorEntityMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_add_to_output(self as *const ::vector::VectorEntityMutPtr, l as *const ::vector::VectorEntityMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::operator+=(const QVector<Qt3DCore::QEntity*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorEntityMutPtr)
                                 -> &'l0 mut ::vector::VectorEntityMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_add_assign_l(self as *mut ::vector::VectorEntityMutPtr, l as *const ::vector::VectorEntityMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::operator+=(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::entity::Entity)
                                               -> &'l0 mut ::vector::VectorEntityMutPtr {
    let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_add_assign_t(self as *mut ::vector::VectorEntityMutPtr, t as *const *mut ::entity::Entity);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::operator=(const QVector<Qt3DCore::QEntity*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorEntityMutPtr)
                             -> &'l0 mut ::vector::VectorEntityMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_assign(self as *mut ::vector::VectorEntityMutPtr,
                                                                         v as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::operator==(const QVector<Qt3DCore::QEntity*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorEntityMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_eq(self as *const ::vector::VectorEntityMutPtr,
                                                                   v as *const ::vector::VectorEntityMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* const & QVector<Qt3DCore::QEntity*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_index_const(self as *const ::vector::VectorEntityMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity*& QVector<Qt3DCore::QEntity*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::entity::Entity {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_index(self as *mut ::vector::VectorEntityMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::operator!=(const QVector<Qt3DCore::QEntity*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorEntityMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_neq(self as *const ::vector::VectorEntityMutPtr,
                                                                    v as *const ::vector::VectorEntityMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::operator<<(const QVector<Qt3DCore::QEntity*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorEntityMutPtr)
                          -> &'l0 mut ::vector::VectorEntityMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_shl_l(self as *mut ::vector::VectorEntityMutPtr,
                                                                        l as *const ::vector::VectorEntityMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>& QVector<Qt3DCore::QEntity*>::operator<<(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::entity::Entity)
                                        -> &'l0 mut ::vector::VectorEntityMutPtr {
    let ffi_result =
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_operator_shl_t(self as *mut ::vector::VectorEntityMutPtr,
                                                                      t as *const *mut ::entity::Entity);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_pop_back(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_pop_front(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::prepend(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_prepend(self as *mut ::vector::VectorEntityMutPtr,
                                                             t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::push_back(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_push_back(self as *mut ::vector::VectorEntityMutPtr,
                                                               t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::push_front(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_push_front(self as *mut ::vector::VectorEntityMutPtr,
                                                                t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QEntity*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorEntityMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::removeAll(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::entity::Entity) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_removeAll(self as *mut ::vector::VectorEntityMutPtr,
                                                               t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_removeAt(self as *mut ::vector::VectorEntityMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_removeFirst(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_removeLast(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::removeOne(Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::entity::Entity) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_removeOne(self as *mut ::vector::VectorEntityMutPtr,
                                                               t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::replace(int i, Qt3DCore::QEntity* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::entity::Entity) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_replace(self as *mut ::vector::VectorEntityMutPtr,
                                                             i,
                                                             t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_reserve(self as *mut ::vector::VectorEntityMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_resize(self as *mut ::vector::VectorEntityMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QEntity*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_size(self as *const ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_squeeze(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QEntity*>::startsWith(Qt3DCore::QEntity* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::entity::Entity) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_startsWith(self as *const ::vector::VectorEntityMutPtr,
                                                                t as *const *mut ::entity::Entity)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QEntity*>::swap(QVector<Qt3DCore::QEntity*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorEntityMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_swap(self as *mut ::vector::VectorEntityMutPtr,
                                                            other as *mut ::vector::VectorEntityMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QVector<Qt3DCore::QEntity*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_takeAt(self as *mut ::vector::VectorEntityMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QVector<Qt3DCore::QEntity*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_takeFirst(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QVector<Qt3DCore::QEntity*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_takeLast(self as *mut ::vector::VectorEntityMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QVector<Qt3DCore::QEntity*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::entity::Entity {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_value_i(self as *const ::vector::VectorEntityMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QEntity* QVector<Qt3DCore::QEntity*>::value(int i, Qt3DCore::QEntity* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::entity::Entity) -> *mut ::entity::Entity {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_value_i_defaultValue(self as *const ::vector::VectorEntityMutPtr, i, default_value as *const *mut ::entity::Entity)
  }
}

impl Drop for ::vector::VectorEntityMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QEntity*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_destructor(self as *mut ::vector::VectorEntityMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>```</span>
#[repr(C)]
pub struct VectorNodeId([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_NODE_ID]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorNodeId {
  unsafe fn new_uninitialized() -> VectorNodeId {
    VectorNodeId(::std::mem::uninitialized())
  }
}

impl VectorNodeId {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorNodeId) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::append(const QVector<Qt3DCore::QNodeId>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::node_id::NodeId) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::append(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeIdAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_at(self as *const ::vector::VectorNodeId, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_back_const(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node_id::NodeId {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_back(self as *mut ::vector::VectorNodeId) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_capacity(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_clear(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId* QVector<Qt3DCore::QNodeId>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::node_id::NodeId {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constData(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constFirst(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constLast(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::contains(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_contains(self as *const ::vector::VectorNodeId,
                                                            t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::node_id::NodeId) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::count(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorNodeIdCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId* QVector<Qt3DCore::QNodeId>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::node_id::NodeId {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_data_const(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId* QVector<Qt3DCore::QNodeId>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::node_id::NodeId {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_data(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_empty(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::endsWith(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_endsWith(self as *const ::vector::VectorNodeId,
                                                            t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::node_id::NodeId) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::fill(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::node_id::NodeId, ::libc::c_int)) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::fill(const Qt3DCore::QNodeId& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeId
    where Args: overloading::VectorNodeIdFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_first_const(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node_id::NodeId {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_first(self as *mut ::vector::VectorNodeId) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_front_const(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node_id::NodeId {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_front(self as *mut ::vector::VectorNodeId) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::node_id::NodeId) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::indexOf(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::node_id::NodeId, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::indexOf(const Qt3DCore::QNodeId& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorNodeIdIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::node_id::NodeId)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::insert(int i, const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::node_id::NodeId)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::insert(int i, int n, const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeIdInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_isEmpty(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_last_const(self as *const ::vector::VectorNodeId) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::node_id::NodeId) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::lastIndexOf(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::node_id::NodeId, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::lastIndexOf(const Qt3DCore::QNodeId& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorNodeIdLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node_id::NodeId {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_last(self as *mut ::vector::VectorNodeId) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_length(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId> QVector<Qt3DCore::QNodeId>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId> QVector<Qt3DCore::QNodeId>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorNodeId
    where Args: overloading::VectorNodeIdMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_move(self as *mut ::vector::VectorNodeId, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeId>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorNodeId) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeId>::QVector(const QVector<Qt3DCore::QNodeId>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeId>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::node_id::NodeId)) -> ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeId>::QVector(int size, const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorNodeId
    where Args: overloading::VectorNodeIdNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId> QVector<Qt3DCore::QNodeId>::operator+(const QVector<Qt3DCore::QNodeId>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorNodeId) -> ::vector::VectorNodeId {
    {
      let mut object: ::vector::VectorNodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_add_to_output(self as *const ::vector::VectorNodeId,
                                                                            l as *const ::vector::VectorNodeId,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorNodeId) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::operator+=(const QVector<Qt3DCore::QNodeId>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::node_id::NodeId) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::operator+=(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeId
    where Args: overloading::VectorNodeIdOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::operator=(const QVector<Qt3DCore::QNodeId>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorNodeId) -> &'l0 mut ::vector::VectorNodeId {
    let ffi_result = unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_assign(self as *mut ::vector::VectorNodeId,
                                                                   v as *const ::vector::VectorNodeId)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::operator==(const QVector<Qt3DCore::QNodeId>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorNodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_eq(self as *const ::vector::VectorNodeId,
                                                               v as *const ::vector::VectorNodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::node_id::NodeId {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_index_const(self as *const ::vector::VectorNodeId, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId& QVector<Qt3DCore::QNodeId>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::node_id::NodeId {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_index(self as *mut ::vector::VectorNodeId, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::operator!=(const QVector<Qt3DCore::QNodeId>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorNodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_neq(self as *const ::vector::VectorNodeId,
                                                                v as *const ::vector::VectorNodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorNodeId) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::operator<<(const QVector<Qt3DCore::QNodeId>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::node_id::NodeId) -> &'l0 mut ::vector::VectorNodeId```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>& QVector<Qt3DCore::QNodeId>::operator<<(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeId
    where Args: overloading::VectorNodeIdOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_pop_back(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_pop_front(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::prepend(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_prepend(self as *mut ::vector::VectorNodeId,
                                                           t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::push_back(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_push_back(self as *mut ::vector::VectorNodeId,
                                                             t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::push_front(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_push_front(self as *mut ::vector::VectorNodeId,
                                                              t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeIdRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::removeAll(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::node_id::NodeId) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_removeAll(self as *mut ::vector::VectorNodeId,
                                                             t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_removeAt(self as *mut ::vector::VectorNodeId, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_removeFirst(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_removeLast(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::removeOne(const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_removeOne(self as *mut ::vector::VectorNodeId,
                                                             t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::replace(int i, const Qt3DCore::QNodeId& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::node_id::NodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_replace(self as *mut ::vector::VectorNodeId,
                                                           i,
                                                           t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_reserve(self as *mut ::vector::VectorNodeId, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_resize(self as *mut ::vector::VectorNodeId, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeId>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_size(self as *const ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_squeeze(self as *mut ::vector::VectorNodeId) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeId>::startsWith(const Qt3DCore::QNodeId& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::node_id::NodeId) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_startsWith(self as *const ::vector::VectorNodeId,
                                                              t as *const ::node_id::NodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeId>::swap(QVector<Qt3DCore::QNodeId>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorNodeId) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_swap(self as *mut ::vector::VectorNodeId,
                                                        other as *mut ::vector::VectorNodeId)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId QVector<Qt3DCore::QNodeId>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_takeAt_to_output(self as *mut ::vector::VectorNodeId,
                                                                      i,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId QVector<Qt3DCore::QNodeId>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_takeFirst_to_output(self as *mut ::vector::VectorNodeId,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId QVector<Qt3DCore::QNodeId>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_takeLast_to_output(self as *mut ::vector::VectorNodeId,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeId>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::node_id::NodeId```<br>
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId QVector<Qt3DCore::QNodeId>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::node_id::NodeId)) -> ::node_id::NodeId```<br>
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId QVector<Qt3DCore::QNodeId>::value(int i, const Qt3DCore::QNodeId& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::node_id::NodeId
    where Args: overloading::VectorNodeIdValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorNodeId {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QNodeId>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_destructor(self as *mut ::vector::VectorNodeId) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QNode*>```</span>
#[repr(C)]
pub struct VectorNodeMutPtr([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_NODE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorNodeMutPtr {
  unsafe fn new_uninitialized() -> VectorNodeMutPtr {
    VectorNodeMutPtr(::std::mem::uninitialized())
  }
}

impl VectorNodeMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::append(const QVector<Qt3DCore::QNode*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorNodeMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_append_l(self as *mut ::vector::VectorNodeMutPtr,
                                                              l as *const ::vector::VectorNodeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::append(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::node::Node) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_append_t(self as *mut ::vector::VectorNodeMutPtr,
                                                            t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_at(self as *const ::vector::VectorNodeMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_back_const(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode*& QVector<Qt3DCore::QNode*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_back(self as *mut ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_capacity(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_clear(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const * QVector<Qt3DCore::QNode*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constData(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constFirst(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constLast(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::contains(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::node::Node) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_contains(self as *const ::vector::VectorNodeMutPtr,
                                                            t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_count_no_args(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::count(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::node::Node) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_count_t(self as *const ::vector::VectorNodeMutPtr,
                                                           t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const * QVector<Qt3DCore::QNode*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_data_const(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode** QVector<Qt3DCore::QNode*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_data(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_empty(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::endsWith(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::node::Node) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_endsWith(self as *const ::vector::VectorNodeMutPtr,
                                                            t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::node::Node) -> &'l0 mut ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::fill(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::node::Node, ::libc::c_int)) -> &'l0 mut ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::fill(Qt3DCore::QNode* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeMutPtr
    where Args: overloading::VectorNodeMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_first_const(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode*& QVector<Qt3DCore::QNode*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_first(self as *mut ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_front_const(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode*& QVector<Qt3DCore::QNode*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_front(self as *mut ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::node::Node) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::indexOf(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::node::Node, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::indexOf(Qt3DCore::QNode* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorNodeMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::node::Node)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::insert(int i, Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::node::Node)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::insert(int i, int n, Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_isEmpty(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_last_const(self as *const ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::node::Node) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::lastIndexOf(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::node::Node, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::lastIndexOf(Qt3DCore::QNode* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorNodeMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode*& QVector<Qt3DCore::QNode*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_last(self as *mut ::vector::VectorNodeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_length(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*> QVector<Qt3DCore::QNode*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*> QVector<Qt3DCore::QNode*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorNodeMutPtr
    where Args: overloading::VectorNodeMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_move(self as *mut ::vector::VectorNodeMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNode*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorNodeMutPtr) -> ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNode*>::QVector(const QVector<Qt3DCore::QNode*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorNodeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNode*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorNodeMutPtr
    where Args: overloading::VectorNodeMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNode*>::QVector(int size, Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::node::Node) -> ::vector::VectorNodeMutPtr {
    {
      let mut object: ::vector::VectorNodeMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constructor_size_t(size,
                                                                        t as *const *mut ::node::Node,
                                                                        &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*> QVector<Qt3DCore::QNode*>::operator+(const QVector<Qt3DCore::QNode*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorNodeMutPtr) -> ::vector::VectorNodeMutPtr {
    {
      let mut object: ::vector::VectorNodeMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_add_to_output(self as *const ::vector::VectorNodeMutPtr, l as *const ::vector::VectorNodeMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::operator+=(const QVector<Qt3DCore::QNode*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorNodeMutPtr)
                                 -> &'l0 mut ::vector::VectorNodeMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_add_assign_l(self as *mut ::vector::VectorNodeMutPtr,
                                                                             l as *const ::vector::VectorNodeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::operator+=(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::node::Node)
                                               -> &'l0 mut ::vector::VectorNodeMutPtr {
    let ffi_result =
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_add_assign_t(self as *mut ::vector::VectorNodeMutPtr,
                                                                           t as *const *mut ::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::operator=(const QVector<Qt3DCore::QNode*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorNodeMutPtr) -> &'l0 mut ::vector::VectorNodeMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_assign(self as *mut ::vector::VectorNodeMutPtr,
                                                                       v as *const ::vector::VectorNodeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::operator==(const QVector<Qt3DCore::QNode*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorNodeMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_eq(self as *const ::vector::VectorNodeMutPtr,
                                                                 v as *const ::vector::VectorNodeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* const & QVector<Qt3DCore::QNode*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_index_const(self as *const ::vector::VectorNodeMutPtr,
                                                                            i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode*& QVector<Qt3DCore::QNode*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_index(self as *mut ::vector::VectorNodeMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::operator!=(const QVector<Qt3DCore::QNode*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorNodeMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_neq(self as *const ::vector::VectorNodeMutPtr,
                                                                  v as *const ::vector::VectorNodeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::operator<<(const QVector<Qt3DCore::QNode*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::vector::VectorNodeMutPtr) -> &'l0 mut ::vector::VectorNodeMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_shl_l(self as *mut ::vector::VectorNodeMutPtr,
                                                                      l as *const ::vector::VectorNodeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>& QVector<Qt3DCore::QNode*>::operator<<(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::node::Node)
                                        -> &'l0 mut ::vector::VectorNodeMutPtr {
    let ffi_result =
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_operator_shl_t(self as *mut ::vector::VectorNodeMutPtr,
                                                                    t as *const *mut ::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_pop_back(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_pop_front(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::prepend(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::node::Node) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_prepend(self as *mut ::vector::VectorNodeMutPtr,
                                                           t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::push_back(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::node::Node) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_push_back(self as *mut ::vector::VectorNodeMutPtr,
                                                             t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::push_front(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::node::Node) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_push_front(self as *mut ::vector::VectorNodeMutPtr,
                                                              t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNode*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::removeAll(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::node::Node) -> ::libc::c_int {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_removeAll(self as *mut ::vector::VectorNodeMutPtr,
                                                             t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_removeAt(self as *mut ::vector::VectorNodeMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_removeFirst(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_removeLast(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::removeOne(Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::node::Node) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_removeOne(self as *mut ::vector::VectorNodeMutPtr,
                                                             t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::replace(int i, Qt3DCore::QNode* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::node::Node) {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_replace(self as *mut ::vector::VectorNodeMutPtr,
                                                           i,
                                                           t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_reserve(self as *mut ::vector::VectorNodeMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_resize(self as *mut ::vector::VectorNodeMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNode*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_size(self as *const ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_squeeze(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNode*>::startsWith(Qt3DCore::QNode* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::node::Node) -> bool {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_startsWith(self as *const ::vector::VectorNodeMutPtr,
                                                              t as *const *mut ::node::Node)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNode*>::swap(QVector<Qt3DCore::QNode*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorNodeMutPtr) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_swap(self as *mut ::vector::VectorNodeMutPtr,
                                                          other as *mut ::vector::VectorNodeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* QVector<Qt3DCore::QNode*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_takeAt(self as *mut ::vector::VectorNodeMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* QVector<Qt3DCore::QNode*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_takeFirst(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* QVector<Qt3DCore::QNode*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_takeLast(self as *mut ::vector::VectorNodeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* QVector<Qt3DCore::QNode*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::node::Node {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_value_i(self as *const ::vector::VectorNodeMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNode* QVector<Qt3DCore::QNode*>::value(int i, Qt3DCore::QNode* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::node::Node) -> *mut ::node::Node {
    ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_value_i_defaultValue(self as *const ::vector::VectorNodeMutPtr,
                                                                        i,
                                                                        default_value as *const *mut ::node::Node)
  }
}

impl Drop for ::vector::VectorNodeMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QNode*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_destructor(self as *mut ::vector::VectorNodeMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>```</span>
#[repr(C)]
pub struct VectorNodeNodeIdTypePair([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_NODE_NODE_ID_TYPE_PAIR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorNodeNodeIdTypePair {
  unsafe fn new_uninitialized() -> VectorNodeNodeIdTypePair {
    VectorNodeNodeIdTypePair(::std::mem::uninitialized())
  }
}

impl VectorNodeNodeIdTypePair {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorNodeNodeIdTypePair) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::append(const QVector<Qt3DCore::QNodeIdTypePair>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::node::NodeIdTypePair) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::append(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeNodeIdTypePairAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_at(self as *const ::vector::VectorNodeNodeIdTypePair, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_back_const(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node::NodeIdTypePair {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_back(self as *mut ::vector::VectorNodeNodeIdTypePair)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeIdTypePair>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_capacity(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_clear(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair* QVector<Qt3DCore::QNodeIdTypePair>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::node::NodeIdTypePair {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constData(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constFirst(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constLast(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeIdTypePair>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_count(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair* QVector<Qt3DCore::QNodeIdTypePair>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::node::NodeIdTypePair {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_data_const(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair* QVector<Qt3DCore::QNodeIdTypePair>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::node::NodeIdTypePair {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_data(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeIdTypePair>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_empty(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::node::NodeIdTypePair) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::fill(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::node::NodeIdTypePair, ::libc::c_int)) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::fill(const Qt3DCore::QNodeIdTypePair& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeNodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_first_const(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node::NodeIdTypePair {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_first(self as *mut ::vector::VectorNodeNodeIdTypePair)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_front_const(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node::NodeIdTypePair {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_front(self as *mut ::vector::VectorNodeNodeIdTypePair)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::node::NodeIdTypePair)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::insert(int i, const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::node::NodeIdTypePair)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::insert(int i, int n, const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeNodeIdTypePairInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DCore::QNodeIdTypePair>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_isEmpty(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_last_const(self as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::node::NodeIdTypePair {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_last(self as *mut ::vector::VectorNodeNodeIdTypePair)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeIdTypePair>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_length(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair> QVector<Qt3DCore::QNodeIdTypePair>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair> QVector<Qt3DCore::QNodeIdTypePair>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorNodeNodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_move(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                from,
                                                                to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeIdTypePair>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorNodeNodeIdTypePair) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeIdTypePair>::QVector(const QVector<Qt3DCore::QNodeIdTypePair>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeIdTypePair>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::node::NodeIdTypePair)) -> ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DCore::QNodeIdTypePair>::QVector(int size, const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorNodeNodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair> QVector<Qt3DCore::QNodeIdTypePair>::operator+(const QVector<Qt3DCore::QNodeIdTypePair>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorNodeNodeIdTypePair) -> ::vector::VectorNodeNodeIdTypePair {
    {
      let mut object: ::vector::VectorNodeNodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_add_to_output(self as *const ::vector::VectorNodeNodeIdTypePair, l as *const ::vector::VectorNodeNodeIdTypePair, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorNodeNodeIdTypePair) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::operator+=(const QVector<Qt3DCore::QNodeIdTypePair>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::node::NodeIdTypePair) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::operator+=(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeNodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::operator=(const QVector<Qt3DCore::QNodeIdTypePair>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorNodeNodeIdTypePair)
                             -> &'l0 mut ::vector::VectorNodeNodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_assign(self as *mut ::vector::VectorNodeNodeIdTypePair, v as *const ::vector::VectorNodeNodeIdTypePair) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_index_const(self as *const ::vector::VectorNodeNodeIdTypePair, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair& QVector<Qt3DCore::QNodeIdTypePair>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::node::NodeIdTypePair {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_index(self as *mut ::vector::VectorNodeNodeIdTypePair, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorNodeNodeIdTypePair) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::operator<<(const QVector<Qt3DCore::QNodeIdTypePair>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::node::NodeIdTypePair) -> &'l0 mut ::vector::VectorNodeNodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>& QVector<Qt3DCore::QNodeIdTypePair>::operator<<(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorNodeNodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_pop_back(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_pop_front(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::prepend(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::node::NodeIdTypePair) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_prepend(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                   t as *const ::node::NodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::push_back(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::node::NodeIdTypePair) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_push_back(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                     t as *const ::node::NodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::push_front(const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::node::NodeIdTypePair) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_push_front(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                      t as *const ::node::NodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorNodeNodeIdTypePairRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_removeAt(self as *mut ::vector::VectorNodeNodeIdTypePair, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_removeFirst(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_removeLast(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::replace(int i, const Qt3DCore::QNodeIdTypePair& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::node::NodeIdTypePair) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_replace(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                   i,
                                                                   t as *const ::node::NodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_reserve(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                   size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_resize(self as *mut ::vector::VectorNodeNodeIdTypePair, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DCore::QNodeIdTypePair>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_size(self as *const ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_squeeze(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DCore::QNodeIdTypePair>::swap(QVector<Qt3DCore::QNodeIdTypePair>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorNodeNodeIdTypePair) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_swap(self as *mut ::vector::VectorNodeNodeIdTypePair,
                                                                other as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair QVector<Qt3DCore::QNodeIdTypePair>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::node::NodeIdTypePair {
    {
      let mut object: ::node::NodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_takeAt_to_output(self as *mut ::vector::VectorNodeNodeIdTypePair, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair QVector<Qt3DCore::QNodeIdTypePair>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::node::NodeIdTypePair {
    {
      let mut object: ::node::NodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_takeFirst_to_output(self as *mut ::vector::VectorNodeNodeIdTypePair, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair QVector<Qt3DCore::QNodeIdTypePair>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::node::NodeIdTypePair {
    {
      let mut object: ::node::NodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_takeLast_to_output(self as *mut ::vector::VectorNodeNodeIdTypePair, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::node::NodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair QVector<Qt3DCore::QNodeIdTypePair>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::node::NodeIdTypePair)) -> ::node::NodeIdTypePair```<br>
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeIdTypePair QVector<Qt3DCore::QNodeIdTypePair>::value(int i, const Qt3DCore::QNodeIdTypePair& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::node::NodeIdTypePair
    where Args: overloading::VectorNodeNodeIdTypePairValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorNodeNodeIdTypePair {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DCore::QNodeIdTypePair>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_destructor(self as *mut ::vector::VectorNodeNodeIdTypePair)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>```</span>
#[repr(C)]
pub struct VectorSharedPointerSharedPointerAspectJob([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_SHARED_POINTER_SHARED_POINTER_ASPECT_JOB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorSharedPointerSharedPointerAspectJob {
  unsafe fn new_uninitialized() -> VectorSharedPointerSharedPointerAspectJob {
    VectorSharedPointerSharedPointerAspectJob(::std::mem::uninitialized())
  }
}

impl VectorSharedPointerSharedPointerAspectJob {
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::shared_pointer::SharedPointerAspectJob) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::append(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorSharedPointerSharedPointerAspectJob) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::append(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_at(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_back_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_back(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_capacity(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_clear(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>* QVector<QSharedPointer<Qt3DCore::QAspectJob>>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::shared_pointer::SharedPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constData(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constFirst(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constLast(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::contains(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::shared_pointer::SharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_contains(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::shared_pointer::SharedPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::count(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>* QVector<QSharedPointer<Qt3DCore::QAspectJob>>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::shared_pointer::SharedPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_data_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>* QVector<QSharedPointer<Qt3DCore::QAspectJob>>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::shared_pointer::SharedPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_data(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_empty(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::endsWith(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::shared_pointer::SharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_endsWith(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::shared_pointer::SharedPointerAspectJob) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::fill(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::shared_pointer::SharedPointerAspectJob, ::libc::c_int)) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::fill(const QSharedPointer<Qt3DCore::QAspectJob>& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self,
                            args: Args)
                            -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_first_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_first(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_front_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_front(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::shared_pointer::SharedPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::indexOf(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::shared_pointer::SharedPointerAspectJob, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::indexOf(const QSharedPointer<Qt3DCore::QAspectJob>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::shared_pointer::SharedPointerAspectJob)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::insert(int i, const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::shared_pointer::SharedPointerAspectJob)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::insert(int i, int n, const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_isEmpty(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_last_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::shared_pointer::SharedPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::lastIndexOf(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::shared_pointer::SharedPointerAspectJob, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::lastIndexOf(const QSharedPointer<Qt3DCore::QAspectJob>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_last(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_length(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorSharedPointerSharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_move(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorSharedPointerSharedPointerAspectJob) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::QVector(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::shared_pointer::SharedPointerAspectJob)) -> ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::QVector(int size, const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorSharedPointerSharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator+(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorSharedPointerSharedPointerAspectJob)
                -> ::vector::VectorSharedPointerSharedPointerAspectJob {
    {
      let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_add_to_output(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, l as *const ::vector::VectorSharedPointerSharedPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::shared_pointer::SharedPointerAspectJob) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator+=(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator+=(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator=(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorSharedPointerSharedPointerAspectJob)
                             -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_assign(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, v as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator==(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorSharedPointerSharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_eq(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, v as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_index_const(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::shared_pointer::SharedPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_index(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator!=(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorSharedPointerSharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_neq(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, v as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::shared_pointer::SharedPointerAspectJob) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator<<(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>& QVector<QSharedPointer<Qt3DCore::QAspectJob>>::operator<<(const QVector<QSharedPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self,
                              args: Args)
                              -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_pop_back(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_pop_front(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::prepend(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::shared_pointer::SharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_prepend(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::push_back(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::shared_pointer::SharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_push_back(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::push_front(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::shared_pointer::SharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_push_front(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::removeAll(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::shared_pointer::SharedPointerAspectJob) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_removeAll(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_removeAt(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_removeFirst(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_removeLast(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::removeOne(const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::shared_pointer::SharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_removeOne(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::replace(int i, const QSharedPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::shared_pointer::SharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_replace(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_reserve(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_resize(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DCore::QAspectJob>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_size(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_squeeze(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DCore::QAspectJob>>::startsWith(const QSharedPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::shared_pointer::SharedPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_startsWith(self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::swap(QVector<QSharedPointer<Qt3DCore::QAspectJob>>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorSharedPointerSharedPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_swap(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, other as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::shared_pointer::SharedPointerAspectJob {
    {
      let mut object: ::shared_pointer::SharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_takeAt_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::shared_pointer::SharedPointerAspectJob {
    {
      let mut object: ::shared_pointer::SharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_takeFirst_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::shared_pointer::SharedPointerAspectJob {
    {
      let mut object: ::shared_pointer::SharedPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_takeLast_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DCore::QAspectJob>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::shared_pointer::SharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::shared_pointer::SharedPointerAspectJob)) -> ::shared_pointer::SharedPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DCore::QAspectJob> QVector<QSharedPointer<Qt3DCore::QAspectJob>>::value(int i, const QSharedPointer<Qt3DCore::QAspectJob>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::shared_pointer::SharedPointerAspectJob
    where Args: overloading::VectorSharedPointerSharedPointerAspectJobValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorSharedPointerSharedPointerAspectJob {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QSharedPointer<Qt3DCore::QAspectJob>>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_destructor(self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>```</span>
#[repr(C)]
pub struct VectorWeakPointerWeakPointerAspectJob([u8; ::type_sizes::QT_3D_CORE_VECTOR_VECTOR_WEAK_POINTER_WEAK_POINTER_ASPECT_JOB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorWeakPointerWeakPointerAspectJob {
  unsafe fn new_uninitialized() -> VectorWeakPointerWeakPointerAspectJob {
    VectorWeakPointerWeakPointerAspectJob(::std::mem::uninitialized())
  }
}

impl VectorWeakPointerWeakPointerAspectJob {
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorWeakPointerWeakPointerAspectJob) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::append(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::weak_pointer::WeakPointerAspectJob) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::append(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_at(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_back_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_back(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_capacity(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_clear(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>* QVector<QWeakPointer<Qt3DCore::QAspectJob>>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::weak_pointer::WeakPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constData(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constFirst(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constLast(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::contains(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::weak_pointer::WeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_contains(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::weak_pointer::WeakPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::count(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>* QVector<QWeakPointer<Qt3DCore::QAspectJob>>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::weak_pointer::WeakPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_data_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>* QVector<QWeakPointer<Qt3DCore::QAspectJob>>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::weak_pointer::WeakPointerAspectJob {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_data(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_empty(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::endsWith(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::weak_pointer::WeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_endsWith(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::weak_pointer::WeakPointerAspectJob) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::fill(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::weak_pointer::WeakPointerAspectJob, ::libc::c_int)) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::fill(const QWeakPointer<Qt3DCore::QAspectJob>& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_first_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_first(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_front_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_front(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::weak_pointer::WeakPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::indexOf(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::weak_pointer::WeakPointerAspectJob, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::indexOf(const QWeakPointer<Qt3DCore::QAspectJob>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::weak_pointer::WeakPointerAspectJob)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::insert(int i, const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::weak_pointer::WeakPointerAspectJob)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::insert(int i, int n, const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_isEmpty(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_last_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::weak_pointer::WeakPointerAspectJob) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::lastIndexOf(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::weak_pointer::WeakPointerAspectJob, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::lastIndexOf(const QWeakPointer<Qt3DCore::QAspectJob>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_last(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_length(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorWeakPointerWeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_move(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorWeakPointerWeakPointerAspectJob) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::QVector(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::weak_pointer::WeakPointerAspectJob)) -> ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::QVector(int size, const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorWeakPointerWeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator+(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorWeakPointerWeakPointerAspectJob)
                -> ::vector::VectorWeakPointerWeakPointerAspectJob {
    {
      let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_add_to_output(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, l as *const ::vector::VectorWeakPointerWeakPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator+=(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::weak_pointer::WeakPointerAspectJob) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator+=(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator=(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorWeakPointerWeakPointerAspectJob)
                             -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_assign(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, v as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator==(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorWeakPointerWeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_eq(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, v as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```const QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_index_const(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::weak_pointer::WeakPointerAspectJob {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_index(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator!=(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorWeakPointerWeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_neq(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, v as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator<<(const QVector<QWeakPointer<Qt3DCore::QAspectJob>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::weak_pointer::WeakPointerAspectJob) -> &'l0 mut ::vector::VectorWeakPointerWeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>& QVector<QWeakPointer<Qt3DCore::QAspectJob>>::operator<<(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self,
                              args: Args)
                              -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_pop_back(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_pop_front(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::prepend(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::weak_pointer::WeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_prepend(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::push_back(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::weak_pointer::WeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_push_back(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::push_front(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::weak_pointer::WeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_push_front(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::removeAll(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::weak_pointer::WeakPointerAspectJob) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_removeAll(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_removeAt(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_removeFirst(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_removeLast(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::removeOne(const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::weak_pointer::WeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_removeOne(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::replace(int i, const QWeakPointer<Qt3DCore::QAspectJob>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::weak_pointer::WeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_replace(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_reserve(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_resize(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QWeakPointer<Qt3DCore::QAspectJob>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_size(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_squeeze(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QWeakPointer<Qt3DCore::QAspectJob>>::startsWith(const QWeakPointer<Qt3DCore::QAspectJob>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::weak_pointer::WeakPointerAspectJob) -> bool {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_startsWith(self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::swap(QVector<QWeakPointer<Qt3DCore::QAspectJob>>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorWeakPointerWeakPointerAspectJob) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_swap(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, other as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::weak_pointer::WeakPointerAspectJob {
    {
      let mut object: ::weak_pointer::WeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_takeAt_to_output(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::weak_pointer::WeakPointerAspectJob {
    {
      let mut object: ::weak_pointer::WeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_takeFirst_to_output(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::weak_pointer::WeakPointerAspectJob {
    {
      let mut object: ::weak_pointer::WeakPointerAspectJob =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_takeLast_to_output(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QWeakPointer<Qt3DCore::QAspectJob>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::weak_pointer::WeakPointerAspectJob)) -> ::weak_pointer::WeakPointerAspectJob```<br>
  /// C++ method: <span style='color: green;'>```QWeakPointer<Qt3DCore::QAspectJob> QVector<QWeakPointer<Qt3DCore::QAspectJob>>::value(int i, const QWeakPointer<Qt3DCore::QAspectJob>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::weak_pointer::WeakPointerAspectJob
    where Args: overloading::VectorWeakPointerWeakPointerAspectJobValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorWeakPointerWeakPointerAspectJob {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QWeakPointer<Qt3DCore::QAspectJob>>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_destructor(self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::fill](../struct.VectorAbstractAspectMutPtr.html#method.fill) method.
  pub trait VectorAbstractAspectMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr)
                   -> &'largs mut ::vector::VectorAbstractAspectMutPtr;
  }
  impl<'largs> VectorAbstractAspectMutPtrFillArgs<'largs> for &'largs *mut ::abstract_aspect::AbstractAspect {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr)
                   -> &'largs mut ::vector::VectorAbstractAspectMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_fill_t(original_self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrFillArgs<'largs>
    for (&'largs *mut ::abstract_aspect::AbstractAspect, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr)
                   -> &'largs mut ::vector::VectorAbstractAspectMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_fill_t_size(original_self as *mut ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::index_of](../struct.VectorAbstractAspectMutPtr.html#method.index_of) method.
  pub trait VectorAbstractAspectMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractAspectMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_aspect::AbstractAspect {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_indexOf_t(original_self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_aspect::AbstractAspect, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_indexOf_t_from(original_self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::insert](../struct.VectorAbstractAspectMutPtr.html#method.insert) method.
  pub trait VectorAbstractAspectMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> ();
  }
  impl<'largs> VectorAbstractAspectMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::abstract_aspect::AbstractAspect) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAbstractAspectMutPtr, i, n, t as *const *mut ::abstract_aspect::AbstractAspect)
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrInsertArgs<'largs>
    for (::libc::c_int, &'largs *mut ::abstract_aspect::AbstractAspect) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_insert_i_t(original_self as *mut ::vector::VectorAbstractAspectMutPtr, i, t as *const *mut ::abstract_aspect::AbstractAspect)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::last_index_of](../struct.VectorAbstractAspectMutPtr.html#method.last_index_of) method.
  pub trait VectorAbstractAspectMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractAspectMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_aspect::AbstractAspect {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect)
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_aspect::AbstractAspect, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAbstractAspectMutPtr, t as *const *mut ::abstract_aspect::AbstractAspect, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::mid](../struct.VectorAbstractAspectMutPtr.html#method.mid) method.
  pub trait VectorAbstractAspectMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::vector::VectorAbstractAspectMutPtr;
  }
  impl<'largs> VectorAbstractAspectMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::vector::VectorAbstractAspectMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAbstractAspectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAbstractAspectMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorAbstractAspectMutPtr) -> ::vector::VectorAbstractAspectMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAbstractAspectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAbstractAspectMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::new](../struct.VectorAbstractAspectMutPtr.html#method.new) method.
  pub trait VectorAbstractAspectMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAbstractAspectMutPtr;
  }
  impl VectorAbstractAspectMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAbstractAspectMutPtr {

      {
        let mut object: ::vector::VectorAbstractAspectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAbstractAspectMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAbstractAspectMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAbstractAspectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractAspectMutPtrNewArgs for &'a ::vector::VectorAbstractAspectMutPtr {
    fn exec(self) -> ::vector::VectorAbstractAspectMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAbstractAspectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_constructor_v(v as *const ::vector::VectorAbstractAspectMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAspectMutPtr::remove](../struct.VectorAbstractAspectMutPtr.html#method.remove) method.
  pub trait VectorAbstractAspectMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> ();
  }
  impl<'largs> VectorAbstractAspectMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_remove_i(original_self as *mut ::vector::VectorAbstractAspectMutPtr, i) }
    }
  }
  impl<'largs> VectorAbstractAspectMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAspectMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QAbstractAspect_ptr_remove_i_n(original_self as *mut ::vector::VectorAbstractAspectMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::fill](../struct.VectorComponentMutPtr.html#method.fill) method.
  pub trait VectorComponentMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorComponentMutPtr)
                   -> &'largs mut ::vector::VectorComponentMutPtr;
  }
  impl<'largs> VectorComponentMutPtrFillArgs<'largs> for &'largs *mut ::component::Component {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorComponentMutPtr)
                   -> &'largs mut ::vector::VectorComponentMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_fill_t(original_self as *mut ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorComponentMutPtrFillArgs<'largs> for (&'largs *mut ::component::Component, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorComponentMutPtr)
                   -> &'largs mut ::vector::VectorComponentMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_fill_t_size(original_self as *mut ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::index_of](../struct.VectorComponentMutPtr.html#method.index_of) method.
  pub trait VectorComponentMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorComponentMutPtrIndexOfArgs<'largs> for &'largs *mut ::component::Component {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_indexOf_t(original_self as *const ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component)
    }
  }
  impl<'largs> VectorComponentMutPtrIndexOfArgs<'largs> for (&'largs *mut ::component::Component, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_indexOf_t_from(original_self as *const ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::insert](../struct.VectorComponentMutPtr.html#method.insert) method.
  pub trait VectorComponentMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> ();
  }
  impl<'largs> VectorComponentMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::component::Component) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_insert_i_n_t(original_self as *mut ::vector::VectorComponentMutPtr, i, n, t as *const *mut ::component::Component)
    }
  }
  impl<'largs> VectorComponentMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::component::Component) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_insert_i_t(original_self as *mut ::vector::VectorComponentMutPtr, i, t as *const *mut ::component::Component)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::last_index_of](../struct.VectorComponentMutPtr.html#method.last_index_of) method.
  pub trait VectorComponentMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorComponentMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::component::Component {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_lastIndexOf_t(original_self as *const ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component)
    }
  }
  impl<'largs> VectorComponentMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::component::Component, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorComponentMutPtr, t as *const *mut ::component::Component, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::mid](../struct.VectorComponentMutPtr.html#method.mid) method.
  pub trait VectorComponentMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::vector::VectorComponentMutPtr;
  }
  impl<'largs> VectorComponentMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::vector::VectorComponentMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorComponentMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_mid_to_output_pos(original_self as *const ::vector::VectorComponentMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorComponentMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorComponentMutPtr) -> ::vector::VectorComponentMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorComponentMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorComponentMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::new](../struct.VectorComponentMutPtr.html#method.new) method.
  pub trait VectorComponentMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorComponentMutPtr;
  }
  impl VectorComponentMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorComponentMutPtr {

      {
        let mut object: ::vector::VectorComponentMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorComponentMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorComponentMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorComponentMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorComponentMutPtrNewArgs for &'a ::vector::VectorComponentMutPtr {
    fn exec(self) -> ::vector::VectorComponentMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorComponentMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_constructor_v(v as *const ::vector::VectorComponentMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorComponentMutPtr::remove](../struct.VectorComponentMutPtr.html#method.remove) method.
  pub trait VectorComponentMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> ();
  }
  impl<'largs> VectorComponentMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_remove_i(original_self as *mut ::vector::VectorComponentMutPtr, i) }
    }
  }
  impl<'largs> VectorComponentMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorComponentMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QComponent_ptr_remove_i_n(original_self as *mut ::vector::VectorComponentMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::fill](../struct.VectorEntityMutPtr.html#method.fill) method.
  pub trait VectorEntityMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorEntityMutPtr)
                   -> &'largs mut ::vector::VectorEntityMutPtr;
  }
  impl<'largs> VectorEntityMutPtrFillArgs<'largs> for &'largs *mut ::entity::Entity {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorEntityMutPtr)
                   -> &'largs mut ::vector::VectorEntityMutPtr {
      let t = self;
      let ffi_result =
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_fill_t(original_self as *mut ::vector::VectorEntityMutPtr,
                                                                t as *const *mut ::entity::Entity);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorEntityMutPtrFillArgs<'largs> for (&'largs *mut ::entity::Entity, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorEntityMutPtr)
                   -> &'largs mut ::vector::VectorEntityMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_fill_t_size(original_self as *mut ::vector::VectorEntityMutPtr, t as *const *mut ::entity::Entity, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::index_of](../struct.VectorEntityMutPtr.html#method.index_of) method.
  pub trait VectorEntityMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorEntityMutPtrIndexOfArgs<'largs> for &'largs *mut ::entity::Entity {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_indexOf_t(original_self as *const ::vector::VectorEntityMutPtr,
                                                                 t as *const *mut ::entity::Entity)
    }
  }
  impl<'largs> VectorEntityMutPtrIndexOfArgs<'largs> for (&'largs *mut ::entity::Entity, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_indexOf_t_from(original_self as *const ::vector::VectorEntityMutPtr, t as *const *mut ::entity::Entity, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::insert](../struct.VectorEntityMutPtr.html#method.insert) method.
  pub trait VectorEntityMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> ();
  }
  impl<'largs> VectorEntityMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::entity::Entity) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_insert_i_n_t(original_self as *mut ::vector::VectorEntityMutPtr, i, n, t as *const *mut ::entity::Entity)
    }
  }
  impl<'largs> VectorEntityMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::entity::Entity) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_insert_i_t(original_self as *mut ::vector::VectorEntityMutPtr,
                                                                  i,
                                                                  t as *const *mut ::entity::Entity)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::last_index_of](../struct.VectorEntityMutPtr.html#method.last_index_of) method.
  pub trait VectorEntityMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorEntityMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::entity::Entity {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_lastIndexOf_t(original_self as *const ::vector::VectorEntityMutPtr, t as *const *mut ::entity::Entity)
    }
  }
  impl<'largs> VectorEntityMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::entity::Entity, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorEntityMutPtr, t as *const *mut ::entity::Entity, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::mid](../struct.VectorEntityMutPtr.html#method.mid) method.
  pub trait VectorEntityMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::vector::VectorEntityMutPtr;
  }
  impl<'largs> VectorEntityMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::vector::VectorEntityMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorEntityMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_mid_to_output_pos(original_self as *const ::vector::VectorEntityMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorEntityMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorEntityMutPtr) -> ::vector::VectorEntityMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorEntityMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorEntityMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::new](../struct.VectorEntityMutPtr.html#method.new) method.
  pub trait VectorEntityMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorEntityMutPtr;
  }
  impl VectorEntityMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorEntityMutPtr {

      {
        let mut object: ::vector::VectorEntityMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorEntityMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorEntityMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorEntityMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorEntityMutPtrNewArgs for &'a ::vector::VectorEntityMutPtr {
    fn exec(self) -> ::vector::VectorEntityMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorEntityMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_constructor_v(v as *const ::vector::VectorEntityMutPtr,
                                                                         &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorEntityMutPtr::remove](../struct.VectorEntityMutPtr.html#method.remove) method.
  pub trait VectorEntityMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> ();
  }
  impl<'largs> VectorEntityMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_remove_i(original_self as *mut ::vector::VectorEntityMutPtr, i)
      }
    }
  }
  impl<'largs> VectorEntityMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorEntityMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QEntity_ptr_remove_i_n(original_self as *mut ::vector::VectorEntityMutPtr, i, n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::append](../struct.VectorNodeId.html#method.append) method.
  pub trait VectorNodeIdAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> ();
  }
  impl<'largs> VectorNodeIdAppendArgs<'largs> for &'largs ::vector::VectorNodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_append_l(original_self as *mut ::vector::VectorNodeId,
                                                              l as *const ::vector::VectorNodeId)
      }
    }
  }
  impl<'largs> VectorNodeIdAppendArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_append_t(original_self as *mut ::vector::VectorNodeId,
                                                              t as *const ::node_id::NodeId)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::count](../struct.VectorNodeId.html#method.count) method.
  pub trait VectorNodeIdCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int;
  }
  impl<'largs> VectorNodeIdCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_count_no_args(original_self as *const ::vector::VectorNodeId)
      }
    }
  }
  impl<'largs> VectorNodeIdCountArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_count_t(original_self as *const ::vector::VectorNodeId,
                                                             t as *const ::node_id::NodeId)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::fill](../struct.VectorNodeId.html#method.fill) method.
  pub trait VectorNodeIdFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId;
  }
  impl<'largs> VectorNodeIdFillArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_fill_t(original_self as *mut ::vector::VectorNodeId,
                                                              t as *const ::node_id::NodeId)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeIdFillArgs<'largs> for (&'largs ::node_id::NodeId, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_fill_t_size(original_self as *mut ::vector::VectorNodeId,
                                                                   t as *const ::node_id::NodeId,
                                                                   size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::index_of](../struct.VectorNodeId.html#method.index_of) method.
  pub trait VectorNodeIdIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int;
  }
  impl<'largs> VectorNodeIdIndexOfArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_indexOf_t(original_self as *const ::vector::VectorNodeId,
                                                               t as *const ::node_id::NodeId)
      }
    }
  }
  impl<'largs> VectorNodeIdIndexOfArgs<'largs> for (&'largs ::node_id::NodeId, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_indexOf_t_from(original_self as *const ::vector::VectorNodeId,
                                                                    t as *const ::node_id::NodeId,
                                                                    from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::insert](../struct.VectorNodeId.html#method.insert) method.
  pub trait VectorNodeIdInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> ();
  }
  impl<'largs> VectorNodeIdInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::node_id::NodeId) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_insert_i_n_t(original_self as *mut ::vector::VectorNodeId,
                                                                  i,
                                                                  n,
                                                                  t as *const ::node_id::NodeId)
      }
    }
  }
  impl<'largs> VectorNodeIdInsertArgs<'largs> for (::libc::c_int, &'largs ::node_id::NodeId) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_insert_i_t(original_self as *mut ::vector::VectorNodeId,
                                                                i,
                                                                t as *const ::node_id::NodeId)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::last_index_of](../struct.VectorNodeId.html#method.last_index_of) method.
  pub trait VectorNodeIdLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int;
  }
  impl<'largs> VectorNodeIdLastIndexOfArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_lastIndexOf_t(original_self as *const ::vector::VectorNodeId,
                                                                   t as *const ::node_id::NodeId)
      }
    }
  }
  impl<'largs> VectorNodeIdLastIndexOfArgs<'largs> for (&'largs ::node_id::NodeId, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_lastIndexOf_t_from(original_self as *const ::vector::VectorNodeId, t as *const ::node_id::NodeId, from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::mid](../struct.VectorNodeId.html#method.mid) method.
  pub trait VectorNodeIdMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::vector::VectorNodeId;
  }
  impl<'largs> VectorNodeIdMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::vector::VectorNodeId {
      let pos = self;
      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_mid_to_output_pos(original_self as *const ::vector::VectorNodeId, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorNodeIdMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::vector::VectorNodeId {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_mid_to_output_pos_len(original_self as *const ::vector::VectorNodeId, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::new](../struct.VectorNodeId.html#method.new) method.
  pub trait VectorNodeIdNewArgs {
    fn exec(self) -> ::vector::VectorNodeId;
  }
  impl VectorNodeIdNewArgs for () {
    fn exec(self) -> ::vector::VectorNodeId {

      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorNodeIdNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorNodeId {
      let size = self;
      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorNodeIdNewArgs for (::libc::c_int, &'a ::node_id::NodeId) {
    fn exec(self) -> ::vector::VectorNodeId {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constructor_size_t(size,
                                                                          t as *const ::node_id::NodeId,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorNodeIdNewArgs for &'a ::vector::VectorNodeId {
    fn exec(self) -> ::vector::VectorNodeId {
      let v = self;
      {
        let mut object: ::vector::VectorNodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_constructor_v(v as *const ::vector::VectorNodeId, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::op_add_assign](../struct.VectorNodeId.html#method.op_add_assign) method.
  pub trait VectorNodeIdOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId;
  }
  impl<'largs> VectorNodeIdOpAddAssignArgs<'largs> for &'largs ::vector::VectorNodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_add_assign_l(original_self as *mut ::vector::VectorNodeId, l as *const ::vector::VectorNodeId) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeIdOpAddAssignArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_add_assign_t(original_self as *mut ::vector::VectorNodeId, t as *const ::node_id::NodeId) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::op_shl](../struct.VectorNodeId.html#method.op_shl) method.
  pub trait VectorNodeIdOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId;
  }
  impl<'largs> VectorNodeIdOpShlArgs<'largs> for &'largs ::vector::VectorNodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_shl_l(original_self as *mut ::vector::VectorNodeId,
                                                                      l as *const ::vector::VectorNodeId)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeIdOpShlArgs<'largs> for &'largs ::node_id::NodeId {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> &'largs mut ::vector::VectorNodeId {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_operator_shl_t(original_self as *mut ::vector::VectorNodeId,
                                                                      t as *const ::node_id::NodeId)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::remove](../struct.VectorNodeId.html#method.remove) method.
  pub trait VectorNodeIdRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> ();
  }
  impl<'largs> VectorNodeIdRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_remove_i(original_self as *mut ::vector::VectorNodeId, i) }
    }
  }
  impl<'largs> VectorNodeIdRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeId) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_remove_i_n(original_self as *mut ::vector::VectorNodeId, i, n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeId::value](../struct.VectorNodeId.html#method.value) method.
  pub trait VectorNodeIdValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::node_id::NodeId;
  }
  impl<'largs> VectorNodeIdValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::node_id::NodeId {
      let i = self;
      {
        let mut object: ::node_id::NodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_value_to_output_i(original_self as *const ::vector::VectorNodeId, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorNodeIdValueArgs<'largs> for (::libc::c_int, &'largs ::node_id::NodeId) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeId) -> ::node_id::NodeId {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::node_id::NodeId =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeId_value_to_output_i_defaultValue(original_self as *const ::vector::VectorNodeId, i, default_value as *const ::node_id::NodeId, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::fill](../struct.VectorNodeMutPtr.html#method.fill) method.
  pub trait VectorNodeMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorNodeMutPtr)
                   -> &'largs mut ::vector::VectorNodeMutPtr;
  }
  impl<'largs> VectorNodeMutPtrFillArgs<'largs> for &'largs *mut ::node::Node {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorNodeMutPtr)
                   -> &'largs mut ::vector::VectorNodeMutPtr {
      let t = self;
      let ffi_result =
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_fill_t(original_self as *mut ::vector::VectorNodeMutPtr,
                                                              t as *const *mut ::node::Node);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeMutPtrFillArgs<'largs> for (&'largs *mut ::node::Node, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorNodeMutPtr)
                   -> &'largs mut ::vector::VectorNodeMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_fill_t_size(original_self as *mut ::vector::VectorNodeMutPtr,
                                                                   t as *const *mut ::node::Node,
                                                                   size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::index_of](../struct.VectorNodeMutPtr.html#method.index_of) method.
  pub trait VectorNodeMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorNodeMutPtrIndexOfArgs<'largs> for &'largs *mut ::node::Node {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_indexOf_t(original_self as *const ::vector::VectorNodeMutPtr,
                                                               t as *const *mut ::node::Node)
    }
  }
  impl<'largs> VectorNodeMutPtrIndexOfArgs<'largs> for (&'largs *mut ::node::Node, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_indexOf_t_from(original_self as *const ::vector::VectorNodeMutPtr, t as *const *mut ::node::Node, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::insert](../struct.VectorNodeMutPtr.html#method.insert) method.
  pub trait VectorNodeMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> ();
  }
  impl<'largs> VectorNodeMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::node::Node) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_insert_i_n_t(original_self as *mut ::vector::VectorNodeMutPtr,
                                                                  i,
                                                                  n,
                                                                  t as *const *mut ::node::Node)
    }
  }
  impl<'largs> VectorNodeMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::node::Node) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_insert_i_t(original_self as *mut ::vector::VectorNodeMutPtr,
                                                                i,
                                                                t as *const *mut ::node::Node)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::last_index_of](../struct.VectorNodeMutPtr.html#method.last_index_of) method.
  pub trait VectorNodeMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorNodeMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::node::Node {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_lastIndexOf_t(original_self as *const ::vector::VectorNodeMutPtr,
                                                                   t as *const *mut ::node::Node)
    }
  }
  impl<'largs> VectorNodeMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::node::Node, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorNodeMutPtr, t as *const *mut ::node::Node, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::mid](../struct.VectorNodeMutPtr.html#method.mid) method.
  pub trait VectorNodeMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::vector::VectorNodeMutPtr;
  }
  impl<'largs> VectorNodeMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::vector::VectorNodeMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorNodeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_mid_to_output_pos(original_self as *const ::vector::VectorNodeMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorNodeMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeMutPtr) -> ::vector::VectorNodeMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorNodeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorNodeMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::new](../struct.VectorNodeMutPtr.html#method.new) method.
  pub trait VectorNodeMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorNodeMutPtr;
  }
  impl VectorNodeMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorNodeMutPtr {

      {
        let mut object: ::vector::VectorNodeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorNodeMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorNodeMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorNodeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorNodeMutPtrNewArgs for &'a ::vector::VectorNodeMutPtr {
    fn exec(self) -> ::vector::VectorNodeMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorNodeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_constructor_v(v as *const ::vector::VectorNodeMutPtr,
                                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeMutPtr::remove](../struct.VectorNodeMutPtr.html#method.remove) method.
  pub trait VectorNodeMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> ();
  }
  impl<'largs> VectorNodeMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_remove_i(original_self as *mut ::vector::VectorNodeMutPtr, i)
      }
    }
  }
  impl<'largs> VectorNodeMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNode_ptr_remove_i_n(original_self as *mut ::vector::VectorNodeMutPtr,
                                                                  i,
                                                                  n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::append](../struct.VectorNodeNodeIdTypePair.html#method.append) method.
  pub trait VectorNodeNodeIdTypePairAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> ();
  }
  impl<'largs> VectorNodeNodeIdTypePairAppendArgs<'largs> for &'largs ::vector::VectorNodeNodeIdTypePair {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let l = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_append_l(original_self as *mut ::vector::VectorNodeNodeIdTypePair, l as *const ::vector::VectorNodeNodeIdTypePair) }
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairAppendArgs<'largs> for &'largs ::node::NodeIdTypePair {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let t = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_append_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, t as *const ::node::NodeIdTypePair) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::fill](../struct.VectorNodeNodeIdTypePair.html#method.fill) method.
  pub trait VectorNodeNodeIdTypePairFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair;
  }
  impl<'largs> VectorNodeNodeIdTypePairFillArgs<'largs> for &'largs ::node::NodeIdTypePair {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_fill_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, t as *const ::node::NodeIdTypePair) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairFillArgs<'largs> for (&'largs ::node::NodeIdTypePair, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_fill_t_size(original_self as *mut ::vector::VectorNodeNodeIdTypePair, t as *const ::node::NodeIdTypePair, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::insert](../struct.VectorNodeNodeIdTypePair.html#method.insert) method.
  pub trait VectorNodeNodeIdTypePairInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> ();
  }
  impl<'largs> VectorNodeNodeIdTypePairInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::node::NodeIdTypePair) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_insert_i_n_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, i, n, t as *const ::node::NodeIdTypePair) }
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairInsertArgs<'largs> for (::libc::c_int, &'largs ::node::NodeIdTypePair) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_insert_i_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, i, t as *const ::node::NodeIdTypePair) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::mid](../struct.VectorNodeNodeIdTypePair.html#method.mid) method.
  pub trait VectorNodeNodeIdTypePairMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::vector::VectorNodeNodeIdTypePair;
  }
  impl<'largs> VectorNodeNodeIdTypePairMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::vector::VectorNodeNodeIdTypePair {
      let pos = self;
      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_mid_to_output_pos(original_self as *const ::vector::VectorNodeNodeIdTypePair, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::vector::VectorNodeNodeIdTypePair {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_mid_to_output_pos_len(original_self as *const ::vector::VectorNodeNodeIdTypePair, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::new](../struct.VectorNodeNodeIdTypePair.html#method.new) method.
  pub trait VectorNodeNodeIdTypePairNewArgs {
    fn exec(self) -> ::vector::VectorNodeNodeIdTypePair;
  }
  impl VectorNodeNodeIdTypePairNewArgs for () {
    fn exec(self) -> ::vector::VectorNodeNodeIdTypePair {

      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorNodeNodeIdTypePairNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorNodeNodeIdTypePair {
      let size = self;
      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorNodeNodeIdTypePairNewArgs for (::libc::c_int, &'a ::node::NodeIdTypePair) {
    fn exec(self) -> ::vector::VectorNodeNodeIdTypePair {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constructor_size_t(size,
                                                                                  t as *const ::node::NodeIdTypePair,
                                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorNodeNodeIdTypePairNewArgs for &'a ::vector::VectorNodeNodeIdTypePair {
    fn exec(self) -> ::vector::VectorNodeNodeIdTypePair {
      let v = self;
      {
        let mut object: ::vector::VectorNodeNodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_constructor_v(v as *const ::vector::VectorNodeNodeIdTypePair, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::op_add_assign](../struct.VectorNodeNodeIdTypePair.html#method.op_add_assign) method.
  pub trait VectorNodeNodeIdTypePairOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair;
  }
  impl<'largs> VectorNodeNodeIdTypePairOpAddAssignArgs<'largs> for &'largs ::vector::VectorNodeNodeIdTypePair {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_add_assign_l(original_self as *mut ::vector::VectorNodeNodeIdTypePair, l as *const ::vector::VectorNodeNodeIdTypePair) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairOpAddAssignArgs<'largs> for &'largs ::node::NodeIdTypePair {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_add_assign_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, t as *const ::node::NodeIdTypePair) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::op_shl](../struct.VectorNodeNodeIdTypePair.html#method.op_shl) method.
  pub trait VectorNodeNodeIdTypePairOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair;
  }
  impl<'largs> VectorNodeNodeIdTypePairOpShlArgs<'largs> for &'largs ::vector::VectorNodeNodeIdTypePair {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_shl_l(original_self as *mut ::vector::VectorNodeNodeIdTypePair, l as *const ::vector::VectorNodeNodeIdTypePair) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairOpShlArgs<'largs> for &'largs ::node::NodeIdTypePair {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair)
            -> &'largs mut ::vector::VectorNodeNodeIdTypePair {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_operator_shl_t(original_self as *mut ::vector::VectorNodeNodeIdTypePair, t as *const ::node::NodeIdTypePair) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::remove](../struct.VectorNodeNodeIdTypePair.html#method.remove) method.
  pub trait VectorNodeNodeIdTypePairRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> ();
  }
  impl<'largs> VectorNodeNodeIdTypePairRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_remove_i(original_self as *mut ::vector::VectorNodeNodeIdTypePair, i) }
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorNodeNodeIdTypePair) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_remove_i_n(original_self as *mut ::vector::VectorNodeNodeIdTypePair, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorNodeNodeIdTypePair::value](../struct.VectorNodeNodeIdTypePair.html#method.value) method.
  pub trait VectorNodeNodeIdTypePairValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::node::NodeIdTypePair;
  }
  impl<'largs> VectorNodeNodeIdTypePairValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::node::NodeIdTypePair {
      let i = self;
      {
        let mut object: ::node::NodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_value_to_output_i(original_self as *const ::vector::VectorNodeNodeIdTypePair, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorNodeNodeIdTypePairValueArgs<'largs> for (::libc::c_int, &'largs ::node::NodeIdTypePair) {
    fn exec(self, original_self: &'largs ::vector::VectorNodeNodeIdTypePair) -> ::node::NodeIdTypePair {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::node::NodeIdTypePair =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_Qt3DCore_QNodeIdTypePair_value_to_output_i_defaultValue(original_self as *const ::vector::VectorNodeNodeIdTypePair, i, default_value as *const ::node::NodeIdTypePair, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::append](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.append) method.
  pub trait VectorSharedPointerSharedPointerAspectJobAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobAppendArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
    let l = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_append_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, l as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobAppendArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_append_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::count](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.count) method.
  pub trait VectorSharedPointerSharedPointerAspectJobCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {

      unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_count_no_args(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob) }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobCountArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_count_t(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::fill](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.fill) method.
  pub trait VectorSharedPointerSharedPointerAspectJobFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobFillArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_fill_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobFillArgs<'largs> for (&'largs ::shared_pointer::SharedPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_fill_t_size(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::index_of](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.index_of) method.
  pub trait VectorSharedPointerSharedPointerAspectJobIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobIndexOfArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_indexOf_t(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobIndexOfArgs<'largs> for (&'largs ::shared_pointer::SharedPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_indexOf_t_from(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::insert](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.insert) method.
  pub trait VectorSharedPointerSharedPointerAspectJobInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::shared_pointer::SharedPointerAspectJob) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_insert_i_n_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i, n, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobInsertArgs<'largs> for (::libc::c_int,&'largs ::shared_pointer::SharedPointerAspectJob) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_insert_i_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::last_index_of](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.last_index_of) method.
  pub trait VectorSharedPointerSharedPointerAspectJobLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobLastIndexOfArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_lastIndexOf_t(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobLastIndexOfArgs<'largs> for (&'largs ::shared_pointer::SharedPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_lastIndexOf_t_from(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::mid](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.mid) method.
  pub trait VectorSharedPointerSharedPointerAspectJobMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> ::vector::VectorSharedPointerSharedPointerAspectJob;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> ::vector::VectorSharedPointerSharedPointerAspectJob {
      let pos = self;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_mid_to_output_pos(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> ::vector::VectorSharedPointerSharedPointerAspectJob {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_mid_to_output_pos_len(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::new](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.new) method.
  pub trait VectorSharedPointerSharedPointerAspectJobNewArgs {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerAspectJob;
  }
  impl VectorSharedPointerSharedPointerAspectJobNewArgs for () {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerAspectJob {

      {
        let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorSharedPointerSharedPointerAspectJobNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerAspectJob {
      let size = self;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorSharedPointerSharedPointerAspectJobNewArgs
    for (::libc::c_int, &'a ::shared_pointer::SharedPointerAspectJob) {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerAspectJob {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constructor_size_t(size, t as *const ::shared_pointer::SharedPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorSharedPointerSharedPointerAspectJobNewArgs for &'a ::vector::VectorSharedPointerSharedPointerAspectJob {

  fn exec(self, ) -> ::vector::VectorSharedPointerSharedPointerAspectJob {
    let v = self;
    {
let mut object: ::vector::VectorSharedPointerSharedPointerAspectJob = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_constructor_v(v as *const ::vector::VectorSharedPointerSharedPointerAspectJob, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::op_add_assign](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.op_add_assign) method.
  pub trait VectorSharedPointerSharedPointerAspectJobOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobOpAddAssignArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_add_assign_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, l as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobOpAddAssignArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_add_assign_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::op_shl](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.op_shl) method.
  pub trait VectorSharedPointerSharedPointerAspectJobOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobOpShlArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_shl_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, l as *const ::vector::VectorSharedPointerSharedPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerAspectJobOpShlArgs<'largs> for &'largs ::shared_pointer::SharedPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_operator_shl_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, t as *const ::shared_pointer::SharedPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::remove](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.remove) method.
  pub trait VectorSharedPointerSharedPointerAspectJobRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_remove_i(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i) }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerAspectJob) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_remove_i_n(original_self as *mut ::vector::VectorSharedPointerSharedPointerAspectJob, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerAspectJob::value](../struct.VectorSharedPointerSharedPointerAspectJob.html#method.value) method.
  pub trait VectorSharedPointerSharedPointerAspectJobValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> ::shared_pointer::SharedPointerAspectJob;
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob)
            -> ::shared_pointer::SharedPointerAspectJob {
      let i = self;
      {
        let mut object: ::shared_pointer::SharedPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_value_to_output_i(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerAspectJobValueArgs<'largs> for (::libc::c_int,&'largs ::shared_pointer::SharedPointerAspectJob) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerAspectJob) -> ::shared_pointer::SharedPointerAspectJob {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::shared_pointer::SharedPointerAspectJob = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_core_c_QVector_QSharedPointer_Qt3DCore_QAspectJob_value_to_output_i_defaultValue(original_self as *const ::vector::VectorSharedPointerSharedPointerAspectJob, i, default_value as *const ::shared_pointer::SharedPointerAspectJob, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::append](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.append) method.
  pub trait VectorWeakPointerWeakPointerAspectJobAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> ();
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobAppendArgs<'largs> for &'largs ::vector::VectorWeakPointerWeakPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
    let l = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_append_l(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, l as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobAppendArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_append_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::count](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.count) method.
  pub trait VectorWeakPointerWeakPointerAspectJobCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {

      unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_count_no_args(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob) }
    }
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobCountArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {
    fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_count_t(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::fill](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.fill) method.
  pub trait VectorWeakPointerWeakPointerAspectJobFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobFillArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_fill_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobFillArgs<'largs> for (&'largs ::weak_pointer::WeakPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_fill_t_size(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::index_of](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.index_of) method.
  pub trait VectorWeakPointerWeakPointerAspectJobIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobIndexOfArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {

  fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_indexOf_t(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobIndexOfArgs<'largs> for (&'largs ::weak_pointer::WeakPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_indexOf_t_from(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::insert](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.insert) method.
  pub trait VectorWeakPointerWeakPointerAspectJobInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> ();
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::weak_pointer::WeakPointerAspectJob) {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_insert_i_n_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i, n, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobInsertArgs<'largs> for (::libc::c_int,&'largs ::weak_pointer::WeakPointerAspectJob) {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_insert_i_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::last_index_of](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.last_index_of) method.
  pub trait VectorWeakPointerWeakPointerAspectJobLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobLastIndexOfArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {

  fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_lastIndexOf_t(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) }
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobLastIndexOfArgs<'largs> for (&'largs ::weak_pointer::WeakPointerAspectJob,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_lastIndexOf_t_from(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::mid](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.mid) method.
  pub trait VectorWeakPointerWeakPointerAspectJobMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> ::vector::VectorWeakPointerWeakPointerAspectJob;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> ::vector::VectorWeakPointerWeakPointerAspectJob {
      let pos = self;
      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_mid_to_output_pos(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> ::vector::VectorWeakPointerWeakPointerAspectJob {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_mid_to_output_pos_len(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::new](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.new) method.
  pub trait VectorWeakPointerWeakPointerAspectJobNewArgs {
    fn exec(self) -> ::vector::VectorWeakPointerWeakPointerAspectJob;
  }
  impl VectorWeakPointerWeakPointerAspectJobNewArgs for () {
    fn exec(self) -> ::vector::VectorWeakPointerWeakPointerAspectJob {

      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorWeakPointerWeakPointerAspectJobNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorWeakPointerWeakPointerAspectJob {
      let size = self;
      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorWeakPointerWeakPointerAspectJobNewArgs for (::libc::c_int, &'a ::weak_pointer::WeakPointerAspectJob) {
    fn exec(self) -> ::vector::VectorWeakPointerWeakPointerAspectJob {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constructor_size_t(size, t as *const ::weak_pointer::WeakPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorWeakPointerWeakPointerAspectJobNewArgs for &'a ::vector::VectorWeakPointerWeakPointerAspectJob {
    fn exec(self) -> ::vector::VectorWeakPointerWeakPointerAspectJob {
      let v = self;
      {
        let mut object: ::vector::VectorWeakPointerWeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_constructor_v(v as *const ::vector::VectorWeakPointerWeakPointerAspectJob, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::op_add_assign](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.op_add_assign) method.
  pub trait VectorWeakPointerWeakPointerAspectJobOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobOpAddAssignArgs<'largs> for &'largs ::vector::VectorWeakPointerWeakPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_add_assign_l(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, l as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobOpAddAssignArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_add_assign_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::op_shl](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.op_shl) method.
  pub trait VectorWeakPointerWeakPointerAspectJobOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobOpShlArgs<'largs> for &'largs ::vector::VectorWeakPointerWeakPointerAspectJob {

  fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_shl_l(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, l as *const ::vector::VectorWeakPointerWeakPointerAspectJob) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorWeakPointerWeakPointerAspectJobOpShlArgs<'largs> for &'largs ::weak_pointer::WeakPointerAspectJob {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_operator_shl_t(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, t as *const ::weak_pointer::WeakPointerAspectJob) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::remove](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.remove) method.
  pub trait VectorWeakPointerWeakPointerAspectJobRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> ();
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_remove_i(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i) }
    }
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorWeakPointerWeakPointerAspectJob) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_remove_i_n(original_self as *mut ::vector::VectorWeakPointerWeakPointerAspectJob, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorWeakPointerWeakPointerAspectJob::value](../struct.VectorWeakPointerWeakPointerAspectJob.html#method.value) method.
  pub trait VectorWeakPointerWeakPointerAspectJobValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> ::weak_pointer::WeakPointerAspectJob;
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob)
            -> ::weak_pointer::WeakPointerAspectJob {
      let i = self;
      {
        let mut object: ::weak_pointer::WeakPointerAspectJob =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_value_to_output_i(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorWeakPointerWeakPointerAspectJobValueArgs<'largs> for (::libc::c_int,&'largs ::weak_pointer::WeakPointerAspectJob) {

  fn exec(self, original_self: &'largs ::vector::VectorWeakPointerWeakPointerAspectJob) -> ::weak_pointer::WeakPointerAspectJob {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::weak_pointer::WeakPointerAspectJob = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_core_c_QVector_QWeakPointer_Qt3DCore_QAspectJob_value_to_output_i_defaultValue(original_self as *const ::vector::VectorWeakPointerWeakPointerAspectJob, i, default_value as *const ::weak_pointer::WeakPointerAspectJob, &mut object); }object
}
  }
}
}
