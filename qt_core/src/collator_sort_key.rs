/// C++ type: <span style='color: green;'>```QCollatorSortKey```</span>
#[repr(C)]
pub struct CollatorSortKey([u8; ::type_sizes::QT_CORE_COLLATOR_SORT_KEY_COLLATOR_SORT_KEY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for CollatorSortKey {
  unsafe fn new_uninitialized() -> CollatorSortKey {
    CollatorSortKey(::std::mem::uninitialized())
  }
}

impl CollatorSortKey {
  /// C++ method: <span style='color: green;'>```int QCollatorSortKey::compare(const QCollatorSortKey& key) const```</span>
  ///
  ///
  pub fn compare(&self, key: &::collator_sort_key::CollatorSortKey) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QCollatorSortKey_compare(self as *const ::collator_sort_key::CollatorSortKey,
                                                key as *const ::collator_sort_key::CollatorSortKey)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QCollatorSortKey::QCollatorSortKey(const QCollatorSortKey& other)```</span>
  ///
  ///
  pub fn new(other: &::collator_sort_key::CollatorSortKey) -> ::collator_sort_key::CollatorSortKey {
    {
      let mut object: ::collator_sort_key::CollatorSortKey =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCollatorSortKey_constructor(other as *const ::collator_sort_key::CollatorSortKey,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCollatorSortKey& QCollatorSortKey::operator=(const QCollatorSortKey& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::collator_sort_key::CollatorSortKey)
                             -> &'l0 mut ::collator_sort_key::CollatorSortKey {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QCollatorSortKey_operator_assign(self as *mut ::collator_sort_key::CollatorSortKey,
                                                          other as *const ::collator_sort_key::CollatorSortKey)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QCollatorSortKey::swap(QCollatorSortKey& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::collator_sort_key::CollatorSortKey) {
    unsafe {
      ::ffi::qt_core_c_QCollatorSortKey_swap(self as *mut ::collator_sort_key::CollatorSortKey,
                                             other as *mut ::collator_sort_key::CollatorSortKey)
    }
  }
}

impl Drop for ::collator_sort_key::CollatorSortKey {
  /// C++ method: <span style='color: green;'>```[destructor] void QCollatorSortKey::~QCollatorSortKey()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCollatorSortKey_destructor(self as *mut ::collator_sort_key::CollatorSortKey) }
  }
}
