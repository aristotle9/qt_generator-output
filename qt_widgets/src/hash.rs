/// C++ type: <span style='color: green;'>```QHash<int, QItemEditorCreatorBase*>```</span>
#[repr(C)]
pub struct HashCIntItemEditorCreatorBaseMutPtr([u8; ::type_sizes::QT_WIDGETS_HASH_HASH_C_INT_ITEM_EDITOR_CREATOR_BASE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for HashCIntItemEditorCreatorBaseMutPtr {
  unsafe fn new_uninitialized() -> HashCIntItemEditorCreatorBaseMutPtr {
    HashCIntItemEditorCreatorBaseMutPtr(::std::mem::uninitialized())
  }
}

impl HashCIntItemEditorCreatorBaseMutPtr {
  /// C++ method: <span style='color: green;'>```int QHash<int, QItemEditorCreatorBase*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_capacity(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QItemEditorCreatorBase*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_clear(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<int, QItemEditorCreatorBase*>::contains(const int& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_contains(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QHash<int, QItemEditorCreatorBase*>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<int, QItemEditorCreatorBase*>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<int, QItemEditorCreatorBase*>::count(const int& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HashCIntItemEditorCreatorBaseMutPtrCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QHash<int, QItemEditorCreatorBase*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_empty(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<int, QItemEditorCreatorBase*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_isEmpty(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QHash<int, QItemEditorCreatorBase*>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &*mut ::item_editor_creator_base::ItemEditorCreatorBase) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QHash<int, QItemEditorCreatorBase*>::key(QItemEditorCreatorBase* const & value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&*mut ::item_editor_creator_base::ItemEditorCreatorBase, &::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QHash<int, QItemEditorCreatorBase*>::key(QItemEditorCreatorBase* const & value, const int& defaultKey) const```</span>
  ///
  ///
  pub unsafe fn key<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HashCIntItemEditorCreatorBaseMutPtrKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QItemEditorCreatorBase*>::keys() const```</span>
  ///
  ///
  pub fn keys(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_keys_to_output_no_args(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QItemEditorCreatorBase*>::keys(QItemEditorCreatorBase* const & value) const```</span>
  ///
  ///
  pub unsafe fn keys_unsafe(&self,
                            value: &*mut ::item_editor_creator_base::ItemEditorCreatorBase)
                            -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_keys_to_output_value(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, value as *const *mut ::item_editor_creator_base::ItemEditorCreatorBase, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHash<int, QItemEditorCreatorBase*>::QHash()```</span>
  ///
  ///
  pub fn new() -> ::hash::HashCIntItemEditorCreatorBaseMutPtr {
    {
      let mut object: ::hash::HashCIntItemEditorCreatorBaseMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorCreatorBase* QHash<int, QItemEditorCreatorBase*>::operator[](const int& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::libc::c_int) -> *mut ::item_editor_creator_base::ItemEditorCreatorBase {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_operator_index_const(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorCreatorBase*& QHash<int, QItemEditorCreatorBase*>::operator[](const int& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self,
                                key: &'l1 ::libc::c_int)
                                -> &'l0 mut *mut ::item_editor_creator_base::ItemEditorCreatorBase {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_operator_index(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QHash<int, QItemEditorCreatorBase*>::remove(const int& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_remove(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QItemEditorCreatorBase*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_reserve(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QHash<int, QItemEditorCreatorBase*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_size(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QItemEditorCreatorBase*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_squeeze(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorCreatorBase* QHash<int, QItemEditorCreatorBase*>::take(const int& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::libc::c_int) -> *mut ::item_editor_creator_base::ItemEditorCreatorBase {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_take(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QItemEditorCreatorBase*>::uniqueKeys() const```</span>
  ///
  ///
  pub fn unique_keys(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_uniqueKeys_to_output(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorCreatorBase* QHash<int, QItemEditorCreatorBase*>::value(const int& key) const```</span>
  ///
  ///
  pub fn value(&self, key: &::libc::c_int) -> *mut ::item_editor_creator_base::ItemEditorCreatorBase {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_value_key(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QItemEditorCreatorBase* QHash<int, QItemEditorCreatorBase*>::value(const int& key, QItemEditorCreatorBase* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             key: &::libc::c_int,
                             default_value: &*mut ::item_editor_creator_base::ItemEditorCreatorBase)
                             -> *mut ::item_editor_creator_base::ItemEditorCreatorBase {
    ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_value_key_defaultValue(self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int, default_value as *const *mut ::item_editor_creator_base::ItemEditorCreatorBase)
  }
}

impl Drop for ::hash::HashCIntItemEditorCreatorBaseMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QHash<int, QItemEditorCreatorBase*>::~QHash()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_destructor(self as *mut ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [HashCIntItemEditorCreatorBaseMutPtr::count](../struct.HashCIntItemEditorCreatorBaseMutPtr.html#method.count) method.
  pub trait HashCIntItemEditorCreatorBaseMutPtrCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int;
  }
  impl<'largs> HashCIntItemEditorCreatorBaseMutPtrCountArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int {
      let key = self;
      unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_count_key(original_self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, key as *const ::libc::c_int) }
    }
  }
  impl<'largs> HashCIntItemEditorCreatorBaseMutPtrCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_count_no_args(original_self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [HashCIntItemEditorCreatorBaseMutPtr::key](../struct.HashCIntItemEditorCreatorBaseMutPtr.html#method.key) method.
  pub trait HashCIntItemEditorCreatorBaseMutPtrKeyArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int;
  }
  impl<'largs> HashCIntItemEditorCreatorBaseMutPtrKeyArgs<'largs> for &'largs *mut ::item_editor_creator_base::ItemEditorCreatorBase {

  unsafe fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int {
    let value = self;
    ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_key_value(original_self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, value as *const *mut ::item_editor_creator_base::ItemEditorCreatorBase)
  }
}
  impl<'largs> HashCIntItemEditorCreatorBaseMutPtrKeyArgs<'largs> for (&'largs *mut ::item_editor_creator_base::ItemEditorCreatorBase,&'largs ::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::hash::HashCIntItemEditorCreatorBaseMutPtr) -> ::libc::c_int {
    let value = self.0;
let default_key = self.1;
    ::ffi::qt_widgets_c_QHash_int_QItemEditorCreatorBase_ptr_key_value_defaultKey(original_self as *const ::hash::HashCIntItemEditorCreatorBaseMutPtr, value as *const *mut ::item_editor_creator_base::ItemEditorCreatorBase, default_key as *const ::libc::c_int)
  }
}
}
