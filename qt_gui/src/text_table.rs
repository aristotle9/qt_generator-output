/// C++ type: <span style='color: green;'>```QTextTable```</span>
#[repr(C)]
pub struct TextTable(u8);

impl TextTable {
  /// C++ method: <span style='color: green;'>```void QTextTable::appendColumns(int count)```</span>
  ///
  ///
  pub fn append_columns(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_appendColumns(self as *mut ::text_table::TextTable, count) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::appendRows(int count)```</span>
  ///
  ///
  pub fn append_rows(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_appendRows(self as *mut ::text_table::TextTable, count) }
  }

  /// C++ method: <span style='color: green;'>```QTextTable::cellAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cell_at(&self, &::text_cursor::TextCursor) -> ::text_table_cell::TextTableCell```<br>
  /// C++ method: <span style='color: green;'>```QTextTableCell QTextTable::cellAt(const QTextCursor& c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cell_at(&self, ::libc::c_int) -> ::text_table_cell::TextTableCell```<br>
  /// C++ method: <span style='color: green;'>```QTextTableCell QTextTable::cellAt(int position) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn cell_at(&self, (::libc::c_int, ::libc::c_int)) -> ::text_table_cell::TextTableCell```<br>
  /// C++ method: <span style='color: green;'>```QTextTableCell QTextTable::cellAt(int row, int col) const```</span>
  ///
  ///
  pub fn cell_at<'largs, Args>(&'largs self, args: Args) -> ::text_table_cell::TextTableCell
    where Args: overloading::TextTableCellAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QTextTable::columns() const```</span>
  ///
  ///
  pub fn columns(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTable_columns(self as *const ::text_table::TextTable) }
  }

  /// C++ method: <span style='color: green;'>```QTextTableFormat QTextTable::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::text_table_format::TextTableFormat {
    {
      let mut object: ::text_table_format::TextTableFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTable_format_to_output(self as *const ::text_table::TextTable, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::insertColumns(int pos, int num)```</span>
  ///
  ///
  pub fn insert_columns(&mut self, pos: ::libc::c_int, num: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_insertColumns(self as *mut ::text_table::TextTable, pos, num) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::insertRows(int pos, int num)```</span>
  ///
  ///
  pub fn insert_rows(&mut self, pos: ::libc::c_int, num: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_insertRows(self as *mut ::text_table::TextTable, pos, num) }
  }

  /// C++ method: <span style='color: green;'>```QTextTable::mergeCells```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn merge_cells(&mut self, &::text_cursor::TextCursor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextTable::mergeCells(const QTextCursor& cursor)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn merge_cells(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextTable::mergeCells(int row, int col, int numRows, int numCols)```</span>
  ///
  ///
  pub fn merge_cells<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextTableMergeCellsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextTable::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextTable_metaObject(self as *const ::text_table::TextTable) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextTable::QTextTable(QTextDocument* doc)```</span>
  ///
  ///
  pub unsafe fn new(doc: *mut ::text_document::TextDocument) -> ::cpp_utils::CppBox<::text_table::TextTable> {
    let ffi_result = ::ffi::qt_gui_c_QTextTable_new(doc);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextTable::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextTable_qt_metacall(self as *mut ::text_table::TextTable,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextTable::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextTable_qt_metacast(self as *mut ::text_table::TextTable, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::removeColumns(int pos, int num)```</span>
  ///
  ///
  pub fn remove_columns(&mut self, pos: ::libc::c_int, num: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_removeColumns(self as *mut ::text_table::TextTable, pos, num) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::removeRows(int pos, int num)```</span>
  ///
  ///
  pub fn remove_rows(&mut self, pos: ::libc::c_int, num: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_removeRows(self as *mut ::text_table::TextTable, pos, num) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::resize(int rows, int cols)```</span>
  ///
  ///
  pub fn resize(&mut self, rows: ::libc::c_int, cols: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTable_resize(self as *mut ::text_table::TextTable, rows, cols) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextTable::rowEnd(const QTextCursor& c) const```</span>
  ///
  ///
  pub fn row_end(&self, c: &::text_cursor::TextCursor) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextTable_rowEnd_as_ptr(self as *const ::text_table::TextTable,
                                               c as *const ::text_cursor::TextCursor)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextTable::rowStart(const QTextCursor& c) const```</span>
  ///
  ///
  pub fn row_start(&self, c: &::text_cursor::TextCursor) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextTable_rowStart_as_ptr(self as *const ::text_table::TextTable,
                                                 c as *const ::text_cursor::TextCursor)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTable::rows() const```</span>
  ///
  ///
  pub fn rows(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTable_rows(self as *const ::text_table::TextTable) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::setFormat(const QTextTableFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::text_table_format::TextTableFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextTable_setFormat(self as *mut ::text_table::TextTable,
                                           format as *const ::text_table_format::TextTableFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTable::splitCell(int row, int col, int numRows, int numCols)```</span>
  ///
  ///
  pub fn split_cell(&mut self,
                    row: ::libc::c_int,
                    col: ::libc::c_int,
                    num_rows: ::libc::c_int,
                    num_cols: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QTextTable_splitCell(self as *mut ::text_table::TextTable,
                                           row,
                                           col,
                                           num_rows,
                                           num_cols)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextTable::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextTable_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextTable::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextTable_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::text_table::TextTable {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextTable_delete
  }
}

impl ::cpp_utils::DynamicCast<::text_table::TextTable> for ::text_frame::TextFrame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_table::TextTable> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextFrame(self as *mut ::text_frame::TextFrame)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_table::TextTable> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextFrame(self as *const ::text_frame::TextFrame as *mut ::text_frame::TextFrame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_table::TextTable> for ::text_object::TextObject {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_table::TextTable> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextObject(self as *mut ::text_object::TextObject)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_table::TextTable> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_dynamic_cast_QTextTable_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_table::TextTable {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QObject_ptr(self as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QObject_ptr(self as *const ::text_table::TextTable as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_frame::TextFrame> for ::text_table::TextTable {
  fn static_cast_mut(&mut self) -> &mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(self as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(self as *const ::text_table::TextTable as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_object::TextObject> for ::text_table::TextTable {
  fn static_cast_mut(&mut self) -> &mut ::text_object::TextObject {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextObject_ptr(self as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_object::TextObject {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextObject_ptr(self as *const ::text_table::TextTable as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table::TextTable> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table::TextTable {
    let ffi_result =
      ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table::TextTable {
    let ffi_result = ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table::TextTable> for ::text_frame::TextFrame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table::TextTable {
    let ffi_result =
      ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextFrame(self as *mut ::text_frame::TextFrame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table::TextTable {
    let ffi_result = ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextFrame(self as *const ::text_frame::TextFrame as *mut ::text_frame::TextFrame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table::TextTable> for ::text_object::TextObject {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table::TextTable {
    let ffi_result =
      ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextObject(self as *mut ::text_object::TextObject);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table::TextTable {
    let ffi_result = ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextTable_ptr_QTextObject(self as *const ::text_object::TextObject as *mut ::text_object::TextObject);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_table::TextTable {
  type Target = ::text_frame::TextFrame;
  fn deref(&self) -> &::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(self as *const ::text_table::TextTable as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_table::TextTable {
  fn deref_mut(&mut self) -> &mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextTable_G_static_cast_QTextFrame_ptr(self as *mut ::text_table::TextTable) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextTable::cell_at](../struct.TextTable.html#method.cell_at) method.
  pub trait TextTableCellAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_table::TextTable) -> ::text_table_cell::TextTableCell;
  }
  impl<'largs> TextTableCellAtArgs<'largs> for &'largs ::text_cursor::TextCursor {
    fn exec(self, original_self: &'largs ::text_table::TextTable) -> ::text_table_cell::TextTableCell {
      let c = self;
      {
        let mut object: ::text_table_cell::TextTableCell =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextTable_cellAt_to_output_c(original_self as *const ::text_table::TextTable,
                                                        c as *const ::text_cursor::TextCursor,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextTableCellAtArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_table::TextTable) -> ::text_table_cell::TextTableCell {
      let position = self;
      {
        let mut object: ::text_table_cell::TextTableCell =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextTable_cellAt_to_output_position(original_self as *const ::text_table::TextTable,
                                                               position,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextTableCellAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::text_table::TextTable) -> ::text_table_cell::TextTableCell {
      let row = self.0;
      let col = self.1;
      {
        let mut object: ::text_table_cell::TextTableCell =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextTable_cellAt_to_output_row_col(original_self as *const ::text_table::TextTable,
                                                              row,
                                                              col,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextTable::merge_cells](../struct.TextTable.html#method.merge_cells) method.
  pub trait TextTableMergeCellsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_table::TextTable) -> ();
  }
  impl<'largs> TextTableMergeCellsArgs<'largs> for &'largs ::text_cursor::TextCursor {
    fn exec(self, original_self: &'largs mut ::text_table::TextTable) -> () {
      let cursor = self;
      unsafe {
        ::ffi::qt_gui_c_QTextTable_mergeCells_cursor(original_self as *mut ::text_table::TextTable,
                                                     cursor as *const ::text_cursor::TextCursor)
      }
    }
  }
  impl<'largs> TextTableMergeCellsArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::text_table::TextTable) -> () {
      let row = self.0;
      let col = self.1;
      let num_rows = self.2;
      let num_cols = self.3;
      unsafe {
        ::ffi::qt_gui_c_QTextTable_mergeCells_row_col_numRows_numCols(original_self as *mut ::text_table::TextTable,
                                                                      row,
                                                                      col,
                                                                      num_rows,
                                                                      num_cols)
      }
    }
  }
}
