/// C++ type: <span style='color: green;'>```QAccessibleTableInterface```</span>
#[repr(C)]
pub struct AccessibleTableInterface(u8);

impl AccessibleTableInterface {
  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleTableInterface::caption() const```</span>
  ///
  ///
  pub fn caption(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_caption(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleTableInterface::cellAt(int row, int column) const```</span>
  ///
  ///
  pub fn cell_at(&self, row: ::libc::c_int, column: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_cellAt(self as *const ::accessible_table_interface::AccessibleTableInterface, row, column) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableInterface::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_columnCount(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QString QAccessibleTableInterface::columnDescription(int column) const```</span>
  ///
  ///
  pub fn column_description(&self, column: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableInterface_columnDescription_to_output(self as *const ::accessible_table_interface::AccessibleTableInterface, column, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::isColumnSelected(int column) const```</span>
  ///
  ///
  pub fn is_column_selected(&self, column: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_isColumnSelected(self as *const ::accessible_table_interface::AccessibleTableInterface, column) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::isRowSelected(int row) const```</span>
  ///
  ///
  pub fn is_row_selected(&self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_isRowSelected(self as *const ::accessible_table_interface::AccessibleTableInterface, row) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTableInterface::modelChange(QAccessibleTableModelChangeEvent* event)```</span>
  ///
  ///
  pub unsafe fn model_change(&mut self,
                             event: *mut ::accessible_table_model_change_event::AccessibleTableModelChangeEvent) {
    ::ffi::qt_gui_c_QAccessibleTableInterface_modelChange(self as *mut ::accessible_table_interface::AccessibleTableInterface, event)
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableInterface::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_rowCount(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QString QAccessibleTableInterface::rowDescription(int row) const```</span>
  ///
  ///
  pub fn row_description(&self, row: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableInterface_rowDescription_to_output(self as *const ::accessible_table_interface::AccessibleTableInterface, row, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::selectColumn(int column)```</span>
  ///
  ///
  pub fn select_column(&mut self, column: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_selectColumn(self as *mut ::accessible_table_interface::AccessibleTableInterface, column) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::selectRow(int row)```</span>
  ///
  ///
  pub fn select_row(&mut self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_selectRow(self as *mut ::accessible_table_interface::AccessibleTableInterface, row) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableInterface::selectedCellCount() const```</span>
  ///
  ///
  pub fn selected_cell_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_selectedCellCount(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QList<QAccessibleInterface*> QAccessibleTableInterface::selectedCells() const```</span>
  ///
  ///
  pub fn selected_cells(&self) -> ::list::ListAccessibleInterfaceMutPtr {
    {
      let mut object: ::list::ListAccessibleInterfaceMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableInterface_selectedCells_to_output(self as *const ::accessible_table_interface::AccessibleTableInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableInterface::selectedColumnCount() const```</span>
  ///
  ///
  pub fn selected_column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_selectedColumnCount(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QList<int> QAccessibleTableInterface::selectedColumns() const```</span>
  ///
  ///
  pub fn selected_columns(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableInterface_selectedColumns_to_output(self as *const ::accessible_table_interface::AccessibleTableInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableInterface::selectedRowCount() const```</span>
  ///
  ///
  pub fn selected_row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_selectedRowCount(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QList<int> QAccessibleTableInterface::selectedRows() const```</span>
  ///
  ///
  pub fn selected_rows(&self) -> ::qt_core::list::ListCInt {
    {
      let mut object: ::qt_core::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableInterface_selectedRows_to_output(self as *const ::accessible_table_interface::AccessibleTableInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleTableInterface::summary() const```</span>
  ///
  ///
  pub fn summary(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_summary(self as *const ::accessible_table_interface::AccessibleTableInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::unselectColumn(int column)```</span>
  ///
  ///
  pub fn unselect_column(&mut self, column: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_unselectColumn(self as *mut ::accessible_table_interface::AccessibleTableInterface, column) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableInterface::unselectRow(int row)```</span>
  ///
  ///
  pub fn unselect_row(&mut self, row: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableInterface_unselectRow(self as *mut ::accessible_table_interface::AccessibleTableInterface, row) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_table_interface::AccessibleTableInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTableInterface_delete
  }
}
