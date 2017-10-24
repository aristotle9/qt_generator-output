/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>```</span>
#[repr(C)]
pub struct VectorAbstractTextureImageMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_ABSTRACT_TEXTURE_IMAGE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAbstractTextureImageMutPtr {
  unsafe fn new_uninitialized() -> VectorAbstractTextureImageMutPtr {
    VectorAbstractTextureImageMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAbstractTextureImageMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::append(const QVector<Qt3DRender::QAbstractTextureImage*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAbstractTextureImageMutPtr) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_append_l(self as *mut ::vector::VectorAbstractTextureImageMutPtr, l as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::append(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_append_t(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_at(self as *const ::vector::VectorAbstractTextureImageMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_back_const(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage*& QVector<Qt3DRender::QAbstractTextureImage*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_back(self as *mut ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_capacity(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_clear(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const * QVector<Qt3DRender::QAbstractTextureImage*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constData(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constFirst(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constLast(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::contains(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_contains(self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_count_no_args(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::count(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_count_t(self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const * QVector<Qt3DRender::QAbstractTextureImage*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_data_const(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage** QVector<Qt3DRender::QAbstractTextureImage*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_data(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_empty(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::endsWith(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_endsWith(self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::abstract_texture_image::AbstractTextureImage) -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::fill(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::abstract_texture_image::AbstractTextureImage, ::libc::c_int)) -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::fill(Qt3DRender::QAbstractTextureImage* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self,
                                   args: Args)
                                   -> &'largs mut ::vector::VectorAbstractTextureImageMutPtr
    where Args: overloading::VectorAbstractTextureImageMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_first_const(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage*& QVector<Qt3DRender::QAbstractTextureImage*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_first(self as *mut ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_front_const(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage*& QVector<Qt3DRender::QAbstractTextureImage*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_front(self as *mut ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_texture_image::AbstractTextureImage) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::indexOf(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_texture_image::AbstractTextureImage, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::indexOf(Qt3DRender::QAbstractTextureImage* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractTextureImageMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::abstract_texture_image::AbstractTextureImage)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::insert(int i, Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::abstract_texture_image::AbstractTextureImage)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::insert(int i, int n, Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractTextureImageMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_isEmpty(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_last_const(self as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_texture_image::AbstractTextureImage) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::lastIndexOf(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_texture_image::AbstractTextureImage, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::lastIndexOf(Qt3DRender::QAbstractTextureImage* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractTextureImageMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage*& QVector<Qt3DRender::QAbstractTextureImage*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_last(self as *mut ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_length(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*> QVector<Qt3DRender::QAbstractTextureImage*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*> QVector<Qt3DRender::QAbstractTextureImage*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAbstractTextureImageMutPtr
    where Args: overloading::VectorAbstractTextureImageMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_move(self as *mut ::vector::VectorAbstractTextureImageMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAbstractTextureImage*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAbstractTextureImageMutPtr) -> ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAbstractTextureImage*>::QVector(const QVector<Qt3DRender::QAbstractTextureImage*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAbstractTextureImageMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAbstractTextureImage*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAbstractTextureImageMutPtr
    where Args: overloading::VectorAbstractTextureImageMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAbstractTextureImage*>::QVector(int size, Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::abstract_texture_image::AbstractTextureImage)
                           -> ::vector::VectorAbstractTextureImageMutPtr {
    {
      let mut object: ::vector::VectorAbstractTextureImageMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constructor_size_t(size, t as *const *mut ::abstract_texture_image::AbstractTextureImage, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*> QVector<Qt3DRender::QAbstractTextureImage*>::operator+(const QVector<Qt3DRender::QAbstractTextureImage*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAbstractTextureImageMutPtr) -> ::vector::VectorAbstractTextureImageMutPtr {
    {
      let mut object: ::vector::VectorAbstractTextureImageMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_add_to_output(self as *const ::vector::VectorAbstractTextureImageMutPtr, l as *const ::vector::VectorAbstractTextureImageMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::operator+=(const QVector<Qt3DRender::QAbstractTextureImage*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAbstractTextureImageMutPtr)
                                 -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_add_assign_l(self as *mut ::vector::VectorAbstractTextureImageMutPtr, l as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::operator+=(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_texture_image::AbstractTextureImage)
                                               -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_add_assign_t(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::operator=(const QVector<Qt3DRender::QAbstractTextureImage*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAbstractTextureImageMutPtr)
                             -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_assign(self as *mut ::vector::VectorAbstractTextureImageMutPtr, v as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::operator==(const QVector<Qt3DRender::QAbstractTextureImage*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAbstractTextureImageMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_eq(self as *const ::vector::VectorAbstractTextureImageMutPtr, v as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* const & QVector<Qt3DRender::QAbstractTextureImage*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_index_const(self as *const ::vector::VectorAbstractTextureImageMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage*& QVector<Qt3DRender::QAbstractTextureImage*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut *mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_index(self as *mut ::vector::VectorAbstractTextureImageMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::operator!=(const QVector<Qt3DRender::QAbstractTextureImage*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAbstractTextureImageMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_neq(self as *const ::vector::VectorAbstractTextureImageMutPtr, v as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::operator<<(const QVector<Qt3DRender::QAbstractTextureImage*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAbstractTextureImageMutPtr)
                          -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_shl_l(self as *mut ::vector::VectorAbstractTextureImageMutPtr, l as *const ::vector::VectorAbstractTextureImageMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>& QVector<Qt3DRender::QAbstractTextureImage*>::operator<<(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_texture_image::AbstractTextureImage)
                                        -> &'l0 mut ::vector::VectorAbstractTextureImageMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_operator_shl_t(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_pop_back(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_pop_front(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::prepend(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_prepend(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::push_back(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_push_back(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::push_front(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_push_front(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAbstractTextureImage*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractTextureImageMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::removeAll(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_removeAll(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_removeAt(self as *mut ::vector::VectorAbstractTextureImageMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_removeFirst(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_removeLast(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::removeOne(Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_removeOne(self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::replace(int i, Qt3DRender::QAbstractTextureImage* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_texture_image::AbstractTextureImage) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_replace(self as *mut ::vector::VectorAbstractTextureImageMutPtr, i, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_reserve(self as *mut ::vector::VectorAbstractTextureImageMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_resize(self as *mut ::vector::VectorAbstractTextureImageMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAbstractTextureImage*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_size(self as *const ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_squeeze(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAbstractTextureImage*>::startsWith(Qt3DRender::QAbstractTextureImage* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_texture_image::AbstractTextureImage) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_startsWith(self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAbstractTextureImage*>::swap(QVector<Qt3DRender::QAbstractTextureImage*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAbstractTextureImageMutPtr) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_swap(self as *mut ::vector::VectorAbstractTextureImageMutPtr, other as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* QVector<Qt3DRender::QAbstractTextureImage*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_takeAt(self as *mut ::vector::VectorAbstractTextureImageMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* QVector<Qt3DRender::QAbstractTextureImage*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_takeFirst(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* QVector<Qt3DRender::QAbstractTextureImage*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_takeLast(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* QVector<Qt3DRender::QAbstractTextureImage*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_texture_image::AbstractTextureImage {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_value_i(self as *const ::vector::VectorAbstractTextureImageMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage* QVector<Qt3DRender::QAbstractTextureImage*>::value(int i, Qt3DRender::QAbstractTextureImage* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_texture_image::AbstractTextureImage)
                             -> *mut ::abstract_texture_image::AbstractTextureImage {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_value_i_defaultValue(self as *const ::vector::VectorAbstractTextureImageMutPtr, i, default_value as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }
}

impl Drop for ::vector::VectorAbstractTextureImageMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QAbstractTextureImage*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_destructor(self as *mut ::vector::VectorAbstractTextureImageMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>```</span>
#[repr(C)]
pub struct VectorAttributeMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_ATTRIBUTE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAttributeMutPtr {
  unsafe fn new_uninitialized() -> VectorAttributeMutPtr {
    VectorAttributeMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAttributeMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::append(const QVector<Qt3DRender::QAttribute*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAttributeMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_append_l(self as *mut ::vector::VectorAttributeMutPtr,
                                                                       l as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::append(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_append_t(self as *mut ::vector::VectorAttributeMutPtr,
                                                                     t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_at(self as *const ::vector::VectorAttributeMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_back_const(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute*& QVector<Qt3DRender::QAttribute*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::attribute::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_back(self as *mut ::vector::VectorAttributeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_capacity(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_clear(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const * QVector<Qt3DRender::QAttribute*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constData(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constFirst(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constLast(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::contains(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::attribute::Attribute) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_contains(self as *const ::vector::VectorAttributeMutPtr,
                                                                     t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_count_no_args(self as *const ::vector::VectorAttributeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::count(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::attribute::Attribute) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_count_t(self as *const ::vector::VectorAttributeMutPtr,
                                                                    t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const * QVector<Qt3DRender::QAttribute*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_data_const(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute** QVector<Qt3DRender::QAttribute*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_data(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_empty(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::endsWith(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::attribute::Attribute) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_endsWith(self as *const ::vector::VectorAttributeMutPtr,
                                                                     t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::attribute::Attribute) -> &'l0 mut ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::fill(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::attribute::Attribute, ::libc::c_int)) -> &'l0 mut ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::fill(Qt3DRender::QAttribute* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorAttributeMutPtr
    where Args: overloading::VectorAttributeMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_first_const(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute*& QVector<Qt3DRender::QAttribute*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::attribute::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_first(self as *mut ::vector::VectorAttributeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_front_const(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute*& QVector<Qt3DRender::QAttribute*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::attribute::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_front(self as *mut ::vector::VectorAttributeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::attribute::Attribute) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::indexOf(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::attribute::Attribute, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::indexOf(Qt3DRender::QAttribute* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAttributeMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::attribute::Attribute)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::insert(int i, Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::attribute::Attribute)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::insert(int i, int n, Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAttributeMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_isEmpty(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_last_const(self as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::attribute::Attribute) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::lastIndexOf(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::attribute::Attribute, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::lastIndexOf(Qt3DRender::QAttribute* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAttributeMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute*& QVector<Qt3DRender::QAttribute*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::attribute::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_last(self as *mut ::vector::VectorAttributeMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_length(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*> QVector<Qt3DRender::QAttribute*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*> QVector<Qt3DRender::QAttribute*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAttributeMutPtr
    where Args: overloading::VectorAttributeMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_move(self as *mut ::vector::VectorAttributeMutPtr,
                                                                   from,
                                                                   to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAttribute*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAttributeMutPtr) -> ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAttribute*>::QVector(const QVector<Qt3DRender::QAttribute*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAttributeMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAttribute*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAttributeMutPtr
    where Args: overloading::VectorAttributeMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QAttribute*>::QVector(int size, Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::attribute::Attribute) -> ::vector::VectorAttributeMutPtr {
    {
      let mut object: ::vector::VectorAttributeMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constructor_size_t(size, t as *const *mut ::attribute::Attribute, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*> QVector<Qt3DRender::QAttribute*>::operator+(const QVector<Qt3DRender::QAttribute*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAttributeMutPtr) -> ::vector::VectorAttributeMutPtr {
    {
      let mut object: ::vector::VectorAttributeMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_add_to_output(self as *const ::vector::VectorAttributeMutPtr, l as *const ::vector::VectorAttributeMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::operator+=(const QVector<Qt3DRender::QAttribute*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAttributeMutPtr)
                                 -> &'l0 mut ::vector::VectorAttributeMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_add_assign_l(self as *mut ::vector::VectorAttributeMutPtr, l as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::operator+=(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::attribute::Attribute)
                                               -> &'l0 mut ::vector::VectorAttributeMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_add_assign_t(self as *mut ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::operator=(const QVector<Qt3DRender::QAttribute*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAttributeMutPtr)
                             -> &'l0 mut ::vector::VectorAttributeMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_assign(self as *mut ::vector::VectorAttributeMutPtr, v as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::operator==(const QVector<Qt3DRender::QAttribute*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAttributeMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_eq(self as *const ::vector::VectorAttributeMutPtr, v as *const ::vector::VectorAttributeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* const & QVector<Qt3DRender::QAttribute*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_index_const(self as *const ::vector::VectorAttributeMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute*& QVector<Qt3DRender::QAttribute*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::attribute::Attribute {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_index(self as *mut ::vector::VectorAttributeMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::operator!=(const QVector<Qt3DRender::QAttribute*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAttributeMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_neq(self as *const ::vector::VectorAttributeMutPtr, v as *const ::vector::VectorAttributeMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::operator<<(const QVector<Qt3DRender::QAttribute*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAttributeMutPtr)
                          -> &'l0 mut ::vector::VectorAttributeMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_shl_l(self as *mut ::vector::VectorAttributeMutPtr, l as *const ::vector::VectorAttributeMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>& QVector<Qt3DRender::QAttribute*>::operator<<(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::attribute::Attribute)
                                        -> &'l0 mut ::vector::VectorAttributeMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_operator_shl_t(self as *mut ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_pop_back(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_pop_front(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::prepend(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_prepend(self as *mut ::vector::VectorAttributeMutPtr,
                                                                    t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::push_back(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_push_back(self as *mut ::vector::VectorAttributeMutPtr,
                                                                      t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::push_front(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_push_front(self as *mut ::vector::VectorAttributeMutPtr,
                                                                       t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAttributeMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::removeAll(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::attribute::Attribute) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_removeAll(self as *mut ::vector::VectorAttributeMutPtr,
                                                                      t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_removeAt(self as *mut ::vector::VectorAttributeMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_removeFirst(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_removeLast(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::removeOne(Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::attribute::Attribute) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_removeOne(self as *mut ::vector::VectorAttributeMutPtr,
                                                                      t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::replace(int i, Qt3DRender::QAttribute* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_replace(self as *mut ::vector::VectorAttributeMutPtr,
                                                                    i,
                                                                    t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_reserve(self as *mut ::vector::VectorAttributeMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_resize(self as *mut ::vector::VectorAttributeMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QAttribute*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_size(self as *const ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_squeeze(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QAttribute*>::startsWith(Qt3DRender::QAttribute* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::attribute::Attribute) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_startsWith(self as *const ::vector::VectorAttributeMutPtr,
                                                                       t as *const *mut ::attribute::Attribute)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QAttribute*>::swap(QVector<Qt3DRender::QAttribute*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAttributeMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_swap(self as *mut ::vector::VectorAttributeMutPtr,
                                                                   other as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* QVector<Qt3DRender::QAttribute*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_takeAt(self as *mut ::vector::VectorAttributeMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* QVector<Qt3DRender::QAttribute*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_takeFirst(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* QVector<Qt3DRender::QAttribute*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_takeLast(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* QVector<Qt3DRender::QAttribute*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_value_i(self as *const ::vector::VectorAttributeMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* QVector<Qt3DRender::QAttribute*>::value(int i, Qt3DRender::QAttribute* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::attribute::Attribute)
                             -> *mut ::attribute::Attribute {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_value_i_defaultValue(self as *const ::vector::VectorAttributeMutPtr, i, default_value as *const *mut ::attribute::Attribute)
  }
}

impl Drop for ::vector::VectorAttributeMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QAttribute*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_destructor(self as *mut ::vector::VectorAttributeMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>```</span>
#[repr(C)]
pub struct VectorFilterKeyMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_FILTER_KEY_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorFilterKeyMutPtr {
  unsafe fn new_uninitialized() -> VectorFilterKeyMutPtr {
    VectorFilterKeyMutPtr(::std::mem::uninitialized())
  }
}

impl VectorFilterKeyMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::append(const QVector<Qt3DRender::QFilterKey*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorFilterKeyMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_append_l(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                       l as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::append(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_append_t(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                     t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_at(self as *const ::vector::VectorFilterKeyMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_back_const(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey*& QVector<Qt3DRender::QFilterKey*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::filter_key::FilterKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_back(self as *mut ::vector::VectorFilterKeyMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_capacity(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_clear(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const * QVector<Qt3DRender::QFilterKey*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constData(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constFirst(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constLast(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::contains(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::filter_key::FilterKey) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_contains(self as *const ::vector::VectorFilterKeyMutPtr,
                                                                     t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_count_no_args(self as *const ::vector::VectorFilterKeyMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::count(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::filter_key::FilterKey) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_count_t(self as *const ::vector::VectorFilterKeyMutPtr,
                                                                    t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const * QVector<Qt3DRender::QFilterKey*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_data_const(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey** QVector<Qt3DRender::QFilterKey*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_data(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_empty(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::endsWith(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::filter_key::FilterKey) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_endsWith(self as *const ::vector::VectorFilterKeyMutPtr,
                                                                     t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::filter_key::FilterKey) -> &'l0 mut ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::fill(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::filter_key::FilterKey, ::libc::c_int)) -> &'l0 mut ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::fill(Qt3DRender::QFilterKey* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorFilterKeyMutPtr
    where Args: overloading::VectorFilterKeyMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_first_const(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey*& QVector<Qt3DRender::QFilterKey*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::filter_key::FilterKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_first(self as *mut ::vector::VectorFilterKeyMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_front_const(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey*& QVector<Qt3DRender::QFilterKey*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::filter_key::FilterKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_front(self as *mut ::vector::VectorFilterKeyMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::filter_key::FilterKey) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::indexOf(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::filter_key::FilterKey, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::indexOf(Qt3DRender::QFilterKey* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorFilterKeyMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::filter_key::FilterKey)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::insert(int i, Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::filter_key::FilterKey)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::insert(int i, int n, Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorFilterKeyMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_isEmpty(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_last_const(self as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::filter_key::FilterKey) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::lastIndexOf(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::filter_key::FilterKey, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::lastIndexOf(Qt3DRender::QFilterKey* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorFilterKeyMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey*& QVector<Qt3DRender::QFilterKey*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::filter_key::FilterKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_last(self as *mut ::vector::VectorFilterKeyMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_length(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> QVector<Qt3DRender::QFilterKey*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> QVector<Qt3DRender::QFilterKey*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorFilterKeyMutPtr
    where Args: overloading::VectorFilterKeyMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_move(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                   from,
                                                                   to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QFilterKey*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorFilterKeyMutPtr) -> ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QFilterKey*>::QVector(const QVector<Qt3DRender::QFilterKey*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorFilterKeyMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QFilterKey*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorFilterKeyMutPtr
    where Args: overloading::VectorFilterKeyMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QFilterKey*>::QVector(int size, Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::filter_key::FilterKey) -> ::vector::VectorFilterKeyMutPtr {
    {
      let mut object: ::vector::VectorFilterKeyMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constructor_size_t(size, t as *const *mut ::filter_key::FilterKey, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*> QVector<Qt3DRender::QFilterKey*>::operator+(const QVector<Qt3DRender::QFilterKey*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorFilterKeyMutPtr) -> ::vector::VectorFilterKeyMutPtr {
    {
      let mut object: ::vector::VectorFilterKeyMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_add_to_output(self as *const ::vector::VectorFilterKeyMutPtr, l as *const ::vector::VectorFilterKeyMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::operator+=(const QVector<Qt3DRender::QFilterKey*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorFilterKeyMutPtr)
                                 -> &'l0 mut ::vector::VectorFilterKeyMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_add_assign_l(self as *mut ::vector::VectorFilterKeyMutPtr, l as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::operator+=(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::filter_key::FilterKey)
                                               -> &'l0 mut ::vector::VectorFilterKeyMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_add_assign_t(self as *mut ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::operator=(const QVector<Qt3DRender::QFilterKey*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorFilterKeyMutPtr)
                             -> &'l0 mut ::vector::VectorFilterKeyMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_assign(self as *mut ::vector::VectorFilterKeyMutPtr, v as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::operator==(const QVector<Qt3DRender::QFilterKey*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorFilterKeyMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_eq(self as *const ::vector::VectorFilterKeyMutPtr, v as *const ::vector::VectorFilterKeyMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* const & QVector<Qt3DRender::QFilterKey*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_index_const(self as *const ::vector::VectorFilterKeyMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey*& QVector<Qt3DRender::QFilterKey*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::filter_key::FilterKey {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_index(self as *mut ::vector::VectorFilterKeyMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::operator!=(const QVector<Qt3DRender::QFilterKey*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorFilterKeyMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_neq(self as *const ::vector::VectorFilterKeyMutPtr, v as *const ::vector::VectorFilterKeyMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::operator<<(const QVector<Qt3DRender::QFilterKey*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorFilterKeyMutPtr)
                          -> &'l0 mut ::vector::VectorFilterKeyMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_shl_l(self as *mut ::vector::VectorFilterKeyMutPtr, l as *const ::vector::VectorFilterKeyMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>& QVector<Qt3DRender::QFilterKey*>::operator<<(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::filter_key::FilterKey)
                                        -> &'l0 mut ::vector::VectorFilterKeyMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_operator_shl_t(self as *mut ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_pop_back(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_pop_front(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::prepend(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_prepend(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                    t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::push_back(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_push_back(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                      t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::push_front(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_push_front(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                       t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QFilterKey*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorFilterKeyMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::removeAll(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::filter_key::FilterKey) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_removeAll(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                      t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_removeAt(self as *mut ::vector::VectorFilterKeyMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_removeFirst(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_removeLast(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::removeOne(Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::filter_key::FilterKey) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_removeOne(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                      t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::replace(int i, Qt3DRender::QFilterKey* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::filter_key::FilterKey) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_replace(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                    i,
                                                                    t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_reserve(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_resize(self as *mut ::vector::VectorFilterKeyMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QFilterKey*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_size(self as *const ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_squeeze(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QFilterKey*>::startsWith(Qt3DRender::QFilterKey* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::filter_key::FilterKey) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_startsWith(self as *const ::vector::VectorFilterKeyMutPtr,
                                                                       t as *const *mut ::filter_key::FilterKey)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QFilterKey*>::swap(QVector<Qt3DRender::QFilterKey*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorFilterKeyMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_swap(self as *mut ::vector::VectorFilterKeyMutPtr,
                                                                   other as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* QVector<Qt3DRender::QFilterKey*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_takeAt(self as *mut ::vector::VectorFilterKeyMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* QVector<Qt3DRender::QFilterKey*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_takeFirst(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* QVector<Qt3DRender::QFilterKey*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_takeLast(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* QVector<Qt3DRender::QFilterKey*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::filter_key::FilterKey {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_value_i(self as *const ::vector::VectorFilterKeyMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QFilterKey* QVector<Qt3DRender::QFilterKey*>::value(int i, Qt3DRender::QFilterKey* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::filter_key::FilterKey)
                             -> *mut ::filter_key::FilterKey {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_value_i_defaultValue(self as *const ::vector::VectorFilterKeyMutPtr, i, default_value as *const *mut ::filter_key::FilterKey)
  }
}

impl Drop for ::vector::VectorFilterKeyMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QFilterKey*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_destructor(self as *mut ::vector::VectorFilterKeyMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>```</span>
#[repr(C)]
pub struct VectorLayerMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_LAYER_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorLayerMutPtr {
  unsafe fn new_uninitialized() -> VectorLayerMutPtr {
    VectorLayerMutPtr(::std::mem::uninitialized())
  }
}

impl VectorLayerMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::append(const QVector<Qt3DRender::QLayer*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorLayerMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_append_l(self as *mut ::vector::VectorLayerMutPtr,
                                                                   l as *const ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::append(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_append_t(self as *mut ::vector::VectorLayerMutPtr,
                                                                 t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_at(self as *const ::vector::VectorLayerMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_back_const(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer*& QVector<Qt3DRender::QLayer*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::layer::Layer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_back(self as *mut ::vector::VectorLayerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_capacity(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_clear(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const * QVector<Qt3DRender::QLayer*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::layer::Layer {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constData(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constFirst(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constLast(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::contains(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::layer::Layer) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_contains(self as *const ::vector::VectorLayerMutPtr,
                                                                 t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_count_no_args(self as *const ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::count(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::layer::Layer) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_count_t(self as *const ::vector::VectorLayerMutPtr,
                                                                t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const * QVector<Qt3DRender::QLayer*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::layer::Layer {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_data_const(self as *const ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer** QVector<Qt3DRender::QLayer*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::layer::Layer {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_data(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_empty(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::endsWith(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::layer::Layer) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_endsWith(self as *const ::vector::VectorLayerMutPtr,
                                                                 t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::layer::Layer) -> &'l0 mut ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::fill(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::layer::Layer, ::libc::c_int)) -> &'l0 mut ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::fill(Qt3DRender::QLayer* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorLayerMutPtr
    where Args: overloading::VectorLayerMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_first_const(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer*& QVector<Qt3DRender::QLayer*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::layer::Layer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_first(self as *mut ::vector::VectorLayerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_front_const(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer*& QVector<Qt3DRender::QLayer*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::layer::Layer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_front(self as *mut ::vector::VectorLayerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::layer::Layer) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::indexOf(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::layer::Layer, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::indexOf(Qt3DRender::QLayer* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorLayerMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::layer::Layer)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::insert(int i, Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::layer::Layer)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::insert(int i, int n, Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorLayerMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_isEmpty(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_last_const(self as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::layer::Layer) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::lastIndexOf(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::layer::Layer, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::lastIndexOf(Qt3DRender::QLayer* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorLayerMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer*& QVector<Qt3DRender::QLayer*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::layer::Layer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_last(self as *mut ::vector::VectorLayerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_length(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*> QVector<Qt3DRender::QLayer*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*> QVector<Qt3DRender::QLayer*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorLayerMutPtr
    where Args: overloading::VectorLayerMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_move(self as *mut ::vector::VectorLayerMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QLayer*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorLayerMutPtr) -> ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QLayer*>::QVector(const QVector<Qt3DRender::QLayer*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorLayerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QLayer*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorLayerMutPtr
    where Args: overloading::VectorLayerMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QLayer*>::QVector(int size, Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::layer::Layer) -> ::vector::VectorLayerMutPtr {
    {
      let mut object: ::vector::VectorLayerMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constructor_size_t(size,
                                                                             t as *const *mut ::layer::Layer,
                                                                             &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*> QVector<Qt3DRender::QLayer*>::operator+(const QVector<Qt3DRender::QLayer*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorLayerMutPtr) -> ::vector::VectorLayerMutPtr {
    {
      let mut object: ::vector::VectorLayerMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_add_to_output(self as *const ::vector::VectorLayerMutPtr, l as *const ::vector::VectorLayerMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::operator+=(const QVector<Qt3DRender::QLayer*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorLayerMutPtr)
                                 -> &'l0 mut ::vector::VectorLayerMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_add_assign_l(self as *mut ::vector::VectorLayerMutPtr, l as *const ::vector::VectorLayerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::operator+=(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::layer::Layer)
                                               -> &'l0 mut ::vector::VectorLayerMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_add_assign_t(self as *mut ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::operator=(const QVector<Qt3DRender::QLayer*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorLayerMutPtr)
                             -> &'l0 mut ::vector::VectorLayerMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_assign(self as *mut ::vector::VectorLayerMutPtr,
                                                                            v as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::operator==(const QVector<Qt3DRender::QLayer*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorLayerMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_eq(self as *const ::vector::VectorLayerMutPtr,
                                                                      v as *const ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* const & QVector<Qt3DRender::QLayer*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::layer::Layer {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_index_const(self as *const ::vector::VectorLayerMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer*& QVector<Qt3DRender::QLayer*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::layer::Layer {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_index(self as *mut ::vector::VectorLayerMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::operator!=(const QVector<Qt3DRender::QLayer*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorLayerMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_neq(self as *const ::vector::VectorLayerMutPtr,
                                                                       v as *const ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::operator<<(const QVector<Qt3DRender::QLayer*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::vector::VectorLayerMutPtr) -> &'l0 mut ::vector::VectorLayerMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_shl_l(self as *mut ::vector::VectorLayerMutPtr,
                                                                           l as *const ::vector::VectorLayerMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>& QVector<Qt3DRender::QLayer*>::operator<<(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::layer::Layer)
                                        -> &'l0 mut ::vector::VectorLayerMutPtr {
    let ffi_result =
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_operator_shl_t(self as *mut ::vector::VectorLayerMutPtr,
                                                                         t as *const *mut ::layer::Layer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_pop_back(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_pop_front(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::prepend(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_prepend(self as *mut ::vector::VectorLayerMutPtr,
                                                                t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::push_back(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_push_back(self as *mut ::vector::VectorLayerMutPtr,
                                                                  t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::push_front(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_push_front(self as *mut ::vector::VectorLayerMutPtr,
                                                                   t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QLayer*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorLayerMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::removeAll(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::layer::Layer) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_removeAll(self as *mut ::vector::VectorLayerMutPtr,
                                                                  t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_removeAt(self as *mut ::vector::VectorLayerMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_removeFirst(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_removeLast(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::removeOne(Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::layer::Layer) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_removeOne(self as *mut ::vector::VectorLayerMutPtr,
                                                                  t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::replace(int i, Qt3DRender::QLayer* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::layer::Layer) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_replace(self as *mut ::vector::VectorLayerMutPtr,
                                                                i,
                                                                t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_reserve(self as *mut ::vector::VectorLayerMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_resize(self as *mut ::vector::VectorLayerMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QLayer*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_size(self as *const ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_squeeze(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QLayer*>::startsWith(Qt3DRender::QLayer* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::layer::Layer) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_startsWith(self as *const ::vector::VectorLayerMutPtr,
                                                                   t as *const *mut ::layer::Layer)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QLayer*>::swap(QVector<Qt3DRender::QLayer*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorLayerMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_swap(self as *mut ::vector::VectorLayerMutPtr,
                                                               other as *mut ::vector::VectorLayerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* QVector<Qt3DRender::QLayer*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::layer::Layer {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_takeAt(self as *mut ::vector::VectorLayerMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* QVector<Qt3DRender::QLayer*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::layer::Layer {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_takeFirst(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* QVector<Qt3DRender::QLayer*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::layer::Layer {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_takeLast(self as *mut ::vector::VectorLayerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* QVector<Qt3DRender::QLayer*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::layer::Layer {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_value_i(self as *const ::vector::VectorLayerMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QLayer* QVector<Qt3DRender::QLayer*>::value(int i, Qt3DRender::QLayer* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::layer::Layer) -> *mut ::layer::Layer {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_value_i_defaultValue(self as *const ::vector::VectorLayerMutPtr, i, default_value as *const *mut ::layer::Layer)
  }
}

impl Drop for ::vector::VectorLayerMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QLayer*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_destructor(self as *mut ::vector::VectorLayerMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>```</span>
#[repr(C)]
pub struct VectorParameterMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_PARAMETER_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorParameterMutPtr {
  unsafe fn new_uninitialized() -> VectorParameterMutPtr {
    VectorParameterMutPtr(::std::mem::uninitialized())
  }
}

impl VectorParameterMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::append(const QVector<Qt3DRender::QParameter*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorParameterMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_append_l(self as *mut ::vector::VectorParameterMutPtr,
                                                                       l as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::append(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_append_t(self as *mut ::vector::VectorParameterMutPtr,
                                                                     t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_at(self as *const ::vector::VectorParameterMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_back_const(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter*& QVector<Qt3DRender::QParameter*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::parameter::Parameter {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_back(self as *mut ::vector::VectorParameterMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_capacity(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_clear(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const * QVector<Qt3DRender::QParameter*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constData(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constFirst(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constLast(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::contains(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::parameter::Parameter) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_contains(self as *const ::vector::VectorParameterMutPtr,
                                                                     t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_count_no_args(self as *const ::vector::VectorParameterMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::count(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::parameter::Parameter) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_count_t(self as *const ::vector::VectorParameterMutPtr,
                                                                    t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const * QVector<Qt3DRender::QParameter*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_data_const(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter** QVector<Qt3DRender::QParameter*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_data(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_empty(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::endsWith(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::parameter::Parameter) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_endsWith(self as *const ::vector::VectorParameterMutPtr,
                                                                     t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::parameter::Parameter) -> &'l0 mut ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::fill(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::parameter::Parameter, ::libc::c_int)) -> &'l0 mut ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::fill(Qt3DRender::QParameter* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorParameterMutPtr
    where Args: overloading::VectorParameterMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_first_const(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter*& QVector<Qt3DRender::QParameter*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::parameter::Parameter {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_first(self as *mut ::vector::VectorParameterMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_front_const(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter*& QVector<Qt3DRender::QParameter*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::parameter::Parameter {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_front(self as *mut ::vector::VectorParameterMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::parameter::Parameter) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::indexOf(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::parameter::Parameter, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::indexOf(Qt3DRender::QParameter* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorParameterMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::parameter::Parameter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::insert(int i, Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::parameter::Parameter)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::insert(int i, int n, Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorParameterMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_isEmpty(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_last_const(self as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::parameter::Parameter) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::lastIndexOf(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::parameter::Parameter, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::lastIndexOf(Qt3DRender::QParameter* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorParameterMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter*& QVector<Qt3DRender::QParameter*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::parameter::Parameter {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_last(self as *mut ::vector::VectorParameterMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_length(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> QVector<Qt3DRender::QParameter*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> QVector<Qt3DRender::QParameter*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorParameterMutPtr
    where Args: overloading::VectorParameterMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_move(self as *mut ::vector::VectorParameterMutPtr,
                                                                   from,
                                                                   to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QParameter*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorParameterMutPtr) -> ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QParameter*>::QVector(const QVector<Qt3DRender::QParameter*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorParameterMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QParameter*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorParameterMutPtr
    where Args: overloading::VectorParameterMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QParameter*>::QVector(int size, Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::parameter::Parameter) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constructor_size_t(size, t as *const *mut ::parameter::Parameter, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*> QVector<Qt3DRender::QParameter*>::operator+(const QVector<Qt3DRender::QParameter*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorParameterMutPtr) -> ::vector::VectorParameterMutPtr {
    {
      let mut object: ::vector::VectorParameterMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_add_to_output(self as *const ::vector::VectorParameterMutPtr, l as *const ::vector::VectorParameterMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::operator+=(const QVector<Qt3DRender::QParameter*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorParameterMutPtr)
                                 -> &'l0 mut ::vector::VectorParameterMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_add_assign_l(self as *mut ::vector::VectorParameterMutPtr, l as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::operator+=(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::parameter::Parameter)
                                               -> &'l0 mut ::vector::VectorParameterMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_add_assign_t(self as *mut ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::operator=(const QVector<Qt3DRender::QParameter*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorParameterMutPtr)
                             -> &'l0 mut ::vector::VectorParameterMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_assign(self as *mut ::vector::VectorParameterMutPtr, v as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::operator==(const QVector<Qt3DRender::QParameter*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorParameterMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_eq(self as *const ::vector::VectorParameterMutPtr, v as *const ::vector::VectorParameterMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* const & QVector<Qt3DRender::QParameter*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_index_const(self as *const ::vector::VectorParameterMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter*& QVector<Qt3DRender::QParameter*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::parameter::Parameter {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_index(self as *mut ::vector::VectorParameterMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::operator!=(const QVector<Qt3DRender::QParameter*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorParameterMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_neq(self as *const ::vector::VectorParameterMutPtr, v as *const ::vector::VectorParameterMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::operator<<(const QVector<Qt3DRender::QParameter*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorParameterMutPtr)
                          -> &'l0 mut ::vector::VectorParameterMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_shl_l(self as *mut ::vector::VectorParameterMutPtr, l as *const ::vector::VectorParameterMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>& QVector<Qt3DRender::QParameter*>::operator<<(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::parameter::Parameter)
                                        -> &'l0 mut ::vector::VectorParameterMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_operator_shl_t(self as *mut ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_pop_back(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_pop_front(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::prepend(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_prepend(self as *mut ::vector::VectorParameterMutPtr,
                                                                    t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::push_back(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_push_back(self as *mut ::vector::VectorParameterMutPtr,
                                                                      t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::push_front(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_push_front(self as *mut ::vector::VectorParameterMutPtr,
                                                                       t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QParameter*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorParameterMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::removeAll(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::parameter::Parameter) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_removeAll(self as *mut ::vector::VectorParameterMutPtr,
                                                                      t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_removeAt(self as *mut ::vector::VectorParameterMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_removeFirst(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_removeLast(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::removeOne(Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::parameter::Parameter) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_removeOne(self as *mut ::vector::VectorParameterMutPtr,
                                                                      t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::replace(int i, Qt3DRender::QParameter* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::parameter::Parameter) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_replace(self as *mut ::vector::VectorParameterMutPtr,
                                                                    i,
                                                                    t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_reserve(self as *mut ::vector::VectorParameterMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_resize(self as *mut ::vector::VectorParameterMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QParameter*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_size(self as *const ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_squeeze(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QParameter*>::startsWith(Qt3DRender::QParameter* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::parameter::Parameter) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_startsWith(self as *const ::vector::VectorParameterMutPtr,
                                                                       t as *const *mut ::parameter::Parameter)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QParameter*>::swap(QVector<Qt3DRender::QParameter*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorParameterMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_swap(self as *mut ::vector::VectorParameterMutPtr,
                                                                   other as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* QVector<Qt3DRender::QParameter*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_takeAt(self as *mut ::vector::VectorParameterMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* QVector<Qt3DRender::QParameter*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_takeFirst(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* QVector<Qt3DRender::QParameter*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_takeLast(self as *mut ::vector::VectorParameterMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* QVector<Qt3DRender::QParameter*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::parameter::Parameter {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_value_i(self as *const ::vector::VectorParameterMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QParameter* QVector<Qt3DRender::QParameter*>::value(int i, Qt3DRender::QParameter* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::parameter::Parameter)
                             -> *mut ::parameter::Parameter {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_value_i_defaultValue(self as *const ::vector::VectorParameterMutPtr, i, default_value as *const *mut ::parameter::Parameter)
  }
}

impl Drop for ::vector::VectorParameterMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QParameter*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_destructor(self as *mut ::vector::VectorParameterMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>```</span>
#[repr(C)]
pub struct VectorRenderPassMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_RENDER_PASS_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorRenderPassMutPtr {
  unsafe fn new_uninitialized() -> VectorRenderPassMutPtr {
    VectorRenderPassMutPtr(::std::mem::uninitialized())
  }
}

impl VectorRenderPassMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::append(const QVector<Qt3DRender::QRenderPass*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorRenderPassMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_append_l(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                        l as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::append(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::render_pass::RenderPass) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_append_t(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                      t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_at(self as *const ::vector::VectorRenderPassMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_back_const(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass*& QVector<Qt3DRender::QRenderPass*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_pass::RenderPass {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_back(self as *mut ::vector::VectorRenderPassMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_capacity(self as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_clear(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const * QVector<Qt3DRender::QRenderPass*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::render_pass::RenderPass {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constData(self as *const ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constFirst(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constLast(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::contains(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::render_pass::RenderPass) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_contains(self as *const ::vector::VectorRenderPassMutPtr,
                                                                      t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_count_no_args(self as *const ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::count(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::render_pass::RenderPass) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_count_t(self as *const ::vector::VectorRenderPassMutPtr,
                                                                     t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const * QVector<Qt3DRender::QRenderPass*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::render_pass::RenderPass {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_data_const(self as *const ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass** QVector<Qt3DRender::QRenderPass*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::render_pass::RenderPass {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_data(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_empty(self as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::endsWith(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::render_pass::RenderPass) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_endsWith(self as *const ::vector::VectorRenderPassMutPtr,
                                                                      t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::render_pass::RenderPass) -> &'l0 mut ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::fill(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::render_pass::RenderPass, ::libc::c_int)) -> &'l0 mut ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::fill(Qt3DRender::QRenderPass* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorRenderPassMutPtr
    where Args: overloading::VectorRenderPassMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_first_const(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass*& QVector<Qt3DRender::QRenderPass*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_pass::RenderPass {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_first(self as *mut ::vector::VectorRenderPassMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_front_const(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass*& QVector<Qt3DRender::QRenderPass*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_pass::RenderPass {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_front(self as *mut ::vector::VectorRenderPassMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::render_pass::RenderPass) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::indexOf(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::render_pass::RenderPass, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::indexOf(Qt3DRender::QRenderPass* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderPassMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::render_pass::RenderPass)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::insert(int i, Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::render_pass::RenderPass)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::insert(int i, int n, Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderPassMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_isEmpty(self as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_last_const(self as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::render_pass::RenderPass) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::lastIndexOf(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::render_pass::RenderPass, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::lastIndexOf(Qt3DRender::QRenderPass* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderPassMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass*& QVector<Qt3DRender::QRenderPass*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_pass::RenderPass {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_last(self as *mut ::vector::VectorRenderPassMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_length(self as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*> QVector<Qt3DRender::QRenderPass*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*> QVector<Qt3DRender::QRenderPass*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorRenderPassMutPtr
    where Args: overloading::VectorRenderPassMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_move(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                    from,
                                                                    to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderPass*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorRenderPassMutPtr) -> ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderPass*>::QVector(const QVector<Qt3DRender::QRenderPass*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorRenderPassMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderPass*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorRenderPassMutPtr
    where Args: overloading::VectorRenderPassMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderPass*>::QVector(int size, Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::render_pass::RenderPass)
                           -> ::vector::VectorRenderPassMutPtr {
    {
      let mut object: ::vector::VectorRenderPassMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constructor_size_t(size, t as *const *mut ::render_pass::RenderPass, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*> QVector<Qt3DRender::QRenderPass*>::operator+(const QVector<Qt3DRender::QRenderPass*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorRenderPassMutPtr) -> ::vector::VectorRenderPassMutPtr {
    {
      let mut object: ::vector::VectorRenderPassMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_add_to_output(self as *const ::vector::VectorRenderPassMutPtr, l as *const ::vector::VectorRenderPassMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::operator+=(const QVector<Qt3DRender::QRenderPass*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorRenderPassMutPtr)
                                 -> &'l0 mut ::vector::VectorRenderPassMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_add_assign_l(self as *mut ::vector::VectorRenderPassMutPtr, l as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::operator+=(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::render_pass::RenderPass)
                                               -> &'l0 mut ::vector::VectorRenderPassMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_add_assign_t(self as *mut ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::operator=(const QVector<Qt3DRender::QRenderPass*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorRenderPassMutPtr)
                             -> &'l0 mut ::vector::VectorRenderPassMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_assign(self as *mut ::vector::VectorRenderPassMutPtr, v as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::operator==(const QVector<Qt3DRender::QRenderPass*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorRenderPassMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_eq(self as *const ::vector::VectorRenderPassMutPtr, v as *const ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* const & QVector<Qt3DRender::QRenderPass*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_index_const(self as *const ::vector::VectorRenderPassMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass*& QVector<Qt3DRender::QRenderPass*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::render_pass::RenderPass {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_index(self as *mut ::vector::VectorRenderPassMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::operator!=(const QVector<Qt3DRender::QRenderPass*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorRenderPassMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_neq(self as *const ::vector::VectorRenderPassMutPtr, v as *const ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::operator<<(const QVector<Qt3DRender::QRenderPass*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorRenderPassMutPtr)
                          -> &'l0 mut ::vector::VectorRenderPassMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_shl_l(self as *mut ::vector::VectorRenderPassMutPtr, l as *const ::vector::VectorRenderPassMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>& QVector<Qt3DRender::QRenderPass*>::operator<<(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::render_pass::RenderPass)
                                        -> &'l0 mut ::vector::VectorRenderPassMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_operator_shl_t(self as *mut ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_pop_back(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_pop_front(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::prepend(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::render_pass::RenderPass) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_prepend(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                     t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::push_back(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::render_pass::RenderPass) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_push_back(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                       t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::push_front(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::render_pass::RenderPass) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_push_front(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                        t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderPass*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderPassMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::removeAll(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::render_pass::RenderPass) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_removeAll(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                       t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_removeAt(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                        i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_removeFirst(self as *mut ::vector::VectorRenderPassMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_removeLast(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::removeOne(Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::render_pass::RenderPass) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_removeOne(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                       t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::replace(int i, Qt3DRender::QRenderPass* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::render_pass::RenderPass) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_replace(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                     i,
                                                                     t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_reserve(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                       size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_resize(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderPass*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_size(self as *const ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_squeeze(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderPass*>::startsWith(Qt3DRender::QRenderPass* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::render_pass::RenderPass) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_startsWith(self as *const ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderPass*>::swap(QVector<Qt3DRender::QRenderPass*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorRenderPassMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_swap(self as *mut ::vector::VectorRenderPassMutPtr,
                                                                    other as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* QVector<Qt3DRender::QRenderPass*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::render_pass::RenderPass {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_takeAt(self as *mut ::vector::VectorRenderPassMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* QVector<Qt3DRender::QRenderPass*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::render_pass::RenderPass {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_takeFirst(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* QVector<Qt3DRender::QRenderPass*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::render_pass::RenderPass {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_takeLast(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* QVector<Qt3DRender::QRenderPass*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::render_pass::RenderPass {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_value_i(self as *const ::vector::VectorRenderPassMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderPass* QVector<Qt3DRender::QRenderPass*>::value(int i, Qt3DRender::QRenderPass* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::render_pass::RenderPass)
                             -> *mut ::render_pass::RenderPass {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_value_i_defaultValue(self as *const ::vector::VectorRenderPassMutPtr, i, default_value as *const *mut ::render_pass::RenderPass)
  }
}

impl Drop for ::vector::VectorRenderPassMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QRenderPass*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_destructor(self as *mut ::vector::VectorRenderPassMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>```</span>
#[repr(C)]
pub struct VectorRenderStateMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_RENDER_STATE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorRenderStateMutPtr {
  unsafe fn new_uninitialized() -> VectorRenderStateMutPtr {
    VectorRenderStateMutPtr(::std::mem::uninitialized())
  }
}

impl VectorRenderStateMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::append(const QVector<Qt3DRender::QRenderState*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorRenderStateMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_append_l(self as *mut ::vector::VectorRenderStateMutPtr, l as *const ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::append(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_append_t(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                       t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_at(self as *const ::vector::VectorRenderStateMutPtr,
                                                                     i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_back_const(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState*& QVector<Qt3DRender::QRenderState*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_back(self as *mut ::vector::VectorRenderStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_capacity(self as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_clear(self as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const * QVector<Qt3DRender::QRenderState*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::render_state::RenderState {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constData(self as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constFirst(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constLast(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::contains(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::render_state::RenderState) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_contains(self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_count_no_args(self as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::count(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::render_state::RenderState) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_count_t(self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const * QVector<Qt3DRender::QRenderState*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::render_state::RenderState {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_data_const(self as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState** QVector<Qt3DRender::QRenderState*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::render_state::RenderState {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_data(self as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_empty(self as *const ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::endsWith(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::render_state::RenderState) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_endsWith(self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::render_state::RenderState) -> &'l0 mut ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::fill(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::render_state::RenderState, ::libc::c_int)) -> &'l0 mut ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::fill(Qt3DRender::QRenderState* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorRenderStateMutPtr
    where Args: overloading::VectorRenderStateMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_first_const(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState*& QVector<Qt3DRender::QRenderState*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_first(self as *mut ::vector::VectorRenderStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_front_const(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState*& QVector<Qt3DRender::QRenderState*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_front(self as *mut ::vector::VectorRenderStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::render_state::RenderState) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::indexOf(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::render_state::RenderState, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::indexOf(Qt3DRender::QRenderState* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderStateMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::render_state::RenderState)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::insert(int i, Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::render_state::RenderState)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::insert(int i, int n, Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderStateMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_isEmpty(self as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_last_const(self as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::render_state::RenderState) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::lastIndexOf(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::render_state::RenderState, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::lastIndexOf(Qt3DRender::QRenderState* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderStateMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState*& QVector<Qt3DRender::QRenderState*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_state::RenderState {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_last(self as *mut ::vector::VectorRenderStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_length(self as *const ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*> QVector<Qt3DRender::QRenderState*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*> QVector<Qt3DRender::QRenderState*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorRenderStateMutPtr
    where Args: overloading::VectorRenderStateMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_move(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                     from,
                                                                     to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderState*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorRenderStateMutPtr) -> ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderState*>::QVector(const QVector<Qt3DRender::QRenderState*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorRenderStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderState*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorRenderStateMutPtr
    where Args: overloading::VectorRenderStateMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderState*>::QVector(int size, Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::render_state::RenderState)
                           -> ::vector::VectorRenderStateMutPtr {
    {
      let mut object: ::vector::VectorRenderStateMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constructor_size_t(size, t as *const *mut ::render_state::RenderState, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*> QVector<Qt3DRender::QRenderState*>::operator+(const QVector<Qt3DRender::QRenderState*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorRenderStateMutPtr) -> ::vector::VectorRenderStateMutPtr {
    {
      let mut object: ::vector::VectorRenderStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_add_to_output(self as *const ::vector::VectorRenderStateMutPtr, l as *const ::vector::VectorRenderStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::operator+=(const QVector<Qt3DRender::QRenderState*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorRenderStateMutPtr)
                                 -> &'l0 mut ::vector::VectorRenderStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_add_assign_l(self as *mut ::vector::VectorRenderStateMutPtr, l as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::operator+=(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::render_state::RenderState)
                                               -> &'l0 mut ::vector::VectorRenderStateMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_add_assign_t(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::operator=(const QVector<Qt3DRender::QRenderState*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorRenderStateMutPtr)
                             -> &'l0 mut ::vector::VectorRenderStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_assign(self as *mut ::vector::VectorRenderStateMutPtr, v as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::operator==(const QVector<Qt3DRender::QRenderState*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorRenderStateMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_eq(self as *const ::vector::VectorRenderStateMutPtr, v as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* const & QVector<Qt3DRender::QRenderState*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_index_const(self as *const ::vector::VectorRenderStateMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState*& QVector<Qt3DRender::QRenderState*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::render_state::RenderState {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_index(self as *mut ::vector::VectorRenderStateMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::operator!=(const QVector<Qt3DRender::QRenderState*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorRenderStateMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_neq(self as *const ::vector::VectorRenderStateMutPtr, v as *const ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::operator<<(const QVector<Qt3DRender::QRenderState*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorRenderStateMutPtr)
                          -> &'l0 mut ::vector::VectorRenderStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_shl_l(self as *mut ::vector::VectorRenderStateMutPtr, l as *const ::vector::VectorRenderStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>& QVector<Qt3DRender::QRenderState*>::operator<<(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::render_state::RenderState)
                                        -> &'l0 mut ::vector::VectorRenderStateMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_operator_shl_t(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_pop_back(self as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_pop_front(self as *mut ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::prepend(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_prepend(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                      t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::push_back(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_push_back(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::push_front(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_push_front(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderState*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderStateMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::removeAll(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::render_state::RenderState) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_removeAll(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_removeAt(self as *mut ::vector::VectorRenderStateMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_removeFirst(self as *mut ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_removeLast(self as *mut ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::removeOne(Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::render_state::RenderState) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_removeOne(self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::replace(int i, Qt3DRender::QRenderState* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::render_state::RenderState) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_replace(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                      i,
                                                                      t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_reserve(self as *mut ::vector::VectorRenderStateMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_resize(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                       size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderState*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_size(self as *const ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_squeeze(self as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderState*>::startsWith(Qt3DRender::QRenderState* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::render_state::RenderState) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_startsWith(self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderState*>::swap(QVector<Qt3DRender::QRenderState*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorRenderStateMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_swap(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                     other as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* QVector<Qt3DRender::QRenderState*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::render_state::RenderState {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_takeAt(self as *mut ::vector::VectorRenderStateMutPtr,
                                                                       i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* QVector<Qt3DRender::QRenderState*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::render_state::RenderState {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_takeFirst(self as *mut ::vector::VectorRenderStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* QVector<Qt3DRender::QRenderState*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::render_state::RenderState {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_takeLast(self as *mut ::vector::VectorRenderStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* QVector<Qt3DRender::QRenderState*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::render_state::RenderState {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_value_i(self as *const ::vector::VectorRenderStateMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderState* QVector<Qt3DRender::QRenderState*>::value(int i, Qt3DRender::QRenderState* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::render_state::RenderState)
                             -> *mut ::render_state::RenderState {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_value_i_defaultValue(self as *const ::vector::VectorRenderStateMutPtr, i, default_value as *const *mut ::render_state::RenderState)
  }
}

impl Drop for ::vector::VectorRenderStateMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QRenderState*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_destructor(self as *mut ::vector::VectorRenderStateMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>```</span>
#[repr(C)]
pub struct VectorRenderTargetOutputAttachmentPoint([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_RENDER_TARGET_OUTPUT_ATTACHMENT_POINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorRenderTargetOutputAttachmentPoint {
  unsafe fn new_uninitialized() -> VectorRenderTargetOutputAttachmentPoint {
    VectorRenderTargetOutputAttachmentPoint(::std::mem::uninitialized())
  }
}

impl VectorRenderTargetOutputAttachmentPoint {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorRenderTargetOutputAttachmentPoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::append(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::render_target_output::AttachmentPoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::append(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderTargetOutputAttachmentPointAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_at(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_back_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_back(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_capacity(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_clear(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint* QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constData(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constFirst(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constLast(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::contains(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::render_target_output::AttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_contains(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::render_target_output::AttachmentPoint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::count(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderTargetOutputAttachmentPointCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint* QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_data_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint* QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_data(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_empty(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::endsWith(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::render_target_output::AttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_endsWith(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::render_target_output::AttachmentPoint) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::fill(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::render_target_output::AttachmentPoint, ::libc::c_int)) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::fill(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self,
                            args: Args)
                            -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_first_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_first(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_front_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_front(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::render_target_output::AttachmentPoint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::indexOf(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::render_target_output::AttachmentPoint, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::indexOf(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderTargetOutputAttachmentPointIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::render_target_output::AttachmentPoint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::insert(int i, const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::render_target_output::AttachmentPoint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::insert(int i, int n, const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderTargetOutputAttachmentPointInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_isEmpty(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_last_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::render_target_output::AttachmentPoint) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::lastIndexOf(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::render_target_output::AttachmentPoint, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::lastIndexOf(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderTargetOutputAttachmentPointLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_last(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_length(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint> QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint> QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorRenderTargetOutputAttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_move(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorRenderTargetOutputAttachmentPoint) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::QVector(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::render_target_output::AttachmentPoint)) -> ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::QVector(int size, const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorRenderTargetOutputAttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint> QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator+(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorRenderTargetOutputAttachmentPoint)
                -> ::vector::VectorRenderTargetOutputAttachmentPoint {
    {
      let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_add_to_output(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, l as *const ::vector::VectorRenderTargetOutputAttachmentPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator+=(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::render_target_output::AttachmentPoint) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator+=(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator=(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorRenderTargetOutputAttachmentPoint)
                             -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_assign(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, v as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator==(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorRenderTargetOutputAttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_eq(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, v as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_index_const(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::render_target_output::AttachmentPoint {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_index(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator!=(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorRenderTargetOutputAttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_neq(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, v as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator<<(const QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::render_target_output::AttachmentPoint) -> &'l0 mut ::vector::VectorRenderTargetOutputAttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::operator<<(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self,
                              args: Args)
                              -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_pop_back(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_pop_front(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::prepend(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::render_target_output::AttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_prepend(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::push_back(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::render_target_output::AttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_push_back(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::push_front(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::render_target_output::AttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_push_front(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderTargetOutputAttachmentPointRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::removeAll(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::render_target_output::AttachmentPoint) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_removeAll(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_removeAt(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_removeFirst(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_removeLast(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::removeOne(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::render_target_output::AttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_removeOne(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::replace(int i, const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::render_target_output::AttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_replace(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_reserve(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_resize(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_size(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_squeeze(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::startsWith(const Qt3DRender::QRenderTargetOutput::AttachmentPoint& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::render_target_output::AttachmentPoint) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_startsWith(self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::swap(QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorRenderTargetOutputAttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_swap(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, other as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_takeAt(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_takeFirst(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_takeLast(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::render_target_output::AttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::render_target_output::AttachmentPoint)) -> ::render_target_output::AttachmentPoint```<br>
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::value(int i, const Qt3DRender::QRenderTargetOutput::AttachmentPoint& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::render_target_output::AttachmentPoint
    where Args: overloading::VectorRenderTargetOutputAttachmentPointValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorRenderTargetOutputAttachmentPoint {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QRenderTargetOutput::AttachmentPoint>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_destructor(self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>```</span>
#[repr(C)]
pub struct VectorRenderTargetOutputMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_RENDER_TARGET_OUTPUT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorRenderTargetOutputMutPtr {
  unsafe fn new_uninitialized() -> VectorRenderTargetOutputMutPtr {
    VectorRenderTargetOutputMutPtr(::std::mem::uninitialized())
  }
}

impl VectorRenderTargetOutputMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::append(const QVector<Qt3DRender::QRenderTargetOutput*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorRenderTargetOutputMutPtr) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_append_l(self as *mut ::vector::VectorRenderTargetOutputMutPtr, l as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::append(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_append_t(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_at(self as *const ::vector::VectorRenderTargetOutputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_back_const(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput*& QVector<Qt3DRender::QRenderTargetOutput*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_back(self as *mut ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_capacity(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_clear(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const * QVector<Qt3DRender::QRenderTargetOutput*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constData(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constFirst(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constLast(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::contains(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::render_target_output::RenderTargetOutput) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_contains(self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_count_no_args(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::count(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::render_target_output::RenderTargetOutput) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_count_t(self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const * QVector<Qt3DRender::QRenderTargetOutput*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_data_const(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput** QVector<Qt3DRender::QRenderTargetOutput*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_data(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_empty(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::endsWith(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::render_target_output::RenderTargetOutput) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_endsWith(self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::render_target_output::RenderTargetOutput) -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::fill(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::render_target_output::RenderTargetOutput, ::libc::c_int)) -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::fill(Qt3DRender::QRenderTargetOutput* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorRenderTargetOutputMutPtr
    where Args: overloading::VectorRenderTargetOutputMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_first_const(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput*& QVector<Qt3DRender::QRenderTargetOutput*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_first(self as *mut ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_front_const(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput*& QVector<Qt3DRender::QRenderTargetOutput*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_front(self as *mut ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::render_target_output::RenderTargetOutput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::indexOf(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::render_target_output::RenderTargetOutput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::indexOf(Qt3DRender::QRenderTargetOutput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderTargetOutputMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::render_target_output::RenderTargetOutput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::insert(int i, Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::render_target_output::RenderTargetOutput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::insert(int i, int n, Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderTargetOutputMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_isEmpty(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_last_const(self as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::render_target_output::RenderTargetOutput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::lastIndexOf(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::render_target_output::RenderTargetOutput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::lastIndexOf(Qt3DRender::QRenderTargetOutput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorRenderTargetOutputMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput*& QVector<Qt3DRender::QRenderTargetOutput*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_last(self as *mut ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_length(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*> QVector<Qt3DRender::QRenderTargetOutput*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*> QVector<Qt3DRender::QRenderTargetOutput*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorRenderTargetOutputMutPtr
    where Args: overloading::VectorRenderTargetOutputMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_move(self as *mut ::vector::VectorRenderTargetOutputMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorRenderTargetOutputMutPtr) -> ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput*>::QVector(const QVector<Qt3DRender::QRenderTargetOutput*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorRenderTargetOutputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorRenderTargetOutputMutPtr
    where Args: overloading::VectorRenderTargetOutputMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QRenderTargetOutput*>::QVector(int size, Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::render_target_output::RenderTargetOutput)
                           -> ::vector::VectorRenderTargetOutputMutPtr {
    {
      let mut object: ::vector::VectorRenderTargetOutputMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constructor_size_t(size, t as *const *mut ::render_target_output::RenderTargetOutput, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*> QVector<Qt3DRender::QRenderTargetOutput*>::operator+(const QVector<Qt3DRender::QRenderTargetOutput*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorRenderTargetOutputMutPtr) -> ::vector::VectorRenderTargetOutputMutPtr {
    {
      let mut object: ::vector::VectorRenderTargetOutputMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_add_to_output(self as *const ::vector::VectorRenderTargetOutputMutPtr, l as *const ::vector::VectorRenderTargetOutputMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::operator+=(const QVector<Qt3DRender::QRenderTargetOutput*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorRenderTargetOutputMutPtr)
                                 -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_add_assign_l(self as *mut ::vector::VectorRenderTargetOutputMutPtr, l as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::operator+=(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::render_target_output::RenderTargetOutput)
                                               -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_add_assign_t(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::operator=(const QVector<Qt3DRender::QRenderTargetOutput*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorRenderTargetOutputMutPtr)
                             -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_assign(self as *mut ::vector::VectorRenderTargetOutputMutPtr, v as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::operator==(const QVector<Qt3DRender::QRenderTargetOutput*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorRenderTargetOutputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_eq(self as *const ::vector::VectorRenderTargetOutputMutPtr, v as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* const & QVector<Qt3DRender::QRenderTargetOutput*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_index_const(self as *const ::vector::VectorRenderTargetOutputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput*& QVector<Qt3DRender::QRenderTargetOutput*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut *mut ::render_target_output::RenderTargetOutput {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_index(self as *mut ::vector::VectorRenderTargetOutputMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::operator!=(const QVector<Qt3DRender::QRenderTargetOutput*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorRenderTargetOutputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_neq(self as *const ::vector::VectorRenderTargetOutputMutPtr, v as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::operator<<(const QVector<Qt3DRender::QRenderTargetOutput*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorRenderTargetOutputMutPtr)
                          -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_shl_l(self as *mut ::vector::VectorRenderTargetOutputMutPtr, l as *const ::vector::VectorRenderTargetOutputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>& QVector<Qt3DRender::QRenderTargetOutput*>::operator<<(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::render_target_output::RenderTargetOutput)
                                        -> &'l0 mut ::vector::VectorRenderTargetOutputMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_operator_shl_t(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_pop_back(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_pop_front(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::prepend(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_prepend(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::push_back(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_push_back(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::push_front(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_push_front(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QRenderTargetOutput*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorRenderTargetOutputMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::removeAll(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_removeAll(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_removeAt(self as *mut ::vector::VectorRenderTargetOutputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_removeFirst(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_removeLast(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::removeOne(Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::render_target_output::RenderTargetOutput) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_removeOne(self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::replace(int i, Qt3DRender::QRenderTargetOutput* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::render_target_output::RenderTargetOutput) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_replace(self as *mut ::vector::VectorRenderTargetOutputMutPtr, i, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_reserve(self as *mut ::vector::VectorRenderTargetOutputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_resize(self as *mut ::vector::VectorRenderTargetOutputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QRenderTargetOutput*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_size(self as *const ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_squeeze(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QRenderTargetOutput*>::startsWith(Qt3DRender::QRenderTargetOutput* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::render_target_output::RenderTargetOutput) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_startsWith(self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QRenderTargetOutput*>::swap(QVector<Qt3DRender::QRenderTargetOutput*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorRenderTargetOutputMutPtr) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_swap(self as *mut ::vector::VectorRenderTargetOutputMutPtr, other as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* QVector<Qt3DRender::QRenderTargetOutput*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_takeAt(self as *mut ::vector::VectorRenderTargetOutputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* QVector<Qt3DRender::QRenderTargetOutput*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_takeFirst(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* QVector<Qt3DRender::QRenderTargetOutput*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_takeLast(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* QVector<Qt3DRender::QRenderTargetOutput*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::render_target_output::RenderTargetOutput {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_value_i(self as *const ::vector::VectorRenderTargetOutputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput* QVector<Qt3DRender::QRenderTargetOutput*>::value(int i, Qt3DRender::QRenderTargetOutput* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::render_target_output::RenderTargetOutput)
                             -> *mut ::render_target_output::RenderTargetOutput {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_value_i_defaultValue(self as *const ::vector::VectorRenderTargetOutputMutPtr, i, default_value as *const *mut ::render_target_output::RenderTargetOutput)
  }
}

impl Drop for ::vector::VectorRenderTargetOutputMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QRenderTargetOutput*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_destructor(self as *mut ::vector::VectorRenderTargetOutputMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>```</span>
#[repr(C)]
pub struct VectorSharedPointerSharedPointerTextureImageData([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_SHARED_POINTER_SHARED_POINTER_TEXTURE_IMAGE_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorSharedPointerSharedPointerTextureImageData {
  unsafe fn new_uninitialized() -> VectorSharedPointerSharedPointerTextureImageData {
    VectorSharedPointerSharedPointerTextureImageData(::std::mem::uninitialized())
  }
}

impl VectorSharedPointerSharedPointerTextureImageData {
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::shared_pointer::SharedPointerTextureImageData) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::append(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorSharedPointerSharedPointerTextureImageData) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::append(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_at(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_back_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_back(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_capacity(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_clear(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>* QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::shared_pointer::SharedPointerTextureImageData {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constData(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constFirst(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constLast(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::contains(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::shared_pointer::SharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_contains(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::shared_pointer::SharedPointerTextureImageData) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::count(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>* QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::shared_pointer::SharedPointerTextureImageData {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_data_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>* QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::shared_pointer::SharedPointerTextureImageData {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_data(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_empty(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::endsWith(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::shared_pointer::SharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_endsWith(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::shared_pointer::SharedPointerTextureImageData) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::fill(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::shared_pointer::SharedPointerTextureImageData, ::libc::c_int)) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::fill(const QSharedPointer<Qt3DRender::QTextureImageData>& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self,
                            args: Args)
                            -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_first_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_first(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_front_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_front(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::shared_pointer::SharedPointerTextureImageData) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::indexOf(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::shared_pointer::SharedPointerTextureImageData, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::indexOf(const QSharedPointer<Qt3DRender::QTextureImageData>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::shared_pointer::SharedPointerTextureImageData)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::insert(int i, const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::shared_pointer::SharedPointerTextureImageData)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::insert(int i, int n, const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_isEmpty(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_last_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::shared_pointer::SharedPointerTextureImageData) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::lastIndexOf(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::shared_pointer::SharedPointerTextureImageData, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::lastIndexOf(const QSharedPointer<Qt3DRender::QTextureImageData>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_last(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_length(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorSharedPointerSharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_move(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::QVector(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::shared_pointer::SharedPointerTextureImageData)) -> ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::QVector(int size, const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorSharedPointerSharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator+(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorSharedPointerSharedPointerTextureImageData)
                -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
    {
      let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_add_to_output(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, l as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::shared_pointer::SharedPointerTextureImageData) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator+=(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator+=(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator=(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorSharedPointerSharedPointerTextureImageData)
                             -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_assign(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, v as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator==(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorSharedPointerSharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_eq(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, v as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```const QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_index_const(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut ::shared_pointer::SharedPointerTextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_index(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator!=(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorSharedPointerSharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_neq(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, v as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::shared_pointer::SharedPointerTextureImageData) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator<<(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'l0 mut ::vector::VectorSharedPointerSharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::operator<<(const QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self,
                              args: Args)
                              -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_pop_back(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_pop_front(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::prepend(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_prepend(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::push_back(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_push_back(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::push_front(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_push_front(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::removeAll(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::shared_pointer::SharedPointerTextureImageData) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_removeAll(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_removeAt(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_removeFirst(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_removeLast(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::removeOne(const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::shared_pointer::SharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_removeOne(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::replace(int i, const QSharedPointer<Qt3DRender::QTextureImageData>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_replace(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_reserve(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_resize(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_size(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_squeeze(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::startsWith(const QSharedPointer<Qt3DRender::QTextureImageData>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::shared_pointer::SharedPointerTextureImageData) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_startsWith(self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::swap(QVector<QSharedPointer<Qt3DRender::QTextureImageData>>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorSharedPointerSharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_swap(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, other as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::shared_pointer::SharedPointerTextureImageData {
    {
      let mut object: ::shared_pointer::SharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_takeAt_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::shared_pointer::SharedPointerTextureImageData {
    {
      let mut object: ::shared_pointer::SharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_takeFirst_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::shared_pointer::SharedPointerTextureImageData {
    {
      let mut object: ::shared_pointer::SharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_takeLast_to_output(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::shared_pointer::SharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::shared_pointer::SharedPointerTextureImageData)) -> ::shared_pointer::SharedPointerTextureImageData```<br>
  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QTextureImageData> QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::value(int i, const QSharedPointer<Qt3DRender::QTextureImageData>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::shared_pointer::SharedPointerTextureImageData
    where Args: overloading::VectorSharedPointerSharedPointerTextureImageDataValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorSharedPointerSharedPointerTextureImageData {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QSharedPointer<Qt3DRender::QTextureImageData>>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_destructor(self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>```</span>
#[repr(C)]
pub struct VectorSortPolicySortType([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_SORT_POLICY_SORT_TYPE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorSortPolicySortType {
  unsafe fn new_uninitialized() -> VectorSortPolicySortType {
    VectorSortPolicySortType(::std::mem::uninitialized())
  }
}

impl VectorSortPolicySortType {
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorSortPolicySortType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::append(const QVector<Qt3DRender::QSortPolicy::SortType>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::sort_policy::SortType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::append(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSortPolicySortTypeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_at(self as *const ::vector::VectorSortPolicySortType, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_back_const(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_back(self as *mut ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_capacity(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_clear(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType* QVector<Qt3DRender::QSortPolicy::SortType>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constData(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constFirst(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constLast(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::contains(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::sort_policy::SortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_contains(self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::sort_policy::SortType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::count(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSortPolicySortTypeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType* QVector<Qt3DRender::QSortPolicy::SortType>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_data_const(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType* QVector<Qt3DRender::QSortPolicy::SortType>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_data(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_empty(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::endsWith(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::sort_policy::SortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_endsWith(self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::sort_policy::SortType) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::fill(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::sort_policy::SortType, ::libc::c_int)) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::fill(const Qt3DRender::QSortPolicy::SortType& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorSortPolicySortType
    where Args: overloading::VectorSortPolicySortTypeFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_first_const(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_first(self as *mut ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_front_const(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_front(self as *mut ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::sort_policy::SortType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::indexOf(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::sort_policy::SortType, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::indexOf(const Qt3DRender::QSortPolicy::SortType& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSortPolicySortTypeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::sort_policy::SortType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::insert(int i, const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::sort_policy::SortType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::insert(int i, int n, const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSortPolicySortTypeInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_isEmpty(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_last_const(self as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::sort_policy::SortType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::lastIndexOf(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::sort_policy::SortType, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::lastIndexOf(const Qt3DRender::QSortPolicy::SortType& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorSortPolicySortTypeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_last(self as *mut ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_length(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType> QVector<Qt3DRender::QSortPolicy::SortType>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType> QVector<Qt3DRender::QSortPolicy::SortType>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorSortPolicySortType
    where Args: overloading::VectorSortPolicySortTypeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_move(self as *mut ::vector::VectorSortPolicySortType, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QSortPolicy::SortType>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorSortPolicySortType) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QSortPolicy::SortType>::QVector(const QVector<Qt3DRender::QSortPolicy::SortType>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QSortPolicy::SortType>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::sort_policy::SortType)) -> ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QSortPolicy::SortType>::QVector(int size, const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorSortPolicySortType
    where Args: overloading::VectorSortPolicySortTypeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType> QVector<Qt3DRender::QSortPolicy::SortType>::operator+(const QVector<Qt3DRender::QSortPolicy::SortType>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorSortPolicySortType) -> ::vector::VectorSortPolicySortType {
    {
      let mut object: ::vector::VectorSortPolicySortType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_add_to_output(self as *const ::vector::VectorSortPolicySortType, l as *const ::vector::VectorSortPolicySortType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorSortPolicySortType) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::operator+=(const QVector<Qt3DRender::QSortPolicy::SortType>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::sort_policy::SortType) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::operator+=(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorSortPolicySortType
    where Args: overloading::VectorSortPolicySortTypeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::operator=(const QVector<Qt3DRender::QSortPolicy::SortType>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorSortPolicySortType)
                             -> &'l0 mut ::vector::VectorSortPolicySortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_assign(self as *mut ::vector::VectorSortPolicySortType, v as *const ::vector::VectorSortPolicySortType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::operator==(const QVector<Qt3DRender::QSortPolicy::SortType>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorSortPolicySortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_eq(self as *const ::vector::VectorSortPolicySortType, v as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```const Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_index_const(self as *const ::vector::VectorSortPolicySortType, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType& QVector<Qt3DRender::QSortPolicy::SortType>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::sort_policy::SortType {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_index(self as *mut ::vector::VectorSortPolicySortType, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::operator!=(const QVector<Qt3DRender::QSortPolicy::SortType>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorSortPolicySortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_neq(self as *const ::vector::VectorSortPolicySortType, v as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorSortPolicySortType) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::operator<<(const QVector<Qt3DRender::QSortPolicy::SortType>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::sort_policy::SortType) -> &'l0 mut ::vector::VectorSortPolicySortType```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>& QVector<Qt3DRender::QSortPolicy::SortType>::operator<<(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorSortPolicySortType
    where Args: overloading::VectorSortPolicySortTypeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_pop_back(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_pop_front(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::prepend(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::sort_policy::SortType) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_prepend(self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::push_back(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::sort_policy::SortType) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_push_back(self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::push_front(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::sort_policy::SortType) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_push_front(self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorSortPolicySortTypeRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::removeAll(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::sort_policy::SortType) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_removeAll(self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_removeAt(self as *mut ::vector::VectorSortPolicySortType, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_removeFirst(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_removeLast(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::removeOne(const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::sort_policy::SortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_removeOne(self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::replace(int i, const Qt3DRender::QSortPolicy::SortType& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::sort_policy::SortType) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_replace(self as *mut ::vector::VectorSortPolicySortType, i, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_reserve(self as *mut ::vector::VectorSortPolicySortType, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_resize(self as *mut ::vector::VectorSortPolicySortType, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QSortPolicy::SortType>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_size(self as *const ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_squeeze(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QSortPolicy::SortType>::startsWith(const Qt3DRender::QSortPolicy::SortType& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::sort_policy::SortType) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_startsWith(self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QSortPolicy::SortType>::swap(QVector<Qt3DRender::QSortPolicy::SortType>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorSortPolicySortType) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_swap(self as *mut ::vector::VectorSortPolicySortType, other as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType QVector<Qt3DRender::QSortPolicy::SortType>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_takeAt(self as *mut ::vector::VectorSortPolicySortType, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType QVector<Qt3DRender::QSortPolicy::SortType>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_takeFirst(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType QVector<Qt3DRender::QSortPolicy::SortType>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::sort_policy::SortType {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_takeLast(self as *mut ::vector::VectorSortPolicySortType) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QSortPolicy::SortType>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::sort_policy::SortType```<br>
  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType QVector<Qt3DRender::QSortPolicy::SortType>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::sort_policy::SortType)) -> ::sort_policy::SortType```<br>
  /// C++ method: <span style='color: green;'>```Qt3DRender::QSortPolicy::SortType QVector<Qt3DRender::QSortPolicy::SortType>::value(int i, const Qt3DRender::QSortPolicy::SortType& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::sort_policy::SortType
    where Args: overloading::VectorSortPolicySortTypeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorSortPolicySortType {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QSortPolicy::SortType>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_destructor(self as *mut ::vector::VectorSortPolicySortType) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>```</span>
#[repr(C)]
pub struct VectorTechniqueMutPtr([u8; ::type_sizes::QT_3D_RENDER_VECTOR_VECTOR_TECHNIQUE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorTechniqueMutPtr {
  unsafe fn new_uninitialized() -> VectorTechniqueMutPtr {
    VectorTechniqueMutPtr(::std::mem::uninitialized())
  }
}

impl VectorTechniqueMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::append(const QVector<Qt3DRender::QTechnique*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorTechniqueMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_append_l(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                       l as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::append(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::technique::Technique) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_append_t(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                     t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::technique::Technique {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_at(self as *const ::vector::VectorTechniqueMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_back_const(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique*& QVector<Qt3DRender::QTechnique*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::technique::Technique {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_back(self as *mut ::vector::VectorTechniqueMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_capacity(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_clear(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const * QVector<Qt3DRender::QTechnique*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constData(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constFirst(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constLast(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::contains(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::technique::Technique) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_contains(self as *const ::vector::VectorTechniqueMutPtr,
                                                                     t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_count_no_args(self as *const ::vector::VectorTechniqueMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::count(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::technique::Technique) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_count_t(self as *const ::vector::VectorTechniqueMutPtr,
                                                                    t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const * QVector<Qt3DRender::QTechnique*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_data_const(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique** QVector<Qt3DRender::QTechnique*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_data(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_empty(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::endsWith(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::technique::Technique) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_endsWith(self as *const ::vector::VectorTechniqueMutPtr,
                                                                     t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::technique::Technique) -> &'l0 mut ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::fill(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::technique::Technique, ::libc::c_int)) -> &'l0 mut ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::fill(Qt3DRender::QTechnique* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTechniqueMutPtr
    where Args: overloading::VectorTechniqueMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_first_const(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique*& QVector<Qt3DRender::QTechnique*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::technique::Technique {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_first(self as *mut ::vector::VectorTechniqueMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_front_const(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique*& QVector<Qt3DRender::QTechnique*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::technique::Technique {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_front(self as *mut ::vector::VectorTechniqueMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::technique::Technique) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::indexOf(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::technique::Technique, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::indexOf(Qt3DRender::QTechnique* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTechniqueMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::technique::Technique)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::insert(int i, Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::technique::Technique)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::insert(int i, int n, Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTechniqueMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_isEmpty(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_last_const(self as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::technique::Technique) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::lastIndexOf(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::technique::Technique, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::lastIndexOf(Qt3DRender::QTechnique* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTechniqueMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique*& QVector<Qt3DRender::QTechnique*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::technique::Technique {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_last(self as *mut ::vector::VectorTechniqueMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_length(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*> QVector<Qt3DRender::QTechnique*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*> QVector<Qt3DRender::QTechnique*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorTechniqueMutPtr
    where Args: overloading::VectorTechniqueMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_move(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                   from,
                                                                   to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QTechnique*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorTechniqueMutPtr) -> ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QTechnique*>::QVector(const QVector<Qt3DRender::QTechnique*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorTechniqueMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QTechnique*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorTechniqueMutPtr
    where Args: overloading::VectorTechniqueMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DRender::QTechnique*>::QVector(int size, Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::technique::Technique) -> ::vector::VectorTechniqueMutPtr {
    {
      let mut object: ::vector::VectorTechniqueMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constructor_size_t(size, t as *const *mut ::technique::Technique, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*> QVector<Qt3DRender::QTechnique*>::operator+(const QVector<Qt3DRender::QTechnique*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorTechniqueMutPtr) -> ::vector::VectorTechniqueMutPtr {
    {
      let mut object: ::vector::VectorTechniqueMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_add_to_output(self as *const ::vector::VectorTechniqueMutPtr, l as *const ::vector::VectorTechniqueMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::operator+=(const QVector<Qt3DRender::QTechnique*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorTechniqueMutPtr)
                                 -> &'l0 mut ::vector::VectorTechniqueMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_add_assign_l(self as *mut ::vector::VectorTechniqueMutPtr, l as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::operator+=(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::technique::Technique)
                                               -> &'l0 mut ::vector::VectorTechniqueMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_add_assign_t(self as *mut ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::operator=(const QVector<Qt3DRender::QTechnique*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorTechniqueMutPtr)
                             -> &'l0 mut ::vector::VectorTechniqueMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_assign(self as *mut ::vector::VectorTechniqueMutPtr, v as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::operator==(const QVector<Qt3DRender::QTechnique*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorTechniqueMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_eq(self as *const ::vector::VectorTechniqueMutPtr, v as *const ::vector::VectorTechniqueMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* const & QVector<Qt3DRender::QTechnique*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_index_const(self as *const ::vector::VectorTechniqueMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique*& QVector<Qt3DRender::QTechnique*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::technique::Technique {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_index(self as *mut ::vector::VectorTechniqueMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::operator!=(const QVector<Qt3DRender::QTechnique*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorTechniqueMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_neq(self as *const ::vector::VectorTechniqueMutPtr, v as *const ::vector::VectorTechniqueMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::operator<<(const QVector<Qt3DRender::QTechnique*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorTechniqueMutPtr)
                          -> &'l0 mut ::vector::VectorTechniqueMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_shl_l(self as *mut ::vector::VectorTechniqueMutPtr, l as *const ::vector::VectorTechniqueMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>& QVector<Qt3DRender::QTechnique*>::operator<<(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::technique::Technique)
                                        -> &'l0 mut ::vector::VectorTechniqueMutPtr {
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_operator_shl_t(self as *mut ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_pop_back(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_pop_front(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::prepend(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::technique::Technique) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_prepend(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                    t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::push_back(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::technique::Technique) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_push_back(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                      t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::push_front(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::technique::Technique) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_push_front(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                       t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QTechnique*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTechniqueMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::removeAll(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::technique::Technique) -> ::libc::c_int {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_removeAll(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                      t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_removeAt(self as *mut ::vector::VectorTechniqueMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_removeFirst(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_removeLast(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::removeOne(Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::technique::Technique) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_removeOne(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                      t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::replace(int i, Qt3DRender::QTechnique* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::technique::Technique) {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_replace(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                    i,
                                                                    t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_reserve(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_resize(self as *mut ::vector::VectorTechniqueMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DRender::QTechnique*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_size(self as *const ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_squeeze(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DRender::QTechnique*>::startsWith(Qt3DRender::QTechnique* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::technique::Technique) -> bool {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_startsWith(self as *const ::vector::VectorTechniqueMutPtr,
                                                                       t as *const *mut ::technique::Technique)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DRender::QTechnique*>::swap(QVector<Qt3DRender::QTechnique*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorTechniqueMutPtr) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_swap(self as *mut ::vector::VectorTechniqueMutPtr,
                                                                   other as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* QVector<Qt3DRender::QTechnique*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_takeAt(self as *mut ::vector::VectorTechniqueMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* QVector<Qt3DRender::QTechnique*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_takeFirst(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* QVector<Qt3DRender::QTechnique*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_takeLast(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* QVector<Qt3DRender::QTechnique*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::technique::Technique {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_value_i(self as *const ::vector::VectorTechniqueMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTechnique* QVector<Qt3DRender::QTechnique*>::value(int i, Qt3DRender::QTechnique* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::technique::Technique)
                             -> *mut ::technique::Technique {
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_value_i_defaultValue(self as *const ::vector::VectorTechniqueMutPtr, i, default_value as *const *mut ::technique::Technique)
  }
}

impl Drop for ::vector::VectorTechniqueMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DRender::QTechnique*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_destructor(self as *mut ::vector::VectorTechniqueMutPtr)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::fill](../struct.VectorAbstractTextureImageMutPtr.html#method.fill) method.
  pub trait VectorAbstractTextureImageMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr)
                   -> &'largs mut ::vector::VectorAbstractTextureImageMutPtr;
  }
  impl<'largs> VectorAbstractTextureImageMutPtrFillArgs<'largs> for &'largs *mut ::abstract_texture_image::AbstractTextureImage {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> &'largs mut ::vector::VectorAbstractTextureImageMutPtr {
    let t = self;
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_fill_t(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorAbstractTextureImageMutPtrFillArgs<'largs> for (&'largs *mut ::abstract_texture_image::AbstractTextureImage,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> &'largs mut ::vector::VectorAbstractTextureImageMutPtr {
    let t = self.0;
let size = self.1;
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_fill_t_size(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage, size);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::index_of](../struct.VectorAbstractTextureImageMutPtr.html#method.index_of) method.
  pub trait VectorAbstractTextureImageMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractTextureImageMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_texture_image::AbstractTextureImage {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_indexOf_t(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }
}
  impl<'largs> VectorAbstractTextureImageMutPtrIndexOfArgs<'largs> for (&'largs *mut ::abstract_texture_image::AbstractTextureImage,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_indexOf_t_from(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::insert](../struct.VectorAbstractTextureImageMutPtr.html#method.insert) method.
  pub trait VectorAbstractTextureImageMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> ();
  }
  impl<'largs> VectorAbstractTextureImageMutPtrInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs *mut ::abstract_texture_image::AbstractTextureImage) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, i, n, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }
}
  impl<'largs> VectorAbstractTextureImageMutPtrInsertArgs<'largs> for (::libc::c_int,&'largs *mut ::abstract_texture_image::AbstractTextureImage) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> () {
    let i = self.0;
let t = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_insert_i_t(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, i, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::last_index_of](../struct.VectorAbstractTextureImageMutPtr.html#method.last_index_of) method.
  pub trait VectorAbstractTextureImageMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractTextureImageMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_texture_image::AbstractTextureImage {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage)
  }
}
  impl<'largs> VectorAbstractTextureImageMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::abstract_texture_image::AbstractTextureImage,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, t as *const *mut ::abstract_texture_image::AbstractTextureImage, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::mid](../struct.VectorAbstractTextureImageMutPtr.html#method.mid) method.
  pub trait VectorAbstractTextureImageMutPtrMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr)
            -> ::vector::VectorAbstractTextureImageMutPtr;
  }
  impl<'largs> VectorAbstractTextureImageMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr)
            -> ::vector::VectorAbstractTextureImageMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAbstractTextureImageMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAbstractTextureImageMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextureImageMutPtr)
            -> ::vector::VectorAbstractTextureImageMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAbstractTextureImageMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAbstractTextureImageMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::new](../struct.VectorAbstractTextureImageMutPtr.html#method.new) method.
  pub trait VectorAbstractTextureImageMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAbstractTextureImageMutPtr;
  }
  impl VectorAbstractTextureImageMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAbstractTextureImageMutPtr {

      {
        let mut object: ::vector::VectorAbstractTextureImageMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAbstractTextureImageMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAbstractTextureImageMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAbstractTextureImageMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractTextureImageMutPtrNewArgs for &'a ::vector::VectorAbstractTextureImageMutPtr {
    fn exec(self) -> ::vector::VectorAbstractTextureImageMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAbstractTextureImageMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_constructor_v(v as *const ::vector::VectorAbstractTextureImageMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractTextureImageMutPtr::remove](../struct.VectorAbstractTextureImageMutPtr.html#method.remove) method.
  pub trait VectorAbstractTextureImageMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> ();
  }
  impl<'largs> VectorAbstractTextureImageMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_remove_i(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, i) }
    }
  }
  impl<'largs> VectorAbstractTextureImageMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextureImageMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAbstractTextureImage_ptr_remove_i_n(original_self as *mut ::vector::VectorAbstractTextureImageMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::fill](../struct.VectorAttributeMutPtr.html#method.fill) method.
  pub trait VectorAttributeMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAttributeMutPtr)
                   -> &'largs mut ::vector::VectorAttributeMutPtr;
  }
  impl<'largs> VectorAttributeMutPtrFillArgs<'largs> for &'largs *mut ::attribute::Attribute {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAttributeMutPtr)
                   -> &'largs mut ::vector::VectorAttributeMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_fill_t(original_self as *mut ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorAttributeMutPtrFillArgs<'largs> for (&'largs *mut ::attribute::Attribute, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAttributeMutPtr)
                   -> &'largs mut ::vector::VectorAttributeMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_fill_t_size(original_self as *mut ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::index_of](../struct.VectorAttributeMutPtr.html#method.index_of) method.
  pub trait VectorAttributeMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAttributeMutPtrIndexOfArgs<'largs> for &'largs *mut ::attribute::Attribute {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_indexOf_t(original_self as *const ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute)
    }
  }
  impl<'largs> VectorAttributeMutPtrIndexOfArgs<'largs> for (&'largs *mut ::attribute::Attribute, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_indexOf_t_from(original_self as *const ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::insert](../struct.VectorAttributeMutPtr.html#method.insert) method.
  pub trait VectorAttributeMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> ();
  }
  impl<'largs> VectorAttributeMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::attribute::Attribute) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAttributeMutPtr, i, n, t as *const *mut ::attribute::Attribute)
    }
  }
  impl<'largs> VectorAttributeMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::attribute::Attribute) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_insert_i_t(original_self as *mut ::vector::VectorAttributeMutPtr, i, t as *const *mut ::attribute::Attribute)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::last_index_of](../struct.VectorAttributeMutPtr.html#method.last_index_of) method.
  pub trait VectorAttributeMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAttributeMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::attribute::Attribute {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute)
    }
  }
  impl<'largs> VectorAttributeMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::attribute::Attribute, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAttributeMutPtr, t as *const *mut ::attribute::Attribute, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::mid](../struct.VectorAttributeMutPtr.html#method.mid) method.
  pub trait VectorAttributeMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::vector::VectorAttributeMutPtr;
  }
  impl<'largs> VectorAttributeMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::vector::VectorAttributeMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAttributeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAttributeMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAttributeMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorAttributeMutPtr) -> ::vector::VectorAttributeMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAttributeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAttributeMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::new](../struct.VectorAttributeMutPtr.html#method.new) method.
  pub trait VectorAttributeMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAttributeMutPtr;
  }
  impl VectorAttributeMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAttributeMutPtr {

      {
        let mut object: ::vector::VectorAttributeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAttributeMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAttributeMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAttributeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAttributeMutPtrNewArgs for &'a ::vector::VectorAttributeMutPtr {
    fn exec(self) -> ::vector::VectorAttributeMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAttributeMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_constructor_v(v as *const ::vector::VectorAttributeMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAttributeMutPtr::remove](../struct.VectorAttributeMutPtr.html#method.remove) method.
  pub trait VectorAttributeMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> ();
  }
  impl<'largs> VectorAttributeMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_remove_i(original_self as *mut ::vector::VectorAttributeMutPtr, i) }
    }
  }
  impl<'largs> VectorAttributeMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAttributeMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QAttribute_ptr_remove_i_n(original_self as *mut ::vector::VectorAttributeMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::fill](../struct.VectorFilterKeyMutPtr.html#method.fill) method.
  pub trait VectorFilterKeyMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorFilterKeyMutPtr)
                   -> &'largs mut ::vector::VectorFilterKeyMutPtr;
  }
  impl<'largs> VectorFilterKeyMutPtrFillArgs<'largs> for &'largs *mut ::filter_key::FilterKey {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorFilterKeyMutPtr)
                   -> &'largs mut ::vector::VectorFilterKeyMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_fill_t(original_self as *mut ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorFilterKeyMutPtrFillArgs<'largs> for (&'largs *mut ::filter_key::FilterKey, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorFilterKeyMutPtr)
                   -> &'largs mut ::vector::VectorFilterKeyMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_fill_t_size(original_self as *mut ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::index_of](../struct.VectorFilterKeyMutPtr.html#method.index_of) method.
  pub trait VectorFilterKeyMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorFilterKeyMutPtrIndexOfArgs<'largs> for &'largs *mut ::filter_key::FilterKey {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_indexOf_t(original_self as *const ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey)
    }
  }
  impl<'largs> VectorFilterKeyMutPtrIndexOfArgs<'largs> for (&'largs *mut ::filter_key::FilterKey, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_indexOf_t_from(original_self as *const ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::insert](../struct.VectorFilterKeyMutPtr.html#method.insert) method.
  pub trait VectorFilterKeyMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> ();
  }
  impl<'largs> VectorFilterKeyMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::filter_key::FilterKey) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_insert_i_n_t(original_self as *mut ::vector::VectorFilterKeyMutPtr, i, n, t as *const *mut ::filter_key::FilterKey)
    }
  }
  impl<'largs> VectorFilterKeyMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::filter_key::FilterKey) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_insert_i_t(original_self as *mut ::vector::VectorFilterKeyMutPtr, i, t as *const *mut ::filter_key::FilterKey)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::last_index_of](../struct.VectorFilterKeyMutPtr.html#method.last_index_of) method.
  pub trait VectorFilterKeyMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorFilterKeyMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::filter_key::FilterKey {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_lastIndexOf_t(original_self as *const ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey)
    }
  }
  impl<'largs> VectorFilterKeyMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::filter_key::FilterKey, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorFilterKeyMutPtr, t as *const *mut ::filter_key::FilterKey, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::mid](../struct.VectorFilterKeyMutPtr.html#method.mid) method.
  pub trait VectorFilterKeyMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::vector::VectorFilterKeyMutPtr;
  }
  impl<'largs> VectorFilterKeyMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::vector::VectorFilterKeyMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorFilterKeyMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_mid_to_output_pos(original_self as *const ::vector::VectorFilterKeyMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorFilterKeyMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorFilterKeyMutPtr) -> ::vector::VectorFilterKeyMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorFilterKeyMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorFilterKeyMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::new](../struct.VectorFilterKeyMutPtr.html#method.new) method.
  pub trait VectorFilterKeyMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorFilterKeyMutPtr;
  }
  impl VectorFilterKeyMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorFilterKeyMutPtr {

      {
        let mut object: ::vector::VectorFilterKeyMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorFilterKeyMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorFilterKeyMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorFilterKeyMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorFilterKeyMutPtrNewArgs for &'a ::vector::VectorFilterKeyMutPtr {
    fn exec(self) -> ::vector::VectorFilterKeyMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorFilterKeyMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_constructor_v(v as *const ::vector::VectorFilterKeyMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorFilterKeyMutPtr::remove](../struct.VectorFilterKeyMutPtr.html#method.remove) method.
  pub trait VectorFilterKeyMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> ();
  }
  impl<'largs> VectorFilterKeyMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_remove_i(original_self as *mut ::vector::VectorFilterKeyMutPtr, i) }
    }
  }
  impl<'largs> VectorFilterKeyMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorFilterKeyMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QFilterKey_ptr_remove_i_n(original_self as *mut ::vector::VectorFilterKeyMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::fill](../struct.VectorLayerMutPtr.html#method.fill) method.
  pub trait VectorLayerMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorLayerMutPtr)
                   -> &'largs mut ::vector::VectorLayerMutPtr;
  }
  impl<'largs> VectorLayerMutPtrFillArgs<'largs> for &'largs *mut ::layer::Layer {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorLayerMutPtr)
                   -> &'largs mut ::vector::VectorLayerMutPtr {
      let t = self;
      let ffi_result =
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_fill_t(original_self as *mut ::vector::VectorLayerMutPtr,
                                                                   t as *const *mut ::layer::Layer);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorLayerMutPtrFillArgs<'largs> for (&'largs *mut ::layer::Layer, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorLayerMutPtr)
                   -> &'largs mut ::vector::VectorLayerMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_fill_t_size(original_self as *mut ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::index_of](../struct.VectorLayerMutPtr.html#method.index_of) method.
  pub trait VectorLayerMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorLayerMutPtrIndexOfArgs<'largs> for &'largs *mut ::layer::Layer {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_indexOf_t(original_self as *const ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer)
    }
  }
  impl<'largs> VectorLayerMutPtrIndexOfArgs<'largs> for (&'largs *mut ::layer::Layer, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_indexOf_t_from(original_self as *const ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::insert](../struct.VectorLayerMutPtr.html#method.insert) method.
  pub trait VectorLayerMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> ();
  }
  impl<'largs> VectorLayerMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::layer::Layer) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_insert_i_n_t(original_self as *mut ::vector::VectorLayerMutPtr, i, n, t as *const *mut ::layer::Layer)
    }
  }
  impl<'largs> VectorLayerMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::layer::Layer) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_insert_i_t(original_self as *mut ::vector::VectorLayerMutPtr, i, t as *const *mut ::layer::Layer)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::last_index_of](../struct.VectorLayerMutPtr.html#method.last_index_of) method.
  pub trait VectorLayerMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorLayerMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::layer::Layer {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_lastIndexOf_t(original_self as *const ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer)
    }
  }
  impl<'largs> VectorLayerMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::layer::Layer, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorLayerMutPtr, t as *const *mut ::layer::Layer, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::mid](../struct.VectorLayerMutPtr.html#method.mid) method.
  pub trait VectorLayerMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::vector::VectorLayerMutPtr;
  }
  impl<'largs> VectorLayerMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::vector::VectorLayerMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorLayerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_mid_to_output_pos(original_self as *const ::vector::VectorLayerMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorLayerMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorLayerMutPtr) -> ::vector::VectorLayerMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorLayerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorLayerMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::new](../struct.VectorLayerMutPtr.html#method.new) method.
  pub trait VectorLayerMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorLayerMutPtr;
  }
  impl VectorLayerMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorLayerMutPtr {

      {
        let mut object: ::vector::VectorLayerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorLayerMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorLayerMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorLayerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorLayerMutPtrNewArgs for &'a ::vector::VectorLayerMutPtr {
    fn exec(self) -> ::vector::VectorLayerMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorLayerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_constructor_v(v as *const ::vector::VectorLayerMutPtr,
                                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorLayerMutPtr::remove](../struct.VectorLayerMutPtr.html#method.remove) method.
  pub trait VectorLayerMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> ();
  }
  impl<'largs> VectorLayerMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_remove_i(original_self as *mut ::vector::VectorLayerMutPtr, i)
      }
    }
  }
  impl<'largs> VectorLayerMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorLayerMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QLayer_ptr_remove_i_n(original_self as *mut ::vector::VectorLayerMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::fill](../struct.VectorParameterMutPtr.html#method.fill) method.
  pub trait VectorParameterMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorParameterMutPtr)
                   -> &'largs mut ::vector::VectorParameterMutPtr;
  }
  impl<'largs> VectorParameterMutPtrFillArgs<'largs> for &'largs *mut ::parameter::Parameter {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorParameterMutPtr)
                   -> &'largs mut ::vector::VectorParameterMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_fill_t(original_self as *mut ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorParameterMutPtrFillArgs<'largs> for (&'largs *mut ::parameter::Parameter, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorParameterMutPtr)
                   -> &'largs mut ::vector::VectorParameterMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_fill_t_size(original_self as *mut ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::index_of](../struct.VectorParameterMutPtr.html#method.index_of) method.
  pub trait VectorParameterMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorParameterMutPtrIndexOfArgs<'largs> for &'largs *mut ::parameter::Parameter {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_indexOf_t(original_self as *const ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter)
    }
  }
  impl<'largs> VectorParameterMutPtrIndexOfArgs<'largs> for (&'largs *mut ::parameter::Parameter, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_indexOf_t_from(original_self as *const ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::insert](../struct.VectorParameterMutPtr.html#method.insert) method.
  pub trait VectorParameterMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> ();
  }
  impl<'largs> VectorParameterMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::parameter::Parameter) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_insert_i_n_t(original_self as *mut ::vector::VectorParameterMutPtr, i, n, t as *const *mut ::parameter::Parameter)
    }
  }
  impl<'largs> VectorParameterMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::parameter::Parameter) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_insert_i_t(original_self as *mut ::vector::VectorParameterMutPtr, i, t as *const *mut ::parameter::Parameter)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::last_index_of](../struct.VectorParameterMutPtr.html#method.last_index_of) method.
  pub trait VectorParameterMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorParameterMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::parameter::Parameter {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_lastIndexOf_t(original_self as *const ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter)
    }
  }
  impl<'largs> VectorParameterMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::parameter::Parameter, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorParameterMutPtr, t as *const *mut ::parameter::Parameter, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::mid](../struct.VectorParameterMutPtr.html#method.mid) method.
  pub trait VectorParameterMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::vector::VectorParameterMutPtr;
  }
  impl<'largs> VectorParameterMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::vector::VectorParameterMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorParameterMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_mid_to_output_pos(original_self as *const ::vector::VectorParameterMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorParameterMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorParameterMutPtr) -> ::vector::VectorParameterMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorParameterMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorParameterMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::new](../struct.VectorParameterMutPtr.html#method.new) method.
  pub trait VectorParameterMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorParameterMutPtr;
  }
  impl VectorParameterMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorParameterMutPtr {

      {
        let mut object: ::vector::VectorParameterMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorParameterMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorParameterMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorParameterMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorParameterMutPtrNewArgs for &'a ::vector::VectorParameterMutPtr {
    fn exec(self) -> ::vector::VectorParameterMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorParameterMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_constructor_v(v as *const ::vector::VectorParameterMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorParameterMutPtr::remove](../struct.VectorParameterMutPtr.html#method.remove) method.
  pub trait VectorParameterMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> ();
  }
  impl<'largs> VectorParameterMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_remove_i(original_self as *mut ::vector::VectorParameterMutPtr, i) }
    }
  }
  impl<'largs> VectorParameterMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorParameterMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QParameter_ptr_remove_i_n(original_self as *mut ::vector::VectorParameterMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::fill](../struct.VectorRenderPassMutPtr.html#method.fill) method.
  pub trait VectorRenderPassMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderPassMutPtr)
                   -> &'largs mut ::vector::VectorRenderPassMutPtr;
  }
  impl<'largs> VectorRenderPassMutPtrFillArgs<'largs> for &'largs *mut ::render_pass::RenderPass {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderPassMutPtr)
                   -> &'largs mut ::vector::VectorRenderPassMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_fill_t(original_self as *mut ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorRenderPassMutPtrFillArgs<'largs> for (&'largs *mut ::render_pass::RenderPass, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderPassMutPtr)
                   -> &'largs mut ::vector::VectorRenderPassMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_fill_t_size(original_self as *mut ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::index_of](../struct.VectorRenderPassMutPtr.html#method.index_of) method.
  pub trait VectorRenderPassMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderPassMutPtrIndexOfArgs<'largs> for &'largs *mut ::render_pass::RenderPass {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_indexOf_t(original_self as *const ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass)
    }
  }
  impl<'largs> VectorRenderPassMutPtrIndexOfArgs<'largs> for (&'largs *mut ::render_pass::RenderPass, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_indexOf_t_from(original_self as *const ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::insert](../struct.VectorRenderPassMutPtr.html#method.insert) method.
  pub trait VectorRenderPassMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> ();
  }
  impl<'largs> VectorRenderPassMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::render_pass::RenderPass) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_insert_i_n_t(original_self as *mut ::vector::VectorRenderPassMutPtr, i, n, t as *const *mut ::render_pass::RenderPass)
    }
  }
  impl<'largs> VectorRenderPassMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::render_pass::RenderPass) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_insert_i_t(original_self as *mut ::vector::VectorRenderPassMutPtr, i, t as *const *mut ::render_pass::RenderPass)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::last_index_of](../struct.VectorRenderPassMutPtr.html#method.last_index_of) method.
  pub trait VectorRenderPassMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderPassMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::render_pass::RenderPass {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_lastIndexOf_t(original_self as *const ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass)
    }
  }
  impl<'largs> VectorRenderPassMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::render_pass::RenderPass, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorRenderPassMutPtr, t as *const *mut ::render_pass::RenderPass, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::mid](../struct.VectorRenderPassMutPtr.html#method.mid) method.
  pub trait VectorRenderPassMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::vector::VectorRenderPassMutPtr;
  }
  impl<'largs> VectorRenderPassMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::vector::VectorRenderPassMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorRenderPassMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_mid_to_output_pos(original_self as *const ::vector::VectorRenderPassMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorRenderPassMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorRenderPassMutPtr) -> ::vector::VectorRenderPassMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorRenderPassMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorRenderPassMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::new](../struct.VectorRenderPassMutPtr.html#method.new) method.
  pub trait VectorRenderPassMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorRenderPassMutPtr;
  }
  impl VectorRenderPassMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorRenderPassMutPtr {

      {
        let mut object: ::vector::VectorRenderPassMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorRenderPassMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorRenderPassMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorRenderPassMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorRenderPassMutPtrNewArgs for &'a ::vector::VectorRenderPassMutPtr {
    fn exec(self) -> ::vector::VectorRenderPassMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorRenderPassMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_constructor_v(v as *const ::vector::VectorRenderPassMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderPassMutPtr::remove](../struct.VectorRenderPassMutPtr.html#method.remove) method.
  pub trait VectorRenderPassMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> ();
  }
  impl<'largs> VectorRenderPassMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_remove_i(original_self as *mut ::vector::VectorRenderPassMutPtr, i) }
    }
  }
  impl<'largs> VectorRenderPassMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderPassMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderPass_ptr_remove_i_n(original_self as *mut ::vector::VectorRenderPassMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::fill](../struct.VectorRenderStateMutPtr.html#method.fill) method.
  pub trait VectorRenderStateMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderStateMutPtr)
                   -> &'largs mut ::vector::VectorRenderStateMutPtr;
  }
  impl<'largs> VectorRenderStateMutPtrFillArgs<'largs> for &'largs *mut ::render_state::RenderState {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderStateMutPtr)
                   -> &'largs mut ::vector::VectorRenderStateMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_fill_t(original_self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorRenderStateMutPtrFillArgs<'largs> for (&'largs *mut ::render_state::RenderState, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderStateMutPtr)
                   -> &'largs mut ::vector::VectorRenderStateMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_fill_t_size(original_self as *mut ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::index_of](../struct.VectorRenderStateMutPtr.html#method.index_of) method.
  pub trait VectorRenderStateMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderStateMutPtrIndexOfArgs<'largs> for &'largs *mut ::render_state::RenderState {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_indexOf_t(original_self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
    }
  }
  impl<'largs> VectorRenderStateMutPtrIndexOfArgs<'largs> for (&'largs *mut ::render_state::RenderState, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_indexOf_t_from(original_self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::insert](../struct.VectorRenderStateMutPtr.html#method.insert) method.
  pub trait VectorRenderStateMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> ();
  }
  impl<'largs> VectorRenderStateMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::render_state::RenderState) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_insert_i_n_t(original_self as *mut ::vector::VectorRenderStateMutPtr, i, n, t as *const *mut ::render_state::RenderState)
    }
  }
  impl<'largs> VectorRenderStateMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::render_state::RenderState) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_insert_i_t(original_self as *mut ::vector::VectorRenderStateMutPtr, i, t as *const *mut ::render_state::RenderState)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::last_index_of](../struct.VectorRenderStateMutPtr.html#method.last_index_of) method.
  pub trait VectorRenderStateMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderStateMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::render_state::RenderState {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_lastIndexOf_t(original_self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState)
    }
  }
  impl<'largs> VectorRenderStateMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::render_state::RenderState, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorRenderStateMutPtr, t as *const *mut ::render_state::RenderState, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::mid](../struct.VectorRenderStateMutPtr.html#method.mid) method.
  pub trait VectorRenderStateMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::vector::VectorRenderStateMutPtr;
  }
  impl<'largs> VectorRenderStateMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::vector::VectorRenderStateMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorRenderStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_mid_to_output_pos(original_self as *const ::vector::VectorRenderStateMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorRenderStateMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorRenderStateMutPtr) -> ::vector::VectorRenderStateMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorRenderStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorRenderStateMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::new](../struct.VectorRenderStateMutPtr.html#method.new) method.
  pub trait VectorRenderStateMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorRenderStateMutPtr;
  }
  impl VectorRenderStateMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorRenderStateMutPtr {

      {
        let mut object: ::vector::VectorRenderStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorRenderStateMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorRenderStateMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorRenderStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorRenderStateMutPtrNewArgs for &'a ::vector::VectorRenderStateMutPtr {
    fn exec(self) -> ::vector::VectorRenderStateMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorRenderStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_constructor_v(v as *const ::vector::VectorRenderStateMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderStateMutPtr::remove](../struct.VectorRenderStateMutPtr.html#method.remove) method.
  pub trait VectorRenderStateMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> ();
  }
  impl<'largs> VectorRenderStateMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_remove_i(original_self as *mut ::vector::VectorRenderStateMutPtr, i) }
    }
  }
  impl<'largs> VectorRenderStateMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderStateMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderState_ptr_remove_i_n(original_self as *mut ::vector::VectorRenderStateMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::append](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.append) method.
  pub trait VectorRenderTargetOutputAttachmentPointAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> ();
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointAppendArgs<'largs> for &'largs ::vector::VectorRenderTargetOutputAttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
    let l = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_append_l(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, l as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointAppendArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_append_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::count](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.count) method.
  pub trait VectorRenderTargetOutputAttachmentPointCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {

      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_count_no_args(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint) }
    }
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointCountArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_count_t(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::fill](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.fill) method.
  pub trait VectorRenderTargetOutputAttachmentPointFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointFillArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_fill_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointFillArgs<'largs> for (&'largs ::render_target_output::AttachmentPoint,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_fill_t_size(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::index_of](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.index_of) method.
  pub trait VectorRenderTargetOutputAttachmentPointIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointIndexOfArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_indexOf_t(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointIndexOfArgs<'largs> for (&'largs ::render_target_output::AttachmentPoint,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_indexOf_t_from(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::insert](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.insert) method.
  pub trait VectorRenderTargetOutputAttachmentPointInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> ();
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::render_target_output::AttachmentPoint) {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_insert_i_n_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i, n, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointInsertArgs<'largs> for (::libc::c_int,&'largs ::render_target_output::AttachmentPoint) {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_insert_i_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::last_index_of](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.last_index_of) method.
  pub trait VectorRenderTargetOutputAttachmentPointLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointLastIndexOfArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_lastIndexOf_t(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) }
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointLastIndexOfArgs<'largs> for (&'largs ::render_target_output::AttachmentPoint,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_lastIndexOf_t_from(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::mid](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.mid) method.
  pub trait VectorRenderTargetOutputAttachmentPointMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> ::vector::VectorRenderTargetOutputAttachmentPoint;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> ::vector::VectorRenderTargetOutputAttachmentPoint {
      let pos = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_mid_to_output_pos(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> ::vector::VectorRenderTargetOutputAttachmentPoint {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_mid_to_output_pos_len(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::new](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.new) method.
  pub trait VectorRenderTargetOutputAttachmentPointNewArgs {
    fn exec(self) -> ::vector::VectorRenderTargetOutputAttachmentPoint;
  }
  impl VectorRenderTargetOutputAttachmentPointNewArgs for () {
    fn exec(self) -> ::vector::VectorRenderTargetOutputAttachmentPoint {

      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorRenderTargetOutputAttachmentPointNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorRenderTargetOutputAttachmentPoint {
      let size = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constructor_size(size,
                                                                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorRenderTargetOutputAttachmentPointNewArgs
    for (::libc::c_int, &'a ::render_target_output::AttachmentPoint) {
    fn exec(self) -> ::vector::VectorRenderTargetOutputAttachmentPoint {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constructor_size_t(size, t as *const ::render_target_output::AttachmentPoint, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorRenderTargetOutputAttachmentPointNewArgs for &'a ::vector::VectorRenderTargetOutputAttachmentPoint {
    fn exec(self) -> ::vector::VectorRenderTargetOutputAttachmentPoint {
      let v = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputAttachmentPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_constructor_v(v as *const ::vector::VectorRenderTargetOutputAttachmentPoint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::op_add_assign](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.op_add_assign) method.
  pub trait VectorRenderTargetOutputAttachmentPointOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointOpAddAssignArgs<'largs> for &'largs ::vector::VectorRenderTargetOutputAttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_add_assign_l(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, l as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointOpAddAssignArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_add_assign_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::op_shl](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.op_shl) method.
  pub trait VectorRenderTargetOutputAttachmentPointOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointOpShlArgs<'largs> for &'largs ::vector::VectorRenderTargetOutputAttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_shl_l(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, l as *const ::vector::VectorRenderTargetOutputAttachmentPoint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorRenderTargetOutputAttachmentPointOpShlArgs<'largs> for &'largs ::render_target_output::AttachmentPoint {

  fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_operator_shl_t(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, t as *const ::render_target_output::AttachmentPoint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::remove](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.remove) method.
  pub trait VectorRenderTargetOutputAttachmentPointRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> ();
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_remove_i(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i) }
    }
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputAttachmentPoint) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_remove_i_n(original_self as *mut ::vector::VectorRenderTargetOutputAttachmentPoint, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputAttachmentPoint::value](../struct.VectorRenderTargetOutputAttachmentPoint.html#method.value) method.
  pub trait VectorRenderTargetOutputAttachmentPointValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> ::render_target_output::AttachmentPoint;
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint)
            -> ::render_target_output::AttachmentPoint {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_value_i(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, i) }
    }
  }
  impl<'largs> VectorRenderTargetOutputAttachmentPointValueArgs<'largs> for (::libc::c_int,&'largs ::render_target_output::AttachmentPoint) {

  fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputAttachmentPoint) -> ::render_target_output::AttachmentPoint {
    let i = self.0;
let default_value = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_AttachmentPoint_value_i_defaultValue(original_self as *const ::vector::VectorRenderTargetOutputAttachmentPoint, i, default_value as *const ::render_target_output::AttachmentPoint) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::fill](../struct.VectorRenderTargetOutputMutPtr.html#method.fill) method.
  pub trait VectorRenderTargetOutputMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr)
                   -> &'largs mut ::vector::VectorRenderTargetOutputMutPtr;
  }
  impl<'largs> VectorRenderTargetOutputMutPtrFillArgs<'largs> for &'largs *mut ::render_target_output::RenderTargetOutput {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> &'largs mut ::vector::VectorRenderTargetOutputMutPtr {
    let t = self;
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_fill_t(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorRenderTargetOutputMutPtrFillArgs<'largs> for (&'largs *mut ::render_target_output::RenderTargetOutput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> &'largs mut ::vector::VectorRenderTargetOutputMutPtr {
    let t = self.0;
let size = self.1;
    let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_fill_t_size(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput, size);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::index_of](../struct.VectorRenderTargetOutputMutPtr.html#method.index_of) method.
  pub trait VectorRenderTargetOutputMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderTargetOutputMutPtrIndexOfArgs<'largs> for &'largs *mut ::render_target_output::RenderTargetOutput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_indexOf_t(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }
}
  impl<'largs> VectorRenderTargetOutputMutPtrIndexOfArgs<'largs> for (&'largs *mut ::render_target_output::RenderTargetOutput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_indexOf_t_from(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::insert](../struct.VectorRenderTargetOutputMutPtr.html#method.insert) method.
  pub trait VectorRenderTargetOutputMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> ();
  }
  impl<'largs> VectorRenderTargetOutputMutPtrInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs *mut ::render_target_output::RenderTargetOutput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_insert_i_n_t(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, i, n, t as *const *mut ::render_target_output::RenderTargetOutput)
  }
}
  impl<'largs> VectorRenderTargetOutputMutPtrInsertArgs<'largs> for (::libc::c_int,&'largs *mut ::render_target_output::RenderTargetOutput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> () {
    let i = self.0;
let t = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_insert_i_t(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, i, t as *const *mut ::render_target_output::RenderTargetOutput)
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::last_index_of](../struct.VectorRenderTargetOutputMutPtr.html#method.last_index_of) method.
  pub trait VectorRenderTargetOutputMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorRenderTargetOutputMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::render_target_output::RenderTargetOutput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_lastIndexOf_t(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput)
  }
}
  impl<'largs> VectorRenderTargetOutputMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::render_target_output::RenderTargetOutput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, t as *const *mut ::render_target_output::RenderTargetOutput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::mid](../struct.VectorRenderTargetOutputMutPtr.html#method.mid) method.
  pub trait VectorRenderTargetOutputMutPtrMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr)
            -> ::vector::VectorRenderTargetOutputMutPtr;
  }
  impl<'largs> VectorRenderTargetOutputMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr)
            -> ::vector::VectorRenderTargetOutputMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_mid_to_output_pos(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorRenderTargetOutputMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorRenderTargetOutputMutPtr)
            -> ::vector::VectorRenderTargetOutputMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorRenderTargetOutputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorRenderTargetOutputMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::new](../struct.VectorRenderTargetOutputMutPtr.html#method.new) method.
  pub trait VectorRenderTargetOutputMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorRenderTargetOutputMutPtr;
  }
  impl VectorRenderTargetOutputMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorRenderTargetOutputMutPtr {

      {
        let mut object: ::vector::VectorRenderTargetOutputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorRenderTargetOutputMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorRenderTargetOutputMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorRenderTargetOutputMutPtrNewArgs for &'a ::vector::VectorRenderTargetOutputMutPtr {
    fn exec(self) -> ::vector::VectorRenderTargetOutputMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorRenderTargetOutputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_constructor_v(v as *const ::vector::VectorRenderTargetOutputMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorRenderTargetOutputMutPtr::remove](../struct.VectorRenderTargetOutputMutPtr.html#method.remove) method.
  pub trait VectorRenderTargetOutputMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> ();
  }
  impl<'largs> VectorRenderTargetOutputMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_remove_i(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, i) }
    }
  }
  impl<'largs> VectorRenderTargetOutputMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorRenderTargetOutputMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QRenderTargetOutput_ptr_remove_i_n(original_self as *mut ::vector::VectorRenderTargetOutputMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::append](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.append) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataAppendArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
    let l = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_append_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, l as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataAppendArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_append_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::count](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.count) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {

      unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_count_no_args(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataCountArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_count_t(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::fill](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.fill) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataFillArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_fill_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataFillArgs<'largs> for (&'largs ::shared_pointer::SharedPointerTextureImageData,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_fill_t_size(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::index_of](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.index_of) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataIndexOfArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_indexOf_t(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataIndexOfArgs<'largs> for (&'largs ::shared_pointer::SharedPointerTextureImageData,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_indexOf_t_from(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::insert](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.insert) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::shared_pointer::SharedPointerTextureImageData) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_insert_i_n_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i, n, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataInsertArgs<'largs> for (::libc::c_int,&'largs ::shared_pointer::SharedPointerTextureImageData) {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_insert_i_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::last_index_of](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.last_index_of) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataLastIndexOfArgs<'largs>
     {
    fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataLastIndexOfArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_lastIndexOf_t(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) }
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataLastIndexOfArgs<'largs> for (&'largs ::shared_pointer::SharedPointerTextureImageData,::libc::c_int) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_lastIndexOf_t_from(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData, from) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::mid](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.mid) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> ::vector::VectorSharedPointerSharedPointerTextureImageData;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
      let pos = self;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_mid_to_output_pos(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_mid_to_output_pos_len(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::new](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.new) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataNewArgs {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerTextureImageData;
  }
  impl VectorSharedPointerSharedPointerTextureImageDataNewArgs for () {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerTextureImageData {

      {
        let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorSharedPointerSharedPointerTextureImageDataNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
      let size = self;
      {
        let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorSharedPointerSharedPointerTextureImageDataNewArgs for (::libc::c_int,&'a ::shared_pointer::SharedPointerTextureImageData) {

  fn exec(self, ) -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let size = self.0;
let t = self.1;
    {
let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constructor_size_t(size, t as *const ::shared_pointer::SharedPointerTextureImageData, &mut object); }object
}
  }
}
  impl<'a> VectorSharedPointerSharedPointerTextureImageDataNewArgs for &'a ::vector::VectorSharedPointerSharedPointerTextureImageData {

  fn exec(self, ) -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let v = self;
    {
let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_constructor_v(v as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::op_add_assign](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.op_add_assign) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataOpAddAssignArgs<'largs>
     {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataOpAddAssignArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_add_assign_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, l as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataOpAddAssignArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_add_assign_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::op_shl](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.op_shl) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataOpShlArgs<'largs> for &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_shl_l(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, l as *const ::vector::VectorSharedPointerSharedPointerTextureImageData) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataOpShlArgs<'largs> for &'largs ::shared_pointer::SharedPointerTextureImageData {

  fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_operator_shl_t(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, t as *const ::shared_pointer::SharedPointerTextureImageData) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::remove](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.remove) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ();
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_remove_i(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i) }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorSharedPointerSharedPointerTextureImageData) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_remove_i_n(original_self as *mut ::vector::VectorSharedPointerSharedPointerTextureImageData, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSharedPointerSharedPointerTextureImageData::value](../struct.VectorSharedPointerSharedPointerTextureImageData.html#method.value) method.
  pub trait VectorSharedPointerSharedPointerTextureImageDataValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> ::shared_pointer::SharedPointerTextureImageData;
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData)
            -> ::shared_pointer::SharedPointerTextureImageData {
      let i = self;
      {
        let mut object: ::shared_pointer::SharedPointerTextureImageData =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_value_to_output_i(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorSharedPointerSharedPointerTextureImageDataValueArgs<'largs> for (::libc::c_int,&'largs ::shared_pointer::SharedPointerTextureImageData) {

  fn exec(self, original_self: &'largs ::vector::VectorSharedPointerSharedPointerTextureImageData) -> ::shared_pointer::SharedPointerTextureImageData {
    let i = self.0;
let default_value = self.1;
    {
let mut object: ::shared_pointer::SharedPointerTextureImageData = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_3d_render_c_QVector_QSharedPointer_Qt3DRender_QTextureImageData_value_to_output_i_defaultValue(original_self as *const ::vector::VectorSharedPointerSharedPointerTextureImageData, i, default_value as *const ::shared_pointer::SharedPointerTextureImageData, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::append](../struct.VectorSortPolicySortType.html#method.append) method.
  pub trait VectorSortPolicySortTypeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> ();
  }
  impl<'largs> VectorSortPolicySortTypeAppendArgs<'largs> for &'largs ::vector::VectorSortPolicySortType {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let l = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_append_l(original_self as *mut ::vector::VectorSortPolicySortType, l as *const ::vector::VectorSortPolicySortType) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeAppendArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let t = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_append_t(original_self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::count](../struct.VectorSortPolicySortType.html#method.count) method.
  pub trait VectorSortPolicySortTypeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int;
  }
  impl<'largs> VectorSortPolicySortTypeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {

      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_count_no_args(original_self as *const ::vector::VectorSortPolicySortType) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeCountArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_count_t(original_self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::fill](../struct.VectorSortPolicySortType.html#method.fill) method.
  pub trait VectorSortPolicySortTypeFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType;
  }
  impl<'largs> VectorSortPolicySortTypeFillArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_fill_t(original_self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorSortPolicySortTypeFillArgs<'largs> for (&'largs ::sort_policy::SortType, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_fill_t_size(original_self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::index_of](../struct.VectorSortPolicySortType.html#method.index_of) method.
  pub trait VectorSortPolicySortTypeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int;
  }
  impl<'largs> VectorSortPolicySortTypeIndexOfArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_indexOf_t(original_self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeIndexOfArgs<'largs> for (&'largs ::sort_policy::SortType, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_indexOf_t_from(original_self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::insert](../struct.VectorSortPolicySortType.html#method.insert) method.
  pub trait VectorSortPolicySortTypeInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> ();
  }
  impl<'largs> VectorSortPolicySortTypeInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::sort_policy::SortType) {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_insert_i_n_t(original_self as *mut ::vector::VectorSortPolicySortType, i, n, t as *const ::sort_policy::SortType) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeInsertArgs<'largs> for (::libc::c_int, &'largs ::sort_policy::SortType) {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_insert_i_t(original_self as *mut ::vector::VectorSortPolicySortType, i, t as *const ::sort_policy::SortType) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::last_index_of](../struct.VectorSortPolicySortType.html#method.last_index_of) method.
  pub trait VectorSortPolicySortTypeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int;
  }
  impl<'largs> VectorSortPolicySortTypeLastIndexOfArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_lastIndexOf_t(original_self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeLastIndexOfArgs<'largs> for (&'largs ::sort_policy::SortType, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_lastIndexOf_t_from(original_self as *const ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::mid](../struct.VectorSortPolicySortType.html#method.mid) method.
  pub trait VectorSortPolicySortTypeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::vector::VectorSortPolicySortType;
  }
  impl<'largs> VectorSortPolicySortTypeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::vector::VectorSortPolicySortType {
      let pos = self;
      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_mid_to_output_pos(original_self as *const ::vector::VectorSortPolicySortType, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorSortPolicySortTypeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::vector::VectorSortPolicySortType {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_mid_to_output_pos_len(original_self as *const ::vector::VectorSortPolicySortType, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::new](../struct.VectorSortPolicySortType.html#method.new) method.
  pub trait VectorSortPolicySortTypeNewArgs {
    fn exec(self) -> ::vector::VectorSortPolicySortType;
  }
  impl VectorSortPolicySortTypeNewArgs for () {
    fn exec(self) -> ::vector::VectorSortPolicySortType {

      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorSortPolicySortTypeNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorSortPolicySortType {
      let size = self;
      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorSortPolicySortTypeNewArgs for (::libc::c_int, &'a ::sort_policy::SortType) {
    fn exec(self) -> ::vector::VectorSortPolicySortType {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constructor_size_t(size, t as *const ::sort_policy::SortType, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorSortPolicySortTypeNewArgs for &'a ::vector::VectorSortPolicySortType {
    fn exec(self) -> ::vector::VectorSortPolicySortType {
      let v = self;
      {
        let mut object: ::vector::VectorSortPolicySortType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_constructor_v(v as *const ::vector::VectorSortPolicySortType, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::op_add_assign](../struct.VectorSortPolicySortType.html#method.op_add_assign) method.
  pub trait VectorSortPolicySortTypeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType;
  }
  impl<'largs> VectorSortPolicySortTypeOpAddAssignArgs<'largs> for &'largs ::vector::VectorSortPolicySortType {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_add_assign_l(original_self as *mut ::vector::VectorSortPolicySortType, l as *const ::vector::VectorSortPolicySortType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorSortPolicySortTypeOpAddAssignArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_add_assign_t(original_self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::op_shl](../struct.VectorSortPolicySortType.html#method.op_shl) method.
  pub trait VectorSortPolicySortTypeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType;
  }
  impl<'largs> VectorSortPolicySortTypeOpShlArgs<'largs> for &'largs ::vector::VectorSortPolicySortType {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_shl_l(original_self as *mut ::vector::VectorSortPolicySortType, l as *const ::vector::VectorSortPolicySortType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorSortPolicySortTypeOpShlArgs<'largs> for &'largs ::sort_policy::SortType {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorSortPolicySortType)
            -> &'largs mut ::vector::VectorSortPolicySortType {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_operator_shl_t(original_self as *mut ::vector::VectorSortPolicySortType, t as *const ::sort_policy::SortType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::remove](../struct.VectorSortPolicySortType.html#method.remove) method.
  pub trait VectorSortPolicySortTypeRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> ();
  }
  impl<'largs> VectorSortPolicySortTypeRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_remove_i(original_self as *mut ::vector::VectorSortPolicySortType, i) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorSortPolicySortType) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_remove_i_n(original_self as *mut ::vector::VectorSortPolicySortType, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorSortPolicySortType::value](../struct.VectorSortPolicySortType.html#method.value) method.
  pub trait VectorSortPolicySortTypeValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::sort_policy::SortType;
  }
  impl<'largs> VectorSortPolicySortTypeValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::sort_policy::SortType {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_value_i(original_self as *const ::vector::VectorSortPolicySortType, i) }
    }
  }
  impl<'largs> VectorSortPolicySortTypeValueArgs<'largs> for (::libc::c_int, &'largs ::sort_policy::SortType) {
    fn exec(self, original_self: &'largs ::vector::VectorSortPolicySortType) -> ::sort_policy::SortType {
      let i = self.0;
      let default_value = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QSortPolicy_SortType_value_i_defaultValue(original_self as *const ::vector::VectorSortPolicySortType, i, default_value as *const ::sort_policy::SortType) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::fill](../struct.VectorTechniqueMutPtr.html#method.fill) method.
  pub trait VectorTechniqueMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorTechniqueMutPtr)
                   -> &'largs mut ::vector::VectorTechniqueMutPtr;
  }
  impl<'largs> VectorTechniqueMutPtrFillArgs<'largs> for &'largs *mut ::technique::Technique {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorTechniqueMutPtr)
                   -> &'largs mut ::vector::VectorTechniqueMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_fill_t(original_self as *mut ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTechniqueMutPtrFillArgs<'largs> for (&'largs *mut ::technique::Technique, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorTechniqueMutPtr)
                   -> &'largs mut ::vector::VectorTechniqueMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_fill_t_size(original_self as *mut ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::index_of](../struct.VectorTechniqueMutPtr.html#method.index_of) method.
  pub trait VectorTechniqueMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorTechniqueMutPtrIndexOfArgs<'largs> for &'largs *mut ::technique::Technique {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_indexOf_t(original_self as *const ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique)
    }
  }
  impl<'largs> VectorTechniqueMutPtrIndexOfArgs<'largs> for (&'largs *mut ::technique::Technique, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_indexOf_t_from(original_self as *const ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::insert](../struct.VectorTechniqueMutPtr.html#method.insert) method.
  pub trait VectorTechniqueMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> ();
  }
  impl<'largs> VectorTechniqueMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::technique::Technique) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_insert_i_n_t(original_self as *mut ::vector::VectorTechniqueMutPtr, i, n, t as *const *mut ::technique::Technique)
    }
  }
  impl<'largs> VectorTechniqueMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::technique::Technique) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_insert_i_t(original_self as *mut ::vector::VectorTechniqueMutPtr, i, t as *const *mut ::technique::Technique)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::last_index_of](../struct.VectorTechniqueMutPtr.html#method.last_index_of) method.
  pub trait VectorTechniqueMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorTechniqueMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::technique::Technique {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_lastIndexOf_t(original_self as *const ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique)
    }
  }
  impl<'largs> VectorTechniqueMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::technique::Technique, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorTechniqueMutPtr, t as *const *mut ::technique::Technique, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::mid](../struct.VectorTechniqueMutPtr.html#method.mid) method.
  pub trait VectorTechniqueMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::vector::VectorTechniqueMutPtr;
  }
  impl<'largs> VectorTechniqueMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::vector::VectorTechniqueMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorTechniqueMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_mid_to_output_pos(original_self as *const ::vector::VectorTechniqueMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTechniqueMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTechniqueMutPtr) -> ::vector::VectorTechniqueMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorTechniqueMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorTechniqueMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::new](../struct.VectorTechniqueMutPtr.html#method.new) method.
  pub trait VectorTechniqueMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorTechniqueMutPtr;
  }
  impl VectorTechniqueMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorTechniqueMutPtr {

      {
        let mut object: ::vector::VectorTechniqueMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorTechniqueMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorTechniqueMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorTechniqueMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTechniqueMutPtrNewArgs for &'a ::vector::VectorTechniqueMutPtr {
    fn exec(self) -> ::vector::VectorTechniqueMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorTechniqueMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_constructor_v(v as *const ::vector::VectorTechniqueMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTechniqueMutPtr::remove](../struct.VectorTechniqueMutPtr.html#method.remove) method.
  pub trait VectorTechniqueMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> ();
  }
  impl<'largs> VectorTechniqueMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_remove_i(original_self as *mut ::vector::VectorTechniqueMutPtr, i) }
    }
  }
  impl<'largs> VectorTechniqueMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTechniqueMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_render_c_QVector_Qt3DRender_QTechnique_ptr_remove_i_n(original_self as *mut ::vector::VectorTechniqueMutPtr, i, n) }
    }
  }
}
