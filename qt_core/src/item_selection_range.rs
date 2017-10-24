/// C++ type: <span style='color: green;'>```QItemSelectionRange```</span>
#[repr(C)]
pub struct ItemSelectionRange([u8; ::type_sizes::QT_CORE_ITEM_SELECTION_RANGE_ITEM_SELECTION_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ItemSelectionRange {
  unsafe fn new_uninitialized() -> ItemSelectionRange {
    ItemSelectionRange(::std::mem::uninitialized())
  }
}

impl ItemSelectionRange {
  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_bottom(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QItemSelectionRange::bottomRight() const```</span>
  ///
  ///
  pub fn bottom_right<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_bottomRight(self as *const ::item_selection_range::ItemSelectionRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::contains(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::contains(int row, int column, const QModelIndex& parentIndex) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ItemSelectionRangeContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_height(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QItemSelectionRange::indexes() const```</span>
  ///
  ///
  pub fn indexes(&self) -> ::list::ListModelIndex {
    {
      let mut object: ::list::ListModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_indexes_to_output(self as *const ::item_selection_range::ItemSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange QItemSelectionRange::intersected(const QItemSelectionRange& other) const```</span>
  ///
  ///
  pub fn intersected(&self,
                     other: &::item_selection_range::ItemSelectionRange)
                     -> ::item_selection_range::ItemSelectionRange {
    {
      let mut object: ::item_selection_range::ItemSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_intersected_to_output(self as *const ::item_selection_range::ItemSelectionRange, other as *const ::item_selection_range::ItemSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::intersects(const QItemSelectionRange& other) const```</span>
  ///
  ///
  pub fn intersects(&self, other: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionRange_intersects(self as *const ::item_selection_range::ItemSelectionRange,
                                                      other as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_isEmpty(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_isValid(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::left() const```</span>
  ///
  ///
  pub fn left(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_left(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractItemModel* QItemSelectionRange::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *const ::abstract_item_model::AbstractItemModel {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_model(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange::QItemSelectionRange```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionRange::QItemSelectionRange()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::item_selection_range::ItemSelectionRange) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionRange::QItemSelectionRange(const QItemSelectionRange& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::model_index::ModelIndex) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionRange::QItemSelectionRange(const QModelIndex& index)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::model_index::ModelIndex, &::model_index::ModelIndex)) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QItemSelectionRange::QItemSelectionRange(const QModelIndex& topL, const QModelIndex& bottomR)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::item_selection_range::ItemSelectionRange
    where Args: overloading::ItemSelectionRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QItemSelectionRange::operator=(const QItemSelectionRange& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::item_selection_range::ItemSelectionRange)
                             -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_operator_assign(self as *mut ::item_selection_range::ItemSelectionRange, other as *const ::item_selection_range::ItemSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::operator==(const QItemSelectionRange& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionRange_operator_eq(self as *const ::item_selection_range::ItemSelectionRange,
                                                       other as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::operator<(const QItemSelectionRange& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionRange_operator_lt(self as *const ::item_selection_range::ItemSelectionRange,
                                                       other as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QItemSelectionRange::operator!=(const QItemSelectionRange& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionRange_operator_neq(self as *const ::item_selection_range::ItemSelectionRange,
                                                        other as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QItemSelectionRange::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_parent_to_output(self as *const ::item_selection_range::ItemSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::right() const```</span>
  ///
  ///
  pub fn right(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_right(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QItemSelectionRange::swap(QItemSelectionRange& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QItemSelectionRange_swap(self as *mut ::item_selection_range::ItemSelectionRange,
                                                other as *mut ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_top(self as *const ::item_selection_range::ItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QItemSelectionRange::topLeft() const```</span>
  ///
  ///
  pub fn top_left<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QItemSelectionRange_topLeft(self as *const ::item_selection_range::ItemSelectionRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QItemSelectionRange::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_width(self as *const ::item_selection_range::ItemSelectionRange) }
  }
}

impl Drop for ::item_selection_range::ItemSelectionRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QItemSelectionRange::~QItemSelectionRange()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QItemSelectionRange_destructor(self as *mut ::item_selection_range::ItemSelectionRange) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ItemSelectionRange::contains](../struct.ItemSelectionRange.html#method.contains) method.
  pub trait ItemSelectionRangeContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::item_selection_range::ItemSelectionRange) -> bool;
  }
  impl<'largs> ItemSelectionRangeContainsArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::item_selection_range::ItemSelectionRange) -> bool {
      let index = self;
      unsafe { ::ffi::qt_core_c_QItemSelectionRange_contains_index(original_self as *const ::item_selection_range::ItemSelectionRange, index as *const ::model_index::ModelIndex) }
    }
  }
  impl<'largs> ItemSelectionRangeContainsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::item_selection_range::ItemSelectionRange) -> bool {
      let row = self.0;
      let column = self.1;
      let parent_index = self.2;
      unsafe { ::ffi::qt_core_c_QItemSelectionRange_contains_row_column_parentIndex(original_self as *const ::item_selection_range::ItemSelectionRange, row, column, parent_index as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [ItemSelectionRange::new](../struct.ItemSelectionRange.html#method.new) method.
  pub trait ItemSelectionRangeNewArgs {
    fn exec(self) -> ::item_selection_range::ItemSelectionRange;
  }
  impl<'a> ItemSelectionRangeNewArgs for &'a ::model_index::ModelIndex {
    fn exec(self) -> ::item_selection_range::ItemSelectionRange {
      let index = self;
      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionRange_constructor_index(index as *const ::model_index::ModelIndex,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl ItemSelectionRangeNewArgs for () {
    fn exec(self) -> ::item_selection_range::ItemSelectionRange {

      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ItemSelectionRangeNewArgs for &'a ::item_selection_range::ItemSelectionRange {
    fn exec(self) -> ::item_selection_range::ItemSelectionRange {
      let other = self;
      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionRange_constructor_other(other as *const ::item_selection_range::ItemSelectionRange, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ItemSelectionRangeNewArgs for (&'a ::model_index::ModelIndex, &'a ::model_index::ModelIndex) {
    fn exec(self) -> ::item_selection_range::ItemSelectionRange {
      let top_l = self.0;
      let bottom_r = self.1;
      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QItemSelectionRange_constructor_topL_bottomR(top_l as *const ::model_index::ModelIndex,
                                                                        bottom_r as *const ::model_index::ModelIndex,
                                                                        &mut object);
        }
        object
      }
    }
  }
}
