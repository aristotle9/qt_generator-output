/// C++ type: <span style='color: green;'>```QAccessibleTableCellInterface```</span>
#[repr(C)]
pub struct AccessibleTableCellInterface(u8);

impl AccessibleTableCellInterface {
  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableCellInterface::columnExtent() const```</span>
  ///
  ///
  pub fn column_extent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_columnExtent(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QList<QAccessibleInterface*> QAccessibleTableCellInterface::columnHeaderCells() const```</span>
  ///
  ///
  pub fn column_header_cells(&self) -> ::list::ListAccessibleInterfaceMutPtr {
    {
      let mut object: ::list::ListAccessibleInterfaceMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableCellInterface_columnHeaderCells_to_output(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableCellInterface::columnIndex() const```</span>
  ///
  ///
  pub fn column_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_columnIndex(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QAccessibleTableCellInterface::isSelected() const```</span>
  ///
  ///
  pub fn is_selected(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_isSelected(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableCellInterface::rowExtent() const```</span>
  ///
  ///
  pub fn row_extent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_rowExtent(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QList<QAccessibleInterface*> QAccessibleTableCellInterface::rowHeaderCells() const```</span>
  ///
  ///
  pub fn row_header_cells(&self) -> ::list::ListAccessibleInterfaceMutPtr {
    {
      let mut object: ::list::ListAccessibleInterfaceMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTableCellInterface_rowHeaderCells_to_output(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTableCellInterface::rowIndex() const```</span>
  ///
  ///
  pub fn row_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_rowIndex(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QAccessibleInterface* QAccessibleTableCellInterface::table() const```</span>
  ///
  ///
  pub fn table(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleTableCellInterface_table(self as *const ::accessible_table_cell_interface::AccessibleTableCellInterface) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_table_cell_interface::AccessibleTableCellInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTableCellInterface_delete
  }
}
