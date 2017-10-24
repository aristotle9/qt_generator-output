/// C++ type: <span style='color: green;'>```QTableWidgetSelectionRange```</span>
#[repr(C)]
pub struct TableWidgetSelectionRange([u8; ::type_sizes::QT_WIDGETS_TABLE_WIDGET_SELECTION_RANGE_TABLE_WIDGET_SELECTION_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TableWidgetSelectionRange {
  unsafe fn new_uninitialized() -> TableWidgetSelectionRange {
    TableWidgetSelectionRange(::std::mem::uninitialized())
  }
}

impl TableWidgetSelectionRange {
  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::bottomRow() const```</span>
  ///
  ///
  pub fn bottom_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_bottomRow(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_columnCount(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::leftColumn() const```</span>
  ///
  ///
  pub fn left_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_leftColumn(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange::QTableWidgetSelectionRange```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::table_widget_selection_range::TableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetSelectionRange::QTableWidgetSelectionRange()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::table_widget_selection_range::TableWidgetSelectionRange) -> ::table_widget_selection_range::TableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetSelectionRange::QTableWidgetSelectionRange(const QTableWidgetSelectionRange& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::table_widget_selection_range::TableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidgetSelectionRange::QTableWidgetSelectionRange(int top, int left, int bottom, int right)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::table_widget_selection_range::TableWidgetSelectionRange
    where Args: overloading::TableWidgetSelectionRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::rightColumn() const```</span>
  ///
  ///
  pub fn right_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_rightColumn(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_rowCount(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidgetSelectionRange::topRow() const```</span>
  ///
  ///
  pub fn top_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_topRow(self as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }
}

impl Drop for ::table_widget_selection_range::TableWidgetSelectionRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QTableWidgetSelectionRange::~QTableWidgetSelectionRange()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableWidgetSelectionRange_destructor(self as *mut ::table_widget_selection_range::TableWidgetSelectionRange) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TableWidgetSelectionRange::new](../struct.TableWidgetSelectionRange.html#method.new) method.
  pub trait TableWidgetSelectionRangeNewArgs {
    fn exec(self) -> ::table_widget_selection_range::TableWidgetSelectionRange;
  }
  impl TableWidgetSelectionRangeNewArgs for () {
    fn exec(self) -> ::table_widget_selection_range::TableWidgetSelectionRange {

      {
        let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTableWidgetSelectionRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TableWidgetSelectionRangeNewArgs for &'a ::table_widget_selection_range::TableWidgetSelectionRange {
    fn exec(self) -> ::table_widget_selection_range::TableWidgetSelectionRange {
      let other = self;
      {
        let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTableWidgetSelectionRange_constructor_other(other as *const ::table_widget_selection_range::TableWidgetSelectionRange, &mut object);
        }
        object
      }
    }
  }
  impl TableWidgetSelectionRangeNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::table_widget_selection_range::TableWidgetSelectionRange {
      let top = self.0;
      let left = self.1;
      let bottom = self.2;
      let right = self.3;
      {
        let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTableWidgetSelectionRange_constructor_top_left_bottom_right(top,
                                                                                           left,
                                                                                           bottom,
                                                                                           right,
                                                                                           &mut object);
        }
        object
      }
    }
  }
}
