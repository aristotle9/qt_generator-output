/// C++ type: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>```</span>
#[repr(C)]
pub struct VectorAbstractTextDocumentLayoutSelection([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_ABSTRACT_TEXT_DOCUMENT_LAYOUT_SELECTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAbstractTextDocumentLayoutSelection {
  unsafe fn new_uninitialized() -> VectorAbstractTextDocumentLayoutSelection {
    VectorAbstractTextDocumentLayoutSelection(::std::mem::uninitialized())
  }
}

impl VectorAbstractTextDocumentLayoutSelection {
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::abstract_text_document_layout::Selection) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::append(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorAbstractTextDocumentLayoutSelection) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::append(const QVector<QAbstractTextDocumentLayout::Selection>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_at(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_back_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_back(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QAbstractTextDocumentLayout::Selection>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_capacity(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_clear(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection* QVector<QAbstractTextDocumentLayout::Selection>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::abstract_text_document_layout::Selection {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constData(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constFirst(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constLast(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QAbstractTextDocumentLayout::Selection>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_count(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection* QVector<QAbstractTextDocumentLayout::Selection>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::abstract_text_document_layout::Selection {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_data_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection* QVector<QAbstractTextDocumentLayout::Selection>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::abstract_text_document_layout::Selection {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_data(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QAbstractTextDocumentLayout::Selection>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_empty(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::abstract_text_document_layout::Selection) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::fill(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::abstract_text_document_layout::Selection, ::libc::c_int)) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::fill(const QAbstractTextDocumentLayout::Selection& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self,
                            args: Args)
                            -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_first_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_first(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_front_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_front(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::abstract_text_document_layout::Selection)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::insert(int i, const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::abstract_text_document_layout::Selection)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::insert(int i, int n, const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QAbstractTextDocumentLayout::Selection>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_isEmpty(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_last_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_last(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QAbstractTextDocumentLayout::Selection>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_length(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection> QVector<QAbstractTextDocumentLayout::Selection>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection> QVector<QAbstractTextDocumentLayout::Selection>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAbstractTextDocumentLayoutSelection
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_move(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QAbstractTextDocumentLayout::Selection>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAbstractTextDocumentLayoutSelection) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QAbstractTextDocumentLayout::Selection>::QVector(const QVector<QAbstractTextDocumentLayout::Selection>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QAbstractTextDocumentLayout::Selection>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::abstract_text_document_layout::Selection)) -> ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QAbstractTextDocumentLayout::Selection>::QVector(int size, const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAbstractTextDocumentLayoutSelection
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection> QVector<QAbstractTextDocumentLayout::Selection>::operator+(const QVector<QAbstractTextDocumentLayout::Selection>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::vector::VectorAbstractTextDocumentLayoutSelection)
                -> ::vector::VectorAbstractTextDocumentLayoutSelection {
    {
      let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_to_output(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, l as *const ::vector::VectorAbstractTextDocumentLayoutSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::abstract_text_document_layout::Selection) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::operator+=(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::operator+=(const QVector<QAbstractTextDocumentLayout::Selection>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::operator=(const QVector<QAbstractTextDocumentLayout::Selection>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAbstractTextDocumentLayoutSelection)
                             -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_assign(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, v as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_index_const(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection& QVector<QAbstractTextDocumentLayout::Selection>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::abstract_text_document_layout::Selection {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_index(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::abstract_text_document_layout::Selection) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::operator<<(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'l0 mut ::vector::VectorAbstractTextDocumentLayoutSelection```<br>
  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>& QVector<QAbstractTextDocumentLayout::Selection>::operator<<(const QVector<QAbstractTextDocumentLayout::Selection>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self,
                              args: Args)
                              -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_pop_back(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_pop_front(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::prepend(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::abstract_text_document_layout::Selection) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_prepend(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::push_back(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::abstract_text_document_layout::Selection) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_push_back(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::push_front(const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::abstract_text_document_layout::Selection) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_push_front(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeAt(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeFirst(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_removeLast(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::replace(int i, const QAbstractTextDocumentLayout::Selection& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::abstract_text_document_layout::Selection) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_replace(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i, t as *const ::abstract_text_document_layout::Selection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_reserve(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_resize(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QAbstractTextDocumentLayout::Selection>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_size(self as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_squeeze(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QAbstractTextDocumentLayout::Selection>::swap(QVector<QAbstractTextDocumentLayout::Selection>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAbstractTextDocumentLayoutSelection) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_swap(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, other as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection QVector<QAbstractTextDocumentLayout::Selection>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeAt_as_ptr(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection QVector<QAbstractTextDocumentLayout::Selection>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeFirst_as_ptr(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection QVector<QAbstractTextDocumentLayout::Selection>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_takeLast_as_ptr(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QAbstractTextDocumentLayout::Selection>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection>```<br>
  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection QVector<QAbstractTextDocumentLayout::Selection>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::abstract_text_document_layout::Selection)) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection>```<br>
  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout::Selection QVector<QAbstractTextDocumentLayout::Selection>::value(int i, const QAbstractTextDocumentLayout::Selection& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self,
                             args: Args)
                             -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection>
    where Args: overloading::VectorAbstractTextDocumentLayoutSelectionValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorAbstractTextDocumentLayoutSelection {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QAbstractTextDocumentLayout::Selection>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_destructor(self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<double>```</span>
#[repr(C)]
pub struct VectorCDouble([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_C_DOUBLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorCDouble {
  unsafe fn new_uninitialized() -> VectorCDouble {
    VectorCDouble(::std::mem::uninitialized())
  }
}

impl VectorCDouble {
  /// C++ method: <span style='color: green;'>```QVector<double>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::append(const QVector<double>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::append(const double& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCDoubleAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const double& QVector<double>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_at(self as *const ::vector::VectorCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_back_const(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QVector<double>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_back(self as *mut ::vector::VectorCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<double>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_double_capacity(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_clear(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const double* QVector<double>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_constData(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_constFirst(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_constLast(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::contains(const double& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_contains(self as *const ::vector::VectorCDouble,
                                              t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::count(const double& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCDoubleCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const double* QVector<double>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_data_const(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```double* QVector<double>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_data(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_double_empty(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::endsWith(const double& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_endsWith(self as *const ::vector::VectorCDouble,
                                              t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::libc::c_double) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::fill(const double& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::libc::c_double, ::libc::c_int)) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::fill(const double& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCDouble
    where Args: overloading::VectorCDoubleFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const double& QVector<double>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_first_const(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QVector<double>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_first(self as *mut ::vector::VectorCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QVector<double> QVector<double>::fromList(const QList<double>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::list::ListCDouble) -> ::vector::VectorCDouble {
    {
      let mut object: ::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_double_fromList_to_output(list as *const ::list::ListCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_front_const(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QVector<double>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_front(self as *mut ::vector::VectorCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::indexOf(const double& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_double, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::indexOf(const double& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCDoubleIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<double>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::insert(int i, const double& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::insert(int i, int n, const double& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCDoubleInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<double>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_double_isEmpty(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_last_const(self as *const ::vector::VectorCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::lastIndexOf(const double& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_double, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<double>::lastIndexOf(const double& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCDoubleLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double& QVector<double>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_last(self as *mut ::vector::VectorCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<double>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_double_length(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double> QVector<double>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double> QVector<double>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorCDouble
    where Args: overloading::VectorCDoubleMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<double>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_double_move(self as *mut ::vector::VectorCDouble, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<double>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCDouble) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<double>::QVector(const QVector<double>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<double>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::libc::c_double)) -> ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<double>::QVector(int size, const double& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorCDouble
    where Args: overloading::VectorCDoubleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<double> QVector<double>::operator+(const QVector<double>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorCDouble) -> ::vector::VectorCDouble {
    {
      let mut object: ::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_double_operator_add_to_output(self as *const ::vector::VectorCDouble,
                                                              l as *const ::vector::VectorCDouble,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorCDouble) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::operator+=(const QVector<double>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_double) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::operator+=(const double& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCDouble
    where Args: overloading::VectorCDoubleOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::operator=(const QVector<double>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorCDouble) -> &'l0 mut ::vector::VectorCDouble {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_double_operator_assign(self as *mut ::vector::VectorCDouble,
                                                     v as *const ::vector::VectorCDouble)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::operator==(const QVector<double>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorCDouble) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_operator_eq(self as *const ::vector::VectorCDouble,
                                                 v as *const ::vector::VectorCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```const double& QVector<double>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_double {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_double_operator_index_const(self as *const ::vector::VectorCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QVector<double>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_double_operator_index(self as *mut ::vector::VectorCDouble, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::operator!=(const QVector<double>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorCDouble) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_operator_neq(self as *const ::vector::VectorCDouble,
                                                  v as *const ::vector::VectorCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorCDouble) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::operator<<(const QVector<double>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_double) -> &'l0 mut ::vector::VectorCDouble```<br>
  /// C++ method: <span style='color: green;'>```QVector<double>& QVector<double>::operator<<(const double& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCDouble
    where Args: overloading::VectorCDoubleOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<double>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_pop_back(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_pop_front(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::prepend(const double& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_prepend(self as *mut ::vector::VectorCDouble,
                                             t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::push_back(const double& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_push_back(self as *mut ::vector::VectorCDouble,
                                               t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::push_front(const double& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_push_front(self as *mut ::vector::VectorCDouble,
                                                t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<double>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCDoubleRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<double>::removeAll(const double& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_double) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_removeAll(self as *mut ::vector::VectorCDouble,
                                               t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_double_removeAt(self as *mut ::vector::VectorCDouble, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_removeFirst(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_removeLast(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::removeOne(const double& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_removeOne(self as *mut ::vector::VectorCDouble,
                                               t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::replace(int i, const double& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_replace(self as *mut ::vector::VectorCDouble,
                                             i,
                                             t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_double_reserve(self as *mut ::vector::VectorCDouble, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_double_resize(self as *mut ::vector::VectorCDouble, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<double>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_double_size(self as *const ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_squeeze(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<double>::startsWith(const double& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_startsWith(self as *const ::vector::VectorCDouble,
                                                t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<double>::swap(QVector<double>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorCDouble) {
    unsafe {
      ::ffi::qt_gui_c_QVector_double_swap(self as *mut ::vector::VectorCDouble,
                                          other as *mut ::vector::VectorCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```double QVector<double>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_takeAt(self as *mut ::vector::VectorCDouble, i) }
  }

  /// C++ method: <span style='color: green;'>```double QVector<double>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_takeFirst(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```double QVector<double>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QVector_double_takeLast(self as *mut ::vector::VectorCDouble) }
  }

  /// C++ method: <span style='color: green;'>```QList<double> QVector<double>::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::list::ListCDouble {
    {
      let mut object: ::list::ListCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_double_toList_to_output(self as *const ::vector::VectorCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<double>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QVector<double>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_double)) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QVector<double>::value(int i, const double& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::VectorCDoubleValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorCDouble {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<double>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_double_destructor(self as *mut ::vector::VectorCDouble) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<float>```</span>
#[repr(C)]
pub struct VectorCFloat([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_C_FLOAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorCFloat {
  unsafe fn new_uninitialized() -> VectorCFloat {
    VectorCFloat(::std::mem::uninitialized())
  }
}

impl VectorCFloat {
  /// C++ method: <span style='color: green;'>```QVector<float>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorCFloat) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::append(const QVector<float>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_float) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::append(const float& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCFloatAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const float& QVector<float>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_at(self as *const ::vector::VectorCFloat, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_back_const(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```float& QVector<float>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_back(self as *mut ::vector::VectorCFloat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<float>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_float_capacity(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_clear(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```const float* QVector<float>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_constData(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_constFirst(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_constLast(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::contains(const float& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_float) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_contains(self as *const ::vector::VectorCFloat,
                                             t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_float) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::count(const float& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCFloatCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const float* QVector<float>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_data_const(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```float* QVector<float>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_data(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_float_empty(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::endsWith(const float& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_float) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_endsWith(self as *const ::vector::VectorCFloat,
                                             t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::libc::c_float) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::fill(const float& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::libc::c_float, ::libc::c_int)) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::fill(const float& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCFloat
    where Args: overloading::VectorCFloatFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const float& QVector<float>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_first_const(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```float& QVector<float>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_first(self as *mut ::vector::VectorCFloat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_front_const(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```float& QVector<float>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_front(self as *mut ::vector::VectorCFloat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_float) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::indexOf(const float& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_float, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::indexOf(const float& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCFloatIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<float>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::insert(int i, const float& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::insert(int i, int n, const float& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCFloatInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<float>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_float_isEmpty(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_last_const(self as *const ::vector::VectorCFloat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_float) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::lastIndexOf(const float& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_float, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<float>::lastIndexOf(const float& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCFloatLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```float& QVector<float>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_last(self as *mut ::vector::VectorCFloat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<float>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_float_length(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float> QVector<float>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float> QVector<float>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorCFloat
    where Args: overloading::VectorCFloatMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<float>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_float_move(self as *mut ::vector::VectorCFloat, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<float>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCFloat) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<float>::QVector(const QVector<float>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<float>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::libc::c_float)) -> ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<float>::QVector(int size, const float& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorCFloat
    where Args: overloading::VectorCFloatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<float> QVector<float>::operator+(const QVector<float>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorCFloat) -> ::vector::VectorCFloat {
    {
      let mut object: ::vector::VectorCFloat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_float_operator_add_to_output(self as *const ::vector::VectorCFloat,
                                                             l as *const ::vector::VectorCFloat,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorCFloat) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::operator+=(const QVector<float>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_float) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::operator+=(const float& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCFloat
    where Args: overloading::VectorCFloatOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::operator=(const QVector<float>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorCFloat) -> &'l0 mut ::vector::VectorCFloat {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_float_operator_assign(self as *mut ::vector::VectorCFloat,
                                                    v as *const ::vector::VectorCFloat)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::operator==(const QVector<float>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorCFloat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_operator_eq(self as *const ::vector::VectorCFloat,
                                                v as *const ::vector::VectorCFloat)
    }
  }

  /// C++ method: <span style='color: green;'>```const float& QVector<float>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_float {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_float_operator_index_const(self as *const ::vector::VectorCFloat, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```float& QVector<float>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_float_operator_index(self as *mut ::vector::VectorCFloat, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::operator!=(const QVector<float>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorCFloat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_operator_neq(self as *const ::vector::VectorCFloat,
                                                 v as *const ::vector::VectorCFloat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorCFloat) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::operator<<(const QVector<float>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_float) -> &'l0 mut ::vector::VectorCFloat```<br>
  /// C++ method: <span style='color: green;'>```QVector<float>& QVector<float>::operator<<(const float& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCFloat
    where Args: overloading::VectorCFloatOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<float>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_pop_back(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_pop_front(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::prepend(const float& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_prepend(self as *mut ::vector::VectorCFloat,
                                            t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::push_back(const float& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_push_back(self as *mut ::vector::VectorCFloat,
                                              t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::push_front(const float& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_push_front(self as *mut ::vector::VectorCFloat,
                                               t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<float>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCFloatRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<float>::removeAll(const float& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_float) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_removeAll(self as *mut ::vector::VectorCFloat,
                                              t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_float_removeAt(self as *mut ::vector::VectorCFloat, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_removeFirst(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_removeLast(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::removeOne(const float& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_float) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_removeOne(self as *mut ::vector::VectorCFloat,
                                              t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::replace(int i, const float& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_replace(self as *mut ::vector::VectorCFloat,
                                            i,
                                            t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_float_reserve(self as *mut ::vector::VectorCFloat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_float_resize(self as *mut ::vector::VectorCFloat, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<float>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_float_size(self as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_squeeze(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<float>::startsWith(const float& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_float) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_startsWith(self as *const ::vector::VectorCFloat,
                                               t as *const ::libc::c_float)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<float>::swap(QVector<float>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorCFloat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_float_swap(self as *mut ::vector::VectorCFloat,
                                         other as *mut ::vector::VectorCFloat)
    }
  }

  /// C++ method: <span style='color: green;'>```float QVector<float>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_takeAt(self as *mut ::vector::VectorCFloat, i) }
  }

  /// C++ method: <span style='color: green;'>```float QVector<float>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_takeFirst(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```float QVector<float>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector_float_takeLast(self as *mut ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```QVector<float>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_float```<br>
  /// C++ method: <span style='color: green;'>```float QVector<float>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_float)) -> ::libc::c_float```<br>
  /// C++ method: <span style='color: green;'>```float QVector<float>::value(int i, const float& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_float
    where Args: overloading::VectorCFloatValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorCFloat {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<float>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_float_destructor(self as *mut ::vector::VectorCFloat) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QPair<double, QColor>>```</span>
#[repr(C)]
pub struct VectorPairPairCDoubleColor([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_PAIR_PAIR_C_DOUBLE_COLOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorPairPairCDoubleColor {
  unsafe fn new_uninitialized() -> VectorPairPairCDoubleColor {
    VectorPairPairCDoubleColor(::std::mem::uninitialized())
  }
}

impl VectorPairPairCDoubleColor {
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::pair::PairCDoubleColor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::append(const QPair<double, QColor>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorPairPairCDoubleColor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::append(const QVector<QPair<double, QColor>>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleColorAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_at(self as *const ::vector::VectorPairPairCDoubleColor, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_back_const(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor>& QVector<QPair<double, QColor>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_back(self as *mut ::vector::VectorPairPairCDoubleColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_capacity(self as *const ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_clear(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>* QVector<QPair<double, QColor>>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::pair::PairCDoubleColor {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_constData(self as *const ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_constFirst(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_constLast(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::contains(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::pair::PairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_contains(self as *const ::vector::VectorPairPairCDoubleColor,
                                                           t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::pair::PairCDoubleColor) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::count(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleColorCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>* QVector<QPair<double, QColor>>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::pair::PairCDoubleColor {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_data_const(self as *const ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor>* QVector<QPair<double, QColor>>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::pair::PairCDoubleColor {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_data(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_empty(self as *const ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::endsWith(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::pair::PairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_endsWith(self as *const ::vector::VectorPairPairCDoubleColor,
                                                           t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::pair::PairCDoubleColor) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::fill(const QPair<double, QColor>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::pair::PairCDoubleColor, ::libc::c_int)) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::fill(const QPair<double, QColor>& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_first_const(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor>& QVector<QPair<double, QColor>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_first(self as *mut ::vector::VectorPairPairCDoubleColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_front_const(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor>& QVector<QPair<double, QColor>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_front(self as *mut ::vector::VectorPairPairCDoubleColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::pair::PairCDoubleColor) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::indexOf(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::pair::PairCDoubleColor, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::indexOf(const QPair<double, QColor>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleColorIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::pair::PairCDoubleColor)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::insert(int i, const QPair<double, QColor>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::pair::PairCDoubleColor)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::insert(int i, int n, const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleColorInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_isEmpty(self as *const ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_last_const(self as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::pair::PairCDoubleColor) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::lastIndexOf(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::pair::PairCDoubleColor, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::lastIndexOf(const QPair<double, QColor>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorPairPairCDoubleColorLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, QColor>& QVector<QPair<double, QColor>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_last(self as *mut ::vector::VectorPairPairCDoubleColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_length(self as *const ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>> QVector<QPair<double, QColor>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>> QVector<QPair<double, QColor>>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorPairPairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_move(self as *mut ::vector::VectorPairPairCDoubleColor, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QColor>>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorPairPairCDoubleColor) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QColor>>::QVector(const QVector<QPair<double, QColor>>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QColor>>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::pair::PairCDoubleColor)) -> ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPair<double, QColor>>::QVector(int size, const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorPairPairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>> QVector<QPair<double, QColor>>::operator+(const QVector<QPair<double, QColor>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorPairPairCDoubleColor) -> ::vector::VectorPairPairCDoubleColor {
    {
      let mut object: ::vector::VectorPairPairCDoubleColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_add_to_output(self as *const ::vector::VectorPairPairCDoubleColor, l as *const ::vector::VectorPairPairCDoubleColor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::pair::PairCDoubleColor) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::operator+=(const QPair<double, QColor>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorPairPairCDoubleColor) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::operator+=(const QVector<QPair<double, QColor>>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::operator=(const QVector<QPair<double, QColor>>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorPairPairCDoubleColor)
                             -> &'l0 mut ::vector::VectorPairPairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_assign(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                                    v as *const ::vector::VectorPairPairCDoubleColor)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::operator==(const QVector<QPair<double, QColor>>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorPairPairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_eq(self as *const ::vector::VectorPairPairCDoubleColor,
                                                              v as *const ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QColor>& QVector<QPair<double, QColor>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleColor {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_index_const(self as *const ::vector::VectorPairPairCDoubleColor, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor>& QVector<QPair<double, QColor>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::pair::PairCDoubleColor {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_index(self as *mut ::vector::VectorPairPairCDoubleColor, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::operator!=(const QVector<QPair<double, QColor>>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorPairPairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_neq(self as *const ::vector::VectorPairPairCDoubleColor,
                                                               v as *const ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::pair::PairCDoubleColor) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::operator<<(const QPair<double, QColor>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorPairPairCDoubleColor) -> &'l0 mut ::vector::VectorPairPairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>& QVector<QPair<double, QColor>>::operator<<(const QVector<QPair<double, QColor>>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorPairPairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_pop_back(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_pop_front(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::prepend(const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::pair::PairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_prepend(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                          t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::push_back(const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::pair::PairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_push_back(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                            t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::push_front(const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::pair::PairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_push_front(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                             t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorPairPairCDoubleColorRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::removeAll(const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::pair::PairCDoubleColor) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_removeAll(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                            t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_removeAt(self as *mut ::vector::VectorPairPairCDoubleColor, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_removeFirst(self as *mut ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_removeLast(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::removeOne(const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::pair::PairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_removeOne(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                            t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::replace(int i, const QPair<double, QColor>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_replace(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                          i,
                                                          t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_reserve(self as *mut ::vector::VectorPairPairCDoubleColor, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_resize(self as *mut ::vector::VectorPairPairCDoubleColor, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPair<double, QColor>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_size(self as *const ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_squeeze(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPair<double, QColor>>::startsWith(const QPair<double, QColor>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::pair::PairCDoubleColor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_startsWith(self as *const ::vector::VectorPairPairCDoubleColor,
                                                             t as *const ::pair::PairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPair<double, QColor>>::swap(QVector<QPair<double, QColor>>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorPairPairCDoubleColor) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPair_double_QColor_swap(self as *mut ::vector::VectorPairPairCDoubleColor,
                                                       other as *mut ::vector::VectorPairPairCDoubleColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor> QVector<QPair<double, QColor>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::pair::PairCDoubleColor {
    {
      let mut object: ::pair::PairCDoubleColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_takeAt_to_output(self as *mut ::vector::VectorPairPairCDoubleColor, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor> QVector<QPair<double, QColor>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::pair::PairCDoubleColor {
    {
      let mut object: ::pair::PairCDoubleColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_takeFirst_to_output(self as *mut ::vector::VectorPairPairCDoubleColor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QColor> QVector<QPair<double, QColor>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::pair::PairCDoubleColor {
    {
      let mut object: ::pair::PairCDoubleColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_takeLast_to_output(self as *mut ::vector::VectorPairPairCDoubleColor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPair<double, QColor>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::pair::PairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QColor> QVector<QPair<double, QColor>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::pair::PairCDoubleColor)) -> ::pair::PairCDoubleColor```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QColor> QVector<QPair<double, QColor>>::value(int i, const QPair<double, QColor>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::pair::PairCDoubleColor
    where Args: overloading::VectorPairPairCDoubleColorValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorPairPairCDoubleColor {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QPair<double, QColor>>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_destructor(self as *mut ::vector::VectorPairPairCDoubleColor) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QLine>```</span>
#[repr(C)]
pub struct VectorQtCoreLine([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_LINE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCoreLine {
  unsafe fn new_uninitialized() -> VectorQtCoreLine {
    VectorQtCoreLine(::std::mem::uninitialized())
  }
}

impl VectorQtCoreLine {
  /// C++ method: <span style='color: green;'>```QVector<QLine>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::line::Line) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::append(const QLine& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCoreLine) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::append(const QVector<QLine>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_at(self as *const ::vector::VectorQtCoreLine, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_back_const(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLine& QVector<QLine>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_back(self as *mut ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLine>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_capacity(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_clear(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```const QLine* QVector<QLine>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::line::Line {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_constData(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_constFirst(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_constLast(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::contains(const QLine& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::line::Line) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_contains(self as *const ::vector::VectorQtCoreLine,
                                             t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::line::Line) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::count(const QLine& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLine* QVector<QLine>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::line::Line {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_data_const(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```QLine* QVector<QLine>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::line::Line {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_data(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_empty(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::endsWith(const QLine& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::line::Line) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_endsWith(self as *const ::vector::VectorQtCoreLine,
                                             t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::line::Line) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::fill(const QLine& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::line::Line, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::fill(const QLine& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLine
    where Args: overloading::VectorQtCoreLineFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_first_const(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLine& QVector<QLine>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_first(self as *mut ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_front_const(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLine& QVector<QLine>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_front(self as *mut ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::line::Line) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::indexOf(const QLine& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::line::Line, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::indexOf(const QLine& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QLine>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::line::Line)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::insert(int i, const QLine& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::line::Line)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::insert(int i, int n, const QLine& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_isEmpty(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_last_const(self as *const ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::line::Line) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::lastIndexOf(const QLine& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::line::Line, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::lastIndexOf(const QLine& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLine& QVector<QLine>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line::Line {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLine_last(self as *mut ::vector::VectorQtCoreLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLine>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_length(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine> QVector<QLine>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine> QVector<QLine>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCoreLine
    where Args: overloading::VectorQtCoreLineMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_move(self as *mut ::vector::VectorQtCoreLine, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLine>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCoreLine) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLine>::QVector(const QVector<QLine>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLine>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::line::Line)) -> ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLine>::QVector(int size, const QLine& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCoreLine
    where Args: overloading::VectorQtCoreLineNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QLine> QVector<QLine>::operator+(const QVector<QLine>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCoreLine) -> ::vector::VectorQtCoreLine {
    {
      let mut object: ::vector::VectorQtCoreLine =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_operator_add_to_output(self as *const ::vector::VectorQtCoreLine,
                                                             l as *const ::vector::VectorQtCoreLine,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::line::Line) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::operator+=(const QLine& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCoreLine) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::operator+=(const QVector<QLine>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLine
    where Args: overloading::VectorQtCoreLineOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::operator=(const QVector<QLine>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorQtCoreLine) -> &'l0 mut ::vector::VectorQtCoreLine {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QLine_operator_assign(self as *mut ::vector::VectorQtCoreLine,
                                                    v as *const ::vector::VectorQtCoreLine)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::operator==(const QVector<QLine>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCoreLine) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_operator_eq(self as *const ::vector::VectorQtCoreLine,
                                                v as *const ::vector::VectorQtCoreLine)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLine& QVector<QLine>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::line::Line {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QLine_operator_index_const(self as *const ::vector::VectorQtCoreLine, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLine& QVector<QLine>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::line::Line {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QLine_operator_index(self as *mut ::vector::VectorQtCoreLine, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::operator!=(const QVector<QLine>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCoreLine) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_operator_neq(self as *const ::vector::VectorQtCoreLine,
                                                 v as *const ::vector::VectorQtCoreLine)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::line::Line) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::operator<<(const QLine& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCoreLine) -> &'l0 mut ::vector::VectorQtCoreLine```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLine>& QVector<QLine>::operator<<(const QVector<QLine>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLine
    where Args: overloading::VectorQtCoreLineOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_pop_back(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_pop_front(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::prepend(const QLine& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::line::Line) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_prepend(self as *mut ::vector::VectorQtCoreLine,
                                            t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::push_back(const QLine& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::line::Line) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_push_back(self as *mut ::vector::VectorQtCoreLine,
                                              t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::push_front(const QLine& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::line::Line) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_push_front(self as *mut ::vector::VectorQtCoreLine,
                                               t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLine>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QLine>::removeAll(const QLine& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::line::Line) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_removeAll(self as *mut ::vector::VectorQtCoreLine,
                                              t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_removeAt(self as *mut ::vector::VectorQtCoreLine, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_removeFirst(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_removeLast(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::removeOne(const QLine& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::line::Line) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_removeOne(self as *mut ::vector::VectorQtCoreLine,
                                              t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::replace(int i, const QLine& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::line::Line) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_replace(self as *mut ::vector::VectorQtCoreLine,
                                            i,
                                            t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_reserve(self as *mut ::vector::VectorQtCoreLine, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_resize(self as *mut ::vector::VectorQtCoreLine, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLine>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_size(self as *const ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_squeeze(self as *mut ::vector::VectorQtCoreLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLine>::startsWith(const QLine& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::line::Line) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_startsWith(self as *const ::vector::VectorQtCoreLine,
                                               t as *const ::qt_core::line::Line)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLine>::swap(QVector<QLine>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCoreLine) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLine_swap(self as *mut ::vector::VectorQtCoreLine,
                                         other as *mut ::vector::VectorQtCoreLine)
    }
  }

  /// C++ method: <span style='color: green;'>```QLine QVector<QLine>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::line::Line {
    {
      let mut object: ::qt_core::line::Line =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_takeAt_to_output(self as *mut ::vector::VectorQtCoreLine, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLine QVector<QLine>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::line::Line {
    {
      let mut object: ::qt_core::line::Line =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_takeFirst_to_output(self as *mut ::vector::VectorQtCoreLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLine QVector<QLine>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::line::Line {
    {
      let mut object: ::qt_core::line::Line =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_takeLast_to_output(self as *mut ::vector::VectorQtCoreLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLine>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QVector<QLine>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::line::Line)) -> ::qt_core::line::Line```<br>
  /// C++ method: <span style='color: green;'>```QLine QVector<QLine>::value(int i, const QLine& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::line::Line
    where Args: overloading::VectorQtCoreLineValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCoreLine {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QLine>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLine_destructor(self as *mut ::vector::VectorQtCoreLine) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QLineF>```</span>
#[repr(C)]
pub struct VectorQtCoreLineF([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_LINE_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCoreLineF {
  unsafe fn new_uninitialized() -> VectorQtCoreLineF {
    VectorQtCoreLineF(::std::mem::uninitialized())
  }
}

impl VectorQtCoreLineF {
  /// C++ method: <span style='color: green;'>```QVector<QLineF>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::line_f::LineF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::append(const QLineF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCoreLineF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::append(const QVector<QLineF>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_at(self as *const ::vector::VectorQtCoreLineF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_back_const(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLineF& QVector<QLineF>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_back(self as *mut ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_capacity(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_clear(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```const QLineF* QVector<QLineF>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::line_f::LineF {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_constData(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_constFirst(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_constLast(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::contains(const QLineF& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::line_f::LineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_contains(self as *const ::vector::VectorQtCoreLineF,
                                              t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::line_f::LineF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::count(const QLineF& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLineF* QVector<QLineF>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::line_f::LineF {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_data_const(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```QLineF* QVector<QLineF>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::line_f::LineF {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_data(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_empty(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::endsWith(const QLineF& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::line_f::LineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_endsWith(self as *const ::vector::VectorQtCoreLineF,
                                              t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::line_f::LineF) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::fill(const QLineF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::line_f::LineF, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::fill(const QLineF& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLineF
    where Args: overloading::VectorQtCoreLineFFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_first_const(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLineF& QVector<QLineF>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_first(self as *mut ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_front_const(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLineF& QVector<QLineF>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_front(self as *mut ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::line_f::LineF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::indexOf(const QLineF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::line_f::LineF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::indexOf(const QLineF& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QLineF>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::line_f::LineF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::insert(int i, const QLineF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::line_f::LineF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::insert(int i, int n, const QLineF& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineFInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_isEmpty(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_last_const(self as *const ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::line_f::LineF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::lastIndexOf(const QLineF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::line_f::LineF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::lastIndexOf(const QLineF& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreLineFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLineF& QVector<QLineF>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::line_f::LineF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QLineF_last(self as *mut ::vector::VectorQtCoreLineF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_length(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF> QVector<QLineF>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF> QVector<QLineF>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCoreLineF
    where Args: overloading::VectorQtCoreLineFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_move(self as *mut ::vector::VectorQtCoreLineF, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLineF>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCoreLineF) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLineF>::QVector(const QVector<QLineF>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLineF>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::line_f::LineF)) -> ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QLineF>::QVector(int size, const QLineF& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCoreLineF
    where Args: overloading::VectorQtCoreLineFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QLineF> QVector<QLineF>::operator+(const QVector<QLineF>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCoreLineF) -> ::vector::VectorQtCoreLineF {
    {
      let mut object: ::vector::VectorQtCoreLineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_operator_add_to_output(self as *const ::vector::VectorQtCoreLineF,
                                                              l as *const ::vector::VectorQtCoreLineF,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::line_f::LineF) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::operator+=(const QLineF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCoreLineF) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::operator+=(const QVector<QLineF>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLineF
    where Args: overloading::VectorQtCoreLineFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::operator=(const QVector<QLineF>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorQtCoreLineF)
                             -> &'l0 mut ::vector::VectorQtCoreLineF {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_operator_assign(self as *mut ::vector::VectorQtCoreLineF,
                                                     v as *const ::vector::VectorQtCoreLineF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::operator==(const QVector<QLineF>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCoreLineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_operator_eq(self as *const ::vector::VectorQtCoreLineF,
                                                 v as *const ::vector::VectorQtCoreLineF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLineF& QVector<QLineF>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::line_f::LineF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QLineF_operator_index_const(self as *const ::vector::VectorQtCoreLineF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLineF& QVector<QLineF>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::line_f::LineF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QLineF_operator_index(self as *mut ::vector::VectorQtCoreLineF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::operator!=(const QVector<QLineF>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCoreLineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_operator_neq(self as *const ::vector::VectorQtCoreLineF,
                                                  v as *const ::vector::VectorQtCoreLineF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::line_f::LineF) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::operator<<(const QLineF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCoreLineF) -> &'l0 mut ::vector::VectorQtCoreLineF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QLineF>& QVector<QLineF>::operator<<(const QVector<QLineF>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreLineF
    where Args: overloading::VectorQtCoreLineFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_pop_back(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_pop_front(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::prepend(const QLineF& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::line_f::LineF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_prepend(self as *mut ::vector::VectorQtCoreLineF,
                                             t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::push_back(const QLineF& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::line_f::LineF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_push_back(self as *mut ::vector::VectorQtCoreLineF,
                                               t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::push_front(const QLineF& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::line_f::LineF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_push_front(self as *mut ::vector::VectorQtCoreLineF,
                                                t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreLineFRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::removeAll(const QLineF& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::line_f::LineF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_removeAll(self as *mut ::vector::VectorQtCoreLineF,
                                               t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_removeAt(self as *mut ::vector::VectorQtCoreLineF, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_removeFirst(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_removeLast(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::removeOne(const QLineF& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::line_f::LineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_removeOne(self as *mut ::vector::VectorQtCoreLineF,
                                               t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::replace(int i, const QLineF& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::line_f::LineF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_replace(self as *mut ::vector::VectorQtCoreLineF,
                                             i,
                                             t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_reserve(self as *mut ::vector::VectorQtCoreLineF, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_resize(self as *mut ::vector::VectorQtCoreLineF, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QLineF>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_size(self as *const ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_squeeze(self as *mut ::vector::VectorQtCoreLineF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QLineF>::startsWith(const QLineF& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::line_f::LineF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_startsWith(self as *const ::vector::VectorQtCoreLineF,
                                                t as *const ::qt_core::line_f::LineF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QLineF>::swap(QVector<QLineF>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCoreLineF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QLineF_swap(self as *mut ::vector::VectorQtCoreLineF,
                                          other as *mut ::vector::VectorQtCoreLineF)
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF QVector<QLineF>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::line_f::LineF {
    {
      let mut object: ::qt_core::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_takeAt_to_output(self as *mut ::vector::VectorQtCoreLineF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF QVector<QLineF>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::line_f::LineF {
    {
      let mut object: ::qt_core::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_takeFirst_to_output(self as *mut ::vector::VectorQtCoreLineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF QVector<QLineF>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::line_f::LineF {
    {
      let mut object: ::qt_core::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_takeLast_to_output(self as *mut ::vector::VectorQtCoreLineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QLineF>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QVector<QLineF>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::line_f::LineF)) -> ::qt_core::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QVector<QLineF>::value(int i, const QLineF& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::line_f::LineF
    where Args: overloading::VectorQtCoreLineFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCoreLineF {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QLineF>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QLineF_destructor(self as *mut ::vector::VectorQtCoreLineF) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QPoint>```</span>
#[repr(C)]
pub struct VectorQtCorePoint([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_POINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCorePoint {
  unsafe fn new_uninitialized() -> VectorQtCorePoint {
    VectorQtCorePoint(::std::mem::uninitialized())
  }
}

impl VectorQtCorePoint {
  /// C++ method: <span style='color: green;'>```QVector<QPoint>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::append(const QPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCorePoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::append(const QVector<QPoint>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCorePointAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_at(self as *const ::vector::VectorQtCorePoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_back_const(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QVector<QPoint>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_back(self as *mut ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_capacity(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_clear(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint* QVector<QPoint>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::point::Point {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_constData(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_constFirst(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_constLast(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::contains(const QPoint& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::point::Point) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_contains(self as *const ::vector::VectorQtCorePoint,
                                              t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::count(const QPoint& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCorePointCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPoint* QVector<QPoint>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::point::Point {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_data_const(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```QPoint* QVector<QPoint>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::point::Point {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_data(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_empty(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::endsWith(const QPoint& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::point::Point) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_endsWith(self as *const ::vector::VectorQtCorePoint,
                                              t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::point::Point) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::fill(const QPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::point::Point, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::fill(const QPoint& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCorePoint
    where Args: overloading::VectorQtCorePointFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_first_const(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QVector<QPoint>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_first(self as *mut ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_front_const(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QVector<QPoint>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_front(self as *mut ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::indexOf(const QPoint& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::point::Point, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::indexOf(const QPoint& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCorePointIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPoint>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::insert(int i, const QPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::insert(int i, int n, const QPoint& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCorePointInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_isEmpty(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_last_const(self as *const ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::lastIndexOf(const QPoint& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::point::Point, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::lastIndexOf(const QPoint& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCorePointLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPoint& QVector<QPoint>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPoint_last(self as *mut ::vector::VectorQtCorePoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_length(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint> QVector<QPoint>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint> QVector<QPoint>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCorePoint
    where Args: overloading::VectorQtCorePointMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_move(self as *mut ::vector::VectorQtCorePoint, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPoint>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCorePoint) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPoint>::QVector(const QVector<QPoint>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPoint>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::point::Point)) -> ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QPoint>::QVector(int size, const QPoint& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCorePoint
    where Args: overloading::VectorQtCorePointNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QPoint> QVector<QPoint>::operator+(const QVector<QPoint>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCorePoint) -> ::vector::VectorQtCorePoint {
    {
      let mut object: ::vector::VectorQtCorePoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_operator_add_to_output(self as *const ::vector::VectorQtCorePoint,
                                                              l as *const ::vector::VectorQtCorePoint,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::point::Point) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::operator+=(const QPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCorePoint) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::operator+=(const QVector<QPoint>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCorePoint
    where Args: overloading::VectorQtCorePointOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::operator=(const QVector<QPoint>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorQtCorePoint)
                             -> &'l0 mut ::vector::VectorQtCorePoint {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_operator_assign(self as *mut ::vector::VectorQtCorePoint,
                                                     v as *const ::vector::VectorQtCorePoint)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::operator==(const QVector<QPoint>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCorePoint) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_operator_eq(self as *const ::vector::VectorQtCorePoint,
                                                 v as *const ::vector::VectorQtCorePoint)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QVector<QPoint>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPoint_operator_index_const(self as *const ::vector::VectorQtCorePoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QVector<QPoint>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QPoint_operator_index(self as *mut ::vector::VectorQtCorePoint, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::operator!=(const QVector<QPoint>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCorePoint) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_operator_neq(self as *const ::vector::VectorQtCorePoint,
                                                  v as *const ::vector::VectorQtCorePoint)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::point::Point) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::operator<<(const QPoint& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCorePoint) -> &'l0 mut ::vector::VectorQtCorePoint```<br>
  /// C++ method: <span style='color: green;'>```QVector<QPoint>& QVector<QPoint>::operator<<(const QVector<QPoint>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCorePoint
    where Args: overloading::VectorQtCorePointOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_pop_back(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_pop_front(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::prepend(const QPoint& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_prepend(self as *mut ::vector::VectorQtCorePoint,
                                             t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::push_back(const QPoint& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_push_back(self as *mut ::vector::VectorQtCorePoint,
                                               t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::push_front(const QPoint& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_push_front(self as *mut ::vector::VectorQtCorePoint,
                                                t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCorePointRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::removeAll(const QPoint& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::point::Point) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_removeAll(self as *mut ::vector::VectorQtCorePoint,
                                               t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_removeAt(self as *mut ::vector::VectorQtCorePoint, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_removeFirst(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_removeLast(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::removeOne(const QPoint& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::point::Point) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_removeOne(self as *mut ::vector::VectorQtCorePoint,
                                               t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::replace(int i, const QPoint& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_replace(self as *mut ::vector::VectorQtCorePoint,
                                             i,
                                             t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_reserve(self as *mut ::vector::VectorQtCorePoint, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_resize(self as *mut ::vector::VectorQtCorePoint, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QPoint>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_size(self as *const ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_squeeze(self as *mut ::vector::VectorQtCorePoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QPoint>::startsWith(const QPoint& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::point::Point) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_startsWith(self as *const ::vector::VectorQtCorePoint,
                                                t as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QPoint>::swap(QVector<QPoint>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCorePoint) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QPoint_swap(self as *mut ::vector::VectorQtCorePoint,
                                          other as *mut ::vector::VectorQtCorePoint)
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVector<QPoint>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_takeAt_to_output(self as *mut ::vector::VectorQtCorePoint, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVector<QPoint>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_takeFirst_to_output(self as *mut ::vector::VectorQtCorePoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVector<QPoint>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_takeLast_to_output(self as *mut ::vector::VectorQtCorePoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPoint>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QVector<QPoint>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::point::Point)) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QVector<QPoint>::value(int i, const QPoint& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::point::Point
    where Args: overloading::VectorQtCorePointValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCorePoint {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QPoint>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QPoint_destructor(self as *mut ::vector::VectorQtCorePoint) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QRect>```</span>
#[repr(C)]
pub struct VectorQtCoreRect([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_RECT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCoreRect {
  unsafe fn new_uninitialized() -> VectorQtCoreRect {
    VectorQtCoreRect(::std::mem::uninitialized())
  }
}

impl VectorQtCoreRect {
  /// C++ method: <span style='color: green;'>```QVector<QRect>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::append(const QRect& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCoreRect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::append(const QVector<QRect>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_at(self as *const ::vector::VectorQtCoreRect, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_back_const(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QVector<QRect>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_back(self as *mut ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRect>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_capacity(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_clear(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```const QRect* QVector<QRect>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_constData(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_constFirst(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_constLast(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::contains(const QRect& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::rect::Rect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_contains(self as *const ::vector::VectorQtCoreRect,
                                             t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::rect::Rect) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::count(const QRect& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect* QVector<QRect>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_data_const(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```QRect* QVector<QRect>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::rect::Rect {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_data(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_empty(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::endsWith(const QRect& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::rect::Rect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_endsWith(self as *const ::vector::VectorQtCoreRect,
                                             t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::rect::Rect) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::fill(const QRect& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::rect::Rect, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::fill(const QRect& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRect
    where Args: overloading::VectorQtCoreRectFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_first_const(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QVector<QRect>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_first(self as *mut ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_front_const(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QVector<QRect>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_front(self as *mut ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::rect::Rect) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::indexOf(const QRect& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::rect::Rect, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::indexOf(const QRect& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QRect>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::insert(int i, const QRect& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::insert(int i, int n, const QRect& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_isEmpty(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_last_const(self as *const ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::rect::Rect) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::lastIndexOf(const QRect& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::rect::Rect, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::lastIndexOf(const QRect& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect& QVector<QRect>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRect_last(self as *mut ::vector::VectorQtCoreRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRect>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_length(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect> QVector<QRect>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect> QVector<QRect>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCoreRect
    where Args: overloading::VectorQtCoreRectMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_move(self as *mut ::vector::VectorQtCoreRect, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRect>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCoreRect) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRect>::QVector(const QVector<QRect>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRect>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::rect::Rect)) -> ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRect>::QVector(int size, const QRect& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCoreRect
    where Args: overloading::VectorQtCoreRectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QRect> QVector<QRect>::operator+(const QVector<QRect>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCoreRect) -> ::vector::VectorQtCoreRect {
    {
      let mut object: ::vector::VectorQtCoreRect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_operator_add_to_output(self as *const ::vector::VectorQtCoreRect,
                                                             l as *const ::vector::VectorQtCoreRect,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::rect::Rect) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::operator+=(const QRect& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCoreRect) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::operator+=(const QVector<QRect>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRect
    where Args: overloading::VectorQtCoreRectOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::operator=(const QVector<QRect>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorQtCoreRect) -> &'l0 mut ::vector::VectorQtCoreRect {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QRect_operator_assign(self as *mut ::vector::VectorQtCoreRect,
                                                    v as *const ::vector::VectorQtCoreRect)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::operator==(const QVector<QRect>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCoreRect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_operator_eq(self as *const ::vector::VectorQtCoreRect,
                                                v as *const ::vector::VectorQtCoreRect)
    }
  }

  /// C++ method: <span style='color: green;'>```const QRect& QVector<QRect>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QRect_operator_index_const(self as *const ::vector::VectorQtCoreRect, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QVector<QRect>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QRect_operator_index(self as *mut ::vector::VectorQtCoreRect, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::operator!=(const QVector<QRect>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCoreRect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_operator_neq(self as *const ::vector::VectorQtCoreRect,
                                                 v as *const ::vector::VectorQtCoreRect)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::rect::Rect) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::operator<<(const QRect& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCoreRect) -> &'l0 mut ::vector::VectorQtCoreRect```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRect>& QVector<QRect>::operator<<(const QVector<QRect>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRect
    where Args: overloading::VectorQtCoreRectOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_pop_back(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_pop_front(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::prepend(const QRect& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_prepend(self as *mut ::vector::VectorQtCoreRect,
                                            t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::push_back(const QRect& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_push_back(self as *mut ::vector::VectorQtCoreRect,
                                              t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::push_front(const QRect& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_push_front(self as *mut ::vector::VectorQtCoreRect,
                                               t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRect>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QRect>::removeAll(const QRect& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::rect::Rect) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_removeAll(self as *mut ::vector::VectorQtCoreRect,
                                              t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_removeAt(self as *mut ::vector::VectorQtCoreRect, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_removeFirst(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_removeLast(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::removeOne(const QRect& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::rect::Rect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_removeOne(self as *mut ::vector::VectorQtCoreRect,
                                              t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::replace(int i, const QRect& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_replace(self as *mut ::vector::VectorQtCoreRect,
                                            i,
                                            t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_reserve(self as *mut ::vector::VectorQtCoreRect, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_resize(self as *mut ::vector::VectorQtCoreRect, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRect>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_size(self as *const ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_squeeze(self as *mut ::vector::VectorQtCoreRect) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRect>::startsWith(const QRect& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::rect::Rect) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_startsWith(self as *const ::vector::VectorQtCoreRect,
                                               t as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRect>::swap(QVector<QRect>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCoreRect) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRect_swap(self as *mut ::vector::VectorQtCoreRect,
                                         other as *mut ::vector::VectorQtCoreRect)
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QVector<QRect>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_takeAt_to_output(self as *mut ::vector::VectorQtCoreRect, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QVector<QRect>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_takeFirst_to_output(self as *mut ::vector::VectorQtCoreRect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QVector<QRect>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_takeLast_to_output(self as *mut ::vector::VectorQtCoreRect, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRect>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QVector<QRect>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::rect::Rect)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QVector<QRect>::value(int i, const QRect& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::VectorQtCoreRectValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCoreRect {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QRect>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRect_destructor(self as *mut ::vector::VectorQtCoreRect) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QRectF>```</span>
#[repr(C)]
pub struct VectorQtCoreRectF([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_RECT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCoreRectF {
  unsafe fn new_uninitialized() -> VectorQtCoreRectF {
    VectorQtCoreRectF(::std::mem::uninitialized())
  }
}

impl VectorQtCoreRectF {
  /// C++ method: <span style='color: green;'>```QVector<QRectF>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::append(const QRectF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCoreRectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::append(const QVector<QRectF>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_at(self as *const ::vector::VectorQtCoreRectF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_back_const(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QVector<QRectF>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_back(self as *mut ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_capacity(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_clear(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```const QRectF* QVector<QRectF>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::rect_f::RectF {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_constData(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_constFirst(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_constLast(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::contains(const QRectF& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_contains(self as *const ::vector::VectorQtCoreRectF,
                                              t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::count(const QRectF& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRectF* QVector<QRectF>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::rect_f::RectF {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_data_const(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```QRectF* QVector<QRectF>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::rect_f::RectF {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_data(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_empty(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::endsWith(const QRectF& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_endsWith(self as *const ::vector::VectorQtCoreRectF,
                                              t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::rect_f::RectF) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::fill(const QRectF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::rect_f::RectF, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::fill(const QRectF& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRectF
    where Args: overloading::VectorQtCoreRectFFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_first_const(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QVector<QRectF>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_first(self as *mut ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_front_const(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QVector<QRectF>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_front(self as *mut ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::indexOf(const QRectF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::indexOf(const QRectF& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QRectF>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::insert(int i, const QRectF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::insert(int i, int n, const QRectF& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectFInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_isEmpty(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_last_const(self as *const ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::lastIndexOf(const QRectF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::lastIndexOf(const QRectF& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreRectFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF& QVector<QRectF>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QRectF_last(self as *mut ::vector::VectorQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_length(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF> QVector<QRectF>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF> QVector<QRectF>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCoreRectF
    where Args: overloading::VectorQtCoreRectFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_move(self as *mut ::vector::VectorQtCoreRectF, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRectF>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCoreRectF) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRectF>::QVector(const QVector<QRectF>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRectF>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::rect_f::RectF)) -> ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QRectF>::QVector(int size, const QRectF& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCoreRectF
    where Args: overloading::VectorQtCoreRectFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QRectF> QVector<QRectF>::operator+(const QVector<QRectF>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCoreRectF) -> ::vector::VectorQtCoreRectF {
    {
      let mut object: ::vector::VectorQtCoreRectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_operator_add_to_output(self as *const ::vector::VectorQtCoreRectF,
                                                              l as *const ::vector::VectorQtCoreRectF,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::rect_f::RectF) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::operator+=(const QRectF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCoreRectF) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::operator+=(const QVector<QRectF>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRectF
    where Args: overloading::VectorQtCoreRectFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::operator=(const QVector<QRectF>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorQtCoreRectF)
                             -> &'l0 mut ::vector::VectorQtCoreRectF {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_operator_assign(self as *mut ::vector::VectorQtCoreRectF,
                                                     v as *const ::vector::VectorQtCoreRectF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::operator==(const QVector<QRectF>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCoreRectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_operator_eq(self as *const ::vector::VectorQtCoreRectF,
                                                 v as *const ::vector::VectorQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QVector<QRectF>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QRectF_operator_index_const(self as *const ::vector::VectorQtCoreRectF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QVector<QRectF>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QRectF_operator_index(self as *mut ::vector::VectorQtCoreRectF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::operator!=(const QVector<QRectF>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCoreRectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_operator_neq(self as *const ::vector::VectorQtCoreRectF,
                                                  v as *const ::vector::VectorQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::rect_f::RectF) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::operator<<(const QRectF& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCoreRectF) -> &'l0 mut ::vector::VectorQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QVector<QRectF>& QVector<QRectF>::operator<<(const QVector<QRectF>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreRectF
    where Args: overloading::VectorQtCoreRectFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_pop_back(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_pop_front(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::prepend(const QRectF& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_prepend(self as *mut ::vector::VectorQtCoreRectF,
                                             t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::push_back(const QRectF& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_push_back(self as *mut ::vector::VectorQtCoreRectF,
                                               t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::push_front(const QRectF& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_push_front(self as *mut ::vector::VectorQtCoreRectF,
                                                t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreRectFRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::removeAll(const QRectF& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::rect_f::RectF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_removeAll(self as *mut ::vector::VectorQtCoreRectF,
                                               t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_removeAt(self as *mut ::vector::VectorQtCoreRectF, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_removeFirst(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_removeLast(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::removeOne(const QRectF& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_removeOne(self as *mut ::vector::VectorQtCoreRectF,
                                               t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::replace(int i, const QRectF& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_replace(self as *mut ::vector::VectorQtCoreRectF,
                                             i,
                                             t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_reserve(self as *mut ::vector::VectorQtCoreRectF, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_resize(self as *mut ::vector::VectorQtCoreRectF, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QRectF>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_size(self as *const ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_squeeze(self as *mut ::vector::VectorQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QRectF>::startsWith(const QRectF& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_startsWith(self as *const ::vector::VectorQtCoreRectF,
                                                t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QRectF>::swap(QVector<QRectF>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCoreRectF) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QRectF_swap(self as *mut ::vector::VectorQtCoreRectF,
                                          other as *mut ::vector::VectorQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QVector<QRectF>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_takeAt_to_output(self as *mut ::vector::VectorQtCoreRectF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QVector<QRectF>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_takeFirst_to_output(self as *mut ::vector::VectorQtCoreRectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QVector<QRectF>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_takeLast_to_output(self as *mut ::vector::VectorQtCoreRectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QVector<QRectF>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::rect_f::RectF)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QVector<QRectF>::value(int i, const QRectF& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::VectorQtCoreRectFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCoreRectF {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QRectF>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QRectF_destructor(self as *mut ::vector::VectorQtCoreRectF) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QSize>```</span>
#[repr(C)]
pub struct VectorQtCoreSize([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_QT_CORE_SIZE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtCoreSize {
  unsafe fn new_uninitialized() -> VectorQtCoreSize {
    VectorQtCoreSize(::std::mem::uninitialized())
  }
}

impl VectorQtCoreSize {
  /// C++ method: <span style='color: green;'>```QVector<QSize>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::append(const QSize& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtCoreSize) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::append(const QVector<QSize>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreSizeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_at(self as *const ::vector::VectorQtCoreSize, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_back_const(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QVector<QSize>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_back(self as *mut ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSize>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_capacity(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_clear(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```const QSize* QVector<QSize>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_core::size::Size {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_constData(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_constFirst(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_constLast(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::contains(const QSize& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_contains(self as *const ::vector::VectorQtCoreSize,
                                             t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::count(const QSize& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreSizeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSize* QVector<QSize>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_core::size::Size {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_data_const(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```QSize* QVector<QSize>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_core::size::Size {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_data(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_empty(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::endsWith(const QSize& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_endsWith(self as *const ::vector::VectorQtCoreSize,
                                             t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_core::size::Size) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::fill(const QSize& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_core::size::Size, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::fill(const QSize& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreSize
    where Args: overloading::VectorQtCoreSizeFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_first_const(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QVector<QSize>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_first(self as *mut ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QVector<QSize> QVector<QSize>::fromList(const QList<QSize>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::list::ListQtCoreSize) -> ::vector::VectorQtCoreSize {
    {
      let mut object: ::vector::VectorQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_fromList_to_output(list as *const ::list::ListQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_front_const(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QVector<QSize>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_front(self as *mut ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::indexOf(const QSize& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::size::Size, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::indexOf(const QSize& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreSizeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSize>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::insert(int i, const QSize& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::insert(int i, int n, const QSize& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreSizeInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_isEmpty(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_last_const(self as *const ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::lastIndexOf(const QSize& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::size::Size, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::lastIndexOf(const QSize& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtCoreSizeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSize& QVector<QSize>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QSize_last(self as *mut ::vector::VectorQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSize>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_length(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize> QVector<QSize>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize> QVector<QSize>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtCoreSize
    where Args: overloading::VectorQtCoreSizeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_move(self as *mut ::vector::VectorQtCoreSize, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSize>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtCoreSize) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSize>::QVector(const QVector<QSize>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSize>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_core::size::Size)) -> ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QSize>::QVector(int size, const QSize& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtCoreSize
    where Args: overloading::VectorQtCoreSizeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QSize> QVector<QSize>::operator+(const QVector<QSize>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtCoreSize) -> ::vector::VectorQtCoreSize {
    {
      let mut object: ::vector::VectorQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_operator_add_to_output(self as *const ::vector::VectorQtCoreSize,
                                                             l as *const ::vector::VectorQtCoreSize,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::size::Size) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::operator+=(const QSize& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtCoreSize) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::operator+=(const QVector<QSize>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreSize
    where Args: overloading::VectorQtCoreSizeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::operator=(const QVector<QSize>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorQtCoreSize) -> &'l0 mut ::vector::VectorQtCoreSize {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QSize_operator_assign(self as *mut ::vector::VectorQtCoreSize,
                                                    v as *const ::vector::VectorQtCoreSize)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::operator==(const QVector<QSize>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtCoreSize) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_operator_eq(self as *const ::vector::VectorQtCoreSize,
                                                v as *const ::vector::VectorQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QVector<QSize>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QSize_operator_index_const(self as *const ::vector::VectorQtCoreSize, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QVector<QSize>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QSize_operator_index(self as *mut ::vector::VectorQtCoreSize, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::operator!=(const QVector<QSize>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtCoreSize) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_operator_neq(self as *const ::vector::VectorQtCoreSize,
                                                 v as *const ::vector::VectorQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::size::Size) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::operator<<(const QSize& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtCoreSize) -> &'l0 mut ::vector::VectorQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QVector<QSize>& QVector<QSize>::operator<<(const QVector<QSize>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtCoreSize
    where Args: overloading::VectorQtCoreSizeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_pop_back(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_pop_front(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::prepend(const QSize& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_prepend(self as *mut ::vector::VectorQtCoreSize,
                                            t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::push_back(const QSize& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_push_back(self as *mut ::vector::VectorQtCoreSize,
                                              t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::push_front(const QSize& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_push_front(self as *mut ::vector::VectorQtCoreSize,
                                               t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QSize>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtCoreSizeRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QSize>::removeAll(const QSize& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::size::Size) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_removeAll(self as *mut ::vector::VectorQtCoreSize,
                                              t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_removeAt(self as *mut ::vector::VectorQtCoreSize, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_removeFirst(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_removeLast(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::removeOne(const QSize& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_removeOne(self as *mut ::vector::VectorQtCoreSize,
                                              t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::replace(int i, const QSize& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_replace(self as *mut ::vector::VectorQtCoreSize,
                                            i,
                                            t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_reserve(self as *mut ::vector::VectorQtCoreSize, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_resize(self as *mut ::vector::VectorQtCoreSize, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QSize>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_size(self as *const ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_squeeze(self as *mut ::vector::VectorQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QSize>::startsWith(const QSize& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_startsWith(self as *const ::vector::VectorQtCoreSize,
                                               t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QSize>::swap(QVector<QSize>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtCoreSize) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QSize_swap(self as *mut ::vector::VectorQtCoreSize,
                                         other as *mut ::vector::VectorQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QVector<QSize>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_takeAt_to_output(self as *mut ::vector::VectorQtCoreSize, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QVector<QSize>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_takeFirst_to_output(self as *mut ::vector::VectorQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QVector<QSize>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_takeLast_to_output(self as *mut ::vector::VectorQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize> QVector<QSize>::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::list::ListQtCoreSize {
    {
      let mut object: ::list::ListQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_toList_to_output(self as *const ::vector::VectorQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QVector<QSize>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::size::Size)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QVector<QSize>::value(int i, const QSize& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::VectorQtCoreSizeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtCoreSize {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QSize>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QSize_destructor(self as *mut ::vector::VectorQtCoreSize) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QTextFormat>```</span>
#[repr(C)]
pub struct VectorTextFormat([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_TEXT_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorTextFormat {
  unsafe fn new_uninitialized() -> VectorTextFormat {
    VectorTextFormat(::std::mem::uninitialized())
  }
}

impl VectorTextFormat {
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_format::TextFormat) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::append(const QTextFormat& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorTextFormat) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::append(const QVector<QTextFormat>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextFormatAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_at(self as *const ::vector::VectorTextFormat, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_back_const(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFormat& QVector<QTextFormat>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_back(self as *mut ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_capacity(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_clear(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat* QVector<QTextFormat>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::text_format::TextFormat {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_constData(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_constFirst(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_constLast(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::contains(const QTextFormat& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_contains(self as *const ::vector::VectorTextFormat,
                                                   t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::text_format::TextFormat) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::count(const QTextFormat& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextFormatCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextFormat* QVector<QTextFormat>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::text_format::TextFormat {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_data_const(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat* QVector<QTextFormat>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::text_format::TextFormat {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_data(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_empty(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::endsWith(const QTextFormat& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_endsWith(self as *const ::vector::VectorTextFormat,
                                                   t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::text_format::TextFormat) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::fill(const QTextFormat& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::text_format::TextFormat, ::libc::c_int)) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::fill(const QTextFormat& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextFormat
    where Args: overloading::VectorTextFormatFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_first_const(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFormat& QVector<QTextFormat>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_first(self as *mut ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_front_const(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFormat& QVector<QTextFormat>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_front(self as *mut ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::text_format::TextFormat) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::indexOf(const QTextFormat& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::text_format::TextFormat, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::indexOf(const QTextFormat& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextFormatIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::text_format::TextFormat)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::insert(int i, const QTextFormat& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::text_format::TextFormat)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::insert(int i, int n, const QTextFormat& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextFormatInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_isEmpty(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_last_const(self as *const ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::text_format::TextFormat) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::lastIndexOf(const QTextFormat& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::text_format::TextFormat, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::lastIndexOf(const QTextFormat& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextFormatLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextFormat& QVector<QTextFormat>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_last(self as *mut ::vector::VectorTextFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_length(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat> QVector<QTextFormat>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat> QVector<QTextFormat>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorTextFormat
    where Args: overloading::VectorTextFormatMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_move(self as *mut ::vector::VectorTextFormat, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextFormat>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorTextFormat) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextFormat>::QVector(const QVector<QTextFormat>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextFormat>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::text_format::TextFormat)) -> ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextFormat>::QVector(int size, const QTextFormat& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorTextFormat
    where Args: overloading::VectorTextFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat> QVector<QTextFormat>::operator+(const QVector<QTextFormat>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorTextFormat) -> ::vector::VectorTextFormat {
    {
      let mut object: ::vector::VectorTextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_operator_add_to_output(self as *const ::vector::VectorTextFormat,
                                                                   l as *const ::vector::VectorTextFormat,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_format::TextFormat) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::operator+=(const QTextFormat& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorTextFormat) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::operator+=(const QVector<QTextFormat>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextFormat
    where Args: overloading::VectorTextFormatOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::operator=(const QVector<QTextFormat>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorTextFormat) -> &'l0 mut ::vector::VectorTextFormat {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_operator_assign(self as *mut ::vector::VectorTextFormat,
                                                          v as *const ::vector::VectorTextFormat)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::operator==(const QVector<QTextFormat>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorTextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_operator_eq(self as *const ::vector::VectorTextFormat,
                                                      v as *const ::vector::VectorTextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextFormat& QVector<QTextFormat>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_operator_index_const(self as *const ::vector::VectorTextFormat, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFormat& QVector<QTextFormat>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_format::TextFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_operator_index(self as *mut ::vector::VectorTextFormat, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::operator!=(const QVector<QTextFormat>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorTextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_operator_neq(self as *const ::vector::VectorTextFormat,
                                                       v as *const ::vector::VectorTextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_format::TextFormat) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::operator<<(const QTextFormat& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorTextFormat) -> &'l0 mut ::vector::VectorTextFormat```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>& QVector<QTextFormat>::operator<<(const QVector<QTextFormat>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextFormat
    where Args: overloading::VectorTextFormatOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_pop_back(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_pop_front(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::prepend(const QTextFormat& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_prepend(self as *mut ::vector::VectorTextFormat,
                                                  t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::push_back(const QTextFormat& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_push_back(self as *mut ::vector::VectorTextFormat,
                                                    t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::push_front(const QTextFormat& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_push_front(self as *mut ::vector::VectorTextFormat,
                                                     t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextFormatRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::removeAll(const QTextFormat& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::text_format::TextFormat) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_removeAll(self as *mut ::vector::VectorTextFormat,
                                                    t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_removeAt(self as *mut ::vector::VectorTextFormat, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_removeFirst(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_removeLast(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::removeOne(const QTextFormat& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_removeOne(self as *mut ::vector::VectorTextFormat,
                                                    t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::replace(int i, const QTextFormat& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_format::TextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_replace(self as *mut ::vector::VectorTextFormat,
                                                  i,
                                                  t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_reserve(self as *mut ::vector::VectorTextFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_resize(self as *mut ::vector::VectorTextFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextFormat>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_size(self as *const ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_squeeze(self as *mut ::vector::VectorTextFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextFormat>::startsWith(const QTextFormat& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::text_format::TextFormat) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_startsWith(self as *const ::vector::VectorTextFormat,
                                                     t as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextFormat>::swap(QVector<QTextFormat>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorTextFormat) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextFormat_swap(self as *mut ::vector::VectorTextFormat,
                                               other as *mut ::vector::VectorTextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QVector<QTextFormat>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_takeAt_to_output(self as *mut ::vector::VectorTextFormat, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QVector<QTextFormat>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_takeFirst_to_output(self as *mut ::vector::VectorTextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFormat QVector<QTextFormat>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::text_format::TextFormat {
    {
      let mut object: ::text_format::TextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_takeLast_to_output(self as *mut ::vector::VectorTextFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::text_format::TextFormat```<br>
  /// C++ method: <span style='color: green;'>```QTextFormat QVector<QTextFormat>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::text_format::TextFormat)) -> ::text_format::TextFormat```<br>
  /// C++ method: <span style='color: green;'>```QTextFormat QVector<QTextFormat>::value(int i, const QTextFormat& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::text_format::TextFormat
    where Args: overloading::VectorTextFormatValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorTextFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QTextFormat>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_destructor(self as *mut ::vector::VectorTextFormat) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QTextLayout::FormatRange>```</span>
#[repr(C)]
pub struct VectorTextLayoutFormatRange([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_TEXT_LAYOUT_FORMAT_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorTextLayoutFormatRange {
  unsafe fn new_uninitialized() -> VectorTextLayoutFormatRange {
    VectorTextLayoutFormatRange(::std::mem::uninitialized())
  }
}

impl VectorTextLayoutFormatRange {
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_layout::FormatRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::append(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorTextLayoutFormatRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::append(const QVector<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLayoutFormatRangeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_at(self as *const ::vector::VectorTextLayoutFormatRange, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_back_const(self as *const ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_back(self as *mut ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLayout::FormatRange>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_capacity(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_clear(self as *mut ::vector::VectorTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange* QVector<QTextLayout::FormatRange>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::text_layout::FormatRange {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constData(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constFirst(self as *const ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constLast(self as *const ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLayout::FormatRange>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_count(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange* QVector<QTextLayout::FormatRange>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::text_layout::FormatRange {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_data_const(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange* QVector<QTextLayout::FormatRange>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::text_layout::FormatRange {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_data(self as *mut ::vector::VectorTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLayout::FormatRange>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_empty(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::text_layout::FormatRange) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::fill(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::text_layout::FormatRange, ::libc::c_int)) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::fill(const QTextLayout::FormatRange& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLayoutFormatRange
    where Args: overloading::VectorTextLayoutFormatRangeFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_first_const(self as *const ::vector::VectorTextLayoutFormatRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_first(self as *mut ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QVector<QTextLayout::FormatRange> QVector<QTextLayout::FormatRange>::fromList(const QList<QTextLayout::FormatRange>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::list::ListTextLayoutFormatRange) -> ::vector::VectorTextLayoutFormatRange {
    {
      let mut object: ::vector::VectorTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_fromList_to_output(list as *const ::list::ListTextLayoutFormatRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_front_const(self as *const ::vector::VectorTextLayoutFormatRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_front(self as *mut ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::text_layout::FormatRange)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::insert(int i, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::text_layout::FormatRange)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::insert(int i, int n, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLayoutFormatRangeInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QTextLayout::FormatRange>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_isEmpty(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_last_const(self as *const ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_last(self as *mut ::vector::VectorTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLayout::FormatRange>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_length(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange> QVector<QTextLayout::FormatRange>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange> QVector<QTextLayout::FormatRange>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorTextLayoutFormatRange
    where Args: overloading::VectorTextLayoutFormatRangeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_move(self as *mut ::vector::VectorTextLayoutFormatRange, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLayout::FormatRange>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorTextLayoutFormatRange) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLayout::FormatRange>::QVector(const QVector<QTextLayout::FormatRange>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLayout::FormatRange>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::text_layout::FormatRange)) -> ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLayout::FormatRange>::QVector(int size, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorTextLayoutFormatRange
    where Args: overloading::VectorTextLayoutFormatRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange> QVector<QTextLayout::FormatRange>::operator+(const QVector<QTextLayout::FormatRange>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorTextLayoutFormatRange) -> ::vector::VectorTextLayoutFormatRange {
    {
      let mut object: ::vector::VectorTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_to_output(self as *const ::vector::VectorTextLayoutFormatRange, l as *const ::vector::VectorTextLayoutFormatRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_layout::FormatRange) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::operator+=(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorTextLayoutFormatRange) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::operator+=(const QVector<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLayoutFormatRange
    where Args: overloading::VectorTextLayoutFormatRangeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::operator=(const QVector<QTextLayout::FormatRange>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorTextLayoutFormatRange)
                             -> &'l0 mut ::vector::VectorTextLayoutFormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_assign(self as *mut ::vector::VectorTextLayoutFormatRange, v as *const ::vector::VectorTextLayoutFormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_layout::FormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_index_const(self as *const ::vector::VectorTextLayoutFormatRange, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QVector<QTextLayout::FormatRange>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_index(self as *mut ::vector::VectorTextLayoutFormatRange, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_layout::FormatRange) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::operator<<(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorTextLayoutFormatRange) -> &'l0 mut ::vector::VectorTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>& QVector<QTextLayout::FormatRange>::operator<<(const QVector<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLayoutFormatRange
    where Args: overloading::VectorTextLayoutFormatRangeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_pop_back(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_pop_front(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::prepend(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_prepend(self as *mut ::vector::VectorTextLayoutFormatRange,
                                                              t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::push_back(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_push_back(self as *mut ::vector::VectorTextLayoutFormatRange,
                                                                t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::push_front(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_push_front(self as *mut ::vector::VectorTextLayoutFormatRange,
                                                                 t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLayoutFormatRangeRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_removeAt(self as *mut ::vector::VectorTextLayoutFormatRange, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_removeFirst(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_removeLast(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::replace(int i, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_replace(self as *mut ::vector::VectorTextLayoutFormatRange,
                                                              i,
                                                              t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_reserve(self as *mut ::vector::VectorTextLayoutFormatRange, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_resize(self as *mut ::vector::VectorTextLayoutFormatRange, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLayout::FormatRange>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_size(self as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_squeeze(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLayout::FormatRange>::swap(QVector<QTextLayout::FormatRange>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorTextLayoutFormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_swap(self as *mut ::vector::VectorTextLayoutFormatRange,
                                                           other as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QVector<QTextLayout::FormatRange>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_takeAt_as_ptr(self as *mut ::vector::VectorTextLayoutFormatRange, i) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QVector<QTextLayout::FormatRange>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_takeFirst_as_ptr(self as *mut ::vector::VectorTextLayoutFormatRange) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QVector<QTextLayout::FormatRange>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_takeLast_as_ptr(self as *mut ::vector::VectorTextLayoutFormatRange) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::text_layout::FormatRange>```<br>
  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QVector<QTextLayout::FormatRange>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::text_layout::FormatRange)) -> ::cpp_utils::CppBox<::text_layout::FormatRange>```<br>
  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QVector<QTextLayout::FormatRange>::value(int i, const QTextLayout::FormatRange& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::text_layout::FormatRange>
    where Args: overloading::VectorTextLayoutFormatRangeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorTextLayoutFormatRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QTextLayout::FormatRange>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_destructor(self as *mut ::vector::VectorTextLayoutFormatRange)
    }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QTextLength>```</span>
#[repr(C)]
pub struct VectorTextLength([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_TEXT_LENGTH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorTextLength {
  unsafe fn new_uninitialized() -> VectorTextLength {
    VectorTextLength(::std::mem::uninitialized())
  }
}

impl VectorTextLength {
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_length::TextLength) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::append(const QTextLength& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorTextLength) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::append(const QVector<QTextLength>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLengthAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_length::TextLength {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLength_at(self as *const ::vector::VectorTextLength, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_back_const(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLength& QVector<QTextLength>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_length::TextLength {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLength_back(self as *mut ::vector::VectorTextLength) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_capacity(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_clear(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLength* QVector<QTextLength>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::text_length::TextLength {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_constData(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_constFirst(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_constLast(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::contains(const QTextLength& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_contains(self as *const ::vector::VectorTextLength,
                                                   t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::text_length::TextLength) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::count(const QTextLength& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextLengthCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLength* QVector<QTextLength>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::text_length::TextLength {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_data_const(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```QTextLength* QVector<QTextLength>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::text_length::TextLength {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_data(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_empty(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::endsWith(const QTextLength& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_endsWith(self as *const ::vector::VectorTextLength,
                                                   t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::text_length::TextLength) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::fill(const QTextLength& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::text_length::TextLength, ::libc::c_int)) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::fill(const QTextLength& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLength
    where Args: overloading::VectorTextLengthFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_first_const(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLength& QVector<QTextLength>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_length::TextLength {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLength_first(self as *mut ::vector::VectorTextLength) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_front_const(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLength& QVector<QTextLength>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_length::TextLength {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLength_front(self as *mut ::vector::VectorTextLength) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::text_length::TextLength) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::indexOf(const QTextLength& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::text_length::TextLength, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::indexOf(const QTextLength& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextLengthIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::text_length::TextLength)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::insert(int i, const QTextLength& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::text_length::TextLength)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::insert(int i, int n, const QTextLength& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLengthInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_isEmpty(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_last_const(self as *const ::vector::VectorTextLength) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::text_length::TextLength) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::lastIndexOf(const QTextLength& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::text_length::TextLength, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::lastIndexOf(const QTextLength& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorTextLengthLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextLength& QVector<QTextLength>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_length::TextLength {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLength_last(self as *mut ::vector::VectorTextLength) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_length(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength> QVector<QTextLength>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength> QVector<QTextLength>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorTextLength
    where Args: overloading::VectorTextLengthMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_move(self as *mut ::vector::VectorTextLength, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLength>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorTextLength) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLength>::QVector(const QVector<QTextLength>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLength>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::text_length::TextLength)) -> ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QTextLength>::QVector(int size, const QTextLength& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorTextLength
    where Args: overloading::VectorTextLengthNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextLength> QVector<QTextLength>::operator+(const QVector<QTextLength>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorTextLength) -> ::vector::VectorTextLength {
    {
      let mut object: ::vector::VectorTextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_operator_add_to_output(self as *const ::vector::VectorTextLength,
                                                                   l as *const ::vector::VectorTextLength,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_length::TextLength) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::operator+=(const QTextLength& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorTextLength) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::operator+=(const QVector<QTextLength>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLength
    where Args: overloading::VectorTextLengthOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::operator=(const QVector<QTextLength>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorTextLength) -> &'l0 mut ::vector::VectorTextLength {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_operator_assign(self as *mut ::vector::VectorTextLength,
                                                          v as *const ::vector::VectorTextLength)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::operator==(const QVector<QTextLength>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorTextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_operator_eq(self as *const ::vector::VectorTextLength,
                                                      v as *const ::vector::VectorTextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLength& QVector<QTextLength>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_operator_index_const(self as *const ::vector::VectorTextLength, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLength& QVector<QTextLength>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_length::TextLength {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_operator_index(self as *mut ::vector::VectorTextLength, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::operator!=(const QVector<QTextLength>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorTextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_operator_neq(self as *const ::vector::VectorTextLength,
                                                       v as *const ::vector::VectorTextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_length::TextLength) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::operator<<(const QTextLength& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorTextLength) -> &'l0 mut ::vector::VectorTextLength```<br>
  /// C++ method: <span style='color: green;'>```QVector<QTextLength>& QVector<QTextLength>::operator<<(const QVector<QTextLength>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorTextLength
    where Args: overloading::VectorTextLengthOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_pop_back(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_pop_front(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::prepend(const QTextLength& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_length::TextLength) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_prepend(self as *mut ::vector::VectorTextLength,
                                                  t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::push_back(const QTextLength& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_length::TextLength) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_push_back(self as *mut ::vector::VectorTextLength,
                                                    t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::push_front(const QTextLength& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_length::TextLength) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_push_front(self as *mut ::vector::VectorTextLength,
                                                     t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorTextLengthRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::removeAll(const QTextLength& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::text_length::TextLength) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_removeAll(self as *mut ::vector::VectorTextLength,
                                                    t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_removeAt(self as *mut ::vector::VectorTextLength, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_removeFirst(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_removeLast(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::removeOne(const QTextLength& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_removeOne(self as *mut ::vector::VectorTextLength,
                                                    t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::replace(int i, const QTextLength& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_length::TextLength) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_replace(self as *mut ::vector::VectorTextLength,
                                                  i,
                                                  t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_reserve(self as *mut ::vector::VectorTextLength, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_resize(self as *mut ::vector::VectorTextLength, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QTextLength>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_size(self as *const ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_squeeze(self as *mut ::vector::VectorTextLength) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QTextLength>::startsWith(const QTextLength& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_startsWith(self as *const ::vector::VectorTextLength,
                                                     t as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QTextLength>::swap(QVector<QTextLength>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorTextLength) {
    unsafe {
      ::ffi::qt_gui_c_QVector_QTextLength_swap(self as *mut ::vector::VectorTextLength,
                                               other as *mut ::vector::VectorTextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QVector<QTextLength>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_takeAt_to_output(self as *mut ::vector::VectorTextLength, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QVector<QTextLength>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_takeFirst_to_output(self as *mut ::vector::VectorTextLength, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLength QVector<QTextLength>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::text_length::TextLength {
    {
      let mut object: ::text_length::TextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_takeLast_to_output(self as *mut ::vector::VectorTextLength, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::text_length::TextLength```<br>
  /// C++ method: <span style='color: green;'>```QTextLength QVector<QTextLength>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::text_length::TextLength)) -> ::text_length::TextLength```<br>
  /// C++ method: <span style='color: green;'>```QTextLength QVector<QTextLength>::value(int i, const QTextLength& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::text_length::TextLength
    where Args: overloading::VectorTextLengthValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorTextLength {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QTextLength>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_QTextLength_destructor(self as *mut ::vector::VectorTextLength) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<quint32>```</span>
#[repr(C)]
pub struct VectorU32([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_U32]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorU32 {
  unsafe fn new_uninitialized() -> VectorU32 {
    VectorU32(::std::mem::uninitialized())
  }
}

impl VectorU32 {
  /// C++ method: <span style='color: green;'>```QVector<quint32>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorU32) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::append(const QVector<quint32>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &u32) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::append(const quint32& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU32AppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_at(self as *const ::vector::VectorU32, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_back_const(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```quint32& QVector<quint32>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_back(self as *mut ::vector::VectorU32) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<quint32>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_capacity(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_clear(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```const quint32* QVector<quint32>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_constData(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_constFirst(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_constLast(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::contains(const quint32& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &u32) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_contains(self as *const ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &u32) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::count(const quint32& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU32CountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const quint32* QVector<quint32>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_data_const(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```quint32* QVector<quint32>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_data(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_empty(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::endsWith(const quint32& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &u32) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_endsWith(self as *const ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 u32) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::fill(const quint32& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 u32, ::libc::c_int)) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::fill(const quint32& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU32
    where Args: overloading::VectorU32FillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_first_const(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```quint32& QVector<quint32>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_first(self as *mut ::vector::VectorU32) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_front_const(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```quint32& QVector<quint32>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_front(self as *mut ::vector::VectorU32) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &u32) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::indexOf(const quint32& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&u32, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::indexOf(const quint32& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU32IndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<quint32>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &u32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::insert(int i, const quint32& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &u32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::insert(int i, int n, const quint32& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU32InsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_isEmpty(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_last_const(self as *const ::vector::VectorU32) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &u32) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::lastIndexOf(const quint32& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&u32, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::lastIndexOf(const quint32& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU32LastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```quint32& QVector<quint32>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_last(self as *mut ::vector::VectorU32) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<quint32>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_length(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32> QVector<quint32>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32> QVector<quint32>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorU32
    where Args: overloading::VectorU32MidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_move(self as *mut ::vector::VectorU32, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<quint32>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorU32) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<quint32>::QVector(const QVector<quint32>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<quint32>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &u32)) -> ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<quint32>::QVector(int size, const quint32& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorU32
    where Args: overloading::VectorU32NewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<quint32> QVector<quint32>::operator+(const QVector<quint32>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorU32) -> ::vector::VectorU32 {
    {
      let mut object: ::vector::VectorU32 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_operator_add_to_output(self as *const ::vector::VectorU32,
                                                               l as *const ::vector::VectorU32,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorU32) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::operator+=(const QVector<quint32>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 u32) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::operator+=(const quint32& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU32
    where Args: overloading::VectorU32OpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::operator=(const QVector<quint32>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorU32) -> &'l0 mut ::vector::VectorU32 {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_quint32_operator_assign(self as *mut ::vector::VectorU32,
                                                      v as *const ::vector::VectorU32)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::operator==(const QVector<quint32>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorU32) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_quint32_operator_eq(self as *const ::vector::VectorU32,
                                                  v as *const ::vector::VectorU32)
    }
  }

  /// C++ method: <span style='color: green;'>```const quint32& QVector<quint32>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 u32 {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_quint32_operator_index_const(self as *const ::vector::VectorU32, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```quint32& QVector<quint32>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut u32 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_quint32_operator_index(self as *mut ::vector::VectorU32, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::operator!=(const QVector<quint32>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorU32) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_quint32_operator_neq(self as *const ::vector::VectorU32,
                                                   v as *const ::vector::VectorU32)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorU32) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::operator<<(const QVector<quint32>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 u32) -> &'l0 mut ::vector::VectorU32```<br>
  /// C++ method: <span style='color: green;'>```QVector<quint32>& QVector<quint32>::operator<<(const quint32& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU32
    where Args: overloading::VectorU32OpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_pop_back(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_pop_front(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::prepend(const quint32& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &u32) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_prepend(self as *mut ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::push_back(const quint32& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &u32) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_push_back(self as *mut ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::push_front(const quint32& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &u32) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_push_front(self as *mut ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<quint32>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU32RemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<quint32>::removeAll(const quint32& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &u32) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_removeAll(self as *mut ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_removeAt(self as *mut ::vector::VectorU32, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_removeFirst(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_removeLast(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::removeOne(const quint32& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &u32) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_removeOne(self as *mut ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::replace(int i, const quint32& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &u32) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_replace(self as *mut ::vector::VectorU32, i, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_reserve(self as *mut ::vector::VectorU32, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_resize(self as *mut ::vector::VectorU32, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<quint32>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_size(self as *const ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_squeeze(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<quint32>::startsWith(const quint32& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &u32) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_startsWith(self as *const ::vector::VectorU32, t as *const u32) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<quint32>::swap(QVector<quint32>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorU32) {
    unsafe {
      ::ffi::qt_gui_c_QVector_quint32_swap(self as *mut ::vector::VectorU32,
                                           other as *mut ::vector::VectorU32)
    }
  }

  /// C++ method: <span style='color: green;'>```quint32 QVector<quint32>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_takeAt(self as *mut ::vector::VectorU32, i) }
  }

  /// C++ method: <span style='color: green;'>```quint32 QVector<quint32>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_takeFirst(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```quint32 QVector<quint32>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_takeLast(self as *mut ::vector::VectorU32) }
  }

  /// C++ method: <span style='color: green;'>```QVector<quint32>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> u32```<br>
  /// C++ method: <span style='color: green;'>```quint32 QVector<quint32>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &u32)) -> u32```<br>
  /// C++ method: <span style='color: green;'>```quint32 QVector<quint32>::value(int i, const quint32& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> u32
    where Args: overloading::VectorU32ValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorU32 {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<quint32>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_quint32_destructor(self as *mut ::vector::VectorU32) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<GLuint64>```</span>
#[repr(C)]
pub struct VectorU64([u8; ::type_sizes::QT_GUI_VECTOR_VECTOR_U64]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorU64 {
  unsafe fn new_uninitialized() -> VectorU64 {
    VectorU64(::std::mem::uninitialized())
  }
}

impl VectorU64 {
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &u64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::append(const GLuint64& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorU64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::append(const QVector<GLuint64>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU64AppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_at(self as *const ::vector::VectorU64, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_back_const(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```GLuint64& QVector<GLuint64>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_back(self as *mut ::vector::VectorU64) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_capacity(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_clear(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```const GLuint64* QVector<GLuint64>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_constData(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_constFirst(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_constLast(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::contains(const GLuint64& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &u64) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_contains(self as *const ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &u64) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::count(const GLuint64& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU64CountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const GLuint64* QVector<GLuint64>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_data_const(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```GLuint64* QVector<GLuint64>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_data(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_empty(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::endsWith(const GLuint64& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &u64) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_endsWith(self as *const ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 u64) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::fill(const GLuint64& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 u64, ::libc::c_int)) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::fill(const GLuint64& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU64
    where Args: overloading::VectorU64FillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_first_const(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```GLuint64& QVector<GLuint64>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_first(self as *mut ::vector::VectorU64) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_front_const(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```GLuint64& QVector<GLuint64>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_front(self as *mut ::vector::VectorU64) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &u64) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::indexOf(const GLuint64& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&u64, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::indexOf(const GLuint64& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU64IndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &u64)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::insert(int i, const GLuint64& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &u64)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::insert(int i, int n, const GLuint64& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU64InsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_isEmpty(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_last_const(self as *const ::vector::VectorU64) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &u64) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::lastIndexOf(const GLuint64& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&u64, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::lastIndexOf(const GLuint64& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorU64LastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```GLuint64& QVector<GLuint64>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_last(self as *mut ::vector::VectorU64) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_length(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64> QVector<GLuint64>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64> QVector<GLuint64>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorU64
    where Args: overloading::VectorU64MidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_move(self as *mut ::vector::VectorU64, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<GLuint64>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorU64) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<GLuint64>::QVector(const QVector<GLuint64>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<GLuint64>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &u64)) -> ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<GLuint64>::QVector(int size, const GLuint64& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorU64
    where Args: overloading::VectorU64NewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<GLuint64> QVector<GLuint64>::operator+(const QVector<GLuint64>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorU64) -> ::vector::VectorU64 {
    {
      let mut object: ::vector::VectorU64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_operator_add_to_output(self as *const ::vector::VectorU64,
                                                                l as *const ::vector::VectorU64,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 u64) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::operator+=(const GLuint64& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorU64) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::operator+=(const QVector<GLuint64>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU64
    where Args: overloading::VectorU64OpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::operator=(const QVector<GLuint64>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorU64) -> &'l0 mut ::vector::VectorU64 {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector_GLuint64_operator_assign(self as *mut ::vector::VectorU64,
                                                       v as *const ::vector::VectorU64)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::operator==(const QVector<GLuint64>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorU64) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_GLuint64_operator_eq(self as *const ::vector::VectorU64,
                                                   v as *const ::vector::VectorU64)
    }
  }

  /// C++ method: <span style='color: green;'>```const GLuint64& QVector<GLuint64>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 u64 {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_operator_index_const(self as *const ::vector::VectorU64, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```GLuint64& QVector<GLuint64>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut u64 {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_GLuint64_operator_index(self as *mut ::vector::VectorU64, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::operator!=(const QVector<GLuint64>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorU64) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QVector_GLuint64_operator_neq(self as *const ::vector::VectorU64,
                                                    v as *const ::vector::VectorU64)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 u64) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::operator<<(const GLuint64& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorU64) -> &'l0 mut ::vector::VectorU64```<br>
  /// C++ method: <span style='color: green;'>```QVector<GLuint64>& QVector<GLuint64>::operator<<(const QVector<GLuint64>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorU64
    where Args: overloading::VectorU64OpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_pop_back(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_pop_front(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::prepend(const GLuint64& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &u64) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_prepend(self as *mut ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::push_back(const GLuint64& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &u64) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_push_back(self as *mut ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::push_front(const GLuint64& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &u64) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_push_front(self as *mut ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorU64RemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::removeAll(const GLuint64& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &u64) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_removeAll(self as *mut ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_removeAt(self as *mut ::vector::VectorU64, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_removeFirst(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_removeLast(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::removeOne(const GLuint64& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &u64) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_removeOne(self as *mut ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::replace(int i, const GLuint64& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &u64) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_replace(self as *mut ::vector::VectorU64, i, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_reserve(self as *mut ::vector::VectorU64, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_resize(self as *mut ::vector::VectorU64, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<GLuint64>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_size(self as *const ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_squeeze(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<GLuint64>::startsWith(const GLuint64& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &u64) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_startsWith(self as *const ::vector::VectorU64, t as *const u64) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<GLuint64>::swap(QVector<GLuint64>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorU64) {
    unsafe {
      ::ffi::qt_gui_c_QVector_GLuint64_swap(self as *mut ::vector::VectorU64,
                                            other as *mut ::vector::VectorU64)
    }
  }

  /// C++ method: <span style='color: green;'>```GLuint64 QVector<GLuint64>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_takeAt(self as *mut ::vector::VectorU64, i) }
  }

  /// C++ method: <span style='color: green;'>```GLuint64 QVector<GLuint64>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_takeFirst(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```GLuint64 QVector<GLuint64>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_takeLast(self as *mut ::vector::VectorU64) }
  }

  /// C++ method: <span style='color: green;'>```QVector<GLuint64>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> u64```<br>
  /// C++ method: <span style='color: green;'>```GLuint64 QVector<GLuint64>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &u64)) -> u64```<br>
  /// C++ method: <span style='color: green;'>```GLuint64 QVector<GLuint64>::value(int i, const GLuint64& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> u64
    where Args: overloading::VectorU64ValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorU64 {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<GLuint64>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector_GLuint64_destructor(self as *mut ::vector::VectorU64) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::append](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.append) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> ();
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionAppendArgs<'largs> for &'largs ::vector::VectorAbstractTextDocumentLayoutSelection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
    let l = self;
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_append_l(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, l as *const ::vector::VectorAbstractTextDocumentLayoutSelection) }
  }
}
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionAppendArgs<'largs> for &'largs ::abstract_text_document_layout::Selection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
    let t = self;
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_append_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::fill](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.fill) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection;
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionFillArgs<'largs> for &'largs ::abstract_text_document_layout::Selection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_fill_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionFillArgs<'largs> for (&'largs ::abstract_text_document_layout::Selection,::libc::c_int) {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let t = self.0;
let size = self.1;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_fill_t_size(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection, size) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::insert](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.insert) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> ();
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs ::abstract_text_document_layout::Selection) {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_insert_i_n_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i, n, t as *const ::abstract_text_document_layout::Selection) }
  }
}
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionInsertArgs<'largs> for (::libc::c_int,&'largs ::abstract_text_document_layout::Selection) {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
    let i = self.0;
let t = self.1;
    unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_insert_i_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i, t as *const ::abstract_text_document_layout::Selection) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::mid](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.mid) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> ::vector::VectorAbstractTextDocumentLayoutSelection;
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> ::vector::VectorAbstractTextDocumentLayoutSelection {
      let pos = self;
      {
        let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_mid_to_output_pos(original_self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> ::vector::VectorAbstractTextDocumentLayoutSelection {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_mid_to_output_pos_len(original_self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::new](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.new) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionNewArgs {
    fn exec(self) -> ::vector::VectorAbstractTextDocumentLayoutSelection;
  }
  impl VectorAbstractTextDocumentLayoutSelectionNewArgs for () {
    fn exec(self) -> ::vector::VectorAbstractTextDocumentLayoutSelection {

      {
        let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAbstractTextDocumentLayoutSelectionNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAbstractTextDocumentLayoutSelection {
      let size = self;
      {
        let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractTextDocumentLayoutSelectionNewArgs
    for (::libc::c_int, &'a ::abstract_text_document_layout::Selection) {
    fn exec(self) -> ::vector::VectorAbstractTextDocumentLayoutSelection {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_size_t(size, t as *const ::abstract_text_document_layout::Selection, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractTextDocumentLayoutSelectionNewArgs for &'a ::vector::VectorAbstractTextDocumentLayoutSelection {

  fn exec(self, ) -> ::vector::VectorAbstractTextDocumentLayoutSelection {
    let v = self;
    {
let mut object: ::vector::VectorAbstractTextDocumentLayoutSelection = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_constructor_v(v as *const ::vector::VectorAbstractTextDocumentLayoutSelection, &mut object); }object
}
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::op_add_assign](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.op_add_assign) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection;
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionOpAddAssignArgs<'largs> for &'largs ::vector::VectorAbstractTextDocumentLayoutSelection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_assign_l(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, l as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionOpAddAssignArgs<'largs> for &'largs ::abstract_text_document_layout::Selection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_add_assign_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::op_shl](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.op_shl) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection;
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionOpShlArgs<'largs> for &'largs ::vector::VectorAbstractTextDocumentLayoutSelection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_shl_l(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, l as *const ::vector::VectorAbstractTextDocumentLayoutSelection) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionOpShlArgs<'largs> for &'largs ::abstract_text_document_layout::Selection {

  fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_operator_shl_t(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, t as *const ::abstract_text_document_layout::Selection) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::remove](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.remove) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> ();
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_remove_i(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i) }
    }
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractTextDocumentLayoutSelection) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_remove_i_n(original_self as *mut ::vector::VectorAbstractTextDocumentLayoutSelection, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractTextDocumentLayoutSelection::value](../struct.VectorAbstractTextDocumentLayoutSelection.html#method.value) method.
  pub trait VectorAbstractTextDocumentLayoutSelectionValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection>;
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection)
            -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection> {
      let i = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_value_as_ptr_i(original_self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, i) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> VectorAbstractTextDocumentLayoutSelectionValueArgs<'largs> for (::libc::c_int,&'largs ::abstract_text_document_layout::Selection) {

  fn exec(self, original_self: &'largs ::vector::VectorAbstractTextDocumentLayoutSelection) -> ::cpp_utils::CppBox<::abstract_text_document_layout::Selection> {
    let i = self.0;
let default_value = self.1;
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QAbstractTextDocumentLayout_Selection_value_as_ptr_i_defaultValue(original_self as *const ::vector::VectorAbstractTextDocumentLayoutSelection, i, default_value as *const ::abstract_text_document_layout::Selection) };
unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}
  /// This trait represents a set of arguments accepted by [VectorCDouble::append](../struct.VectorCDouble.html#method.append) method.
  pub trait VectorCDoubleAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> ();
  }
  impl<'largs> VectorCDoubleAppendArgs<'largs> for &'largs ::vector::VectorCDouble {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_append_l(original_self as *mut ::vector::VectorCDouble,
                                                l as *const ::vector::VectorCDouble)
      }
    }
  }
  impl<'largs> VectorCDoubleAppendArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_append_t(original_self as *mut ::vector::VectorCDouble,
                                                t as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::count](../struct.VectorCDouble.html#method.count) method.
  pub trait VectorCDoubleCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int;
  }
  impl<'largs> VectorCDoubleCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_double_count_no_args(original_self as *const ::vector::VectorCDouble) }
    }
  }
  impl<'largs> VectorCDoubleCountArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_count_t(original_self as *const ::vector::VectorCDouble,
                                               t as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::fill](../struct.VectorCDouble.html#method.fill) method.
  pub trait VectorCDoubleFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble;
  }
  impl<'largs> VectorCDoubleFillArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_double_fill_t(original_self as *mut ::vector::VectorCDouble,
                                              t as *const ::libc::c_double)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCDoubleFillArgs<'largs> for (&'largs ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_double_fill_t_size(original_self as *mut ::vector::VectorCDouble,
                                                   t as *const ::libc::c_double,
                                                   size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::index_of](../struct.VectorCDouble.html#method.index_of) method.
  pub trait VectorCDoubleIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int;
  }
  impl<'largs> VectorCDoubleIndexOfArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_indexOf_t(original_self as *const ::vector::VectorCDouble,
                                                 t as *const ::libc::c_double)
      }
    }
  }
  impl<'largs> VectorCDoubleIndexOfArgs<'largs> for (&'largs ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_indexOf_t_from(original_self as *const ::vector::VectorCDouble,
                                                      t as *const ::libc::c_double,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::insert](../struct.VectorCDouble.html#method.insert) method.
  pub trait VectorCDoubleInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> ();
  }
  impl<'largs> VectorCDoubleInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_insert_i_n_t(original_self as *mut ::vector::VectorCDouble,
                                                    i,
                                                    n,
                                                    t as *const ::libc::c_double)
      }
    }
  }
  impl<'largs> VectorCDoubleInsertArgs<'largs> for (::libc::c_int, &'largs ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_insert_i_t(original_self as *mut ::vector::VectorCDouble,
                                                  i,
                                                  t as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::last_index_of](../struct.VectorCDouble.html#method.last_index_of) method.
  pub trait VectorCDoubleLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int;
  }
  impl<'largs> VectorCDoubleLastIndexOfArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_lastIndexOf_t(original_self as *const ::vector::VectorCDouble,
                                                     t as *const ::libc::c_double)
      }
    }
  }
  impl<'largs> VectorCDoubleLastIndexOfArgs<'largs> for (&'largs ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_lastIndexOf_t_from(original_self as *const ::vector::VectorCDouble,
                                                          t as *const ::libc::c_double,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::mid](../struct.VectorCDouble.html#method.mid) method.
  pub trait VectorCDoubleMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::vector::VectorCDouble;
  }
  impl<'largs> VectorCDoubleMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::vector::VectorCDouble {
      let pos = self;
      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_mid_to_output_pos(original_self as *const ::vector::VectorCDouble,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorCDoubleMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::vector::VectorCDouble {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_mid_to_output_pos_len(original_self as *const ::vector::VectorCDouble,
                                                               pos,
                                                               len,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::new](../struct.VectorCDouble.html#method.new) method.
  pub trait VectorCDoubleNewArgs {
    fn exec(self) -> ::vector::VectorCDouble;
  }
  impl VectorCDoubleNewArgs for () {
    fn exec(self) -> ::vector::VectorCDouble {

      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorCDoubleNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorCDouble {
      let size = self;
      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCDoubleNewArgs for (::libc::c_int, &'a ::libc::c_double) {
    fn exec(self) -> ::vector::VectorCDouble {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_constructor_size_t(size, t as *const ::libc::c_double, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCDoubleNewArgs for &'a ::vector::VectorCDouble {
    fn exec(self) -> ::vector::VectorCDouble {
      let v = self;
      {
        let mut object: ::vector::VectorCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_double_constructor_v(v as *const ::vector::VectorCDouble, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::op_add_assign](../struct.VectorCDouble.html#method.op_add_assign) method.
  pub trait VectorCDoubleOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble;
  }
  impl<'largs> VectorCDoubleOpAddAssignArgs<'largs> for &'largs ::vector::VectorCDouble {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_double_operator_add_assign_l(original_self as *mut ::vector::VectorCDouble,
                                                               l as *const ::vector::VectorCDouble)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCDoubleOpAddAssignArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_double_operator_add_assign_t(original_self as *mut ::vector::VectorCDouble,
                                                               t as *const ::libc::c_double)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::op_shl](../struct.VectorCDouble.html#method.op_shl) method.
  pub trait VectorCDoubleOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble;
  }
  impl<'largs> VectorCDoubleOpShlArgs<'largs> for &'largs ::vector::VectorCDouble {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_double_operator_shl_l(original_self as *mut ::vector::VectorCDouble,
                                                      l as *const ::vector::VectorCDouble)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCDoubleOpShlArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> &'largs mut ::vector::VectorCDouble {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_double_operator_shl_t(original_self as *mut ::vector::VectorCDouble,
                                                      t as *const ::libc::c_double)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::remove](../struct.VectorCDouble.html#method.remove) method.
  pub trait VectorCDoubleRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> ();
  }
  impl<'largs> VectorCDoubleRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_double_remove_i(original_self as *mut ::vector::VectorCDouble, i) }
    }
  }
  impl<'largs> VectorCDoubleRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCDouble) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_double_remove_i_n(original_self as *mut ::vector::VectorCDouble, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCDouble::value](../struct.VectorCDouble.html#method.value) method.
  pub trait VectorCDoubleValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_double;
  }
  impl<'largs> VectorCDoubleValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_double {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_double_value_i(original_self as *const ::vector::VectorCDouble, i) }
    }
  }
  impl<'largs> VectorCDoubleValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_double) {
    fn exec(self, original_self: &'largs ::vector::VectorCDouble) -> ::libc::c_double {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_double_value_i_defaultValue(original_self as *const ::vector::VectorCDouble,
                                                            i,
                                                            default_value as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::append](../struct.VectorCFloat.html#method.append) method.
  pub trait VectorCFloatAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> ();
  }
  impl<'largs> VectorCFloatAppendArgs<'largs> for &'largs ::vector::VectorCFloat {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_append_l(original_self as *mut ::vector::VectorCFloat,
                                               l as *const ::vector::VectorCFloat)
      }
    }
  }
  impl<'largs> VectorCFloatAppendArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_append_t(original_self as *mut ::vector::VectorCFloat,
                                               t as *const ::libc::c_float)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::count](../struct.VectorCFloat.html#method.count) method.
  pub trait VectorCFloatCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int;
  }
  impl<'largs> VectorCFloatCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_float_count_no_args(original_self as *const ::vector::VectorCFloat) }
    }
  }
  impl<'largs> VectorCFloatCountArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_count_t(original_self as *const ::vector::VectorCFloat,
                                              t as *const ::libc::c_float)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::fill](../struct.VectorCFloat.html#method.fill) method.
  pub trait VectorCFloatFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat;
  }
  impl<'largs> VectorCFloatFillArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_float_fill_t(original_self as *mut ::vector::VectorCFloat,
                                             t as *const ::libc::c_float)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCFloatFillArgs<'largs> for (&'largs ::libc::c_float, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_float_fill_t_size(original_self as *mut ::vector::VectorCFloat,
                                                  t as *const ::libc::c_float,
                                                  size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::index_of](../struct.VectorCFloat.html#method.index_of) method.
  pub trait VectorCFloatIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int;
  }
  impl<'largs> VectorCFloatIndexOfArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_indexOf_t(original_self as *const ::vector::VectorCFloat,
                                                t as *const ::libc::c_float)
      }
    }
  }
  impl<'largs> VectorCFloatIndexOfArgs<'largs> for (&'largs ::libc::c_float, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_indexOf_t_from(original_self as *const ::vector::VectorCFloat,
                                                     t as *const ::libc::c_float,
                                                     from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::insert](../struct.VectorCFloat.html#method.insert) method.
  pub trait VectorCFloatInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> ();
  }
  impl<'largs> VectorCFloatInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_insert_i_n_t(original_self as *mut ::vector::VectorCFloat,
                                                   i,
                                                   n,
                                                   t as *const ::libc::c_float)
      }
    }
  }
  impl<'largs> VectorCFloatInsertArgs<'largs> for (::libc::c_int, &'largs ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_insert_i_t(original_self as *mut ::vector::VectorCFloat,
                                                 i,
                                                 t as *const ::libc::c_float)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::last_index_of](../struct.VectorCFloat.html#method.last_index_of) method.
  pub trait VectorCFloatLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int;
  }
  impl<'largs> VectorCFloatLastIndexOfArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_lastIndexOf_t(original_self as *const ::vector::VectorCFloat,
                                                    t as *const ::libc::c_float)
      }
    }
  }
  impl<'largs> VectorCFloatLastIndexOfArgs<'largs> for (&'largs ::libc::c_float, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_lastIndexOf_t_from(original_self as *const ::vector::VectorCFloat,
                                                         t as *const ::libc::c_float,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::mid](../struct.VectorCFloat.html#method.mid) method.
  pub trait VectorCFloatMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::vector::VectorCFloat;
  }
  impl<'largs> VectorCFloatMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::vector::VectorCFloat {
      let pos = self;
      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_mid_to_output_pos(original_self as *const ::vector::VectorCFloat,
                                                          pos,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorCFloatMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::vector::VectorCFloat {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_mid_to_output_pos_len(original_self as *const ::vector::VectorCFloat,
                                                              pos,
                                                              len,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::new](../struct.VectorCFloat.html#method.new) method.
  pub trait VectorCFloatNewArgs {
    fn exec(self) -> ::vector::VectorCFloat;
  }
  impl VectorCFloatNewArgs for () {
    fn exec(self) -> ::vector::VectorCFloat {

      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorCFloatNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorCFloat {
      let size = self;
      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCFloatNewArgs for (::libc::c_int, &'a ::libc::c_float) {
    fn exec(self) -> ::vector::VectorCFloat {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_constructor_size_t(size, t as *const ::libc::c_float, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCFloatNewArgs for &'a ::vector::VectorCFloat {
    fn exec(self) -> ::vector::VectorCFloat {
      let v = self;
      {
        let mut object: ::vector::VectorCFloat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_float_constructor_v(v as *const ::vector::VectorCFloat, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::op_add_assign](../struct.VectorCFloat.html#method.op_add_assign) method.
  pub trait VectorCFloatOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat;
  }
  impl<'largs> VectorCFloatOpAddAssignArgs<'largs> for &'largs ::vector::VectorCFloat {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_float_operator_add_assign_l(original_self as *mut ::vector::VectorCFloat,
                                                              l as *const ::vector::VectorCFloat)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCFloatOpAddAssignArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_float_operator_add_assign_t(original_self as *mut ::vector::VectorCFloat,
                                                              t as *const ::libc::c_float)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::op_shl](../struct.VectorCFloat.html#method.op_shl) method.
  pub trait VectorCFloatOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat;
  }
  impl<'largs> VectorCFloatOpShlArgs<'largs> for &'largs ::vector::VectorCFloat {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_float_operator_shl_l(original_self as *mut ::vector::VectorCFloat,
                                                     l as *const ::vector::VectorCFloat)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCFloatOpShlArgs<'largs> for &'largs ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> &'largs mut ::vector::VectorCFloat {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_float_operator_shl_t(original_self as *mut ::vector::VectorCFloat,
                                                     t as *const ::libc::c_float)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::remove](../struct.VectorCFloat.html#method.remove) method.
  pub trait VectorCFloatRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> ();
  }
  impl<'largs> VectorCFloatRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_float_remove_i(original_self as *mut ::vector::VectorCFloat, i) }
    }
  }
  impl<'largs> VectorCFloatRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCFloat) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_float_remove_i_n(original_self as *mut ::vector::VectorCFloat, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCFloat::value](../struct.VectorCFloat.html#method.value) method.
  pub trait VectorCFloatValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_float;
  }
  impl<'largs> VectorCFloatValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_float {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_float_value_i(original_self as *const ::vector::VectorCFloat, i) }
    }
  }
  impl<'largs> VectorCFloatValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_float) {
    fn exec(self, original_self: &'largs ::vector::VectorCFloat) -> ::libc::c_float {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_float_value_i_defaultValue(original_self as *const ::vector::VectorCFloat,
                                                           i,
                                                           default_value as *const ::libc::c_float)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::append](../struct.VectorPairPairCDoubleColor.html#method.append) method.
  pub trait VectorPairPairCDoubleColorAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> ();
  }
  impl<'largs> VectorPairPairCDoubleColorAppendArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleColor {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_append_l(original_self as *mut ::vector::VectorPairPairCDoubleColor, l as *const ::vector::VectorPairPairCDoubleColor)
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorAppendArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_append_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::count](../struct.VectorPairPairCDoubleColor.html#method.count) method.
  pub trait VectorPairPairCDoubleColorCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleColorCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_count_no_args(original_self as *const ::vector::VectorPairPairCDoubleColor) }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorCountArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_count_t(original_self as *const ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::fill](../struct.VectorPairPairCDoubleColor.html#method.fill) method.
  pub trait VectorPairPairCDoubleColorFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor;
  }
  impl<'largs> VectorPairPairCDoubleColorFillArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_fill_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleColorFillArgs<'largs> for (&'largs ::pair::PairCDoubleColor, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_fill_t_size(original_self as *mut ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::index_of](../struct.VectorPairPairCDoubleColor.html#method.index_of) method.
  pub trait VectorPairPairCDoubleColorIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleColorIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_indexOf_t(original_self as *const ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor) }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorIndexOfArgs<'largs> for (&'largs ::pair::PairCDoubleColor, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_indexOf_t_from(original_self as *const ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::insert](../struct.VectorPairPairCDoubleColor.html#method.insert) method.
  pub trait VectorPairPairCDoubleColorInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> ();
  }
  impl<'largs> VectorPairPairCDoubleColorInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::pair::PairCDoubleColor) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_insert_i_n_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, i, n, t as *const ::pair::PairCDoubleColor) }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorInsertArgs<'largs> for (::libc::c_int, &'largs ::pair::PairCDoubleColor) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_insert_i_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, i, t as *const ::pair::PairCDoubleColor) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::last_index_of](../struct.VectorPairPairCDoubleColor.html#method.last_index_of) method.
  pub trait VectorPairPairCDoubleColorLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int;
  }
  impl<'largs> VectorPairPairCDoubleColorLastIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_lastIndexOf_t(original_self as *const ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor) }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorLastIndexOfArgs<'largs> for (&'largs ::pair::PairCDoubleColor, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_lastIndexOf_t_from(original_self as *const ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::mid](../struct.VectorPairPairCDoubleColor.html#method.mid) method.
  pub trait VectorPairPairCDoubleColorMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::vector::VectorPairPairCDoubleColor;
  }
  impl<'largs> VectorPairPairCDoubleColorMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::vector::VectorPairPairCDoubleColor {
      let pos = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_mid_to_output_pos(original_self as *const ::vector::VectorPairPairCDoubleColor, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::vector::VectorPairPairCDoubleColor {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_mid_to_output_pos_len(original_self as *const ::vector::VectorPairPairCDoubleColor, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::new](../struct.VectorPairPairCDoubleColor.html#method.new) method.
  pub trait VectorPairPairCDoubleColorNewArgs {
    fn exec(self) -> ::vector::VectorPairPairCDoubleColor;
  }
  impl VectorPairPairCDoubleColorNewArgs for () {
    fn exec(self) -> ::vector::VectorPairPairCDoubleColor {

      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorPairPairCDoubleColorNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorPairPairCDoubleColor {
      let size = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPairPairCDoubleColorNewArgs for (::libc::c_int, &'a ::pair::PairCDoubleColor) {
    fn exec(self) -> ::vector::VectorPairPairCDoubleColor {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_constructor_size_t(size,
                                                                         t as *const ::pair::PairCDoubleColor,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorPairPairCDoubleColorNewArgs for &'a ::vector::VectorPairPairCDoubleColor {
    fn exec(self) -> ::vector::VectorPairPairCDoubleColor {
      let v = self;
      {
        let mut object: ::vector::VectorPairPairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_constructor_v(v as *const ::vector::VectorPairPairCDoubleColor,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::op_add_assign](../struct.VectorPairPairCDoubleColor.html#method.op_add_assign) method.
  pub trait VectorPairPairCDoubleColorOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor;
  }
  impl<'largs> VectorPairPairCDoubleColorOpAddAssignArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleColor {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_add_assign_l(original_self as *mut ::vector::VectorPairPairCDoubleColor, l as *const ::vector::VectorPairPairCDoubleColor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleColorOpAddAssignArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_add_assign_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::op_shl](../struct.VectorPairPairCDoubleColor.html#method.op_shl) method.
  pub trait VectorPairPairCDoubleColorOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor;
  }
  impl<'largs> VectorPairPairCDoubleColorOpShlArgs<'largs> for &'largs ::vector::VectorPairPairCDoubleColor {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_shl_l(original_self as *mut ::vector::VectorPairPairCDoubleColor, l as *const ::vector::VectorPairPairCDoubleColor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorPairPairCDoubleColorOpShlArgs<'largs> for &'largs ::pair::PairCDoubleColor {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorPairPairCDoubleColor)
            -> &'largs mut ::vector::VectorPairPairCDoubleColor {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_operator_shl_t(original_self as *mut ::vector::VectorPairPairCDoubleColor, t as *const ::pair::PairCDoubleColor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::remove](../struct.VectorPairPairCDoubleColor.html#method.remove) method.
  pub trait VectorPairPairCDoubleColorRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> ();
  }
  impl<'largs> VectorPairPairCDoubleColorRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPair_double_QColor_remove_i(original_self as *mut ::vector::VectorPairPairCDoubleColor, i)
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorPairPairCDoubleColor) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QPair_double_QColor_remove_i_n(original_self as *mut ::vector::VectorPairPairCDoubleColor, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorPairPairCDoubleColor::value](../struct.VectorPairPairCDoubleColor.html#method.value) method.
  pub trait VectorPairPairCDoubleColorValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::pair::PairCDoubleColor;
  }
  impl<'largs> VectorPairPairCDoubleColorValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::pair::PairCDoubleColor {
      let i = self;
      {
        let mut object: ::pair::PairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_value_to_output_i(original_self as *const ::vector::VectorPairPairCDoubleColor, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorPairPairCDoubleColorValueArgs<'largs> for (::libc::c_int, &'largs ::pair::PairCDoubleColor) {
    fn exec(self, original_self: &'largs ::vector::VectorPairPairCDoubleColor) -> ::pair::PairCDoubleColor {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::pair::PairCDoubleColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPair_double_QColor_value_to_output_i_defaultValue(original_self as *const ::vector::VectorPairPairCDoubleColor, i, default_value as *const ::pair::PairCDoubleColor, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::append](../struct.VectorQtCoreLine.html#method.append) method.
  pub trait VectorQtCoreLineAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> ();
  }
  impl<'largs> VectorQtCoreLineAppendArgs<'largs> for &'largs ::vector::VectorQtCoreLine {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_append_l(original_self as *mut ::vector::VectorQtCoreLine,
                                               l as *const ::vector::VectorQtCoreLine)
      }
    }
  }
  impl<'largs> VectorQtCoreLineAppendArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_append_t(original_self as *mut ::vector::VectorQtCoreLine,
                                               t as *const ::qt_core::line::Line)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::count](../struct.VectorQtCoreLine.html#method.count) method.
  pub trait VectorQtCoreLineCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QLine_count_no_args(original_self as *const ::vector::VectorQtCoreLine) }
    }
  }
  impl<'largs> VectorQtCoreLineCountArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_count_t(original_self as *const ::vector::VectorQtCoreLine,
                                              t as *const ::qt_core::line::Line)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::append](../struct.VectorQtCoreLineF.html#method.append) method.
  pub trait VectorQtCoreLineFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> ();
  }
  impl<'largs> VectorQtCoreLineFAppendArgs<'largs> for &'largs ::vector::VectorQtCoreLineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_append_l(original_self as *mut ::vector::VectorQtCoreLineF,
                                                l as *const ::vector::VectorQtCoreLineF)
      }
    }
  }
  impl<'largs> VectorQtCoreLineFAppendArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_append_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                                t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::count](../struct.VectorQtCoreLineF.html#method.count) method.
  pub trait VectorQtCoreLineFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QLineF_count_no_args(original_self as *const ::vector::VectorQtCoreLineF) }
    }
  }
  impl<'largs> VectorQtCoreLineFCountArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_count_t(original_self as *const ::vector::VectorQtCoreLineF,
                                               t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::fill](../struct.VectorQtCoreLineF.html#method.fill) method.
  pub trait VectorQtCoreLineFFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF;
  }
  impl<'largs> VectorQtCoreLineFFillArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_fill_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                              t as *const ::qt_core::line_f::LineF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineFFillArgs<'largs> for (&'largs ::qt_core::line_f::LineF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_fill_t_size(original_self as *mut ::vector::VectorQtCoreLineF,
                                                   t as *const ::qt_core::line_f::LineF,
                                                   size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::index_of](../struct.VectorQtCoreLineF.html#method.index_of) method.
  pub trait VectorQtCoreLineFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineFIndexOfArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_indexOf_t(original_self as *const ::vector::VectorQtCoreLineF,
                                                 t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> VectorQtCoreLineFIndexOfArgs<'largs> for (&'largs ::qt_core::line_f::LineF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_indexOf_t_from(original_self as *const ::vector::VectorQtCoreLineF,
                                                      t as *const ::qt_core::line_f::LineF,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::insert](../struct.VectorQtCoreLineF.html#method.insert) method.
  pub trait VectorQtCoreLineFInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> ();
  }
  impl<'largs> VectorQtCoreLineFInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::line_f::LineF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_insert_i_n_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                                    i,
                                                    n,
                                                    t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> VectorQtCoreLineFInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::line_f::LineF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_insert_i_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                                  i,
                                                  t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::last_index_of](../struct.VectorQtCoreLineF.html#method.last_index_of) method.
  pub trait VectorQtCoreLineFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineFLastIndexOfArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_lastIndexOf_t(original_self as *const ::vector::VectorQtCoreLineF,
                                                     t as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> VectorQtCoreLineFLastIndexOfArgs<'largs> for (&'largs ::qt_core::line_f::LineF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLineF_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCoreLineF,
                                                          t as *const ::qt_core::line_f::LineF,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::mid](../struct.VectorQtCoreLineF.html#method.mid) method.
  pub trait VectorQtCoreLineFMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::vector::VectorQtCoreLineF;
  }
  impl<'largs> VectorQtCoreLineFMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::vector::VectorQtCoreLineF {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_mid_to_output_pos(original_self as *const ::vector::VectorQtCoreLineF,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreLineFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::vector::VectorQtCoreLineF {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCoreLineF,
                                                               pos,
                                                               len,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::new](../struct.VectorQtCoreLineF.html#method.new) method.
  pub trait VectorQtCoreLineFNewArgs {
    fn exec(self) -> ::vector::VectorQtCoreLineF;
  }
  impl VectorQtCoreLineFNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCoreLineF {

      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCoreLineFNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCoreLineF {
      let size = self;
      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreLineFNewArgs for (::libc::c_int, &'a ::qt_core::line_f::LineF) {
    fn exec(self) -> ::vector::VectorQtCoreLineF {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_constructor_size_t(size, t as *const ::qt_core::line_f::LineF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreLineFNewArgs for &'a ::vector::VectorQtCoreLineF {
    fn exec(self) -> ::vector::VectorQtCoreLineF {
      let v = self;
      {
        let mut object: ::vector::VectorQtCoreLineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_constructor_v(v as *const ::vector::VectorQtCoreLineF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::op_add_assign](../struct.VectorQtCoreLineF.html#method.op_add_assign) method.
  pub trait VectorQtCoreLineFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF;
  }
  impl<'largs> VectorQtCoreLineFOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCoreLineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_operator_add_assign_l(original_self as *mut ::vector::VectorQtCoreLineF,
                                                               l as *const ::vector::VectorQtCoreLineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineFOpAddAssignArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_operator_add_assign_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                                               t as *const ::qt_core::line_f::LineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::op_shl](../struct.VectorQtCoreLineF.html#method.op_shl) method.
  pub trait VectorQtCoreLineFOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF;
  }
  impl<'largs> VectorQtCoreLineFOpShlArgs<'largs> for &'largs ::vector::VectorQtCoreLineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_operator_shl_l(original_self as *mut ::vector::VectorQtCoreLineF,
                                                        l as *const ::vector::VectorQtCoreLineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineFOpShlArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> &'largs mut ::vector::VectorQtCoreLineF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_operator_shl_t(original_self as *mut ::vector::VectorQtCoreLineF,
                                                        t as *const ::qt_core::line_f::LineF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::remove](../struct.VectorQtCoreLineF.html#method.remove) method.
  pub trait VectorQtCoreLineFRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> ();
  }
  impl<'largs> VectorQtCoreLineFRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QLineF_remove_i(original_self as *mut ::vector::VectorQtCoreLineF, i) }
    }
  }
  impl<'largs> VectorQtCoreLineFRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLineF) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QLineF_remove_i_n(original_self as *mut ::vector::VectorQtCoreLineF, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLineF::value](../struct.VectorQtCoreLineF.html#method.value) method.
  pub trait VectorQtCoreLineFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::qt_core::line_f::LineF;
  }
  impl<'largs> VectorQtCoreLineFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::qt_core::line_f::LineF {
      let i = self;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_value_to_output_i(original_self as *const ::vector::VectorQtCoreLineF,
                                                           i,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreLineFValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::line_f::LineF) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLineF) -> ::qt_core::line_f::LineF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLineF_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCoreLineF, i, default_value as *const ::qt_core::line_f::LineF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::fill](../struct.VectorQtCoreLine.html#method.fill) method.
  pub trait VectorQtCoreLineFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine;
  }
  impl<'largs> VectorQtCoreLineFillArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLine_fill_t(original_self as *mut ::vector::VectorQtCoreLine,
                                             t as *const ::qt_core::line::Line)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineFillArgs<'largs> for (&'largs ::qt_core::line::Line, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLine_fill_t_size(original_self as *mut ::vector::VectorQtCoreLine,
                                                  t as *const ::qt_core::line::Line,
                                                  size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::index_of](../struct.VectorQtCoreLine.html#method.index_of) method.
  pub trait VectorQtCoreLineIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineIndexOfArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_indexOf_t(original_self as *const ::vector::VectorQtCoreLine,
                                                t as *const ::qt_core::line::Line)
      }
    }
  }
  impl<'largs> VectorQtCoreLineIndexOfArgs<'largs> for (&'largs ::qt_core::line::Line, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_indexOf_t_from(original_self as *const ::vector::VectorQtCoreLine,
                                                     t as *const ::qt_core::line::Line,
                                                     from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::insert](../struct.VectorQtCoreLine.html#method.insert) method.
  pub trait VectorQtCoreLineInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> ();
  }
  impl<'largs> VectorQtCoreLineInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::line::Line) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_insert_i_n_t(original_self as *mut ::vector::VectorQtCoreLine,
                                                   i,
                                                   n,
                                                   t as *const ::qt_core::line::Line)
      }
    }
  }
  impl<'largs> VectorQtCoreLineInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::line::Line) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_insert_i_t(original_self as *mut ::vector::VectorQtCoreLine,
                                                 i,
                                                 t as *const ::qt_core::line::Line)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::last_index_of](../struct.VectorQtCoreLine.html#method.last_index_of) method.
  pub trait VectorQtCoreLineLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreLineLastIndexOfArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_lastIndexOf_t(original_self as *const ::vector::VectorQtCoreLine,
                                                    t as *const ::qt_core::line::Line)
      }
    }
  }
  impl<'largs> VectorQtCoreLineLastIndexOfArgs<'largs> for (&'largs ::qt_core::line::Line, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QLine_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCoreLine,
                                                         t as *const ::qt_core::line::Line,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::mid](../struct.VectorQtCoreLine.html#method.mid) method.
  pub trait VectorQtCoreLineMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::vector::VectorQtCoreLine;
  }
  impl<'largs> VectorQtCoreLineMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::vector::VectorQtCoreLine {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_mid_to_output_pos(original_self as *const ::vector::VectorQtCoreLine,
                                                          pos,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreLineMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::vector::VectorQtCoreLine {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCoreLine,
                                                              pos,
                                                              len,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::new](../struct.VectorQtCoreLine.html#method.new) method.
  pub trait VectorQtCoreLineNewArgs {
    fn exec(self) -> ::vector::VectorQtCoreLine;
  }
  impl VectorQtCoreLineNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCoreLine {

      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCoreLineNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCoreLine {
      let size = self;
      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreLineNewArgs for (::libc::c_int, &'a ::qt_core::line::Line) {
    fn exec(self) -> ::vector::VectorQtCoreLine {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_constructor_size_t(size, t as *const ::qt_core::line::Line, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreLineNewArgs for &'a ::vector::VectorQtCoreLine {
    fn exec(self) -> ::vector::VectorQtCoreLine {
      let v = self;
      {
        let mut object: ::vector::VectorQtCoreLine =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_constructor_v(v as *const ::vector::VectorQtCoreLine, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::op_add_assign](../struct.VectorQtCoreLine.html#method.op_add_assign) method.
  pub trait VectorQtCoreLineOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine;
  }
  impl<'largs> VectorQtCoreLineOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCoreLine {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_operator_add_assign_l(original_self as *mut ::vector::VectorQtCoreLine,
                                                              l as *const ::vector::VectorQtCoreLine)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineOpAddAssignArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_operator_add_assign_t(original_self as *mut ::vector::VectorQtCoreLine,
                                                              t as *const ::qt_core::line::Line)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::op_shl](../struct.VectorQtCoreLine.html#method.op_shl) method.
  pub trait VectorQtCoreLineOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine;
  }
  impl<'largs> VectorQtCoreLineOpShlArgs<'largs> for &'largs ::vector::VectorQtCoreLine {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLine_operator_shl_l(original_self as *mut ::vector::VectorQtCoreLine,
                                                     l as *const ::vector::VectorQtCoreLine)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreLineOpShlArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> &'largs mut ::vector::VectorQtCoreLine {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QLine_operator_shl_t(original_self as *mut ::vector::VectorQtCoreLine,
                                                     t as *const ::qt_core::line::Line)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::remove](../struct.VectorQtCoreLine.html#method.remove) method.
  pub trait VectorQtCoreLineRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> ();
  }
  impl<'largs> VectorQtCoreLineRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QLine_remove_i(original_self as *mut ::vector::VectorQtCoreLine, i) }
    }
  }
  impl<'largs> VectorQtCoreLineRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreLine) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QLine_remove_i_n(original_self as *mut ::vector::VectorQtCoreLine, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreLine::value](../struct.VectorQtCoreLine.html#method.value) method.
  pub trait VectorQtCoreLineValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::qt_core::line::Line;
  }
  impl<'largs> VectorQtCoreLineValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::qt_core::line::Line {
      let i = self;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_value_to_output_i(original_self as *const ::vector::VectorQtCoreLine,
                                                          i,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreLineValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::line::Line) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreLine) -> ::qt_core::line::Line {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::line::Line =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QLine_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCoreLine, i, default_value as *const ::qt_core::line::Line, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::append](../struct.VectorQtCorePoint.html#method.append) method.
  pub trait VectorQtCorePointAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> ();
  }
  impl<'largs> VectorQtCorePointAppendArgs<'largs> for &'largs ::vector::VectorQtCorePoint {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_append_l(original_self as *mut ::vector::VectorQtCorePoint,
                                                l as *const ::vector::VectorQtCorePoint)
      }
    }
  }
  impl<'largs> VectorQtCorePointAppendArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_append_t(original_self as *mut ::vector::VectorQtCorePoint,
                                                t as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::count](../struct.VectorQtCorePoint.html#method.count) method.
  pub trait VectorQtCorePointCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCorePointCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QPoint_count_no_args(original_self as *const ::vector::VectorQtCorePoint) }
    }
  }
  impl<'largs> VectorQtCorePointCountArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_count_t(original_self as *const ::vector::VectorQtCorePoint,
                                               t as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::fill](../struct.VectorQtCorePoint.html#method.fill) method.
  pub trait VectorQtCorePointFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint;
  }
  impl<'largs> VectorQtCorePointFillArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_fill_t(original_self as *mut ::vector::VectorQtCorePoint,
                                              t as *const ::qt_core::point::Point)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCorePointFillArgs<'largs> for (&'largs ::qt_core::point::Point, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_fill_t_size(original_self as *mut ::vector::VectorQtCorePoint,
                                                   t as *const ::qt_core::point::Point,
                                                   size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::index_of](../struct.VectorQtCorePoint.html#method.index_of) method.
  pub trait VectorQtCorePointIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCorePointIndexOfArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_indexOf_t(original_self as *const ::vector::VectorQtCorePoint,
                                                 t as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> VectorQtCorePointIndexOfArgs<'largs> for (&'largs ::qt_core::point::Point, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_indexOf_t_from(original_self as *const ::vector::VectorQtCorePoint,
                                                      t as *const ::qt_core::point::Point,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::insert](../struct.VectorQtCorePoint.html#method.insert) method.
  pub trait VectorQtCorePointInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> ();
  }
  impl<'largs> VectorQtCorePointInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_insert_i_n_t(original_self as *mut ::vector::VectorQtCorePoint,
                                                    i,
                                                    n,
                                                    t as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> VectorQtCorePointInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_insert_i_t(original_self as *mut ::vector::VectorQtCorePoint,
                                                  i,
                                                  t as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::last_index_of](../struct.VectorQtCorePoint.html#method.last_index_of) method.
  pub trait VectorQtCorePointLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCorePointLastIndexOfArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_lastIndexOf_t(original_self as *const ::vector::VectorQtCorePoint,
                                                     t as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> VectorQtCorePointLastIndexOfArgs<'largs> for (&'largs ::qt_core::point::Point, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QPoint_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCorePoint,
                                                          t as *const ::qt_core::point::Point,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::mid](../struct.VectorQtCorePoint.html#method.mid) method.
  pub trait VectorQtCorePointMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::vector::VectorQtCorePoint;
  }
  impl<'largs> VectorQtCorePointMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::vector::VectorQtCorePoint {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_mid_to_output_pos(original_self as *const ::vector::VectorQtCorePoint,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCorePointMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::vector::VectorQtCorePoint {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCorePoint,
                                                               pos,
                                                               len,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::new](../struct.VectorQtCorePoint.html#method.new) method.
  pub trait VectorQtCorePointNewArgs {
    fn exec(self) -> ::vector::VectorQtCorePoint;
  }
  impl VectorQtCorePointNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCorePoint {

      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCorePointNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCorePoint {
      let size = self;
      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCorePointNewArgs for (::libc::c_int, &'a ::qt_core::point::Point) {
    fn exec(self) -> ::vector::VectorQtCorePoint {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_constructor_size_t(size, t as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCorePointNewArgs for &'a ::vector::VectorQtCorePoint {
    fn exec(self) -> ::vector::VectorQtCorePoint {
      let v = self;
      {
        let mut object: ::vector::VectorQtCorePoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_constructor_v(v as *const ::vector::VectorQtCorePoint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::op_add_assign](../struct.VectorQtCorePoint.html#method.op_add_assign) method.
  pub trait VectorQtCorePointOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint;
  }
  impl<'largs> VectorQtCorePointOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCorePoint {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_operator_add_assign_l(original_self as *mut ::vector::VectorQtCorePoint,
                                                               l as *const ::vector::VectorQtCorePoint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCorePointOpAddAssignArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_operator_add_assign_t(original_self as *mut ::vector::VectorQtCorePoint,
                                                               t as *const ::qt_core::point::Point)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::op_shl](../struct.VectorQtCorePoint.html#method.op_shl) method.
  pub trait VectorQtCorePointOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint;
  }
  impl<'largs> VectorQtCorePointOpShlArgs<'largs> for &'largs ::vector::VectorQtCorePoint {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_operator_shl_l(original_self as *mut ::vector::VectorQtCorePoint,
                                                        l as *const ::vector::VectorQtCorePoint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCorePointOpShlArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> &'largs mut ::vector::VectorQtCorePoint {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_operator_shl_t(original_self as *mut ::vector::VectorQtCorePoint,
                                                        t as *const ::qt_core::point::Point)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::remove](../struct.VectorQtCorePoint.html#method.remove) method.
  pub trait VectorQtCorePointRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> ();
  }
  impl<'largs> VectorQtCorePointRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QPoint_remove_i(original_self as *mut ::vector::VectorQtCorePoint, i) }
    }
  }
  impl<'largs> VectorQtCorePointRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCorePoint) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QPoint_remove_i_n(original_self as *mut ::vector::VectorQtCorePoint, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCorePoint::value](../struct.VectorQtCorePoint.html#method.value) method.
  pub trait VectorQtCorePointValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::qt_core::point::Point;
  }
  impl<'largs> VectorQtCorePointValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::qt_core::point::Point {
      let i = self;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_value_to_output_i(original_self as *const ::vector::VectorQtCorePoint,
                                                           i,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCorePointValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCorePoint) -> ::qt_core::point::Point {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QPoint_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCorePoint, i, default_value as *const ::qt_core::point::Point, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::append](../struct.VectorQtCoreRect.html#method.append) method.
  pub trait VectorQtCoreRectAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> ();
  }
  impl<'largs> VectorQtCoreRectAppendArgs<'largs> for &'largs ::vector::VectorQtCoreRect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_append_l(original_self as *mut ::vector::VectorQtCoreRect,
                                               l as *const ::vector::VectorQtCoreRect)
      }
    }
  }
  impl<'largs> VectorQtCoreRectAppendArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_append_t(original_self as *mut ::vector::VectorQtCoreRect,
                                               t as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::count](../struct.VectorQtCoreRect.html#method.count) method.
  pub trait VectorQtCoreRectCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QRect_count_no_args(original_self as *const ::vector::VectorQtCoreRect) }
    }
  }
  impl<'largs> VectorQtCoreRectCountArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_count_t(original_self as *const ::vector::VectorQtCoreRect,
                                              t as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::append](../struct.VectorQtCoreRectF.html#method.append) method.
  pub trait VectorQtCoreRectFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> ();
  }
  impl<'largs> VectorQtCoreRectFAppendArgs<'largs> for &'largs ::vector::VectorQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_append_l(original_self as *mut ::vector::VectorQtCoreRectF,
                                                l as *const ::vector::VectorQtCoreRectF)
      }
    }
  }
  impl<'largs> VectorQtCoreRectFAppendArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_append_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                                t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::count](../struct.VectorQtCoreRectF.html#method.count) method.
  pub trait VectorQtCoreRectFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QRectF_count_no_args(original_self as *const ::vector::VectorQtCoreRectF) }
    }
  }
  impl<'largs> VectorQtCoreRectFCountArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_count_t(original_self as *const ::vector::VectorQtCoreRectF,
                                               t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::fill](../struct.VectorQtCoreRectF.html#method.fill) method.
  pub trait VectorQtCoreRectFFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF;
  }
  impl<'largs> VectorQtCoreRectFFillArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_fill_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                              t as *const ::qt_core::rect_f::RectF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectFFillArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_fill_t_size(original_self as *mut ::vector::VectorQtCoreRectF,
                                                   t as *const ::qt_core::rect_f::RectF,
                                                   size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::index_of](../struct.VectorQtCoreRectF.html#method.index_of) method.
  pub trait VectorQtCoreRectFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectFIndexOfArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_indexOf_t(original_self as *const ::vector::VectorQtCoreRectF,
                                                 t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> VectorQtCoreRectFIndexOfArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_indexOf_t_from(original_self as *const ::vector::VectorQtCoreRectF,
                                                      t as *const ::qt_core::rect_f::RectF,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::insert](../struct.VectorQtCoreRectF.html#method.insert) method.
  pub trait VectorQtCoreRectFInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> ();
  }
  impl<'largs> VectorQtCoreRectFInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_insert_i_n_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                                    i,
                                                    n,
                                                    t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> VectorQtCoreRectFInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_insert_i_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                                  i,
                                                  t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::last_index_of](../struct.VectorQtCoreRectF.html#method.last_index_of) method.
  pub trait VectorQtCoreRectFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectFLastIndexOfArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_lastIndexOf_t(original_self as *const ::vector::VectorQtCoreRectF,
                                                     t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> VectorQtCoreRectFLastIndexOfArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRectF_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCoreRectF,
                                                          t as *const ::qt_core::rect_f::RectF,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::mid](../struct.VectorQtCoreRectF.html#method.mid) method.
  pub trait VectorQtCoreRectFMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::vector::VectorQtCoreRectF;
  }
  impl<'largs> VectorQtCoreRectFMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::vector::VectorQtCoreRectF {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_mid_to_output_pos(original_self as *const ::vector::VectorQtCoreRectF,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreRectFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::vector::VectorQtCoreRectF {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCoreRectF,
                                                               pos,
                                                               len,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::new](../struct.VectorQtCoreRectF.html#method.new) method.
  pub trait VectorQtCoreRectFNewArgs {
    fn exec(self) -> ::vector::VectorQtCoreRectF;
  }
  impl VectorQtCoreRectFNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCoreRectF {

      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCoreRectFNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCoreRectF {
      let size = self;
      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreRectFNewArgs for (::libc::c_int, &'a ::qt_core::rect_f::RectF) {
    fn exec(self) -> ::vector::VectorQtCoreRectF {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_constructor_size_t(size, t as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreRectFNewArgs for &'a ::vector::VectorQtCoreRectF {
    fn exec(self) -> ::vector::VectorQtCoreRectF {
      let v = self;
      {
        let mut object: ::vector::VectorQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_constructor_v(v as *const ::vector::VectorQtCoreRectF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::op_add_assign](../struct.VectorQtCoreRectF.html#method.op_add_assign) method.
  pub trait VectorQtCoreRectFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF;
  }
  impl<'largs> VectorQtCoreRectFOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_operator_add_assign_l(original_self as *mut ::vector::VectorQtCoreRectF,
                                                               l as *const ::vector::VectorQtCoreRectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectFOpAddAssignArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_operator_add_assign_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                                               t as *const ::qt_core::rect_f::RectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::op_shl](../struct.VectorQtCoreRectF.html#method.op_shl) method.
  pub trait VectorQtCoreRectFOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF;
  }
  impl<'largs> VectorQtCoreRectFOpShlArgs<'largs> for &'largs ::vector::VectorQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_operator_shl_l(original_self as *mut ::vector::VectorQtCoreRectF,
                                                        l as *const ::vector::VectorQtCoreRectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectFOpShlArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> &'largs mut ::vector::VectorQtCoreRectF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_operator_shl_t(original_self as *mut ::vector::VectorQtCoreRectF,
                                                        t as *const ::qt_core::rect_f::RectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::remove](../struct.VectorQtCoreRectF.html#method.remove) method.
  pub trait VectorQtCoreRectFRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> ();
  }
  impl<'largs> VectorQtCoreRectFRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QRectF_remove_i(original_self as *mut ::vector::VectorQtCoreRectF, i) }
    }
  }
  impl<'largs> VectorQtCoreRectFRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRectF) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QRectF_remove_i_n(original_self as *mut ::vector::VectorQtCoreRectF, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRectF::value](../struct.VectorQtCoreRectF.html#method.value) method.
  pub trait VectorQtCoreRectFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> VectorQtCoreRectFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::qt_core::rect_f::RectF {
      let i = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_value_to_output_i(original_self as *const ::vector::VectorQtCoreRectF,
                                                           i,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreRectFValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRectF) -> ::qt_core::rect_f::RectF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRectF_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCoreRectF, i, default_value as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::fill](../struct.VectorQtCoreRect.html#method.fill) method.
  pub trait VectorQtCoreRectFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect;
  }
  impl<'largs> VectorQtCoreRectFillArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRect_fill_t(original_self as *mut ::vector::VectorQtCoreRect,
                                             t as *const ::qt_core::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectFillArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRect_fill_t_size(original_self as *mut ::vector::VectorQtCoreRect,
                                                  t as *const ::qt_core::rect::Rect,
                                                  size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::index_of](../struct.VectorQtCoreRect.html#method.index_of) method.
  pub trait VectorQtCoreRectIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectIndexOfArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_indexOf_t(original_self as *const ::vector::VectorQtCoreRect,
                                                t as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> VectorQtCoreRectIndexOfArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_indexOf_t_from(original_self as *const ::vector::VectorQtCoreRect,
                                                     t as *const ::qt_core::rect::Rect,
                                                     from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::insert](../struct.VectorQtCoreRect.html#method.insert) method.
  pub trait VectorQtCoreRectInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> ();
  }
  impl<'largs> VectorQtCoreRectInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_insert_i_n_t(original_self as *mut ::vector::VectorQtCoreRect,
                                                   i,
                                                   n,
                                                   t as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> VectorQtCoreRectInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_insert_i_t(original_self as *mut ::vector::VectorQtCoreRect,
                                                 i,
                                                 t as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::last_index_of](../struct.VectorQtCoreRect.html#method.last_index_of) method.
  pub trait VectorQtCoreRectLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreRectLastIndexOfArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_lastIndexOf_t(original_self as *const ::vector::VectorQtCoreRect,
                                                    t as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> VectorQtCoreRectLastIndexOfArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QRect_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCoreRect,
                                                         t as *const ::qt_core::rect::Rect,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::mid](../struct.VectorQtCoreRect.html#method.mid) method.
  pub trait VectorQtCoreRectMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::vector::VectorQtCoreRect;
  }
  impl<'largs> VectorQtCoreRectMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::vector::VectorQtCoreRect {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_mid_to_output_pos(original_self as *const ::vector::VectorQtCoreRect,
                                                          pos,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreRectMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::vector::VectorQtCoreRect {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCoreRect,
                                                              pos,
                                                              len,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::new](../struct.VectorQtCoreRect.html#method.new) method.
  pub trait VectorQtCoreRectNewArgs {
    fn exec(self) -> ::vector::VectorQtCoreRect;
  }
  impl VectorQtCoreRectNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCoreRect {

      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCoreRectNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCoreRect {
      let size = self;
      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreRectNewArgs for (::libc::c_int, &'a ::qt_core::rect::Rect) {
    fn exec(self) -> ::vector::VectorQtCoreRect {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_constructor_size_t(size, t as *const ::qt_core::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreRectNewArgs for &'a ::vector::VectorQtCoreRect {
    fn exec(self) -> ::vector::VectorQtCoreRect {
      let v = self;
      {
        let mut object: ::vector::VectorQtCoreRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_constructor_v(v as *const ::vector::VectorQtCoreRect, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::op_add_assign](../struct.VectorQtCoreRect.html#method.op_add_assign) method.
  pub trait VectorQtCoreRectOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect;
  }
  impl<'largs> VectorQtCoreRectOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCoreRect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_operator_add_assign_l(original_self as *mut ::vector::VectorQtCoreRect,
                                                              l as *const ::vector::VectorQtCoreRect)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectOpAddAssignArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_operator_add_assign_t(original_self as *mut ::vector::VectorQtCoreRect,
                                                              t as *const ::qt_core::rect::Rect)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::op_shl](../struct.VectorQtCoreRect.html#method.op_shl) method.
  pub trait VectorQtCoreRectOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect;
  }
  impl<'largs> VectorQtCoreRectOpShlArgs<'largs> for &'largs ::vector::VectorQtCoreRect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRect_operator_shl_l(original_self as *mut ::vector::VectorQtCoreRect,
                                                     l as *const ::vector::VectorQtCoreRect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreRectOpShlArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> &'largs mut ::vector::VectorQtCoreRect {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QRect_operator_shl_t(original_self as *mut ::vector::VectorQtCoreRect,
                                                     t as *const ::qt_core::rect::Rect)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::remove](../struct.VectorQtCoreRect.html#method.remove) method.
  pub trait VectorQtCoreRectRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> ();
  }
  impl<'largs> VectorQtCoreRectRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QRect_remove_i(original_self as *mut ::vector::VectorQtCoreRect, i) }
    }
  }
  impl<'largs> VectorQtCoreRectRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreRect) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QRect_remove_i_n(original_self as *mut ::vector::VectorQtCoreRect, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreRect::value](../struct.VectorQtCoreRect.html#method.value) method.
  pub trait VectorQtCoreRectValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::qt_core::rect::Rect;
  }
  impl<'largs> VectorQtCoreRectValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::qt_core::rect::Rect {
      let i = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_value_to_output_i(original_self as *const ::vector::VectorQtCoreRect,
                                                          i,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreRectValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreRect) -> ::qt_core::rect::Rect {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QRect_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCoreRect, i, default_value as *const ::qt_core::rect::Rect, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::append](../struct.VectorQtCoreSize.html#method.append) method.
  pub trait VectorQtCoreSizeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> ();
  }
  impl<'largs> VectorQtCoreSizeAppendArgs<'largs> for &'largs ::vector::VectorQtCoreSize {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_append_l(original_self as *mut ::vector::VectorQtCoreSize,
                                               l as *const ::vector::VectorQtCoreSize)
      }
    }
  }
  impl<'largs> VectorQtCoreSizeAppendArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_append_t(original_self as *mut ::vector::VectorQtCoreSize,
                                               t as *const ::qt_core::size::Size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::count](../struct.VectorQtCoreSize.html#method.count) method.
  pub trait VectorQtCoreSizeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreSizeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QSize_count_no_args(original_self as *const ::vector::VectorQtCoreSize) }
    }
  }
  impl<'largs> VectorQtCoreSizeCountArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_count_t(original_self as *const ::vector::VectorQtCoreSize,
                                              t as *const ::qt_core::size::Size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::fill](../struct.VectorQtCoreSize.html#method.fill) method.
  pub trait VectorQtCoreSizeFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize;
  }
  impl<'largs> VectorQtCoreSizeFillArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QSize_fill_t(original_self as *mut ::vector::VectorQtCoreSize,
                                             t as *const ::qt_core::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreSizeFillArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QSize_fill_t_size(original_self as *mut ::vector::VectorQtCoreSize,
                                                  t as *const ::qt_core::size::Size,
                                                  size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::index_of](../struct.VectorQtCoreSize.html#method.index_of) method.
  pub trait VectorQtCoreSizeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreSizeIndexOfArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_indexOf_t(original_self as *const ::vector::VectorQtCoreSize,
                                                t as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> VectorQtCoreSizeIndexOfArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_indexOf_t_from(original_self as *const ::vector::VectorQtCoreSize,
                                                     t as *const ::qt_core::size::Size,
                                                     from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::insert](../struct.VectorQtCoreSize.html#method.insert) method.
  pub trait VectorQtCoreSizeInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> ();
  }
  impl<'largs> VectorQtCoreSizeInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_insert_i_n_t(original_self as *mut ::vector::VectorQtCoreSize,
                                                   i,
                                                   n,
                                                   t as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> VectorQtCoreSizeInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_insert_i_t(original_self as *mut ::vector::VectorQtCoreSize,
                                                 i,
                                                 t as *const ::qt_core::size::Size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::last_index_of](../struct.VectorQtCoreSize.html#method.last_index_of) method.
  pub trait VectorQtCoreSizeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> VectorQtCoreSizeLastIndexOfArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_lastIndexOf_t(original_self as *const ::vector::VectorQtCoreSize,
                                                    t as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> VectorQtCoreSizeLastIndexOfArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QSize_lastIndexOf_t_from(original_self as *const ::vector::VectorQtCoreSize,
                                                         t as *const ::qt_core::size::Size,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::mid](../struct.VectorQtCoreSize.html#method.mid) method.
  pub trait VectorQtCoreSizeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::vector::VectorQtCoreSize;
  }
  impl<'largs> VectorQtCoreSizeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::vector::VectorQtCoreSize {
      let pos = self;
      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_mid_to_output_pos(original_self as *const ::vector::VectorQtCoreSize,
                                                          pos,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreSizeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::vector::VectorQtCoreSize {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_mid_to_output_pos_len(original_self as *const ::vector::VectorQtCoreSize,
                                                              pos,
                                                              len,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::new](../struct.VectorQtCoreSize.html#method.new) method.
  pub trait VectorQtCoreSizeNewArgs {
    fn exec(self) -> ::vector::VectorQtCoreSize;
  }
  impl VectorQtCoreSizeNewArgs for () {
    fn exec(self) -> ::vector::VectorQtCoreSize {

      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtCoreSizeNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtCoreSize {
      let size = self;
      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreSizeNewArgs for (::libc::c_int, &'a ::qt_core::size::Size) {
    fn exec(self) -> ::vector::VectorQtCoreSize {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_constructor_size_t(size, t as *const ::qt_core::size::Size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtCoreSizeNewArgs for &'a ::vector::VectorQtCoreSize {
    fn exec(self) -> ::vector::VectorQtCoreSize {
      let v = self;
      {
        let mut object: ::vector::VectorQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_constructor_v(v as *const ::vector::VectorQtCoreSize, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::op_add_assign](../struct.VectorQtCoreSize.html#method.op_add_assign) method.
  pub trait VectorQtCoreSizeOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize;
  }
  impl<'largs> VectorQtCoreSizeOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtCoreSize {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_operator_add_assign_l(original_self as *mut ::vector::VectorQtCoreSize,
                                                              l as *const ::vector::VectorQtCoreSize)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreSizeOpAddAssignArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_operator_add_assign_t(original_self as *mut ::vector::VectorQtCoreSize,
                                                              t as *const ::qt_core::size::Size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::op_shl](../struct.VectorQtCoreSize.html#method.op_shl) method.
  pub trait VectorQtCoreSizeOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize;
  }
  impl<'largs> VectorQtCoreSizeOpShlArgs<'largs> for &'largs ::vector::VectorQtCoreSize {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QSize_operator_shl_l(original_self as *mut ::vector::VectorQtCoreSize,
                                                     l as *const ::vector::VectorQtCoreSize)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtCoreSizeOpShlArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> &'largs mut ::vector::VectorQtCoreSize {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QSize_operator_shl_t(original_self as *mut ::vector::VectorQtCoreSize,
                                                     t as *const ::qt_core::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::remove](../struct.VectorQtCoreSize.html#method.remove) method.
  pub trait VectorQtCoreSizeRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> ();
  }
  impl<'largs> VectorQtCoreSizeRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QSize_remove_i(original_self as *mut ::vector::VectorQtCoreSize, i) }
    }
  }
  impl<'largs> VectorQtCoreSizeRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtCoreSize) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QSize_remove_i_n(original_self as *mut ::vector::VectorQtCoreSize, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtCoreSize::value](../struct.VectorQtCoreSize.html#method.value) method.
  pub trait VectorQtCoreSizeValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::qt_core::size::Size;
  }
  impl<'largs> VectorQtCoreSizeValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::qt_core::size::Size {
      let i = self;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_value_to_output_i(original_self as *const ::vector::VectorQtCoreSize,
                                                          i,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtCoreSizeValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs ::vector::VectorQtCoreSize) -> ::qt_core::size::Size {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QSize_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtCoreSize, i, default_value as *const ::qt_core::size::Size, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::append](../struct.VectorTextFormat.html#method.append) method.
  pub trait VectorTextFormatAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> ();
  }
  impl<'largs> VectorTextFormatAppendArgs<'largs> for &'largs ::vector::VectorTextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_append_l(original_self as *mut ::vector::VectorTextFormat,
                                                     l as *const ::vector::VectorTextFormat)
      }
    }
  }
  impl<'largs> VectorTextFormatAppendArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_append_t(original_self as *mut ::vector::VectorTextFormat,
                                                     t as *const ::text_format::TextFormat)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::count](../struct.VectorTextFormat.html#method.count) method.
  pub trait VectorTextFormatCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int;
  }
  impl<'largs> VectorTextFormatCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_count_no_args(original_self as *const ::vector::VectorTextFormat) }
    }
  }
  impl<'largs> VectorTextFormatCountArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_count_t(original_self as *const ::vector::VectorTextFormat,
                                                    t as *const ::text_format::TextFormat)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::fill](../struct.VectorTextFormat.html#method.fill) method.
  pub trait VectorTextFormatFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat;
  }
  impl<'largs> VectorTextFormatFillArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_fill_t(original_self as *mut ::vector::VectorTextFormat,
                                                   t as *const ::text_format::TextFormat)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextFormatFillArgs<'largs> for (&'largs ::text_format::TextFormat, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_fill_t_size(original_self as *mut ::vector::VectorTextFormat,
                                                          t as *const ::text_format::TextFormat,
                                                          size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::index_of](../struct.VectorTextFormat.html#method.index_of) method.
  pub trait VectorTextFormatIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int;
  }
  impl<'largs> VectorTextFormatIndexOfArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_indexOf_t(original_self as *const ::vector::VectorTextFormat,
                                                      t as *const ::text_format::TextFormat)
      }
    }
  }
  impl<'largs> VectorTextFormatIndexOfArgs<'largs> for (&'largs ::text_format::TextFormat, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_indexOf_t_from(original_self as *const ::vector::VectorTextFormat,
                                                           t as *const ::text_format::TextFormat,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::insert](../struct.VectorTextFormat.html#method.insert) method.
  pub trait VectorTextFormatInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> ();
  }
  impl<'largs> VectorTextFormatInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::text_format::TextFormat) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_insert_i_n_t(original_self as *mut ::vector::VectorTextFormat,
                                                         i,
                                                         n,
                                                         t as *const ::text_format::TextFormat)
      }
    }
  }
  impl<'largs> VectorTextFormatInsertArgs<'largs> for (::libc::c_int, &'largs ::text_format::TextFormat) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_insert_i_t(original_self as *mut ::vector::VectorTextFormat,
                                                       i,
                                                       t as *const ::text_format::TextFormat)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::last_index_of](../struct.VectorTextFormat.html#method.last_index_of) method.
  pub trait VectorTextFormatLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int;
  }
  impl<'largs> VectorTextFormatLastIndexOfArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_lastIndexOf_t(original_self as *const ::vector::VectorTextFormat,
                                                          t as *const ::text_format::TextFormat)
      }
    }
  }
  impl<'largs> VectorTextFormatLastIndexOfArgs<'largs> for (&'largs ::text_format::TextFormat, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextFormat_lastIndexOf_t_from(original_self as *const ::vector::VectorTextFormat,
                                                               t as *const ::text_format::TextFormat,
                                                               from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::mid](../struct.VectorTextFormat.html#method.mid) method.
  pub trait VectorTextFormatMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::vector::VectorTextFormat;
  }
  impl<'largs> VectorTextFormatMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::vector::VectorTextFormat {
      let pos = self;
      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_mid_to_output_pos(original_self as *const ::vector::VectorTextFormat,
                                                                pos,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTextFormatMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::vector::VectorTextFormat {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_mid_to_output_pos_len(original_self as *const ::vector::VectorTextFormat, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::new](../struct.VectorTextFormat.html#method.new) method.
  pub trait VectorTextFormatNewArgs {
    fn exec(self) -> ::vector::VectorTextFormat;
  }
  impl VectorTextFormatNewArgs for () {
    fn exec(self) -> ::vector::VectorTextFormat {

      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorTextFormatNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorTextFormat {
      let size = self;
      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextFormatNewArgs for (::libc::c_int, &'a ::text_format::TextFormat) {
    fn exec(self) -> ::vector::VectorTextFormat {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_constructor_size_t(size,
                                                                 t as *const ::text_format::TextFormat,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextFormatNewArgs for &'a ::vector::VectorTextFormat {
    fn exec(self) -> ::vector::VectorTextFormat {
      let v = self;
      {
        let mut object: ::vector::VectorTextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_constructor_v(v as *const ::vector::VectorTextFormat, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::op_add_assign](../struct.VectorTextFormat.html#method.op_add_assign) method.
  pub trait VectorTextFormatOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat;
  }
  impl<'largs> VectorTextFormatOpAddAssignArgs<'largs> for &'largs ::vector::VectorTextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_operator_add_assign_l(original_self as *mut ::vector::VectorTextFormat,
                                                                    l as *const ::vector::VectorTextFormat)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextFormatOpAddAssignArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_operator_add_assign_t(original_self as *mut ::vector::VectorTextFormat,
                                                                    t as *const ::text_format::TextFormat)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::op_shl](../struct.VectorTextFormat.html#method.op_shl) method.
  pub trait VectorTextFormatOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat;
  }
  impl<'largs> VectorTextFormatOpShlArgs<'largs> for &'largs ::vector::VectorTextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_operator_shl_l(original_self as *mut ::vector::VectorTextFormat,
                                                             l as *const ::vector::VectorTextFormat)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextFormatOpShlArgs<'largs> for &'largs ::text_format::TextFormat {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> &'largs mut ::vector::VectorTextFormat {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_operator_shl_t(original_self as *mut ::vector::VectorTextFormat,
                                                             t as *const ::text_format::TextFormat)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::remove](../struct.VectorTextFormat.html#method.remove) method.
  pub trait VectorTextFormatRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> ();
  }
  impl<'largs> VectorTextFormatRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_remove_i(original_self as *mut ::vector::VectorTextFormat, i) }
    }
  }
  impl<'largs> VectorTextFormatRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextFormat) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QTextFormat_remove_i_n(original_self as *mut ::vector::VectorTextFormat, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextFormat::value](../struct.VectorTextFormat.html#method.value) method.
  pub trait VectorTextFormatValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::text_format::TextFormat;
  }
  impl<'largs> VectorTextFormatValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::text_format::TextFormat {
      let i = self;
      {
        let mut object: ::text_format::TextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_value_to_output_i(original_self as *const ::vector::VectorTextFormat,
                                                                i,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTextFormatValueArgs<'largs> for (::libc::c_int, &'largs ::text_format::TextFormat) {
    fn exec(self, original_self: &'largs ::vector::VectorTextFormat) -> ::text_format::TextFormat {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::text_format::TextFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextFormat_value_to_output_i_defaultValue(original_self as *const ::vector::VectorTextFormat, i, default_value as *const ::text_format::TextFormat, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::append](../struct.VectorTextLayoutFormatRange.html#method.append) method.
  pub trait VectorTextLayoutFormatRangeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> ();
  }
  impl<'largs> VectorTextLayoutFormatRangeAppendArgs<'largs> for &'largs ::vector::VectorTextLayoutFormatRange {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let l = self;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_append_l(original_self as *mut ::vector::VectorTextLayoutFormatRange, l as *const ::vector::VectorTextLayoutFormatRange) }
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeAppendArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_append_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, t as *const ::text_layout::FormatRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::fill](../struct.VectorTextLayoutFormatRange.html#method.fill) method.
  pub trait VectorTextLayoutFormatRangeFillArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange;
  }
  impl<'largs> VectorTextLayoutFormatRangeFillArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_fill_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, t as *const ::text_layout::FormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeFillArgs<'largs> for (&'largs ::text_layout::FormatRange, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_fill_t_size(original_self as *mut ::vector::VectorTextLayoutFormatRange, t as *const ::text_layout::FormatRange, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::insert](../struct.VectorTextLayoutFormatRange.html#method.insert) method.
  pub trait VectorTextLayoutFormatRangeInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> ();
  }
  impl<'largs> VectorTextLayoutFormatRangeInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::text_layout::FormatRange) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_insert_i_n_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, i, n, t as *const ::text_layout::FormatRange) }
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeInsertArgs<'largs> for (::libc::c_int, &'largs ::text_layout::FormatRange) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let i = self.0;
      let t = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_insert_i_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, i, t as *const ::text_layout::FormatRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::mid](../struct.VectorTextLayoutFormatRange.html#method.mid) method.
  pub trait VectorTextLayoutFormatRangeMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::vector::VectorTextLayoutFormatRange;
  }
  impl<'largs> VectorTextLayoutFormatRangeMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::vector::VectorTextLayoutFormatRange {
      let pos = self;
      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_mid_to_output_pos(original_self as *const ::vector::VectorTextLayoutFormatRange, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::vector::VectorTextLayoutFormatRange {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_mid_to_output_pos_len(original_self as *const ::vector::VectorTextLayoutFormatRange, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::new](../struct.VectorTextLayoutFormatRange.html#method.new) method.
  pub trait VectorTextLayoutFormatRangeNewArgs {
    fn exec(self) -> ::vector::VectorTextLayoutFormatRange;
  }
  impl VectorTextLayoutFormatRangeNewArgs for () {
    fn exec(self) -> ::vector::VectorTextLayoutFormatRange {

      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorTextLayoutFormatRangeNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorTextLayoutFormatRange {
      let size = self;
      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextLayoutFormatRangeNewArgs for (::libc::c_int, &'a ::text_layout::FormatRange) {
    fn exec(self) -> ::vector::VectorTextLayoutFormatRange {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constructor_size_t(size,
                                                                             t as *const ::text_layout::FormatRange,
                                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextLayoutFormatRangeNewArgs for &'a ::vector::VectorTextLayoutFormatRange {
    fn exec(self) -> ::vector::VectorTextLayoutFormatRange {
      let v = self;
      {
        let mut object: ::vector::VectorTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_constructor_v(v as *const ::vector::VectorTextLayoutFormatRange, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::op_add_assign](../struct.VectorTextLayoutFormatRange.html#method.op_add_assign) method.
  pub trait VectorTextLayoutFormatRangeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange;
  }
  impl<'largs> VectorTextLayoutFormatRangeOpAddAssignArgs<'largs> for &'largs ::vector::VectorTextLayoutFormatRange {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_assign_l(original_self as *mut ::vector::VectorTextLayoutFormatRange, l as *const ::vector::VectorTextLayoutFormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeOpAddAssignArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_add_assign_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, t as *const ::text_layout::FormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::op_shl](../struct.VectorTextLayoutFormatRange.html#method.op_shl) method.
  pub trait VectorTextLayoutFormatRangeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange;
  }
  impl<'largs> VectorTextLayoutFormatRangeOpShlArgs<'largs> for &'largs ::vector::VectorTextLayoutFormatRange {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_shl_l(original_self as *mut ::vector::VectorTextLayoutFormatRange, l as *const ::vector::VectorTextLayoutFormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeOpShlArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self,
            original_self: &'largs mut ::vector::VectorTextLayoutFormatRange)
            -> &'largs mut ::vector::VectorTextLayoutFormatRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_operator_shl_t(original_self as *mut ::vector::VectorTextLayoutFormatRange, t as *const ::text_layout::FormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::remove](../struct.VectorTextLayoutFormatRange.html#method.remove) method.
  pub trait VectorTextLayoutFormatRangeRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> ();
  }
  impl<'largs> VectorTextLayoutFormatRangeRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_remove_i(original_self as *mut ::vector::VectorTextLayoutFormatRange, i) }
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLayoutFormatRange) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_remove_i_n(original_self as *mut ::vector::VectorTextLayoutFormatRange, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLayoutFormatRange::value](../struct.VectorTextLayoutFormatRange.html#method.value) method.
  pub trait VectorTextLayoutFormatRangeValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::cpp_utils::CppBox<::text_layout::FormatRange>;
  }
  impl<'largs> VectorTextLayoutFormatRangeValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
      let i = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_value_as_ptr_i(original_self as *const ::vector::VectorTextLayoutFormatRange, i) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> VectorTextLayoutFormatRangeValueArgs<'largs> for (::libc::c_int, &'largs ::text_layout::FormatRange) {
    fn exec(self,
            original_self: &'largs ::vector::VectorTextLayoutFormatRange)
            -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
      let i = self.0;
      let default_value = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector_QTextLayout_FormatRange_value_as_ptr_i_defaultValue(original_self as *const ::vector::VectorTextLayoutFormatRange, i, default_value as *const ::text_layout::FormatRange) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::append](../struct.VectorTextLength.html#method.append) method.
  pub trait VectorTextLengthAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> ();
  }
  impl<'largs> VectorTextLengthAppendArgs<'largs> for &'largs ::vector::VectorTextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_append_l(original_self as *mut ::vector::VectorTextLength,
                                                     l as *const ::vector::VectorTextLength)
      }
    }
  }
  impl<'largs> VectorTextLengthAppendArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_append_t(original_self as *mut ::vector::VectorTextLength,
                                                     t as *const ::text_length::TextLength)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::count](../struct.VectorTextLength.html#method.count) method.
  pub trait VectorTextLengthCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int;
  }
  impl<'largs> VectorTextLengthCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_count_no_args(original_self as *const ::vector::VectorTextLength) }
    }
  }
  impl<'largs> VectorTextLengthCountArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_count_t(original_self as *const ::vector::VectorTextLength,
                                                    t as *const ::text_length::TextLength)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::fill](../struct.VectorTextLength.html#method.fill) method.
  pub trait VectorTextLengthFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength;
  }
  impl<'largs> VectorTextLengthFillArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_fill_t(original_self as *mut ::vector::VectorTextLength,
                                                   t as *const ::text_length::TextLength)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLengthFillArgs<'largs> for (&'largs ::text_length::TextLength, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_fill_t_size(original_self as *mut ::vector::VectorTextLength,
                                                          t as *const ::text_length::TextLength,
                                                          size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::index_of](../struct.VectorTextLength.html#method.index_of) method.
  pub trait VectorTextLengthIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int;
  }
  impl<'largs> VectorTextLengthIndexOfArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_indexOf_t(original_self as *const ::vector::VectorTextLength,
                                                      t as *const ::text_length::TextLength)
      }
    }
  }
  impl<'largs> VectorTextLengthIndexOfArgs<'largs> for (&'largs ::text_length::TextLength, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_indexOf_t_from(original_self as *const ::vector::VectorTextLength,
                                                           t as *const ::text_length::TextLength,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::insert](../struct.VectorTextLength.html#method.insert) method.
  pub trait VectorTextLengthInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> ();
  }
  impl<'largs> VectorTextLengthInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::text_length::TextLength) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_insert_i_n_t(original_self as *mut ::vector::VectorTextLength,
                                                         i,
                                                         n,
                                                         t as *const ::text_length::TextLength)
      }
    }
  }
  impl<'largs> VectorTextLengthInsertArgs<'largs> for (::libc::c_int, &'largs ::text_length::TextLength) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_insert_i_t(original_self as *mut ::vector::VectorTextLength,
                                                       i,
                                                       t as *const ::text_length::TextLength)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::last_index_of](../struct.VectorTextLength.html#method.last_index_of) method.
  pub trait VectorTextLengthLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int;
  }
  impl<'largs> VectorTextLengthLastIndexOfArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_lastIndexOf_t(original_self as *const ::vector::VectorTextLength,
                                                          t as *const ::text_length::TextLength)
      }
    }
  }
  impl<'largs> VectorTextLengthLastIndexOfArgs<'largs> for (&'largs ::text_length::TextLength, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_QTextLength_lastIndexOf_t_from(original_self as *const ::vector::VectorTextLength,
                                                               t as *const ::text_length::TextLength,
                                                               from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::mid](../struct.VectorTextLength.html#method.mid) method.
  pub trait VectorTextLengthMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::vector::VectorTextLength;
  }
  impl<'largs> VectorTextLengthMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::vector::VectorTextLength {
      let pos = self;
      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_mid_to_output_pos(original_self as *const ::vector::VectorTextLength,
                                                                pos,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTextLengthMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::vector::VectorTextLength {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_mid_to_output_pos_len(original_self as *const ::vector::VectorTextLength, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::new](../struct.VectorTextLength.html#method.new) method.
  pub trait VectorTextLengthNewArgs {
    fn exec(self) -> ::vector::VectorTextLength;
  }
  impl VectorTextLengthNewArgs for () {
    fn exec(self) -> ::vector::VectorTextLength {

      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorTextLengthNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorTextLength {
      let size = self;
      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextLengthNewArgs for (::libc::c_int, &'a ::text_length::TextLength) {
    fn exec(self) -> ::vector::VectorTextLength {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_constructor_size_t(size,
                                                                 t as *const ::text_length::TextLength,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorTextLengthNewArgs for &'a ::vector::VectorTextLength {
    fn exec(self) -> ::vector::VectorTextLength {
      let v = self;
      {
        let mut object: ::vector::VectorTextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_constructor_v(v as *const ::vector::VectorTextLength, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::op_add_assign](../struct.VectorTextLength.html#method.op_add_assign) method.
  pub trait VectorTextLengthOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength;
  }
  impl<'largs> VectorTextLengthOpAddAssignArgs<'largs> for &'largs ::vector::VectorTextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_operator_add_assign_l(original_self as *mut ::vector::VectorTextLength,
                                                                    l as *const ::vector::VectorTextLength)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLengthOpAddAssignArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_operator_add_assign_t(original_self as *mut ::vector::VectorTextLength,
                                                                    t as *const ::text_length::TextLength)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::op_shl](../struct.VectorTextLength.html#method.op_shl) method.
  pub trait VectorTextLengthOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength;
  }
  impl<'largs> VectorTextLengthOpShlArgs<'largs> for &'largs ::vector::VectorTextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_operator_shl_l(original_self as *mut ::vector::VectorTextLength,
                                                             l as *const ::vector::VectorTextLength)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorTextLengthOpShlArgs<'largs> for &'largs ::text_length::TextLength {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> &'largs mut ::vector::VectorTextLength {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_operator_shl_t(original_self as *mut ::vector::VectorTextLength,
                                                             t as *const ::text_length::TextLength)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::remove](../struct.VectorTextLength.html#method.remove) method.
  pub trait VectorTextLengthRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> ();
  }
  impl<'largs> VectorTextLengthRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_remove_i(original_self as *mut ::vector::VectorTextLength, i) }
    }
  }
  impl<'largs> VectorTextLengthRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorTextLength) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_QTextLength_remove_i_n(original_self as *mut ::vector::VectorTextLength, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorTextLength::value](../struct.VectorTextLength.html#method.value) method.
  pub trait VectorTextLengthValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::text_length::TextLength;
  }
  impl<'largs> VectorTextLengthValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::text_length::TextLength {
      let i = self;
      {
        let mut object: ::text_length::TextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_value_to_output_i(original_self as *const ::vector::VectorTextLength,
                                                                i,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorTextLengthValueArgs<'largs> for (::libc::c_int, &'largs ::text_length::TextLength) {
    fn exec(self, original_self: &'largs ::vector::VectorTextLength) -> ::text_length::TextLength {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::text_length::TextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_QTextLength_value_to_output_i_defaultValue(original_self as *const ::vector::VectorTextLength, i, default_value as *const ::text_length::TextLength, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::append](../struct.VectorU32.html#method.append) method.
  pub trait VectorU32AppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> ();
  }
  impl<'largs> VectorU32AppendArgs<'largs> for &'largs ::vector::VectorU32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_append_l(original_self as *mut ::vector::VectorU32,
                                                 l as *const ::vector::VectorU32)
      }
    }
  }
  impl<'largs> VectorU32AppendArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_append_t(original_self as *mut ::vector::VectorU32, t as *const u32) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::count](../struct.VectorU32.html#method.count) method.
  pub trait VectorU32CountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int;
  }
  impl<'largs> VectorU32CountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_quint32_count_no_args(original_self as *const ::vector::VectorU32) }
    }
  }
  impl<'largs> VectorU32CountArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_count_t(original_self as *const ::vector::VectorU32, t as *const u32) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::fill](../struct.VectorU32.html#method.fill) method.
  pub trait VectorU32FillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32;
  }
  impl<'largs> VectorU32FillArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let t = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QVector_quint32_fill_t(original_self as *mut ::vector::VectorU32, t as *const u32) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU32FillArgs<'largs> for (&'largs u32, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_quint32_fill_t_size(original_self as *mut ::vector::VectorU32,
                                                    t as *const u32,
                                                    size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::index_of](../struct.VectorU32.html#method.index_of) method.
  pub trait VectorU32IndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int;
  }
  impl<'largs> VectorU32IndexOfArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_indexOf_t(original_self as *const ::vector::VectorU32, t as *const u32) }
    }
  }
  impl<'largs> VectorU32IndexOfArgs<'largs> for (&'largs u32, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_indexOf_t_from(original_self as *const ::vector::VectorU32,
                                                       t as *const u32,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::insert](../struct.VectorU32.html#method.insert) method.
  pub trait VectorU32InsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> ();
  }
  impl<'largs> VectorU32InsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs u32) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_insert_i_n_t(original_self as *mut ::vector::VectorU32,
                                                     i,
                                                     n,
                                                     t as *const u32)
      }
    }
  }
  impl<'largs> VectorU32InsertArgs<'largs> for (::libc::c_int, &'largs u32) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_insert_i_t(original_self as *mut ::vector::VectorU32,
                                                   i,
                                                   t as *const u32)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::last_index_of](../struct.VectorU32.html#method.last_index_of) method.
  pub trait VectorU32LastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int;
  }
  impl<'largs> VectorU32LastIndexOfArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_lastIndexOf_t(original_self as *const ::vector::VectorU32, t as *const u32)
      }
    }
  }
  impl<'largs> VectorU32LastIndexOfArgs<'largs> for (&'largs u32, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_lastIndexOf_t_from(original_self as *const ::vector::VectorU32,
                                                           t as *const u32,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::mid](../struct.VectorU32.html#method.mid) method.
  pub trait VectorU32MidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::vector::VectorU32;
  }
  impl<'largs> VectorU32MidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::vector::VectorU32 {
      let pos = self;
      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_mid_to_output_pos(original_self as *const ::vector::VectorU32,
                                                            pos,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorU32MidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> ::vector::VectorU32 {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_mid_to_output_pos_len(original_self as *const ::vector::VectorU32,
                                                                pos,
                                                                len,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::new](../struct.VectorU32.html#method.new) method.
  pub trait VectorU32NewArgs {
    fn exec(self) -> ::vector::VectorU32;
  }
  impl VectorU32NewArgs for () {
    fn exec(self) -> ::vector::VectorU32 {

      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorU32NewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorU32 {
      let size = self;
      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorU32NewArgs for (::libc::c_int, &'a u32) {
    fn exec(self) -> ::vector::VectorU32 {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_constructor_size_t(size, t as *const u32, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorU32NewArgs for &'a ::vector::VectorU32 {
    fn exec(self) -> ::vector::VectorU32 {
      let v = self;
      {
        let mut object: ::vector::VectorU32 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_constructor_v(v as *const ::vector::VectorU32, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::op_add_assign](../struct.VectorU32.html#method.op_add_assign) method.
  pub trait VectorU32OpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32;
  }
  impl<'largs> VectorU32OpAddAssignArgs<'largs> for &'largs ::vector::VectorU32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_operator_add_assign_l(original_self as *mut ::vector::VectorU32,
                                                                l as *const ::vector::VectorU32)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU32OpAddAssignArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_quint32_operator_add_assign_t(original_self as *mut ::vector::VectorU32,
                                                                t as *const u32)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::op_shl](../struct.VectorU32.html#method.op_shl) method.
  pub trait VectorU32OpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32;
  }
  impl<'largs> VectorU32OpShlArgs<'largs> for &'largs ::vector::VectorU32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_quint32_operator_shl_l(original_self as *mut ::vector::VectorU32,
                                                       l as *const ::vector::VectorU32)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU32OpShlArgs<'largs> for &'largs u32 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> &'largs mut ::vector::VectorU32 {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_quint32_operator_shl_t(original_self as *mut ::vector::VectorU32, t as *const u32)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::remove](../struct.VectorU32.html#method.remove) method.
  pub trait VectorU32RemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> ();
  }
  impl<'largs> VectorU32RemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_remove_i(original_self as *mut ::vector::VectorU32, i) }
    }
  }
  impl<'largs> VectorU32RemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU32) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_remove_i_n(original_self as *mut ::vector::VectorU32, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU32::value](../struct.VectorU32.html#method.value) method.
  pub trait VectorU32ValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> u32;
  }
  impl<'largs> VectorU32ValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> u32 {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_quint32_value_i(original_self as *const ::vector::VectorU32, i) }
    }
  }
  impl<'largs> VectorU32ValueArgs<'largs> for (::libc::c_int, &'largs u32) {
    fn exec(self, original_self: &'largs ::vector::VectorU32) -> u32 {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_quint32_value_i_defaultValue(original_self as *const ::vector::VectorU32,
                                                             i,
                                                             default_value as *const u32)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::append](../struct.VectorU64.html#method.append) method.
  pub trait VectorU64AppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> ();
  }
  impl<'largs> VectorU64AppendArgs<'largs> for &'largs ::vector::VectorU64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_append_l(original_self as *mut ::vector::VectorU64,
                                                  l as *const ::vector::VectorU64)
      }
    }
  }
  impl<'largs> VectorU64AppendArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_append_t(original_self as *mut ::vector::VectorU64, t as *const u64) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::count](../struct.VectorU64.html#method.count) method.
  pub trait VectorU64CountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int;
  }
  impl<'largs> VectorU64CountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_count_no_args(original_self as *const ::vector::VectorU64) }
    }
  }
  impl<'largs> VectorU64CountArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_count_t(original_self as *const ::vector::VectorU64, t as *const u64) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::fill](../struct.VectorU64.html#method.fill) method.
  pub trait VectorU64FillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64;
  }
  impl<'largs> VectorU64FillArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let t = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QVector_GLuint64_fill_t(original_self as *mut ::vector::VectorU64, t as *const u64) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU64FillArgs<'largs> for (&'largs u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let t = self.0;
      let size = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_fill_t_size(original_self as *mut ::vector::VectorU64,
                                                     t as *const u64,
                                                     size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::index_of](../struct.VectorU64.html#method.index_of) method.
  pub trait VectorU64IndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int;
  }
  impl<'largs> VectorU64IndexOfArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_indexOf_t(original_self as *const ::vector::VectorU64, t as *const u64)
      }
    }
  }
  impl<'largs> VectorU64IndexOfArgs<'largs> for (&'largs u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_indexOf_t_from(original_self as *const ::vector::VectorU64,
                                                        t as *const u64,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::insert](../struct.VectorU64.html#method.insert) method.
  pub trait VectorU64InsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> ();
  }
  impl<'largs> VectorU64InsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs u64) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_insert_i_n_t(original_self as *mut ::vector::VectorU64,
                                                      i,
                                                      n,
                                                      t as *const u64)
      }
    }
  }
  impl<'largs> VectorU64InsertArgs<'largs> for (::libc::c_int, &'largs u64) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_insert_i_t(original_self as *mut ::vector::VectorU64,
                                                    i,
                                                    t as *const u64)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::last_index_of](../struct.VectorU64.html#method.last_index_of) method.
  pub trait VectorU64LastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int;
  }
  impl<'largs> VectorU64LastIndexOfArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_lastIndexOf_t(original_self as *const ::vector::VectorU64, t as *const u64)
      }
    }
  }
  impl<'largs> VectorU64LastIndexOfArgs<'largs> for (&'largs u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_lastIndexOf_t_from(original_self as *const ::vector::VectorU64,
                                                            t as *const u64,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::mid](../struct.VectorU64.html#method.mid) method.
  pub trait VectorU64MidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::vector::VectorU64;
  }
  impl<'largs> VectorU64MidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::vector::VectorU64 {
      let pos = self;
      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_mid_to_output_pos(original_self as *const ::vector::VectorU64,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorU64MidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> ::vector::VectorU64 {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_mid_to_output_pos_len(original_self as *const ::vector::VectorU64,
                                                                 pos,
                                                                 len,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::new](../struct.VectorU64.html#method.new) method.
  pub trait VectorU64NewArgs {
    fn exec(self) -> ::vector::VectorU64;
  }
  impl VectorU64NewArgs for () {
    fn exec(self) -> ::vector::VectorU64 {

      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorU64NewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorU64 {
      let size = self;
      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorU64NewArgs for (::libc::c_int, &'a u64) {
    fn exec(self) -> ::vector::VectorU64 {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_constructor_size_t(size, t as *const u64, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorU64NewArgs for &'a ::vector::VectorU64 {
    fn exec(self) -> ::vector::VectorU64 {
      let v = self;
      {
        let mut object: ::vector::VectorU64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_constructor_v(v as *const ::vector::VectorU64, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::op_add_assign](../struct.VectorU64.html#method.op_add_assign) method.
  pub trait VectorU64OpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64;
  }
  impl<'largs> VectorU64OpAddAssignArgs<'largs> for &'largs ::vector::VectorU64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_operator_add_assign_l(original_self as *mut ::vector::VectorU64,
                                                                 l as *const ::vector::VectorU64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU64OpAddAssignArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector_GLuint64_operator_add_assign_t(original_self as *mut ::vector::VectorU64,
                                                                 t as *const u64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::op_shl](../struct.VectorU64.html#method.op_shl) method.
  pub trait VectorU64OpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64;
  }
  impl<'largs> VectorU64OpShlArgs<'largs> for &'largs ::vector::VectorU64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_operator_shl_l(original_self as *mut ::vector::VectorU64,
                                                        l as *const ::vector::VectorU64)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorU64OpShlArgs<'largs> for &'largs u64 {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> &'largs mut ::vector::VectorU64 {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_operator_shl_t(original_self as *mut ::vector::VectorU64, t as *const u64)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::remove](../struct.VectorU64.html#method.remove) method.
  pub trait VectorU64RemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> ();
  }
  impl<'largs> VectorU64RemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_remove_i(original_self as *mut ::vector::VectorU64, i) }
    }
  }
  impl<'largs> VectorU64RemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorU64) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_remove_i_n(original_self as *mut ::vector::VectorU64, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorU64::value](../struct.VectorU64.html#method.value) method.
  pub trait VectorU64ValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> u64;
  }
  impl<'largs> VectorU64ValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> u64 {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QVector_GLuint64_value_i(original_self as *const ::vector::VectorU64, i) }
    }
  }
  impl<'largs> VectorU64ValueArgs<'largs> for (::libc::c_int, &'largs u64) {
    fn exec(self, original_self: &'largs ::vector::VectorU64) -> u64 {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector_GLuint64_value_i_defaultValue(original_self as *const ::vector::VectorU64,
                                                              i,
                                                              default_value as *const u64)
      }
    }
  }
}
