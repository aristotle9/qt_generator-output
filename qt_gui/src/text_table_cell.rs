/// C++ type: <span style='color: green;'>```QTextTableCell```</span>
#[repr(C)]
pub struct TextTableCell([u8; ::type_sizes::QT_GUI_TEXT_TABLE_CELL_TEXT_TABLE_CELL]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextTableCell {
  unsafe fn new_uninitialized() -> TextTableCell {
    TextTableCell(::std::mem::uninitialized())
  }
}

impl TextTableCell {
  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextTableCell::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableCell_begin_to_output(self as *const ::text_table_cell::TextTableCell, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::column() const```</span>
  ///
  ///
  pub fn column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_column(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::columnSpan() const```</span>
  ///
  ///
  pub fn column_span(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_columnSpan(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame::iterator QTextTableCell::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::text_frame::Iterator {
    {
      let mut object: ::text_frame::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableCell_end_to_output(self as *const ::text_table_cell::TextTableCell, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextTableCell::firstCursorPosition() const```</span>
  ///
  ///
  pub fn first_cursor_position(&self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextTableCell_firstCursorPosition_as_ptr(self as *const ::text_table_cell::TextTableCell)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::firstPosition() const```</span>
  ///
  ///
  pub fn first_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_firstPosition(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextTableCell::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableCell_format_to_output(self as *const ::text_table_cell::TextTableCell, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextTableCell::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_isValid(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextTableCell::lastCursorPosition() const```</span>
  ///
  ///
  pub fn last_cursor_position(&self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextTableCell_lastCursorPosition_as_ptr(self as *const ::text_table_cell::TextTableCell)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::lastPosition() const```</span>
  ///
  ///
  pub fn last_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_lastPosition(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```QTextTableCell::QTextTableCell```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_table_cell::TextTableCell```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextTableCell::QTextTableCell()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_table_cell::TextTableCell) -> ::text_table_cell::TextTableCell```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextTableCell::QTextTableCell(const QTextTableCell& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_table_cell::TextTableCell
    where Args: overloading::TextTableCellNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextTableCell& QTextTableCell::operator=(const QTextTableCell& o)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             o: &'l1 ::text_table_cell::TextTableCell)
                             -> &'l0 mut ::text_table_cell::TextTableCell {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextTableCell_operator_assign(self as *mut ::text_table_cell::TextTableCell,
                                                     o as *const ::text_table_cell::TextTableCell)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTextTableCell::operator==(const QTextTableCell& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::text_table_cell::TextTableCell) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCell_operator_eq(self as *const ::text_table_cell::TextTableCell,
                                                 other as *const ::text_table_cell::TextTableCell)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextTableCell::operator!=(const QTextTableCell& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::text_table_cell::TextTableCell) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCell_operator_neq(self as *const ::text_table_cell::TextTableCell,
                                                  other as *const ::text_table_cell::TextTableCell)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_row(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::rowSpan() const```</span>
  ///
  ///
  pub fn row_span(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_rowSpan(self as *const ::text_table_cell::TextTableCell) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCell::setFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCell_setFormat(self as *mut ::text_table_cell::TextTableCell,
                                               format as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableCell::tableCellFormatIndex() const```</span>
  ///
  ///
  pub fn table_cell_format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_tableCellFormatIndex(self as *const ::text_table_cell::TextTableCell) }
  }
}

impl Drop for ::text_table_cell::TextTableCell {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextTableCell::~QTextTableCell()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextTableCell_destructor(self as *mut ::text_table_cell::TextTableCell) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextTableCell::new](../struct.TextTableCell.html#method.new) method.
  pub trait TextTableCellNewArgs {
    fn exec(self) -> ::text_table_cell::TextTableCell;
  }
  impl TextTableCellNewArgs for () {
    fn exec(self) -> ::text_table_cell::TextTableCell {

      {
        let mut object: ::text_table_cell::TextTableCell =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextTableCell_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextTableCellNewArgs for &'a ::text_table_cell::TextTableCell {
    fn exec(self) -> ::text_table_cell::TextTableCell {
      let o = self;
      {
        let mut object: ::text_table_cell::TextTableCell =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextTableCell_constructor_o(o as *const ::text_table_cell::TextTableCell, &mut object);
        }
        object
      }
    }
  }
}
