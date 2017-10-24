/// C++ type: <span style='color: green;'>```QTextTableFormat```</span>
#[repr(C)]
pub struct TextTableFormat([u8; ::type_sizes::QT_GUI_TEXT_TABLE_FORMAT_TEXT_TABLE_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextTableFormat {
  unsafe fn new_uninitialized() -> TextTableFormat {
    TextTableFormat(::std::mem::uninitialized())
  }
}

impl TextTableFormat {
  /// C++ method: <span style='color: green;'>```double QTextTableFormat::cellPadding() const```</span>
  ///
  ///
  pub fn cell_padding(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_cellPadding(self as *const ::text_table_format::TextTableFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextTableFormat::cellSpacing() const```</span>
  ///
  ///
  pub fn cell_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_cellSpacing(self as *const ::text_table_format::TextTableFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::clearColumnWidthConstraints()```</span>
  ///
  ///
  pub fn clear_column_width_constraints(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableFormat_clearColumnWidthConstraints(self as *mut ::text_table_format::TextTableFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLength> QTextTableFormat::columnWidthConstraints() const```</span>
  ///
  ///
  pub fn column_width_constraints(&self) -> ::vector::VectorTextLength {
    {
      let mut object: ::vector::VectorTextLength =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableFormat_columnWidthConstraints_to_output(self as *const ::text_table_format::TextTableFormat, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableFormat::columns() const```</span>
  ///
  ///
  pub fn columns(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_columns(self as *const ::text_table_format::TextTableFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextTableFormat::headerRowCount() const```</span>
  ///
  ///
  pub fn header_row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_headerRowCount(self as *const ::text_table_format::TextTableFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextTableFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_isValid(self as *const ::text_table_format::TextTableFormat) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextTableFormat::QTextTableFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_table_format::TextTableFormat {
    {
      let mut object: ::text_table_format::TextTableFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::setCellPadding(double padding)```</span>
  ///
  ///
  pub fn set_cell_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableFormat_setCellPadding(self as *mut ::text_table_format::TextTableFormat, padding)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::setCellSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_cell_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableFormat_setCellSpacing(self as *mut ::text_table_format::TextTableFormat, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::setColumnWidthConstraints(const QVector<QTextLength>& constraints)```</span>
  ///
  ///
  pub fn set_column_width_constraints(&mut self, constraints: &::vector::VectorTextLength) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableFormat_setColumnWidthConstraints(self as *mut ::text_table_format::TextTableFormat,
                                                                 constraints as *const ::vector::VectorTextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::setColumns(int columns)```</span>
  ///
  ///
  pub fn set_columns(&mut self, columns: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_setColumns(self as *mut ::text_table_format::TextTableFormat, columns) }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableFormat::setHeaderRowCount(int count)```</span>
  ///
  ///
  pub fn set_header_row_count(&mut self, count: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableFormat_setHeaderRowCount(self as *mut ::text_table_format::TextTableFormat, count)
    }
  }
}

impl Drop for ::text_table_format::TextTableFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextTableFormat::~QTextTableFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextTableFormat_destructor(self as *mut ::text_table_format::TextTableFormat) }
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_table_format::TextTableFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_table_format::TextTableFormat as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_frame_format::TextFrameFormat> for ::text_table_format::TextTableFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_frame_format::TextFrameFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(self as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_frame_format::TextFrameFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(self as *const ::text_table_format::TextTableFormat as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table_format::TextTableFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table_format::TextTableFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFormat(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table_format::TextTableFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFormat(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table_format::TextTableFormat> for ::text_frame_format::TextFrameFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table_format::TextTableFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFrameFormat(self as *mut ::text_frame_format::TextFrameFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table_format::TextTableFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextTableFormat_ptr_QTextFrameFormat(self as *const ::text_frame_format::TextFrameFormat as *mut ::text_frame_format::TextFrameFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_table_format::TextTableFormat {
  type Target = ::text_frame_format::TextFrameFormat;
  fn deref(&self) -> &::text_frame_format::TextFrameFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(self as *const ::text_table_format::TextTableFormat as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_table_format::TextTableFormat {
  fn deref_mut(&mut self) -> &mut ::text_frame_format::TextFrameFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableFormat_G_static_cast_QTextFrameFormat_ptr(self as *mut ::text_table_format::TextTableFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
