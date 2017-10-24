/// C++ type: <span style='color: green;'>```QItemSelection```</span>
#[repr(C)]
pub struct ItemSelection([u8; ::type_sizes::QT_CORE_ITEM_SELECTION_ITEM_SELECTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ItemSelection {
  unsafe fn new_uninitialized() -> ItemSelection {
    ItemSelection(::std::mem::uninitialized())
  }
}

impl ItemSelection {
  /// C++ method: <span style='color: green;'>```bool QItemSelection::contains(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn contains(&self, index: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelection_contains(self as *const ::item_selection::ItemSelection,
                                               index as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelection::indexes() const```</span>
  ///
  ///
  pub fn indexes(&self) -> ::list::ListModelIndex {
    {
      let mut object: ::list::ListModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelection_indexes_to_output(self as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemSelection::QItemSelection```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::item_selection::ItemSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelection::QItemSelection()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::model_index::ModelIndex, &::model_index::ModelIndex)) -> ::item_selection::ItemSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelection::QItemSelection(const QModelIndex& topLeft, const QModelIndex& bottomRight)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::item_selection::ItemSelection
    where Args: overloading::ItemSelectionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QItemSelection::select(const QModelIndex& topLeft, const QModelIndex& bottomRight)```</span>
  ///
  ///
  pub fn select(&mut self, top_left: &::model_index::ModelIndex, bottom_right: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QItemSelection_select(self as *mut ::item_selection::ItemSelection,
                                             top_left as *const ::model_index::ModelIndex,
                                             bottom_right as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QItemSelection::split(const QItemSelectionRange& range, const QItemSelectionRange& other, QItemSelection* result)```</span>
  ///
  ///
  pub unsafe fn split(range: &::item_selection_range::ItemSelectionRange,
                      other: &::item_selection_range::ItemSelectionRange,
                      result: *mut ::item_selection::ItemSelection) {
    ::ffi::qt_core_c_QItemSelection_split(range as *const ::item_selection_range::ItemSelectionRange,
                                          other as *const ::item_selection_range::ItemSelectionRange,
                                          result)
  }
}

impl Drop for ::item_selection::ItemSelection {
  /// C++ method: <span style='color: green;'>```[destructor] void QItemSelection::~QItemSelection()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QItemSelection_destructor(self as *mut ::item_selection::ItemSelection) }
  }
}

impl ::cpp_utils::StaticCast<::list::ListItemSelectionRange> for ::item_selection::ItemSelection {
  fn static_cast_mut(&mut self) -> &mut ::list::ListItemSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(self as *mut ::item_selection::ItemSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::list::ListItemSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(self as *const ::item_selection::ItemSelection as *mut ::item_selection::ItemSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::item_selection::ItemSelection> for ::list::ListItemSelectionRange {
  unsafe fn static_cast_mut(&mut self) -> &mut ::item_selection::ItemSelection {
    let ffi_result =
      ::ffi::qt_core_c_QItemSelection_G_static_cast_QItemSelection_ptr(self as *mut ::list::ListItemSelectionRange);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::item_selection::ItemSelection {
    let ffi_result = ::ffi::qt_core_c_QItemSelection_G_static_cast_QItemSelection_ptr(self as *const ::list::ListItemSelectionRange as *mut ::list::ListItemSelectionRange);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::item_selection::ItemSelection {
  type Target = ::list::ListItemSelectionRange;
  fn deref(&self) -> &::list::ListItemSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(self as *const ::item_selection::ItemSelection as *mut ::item_selection::ItemSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::item_selection::ItemSelection {
  fn deref_mut(&mut self) -> &mut ::list::ListItemSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_core_c_QItemSelection_G_static_cast_QList_QItemSelectionRange_ptr(self as *mut ::item_selection::ItemSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ItemSelection::new](../struct.ItemSelection.html#method.new) method.
  pub trait ItemSelectionNewArgs {
    fn exec(self) -> ::item_selection::ItemSelection;
  }
  impl ItemSelectionNewArgs for () {
    fn exec(self) -> ::item_selection::ItemSelection {

      {
        let mut object: ::item_selection::ItemSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelection_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ItemSelectionNewArgs for (&'a ::model_index::ModelIndex, &'a ::model_index::ModelIndex) {
    fn exec(self) -> ::item_selection::ItemSelection {
      let top_left = self.0;
      let bottom_right = self.1;
      {
        let mut object: ::item_selection::ItemSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelection_constructor_topLeft_bottomRight(top_left as *const ::model_index::ModelIndex, bottom_right as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
}
