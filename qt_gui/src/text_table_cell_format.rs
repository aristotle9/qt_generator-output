/// C++ type: <span style='color: green;'>```QTextTableCellFormat```</span>
#[repr(C)]
pub struct TextTableCellFormat([u8; ::type_sizes::QT_GUI_TEXT_TABLE_CELL_FORMAT_TEXT_TABLE_CELL_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextTableCellFormat {
  unsafe fn new_uninitialized() -> TextTableCellFormat {
    TextTableCellFormat(::std::mem::uninitialized())
  }
}

impl TextTableCellFormat {
  /// C++ method: <span style='color: green;'>```double QTextTableCellFormat::bottomPadding() const```</span>
  ///
  ///
  pub fn bottom_padding(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_bottomPadding(self as *const ::text_table_cell_format::TextTableCellFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextTableCellFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_isValid(self as *const ::text_table_cell_format::TextTableCellFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextTableCellFormat::leftPadding() const```</span>
  ///
  ///
  pub fn left_padding(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_leftPadding(self as *const ::text_table_cell_format::TextTableCellFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextTableCellFormat::QTextTableCellFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_table_cell_format::TextTableCellFormat {
    {
      let mut object: ::text_table_cell_format::TextTableCellFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextTableCellFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextTableCellFormat::rightPadding() const```</span>
  ///
  ///
  pub fn right_padding(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_rightPadding(self as *const ::text_table_cell_format::TextTableCellFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCellFormat::setBottomPadding(double padding)```</span>
  ///
  ///
  pub fn set_bottom_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_setBottomPadding(self as *mut ::text_table_cell_format::TextTableCellFormat, padding)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCellFormat::setLeftPadding(double padding)```</span>
  ///
  ///
  pub fn set_left_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_setLeftPadding(self as *mut ::text_table_cell_format::TextTableCellFormat,
                                                          padding)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCellFormat::setPadding(double padding)```</span>
  ///
  ///
  pub fn set_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_setPadding(self as *mut ::text_table_cell_format::TextTableCellFormat,
                                                      padding)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCellFormat::setRightPadding(double padding)```</span>
  ///
  ///
  pub fn set_right_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_setRightPadding(self as *mut ::text_table_cell_format::TextTableCellFormat,
                                                           padding)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextTableCellFormat::setTopPadding(double padding)```</span>
  ///
  ///
  pub fn set_top_padding(&mut self, padding: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_setTopPadding(self as *mut ::text_table_cell_format::TextTableCellFormat,
                                                         padding)
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextTableCellFormat::topPadding() const```</span>
  ///
  ///
  pub fn top_padding(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_topPadding(self as *const ::text_table_cell_format::TextTableCellFormat)
    }
  }
}

impl Drop for ::text_table_cell_format::TextTableCellFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextTableCellFormat::~QTextTableCellFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QTextTableCellFormat_destructor(self as *mut ::text_table_cell_format::TextTableCellFormat)
    }
  }
}

impl ::cpp_utils::StaticCast<::text_char_format::TextCharFormat> for ::text_table_cell_format::TextTableCellFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(self as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(self as *const ::text_table_cell_format::TextTableCellFormat as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_table_cell_format::TextTableCellFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_table_cell_format::TextTableCellFormat as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_table_cell_format::TextTableCellFormat> for ::text_char_format::TextCharFormat {
unsafe fn static_cast_mut(&mut self) -> &mut ::text_table_cell_format::TextTableCellFormat {
let ffi_result = ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextCharFormat(self as *mut ::text_char_format::TextCharFormat);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::text_table_cell_format::TextTableCellFormat {
let ffi_result = ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextCharFormat(self as *const ::text_char_format::TextCharFormat as *mut ::text_char_format::TextCharFormat);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::text_table_cell_format::TextTableCellFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_table_cell_format::TextTableCellFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextFormat(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_table_cell_format::TextTableCellFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextTableCellFormat_ptr_QTextFormat(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_table_cell_format::TextTableCellFormat {
  type Target = ::text_char_format::TextCharFormat;
  fn deref(&self) -> &::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(self as *const ::text_table_cell_format::TextTableCellFormat as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_table_cell_format::TextTableCellFormat {
  fn deref_mut(&mut self) -> &mut ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextTableCellFormat_G_static_cast_QTextCharFormat_ptr(self as *mut ::text_table_cell_format::TextTableCellFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
