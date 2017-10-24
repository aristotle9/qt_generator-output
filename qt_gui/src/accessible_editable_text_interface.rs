/// C++ type: <span style='color: green;'>```QAccessibleEditableTextInterface```</span>
#[repr(C)]
pub struct AccessibleEditableTextInterface(u8);

impl AccessibleEditableTextInterface {
  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleEditableTextInterface::deleteText(int startOffset, int endOffset)```</span>
  ///
  ///
  pub fn delete_text(&mut self, start_offset: ::libc::c_int, end_offset: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleEditableTextInterface_deleteText(self as *mut ::accessible_editable_text_interface::AccessibleEditableTextInterface, start_offset, end_offset) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleEditableTextInterface::insertText(int offset, const QString& text)```</span>
  ///
  ///
  pub fn insert_text(&mut self, offset: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QAccessibleEditableTextInterface_insertText(self as *mut ::accessible_editable_text_interface::AccessibleEditableTextInterface, offset, text as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleEditableTextInterface::replaceText(int startOffset, int endOffset, const QString& text)```</span>
  ///
  ///
  pub fn replace_text(&mut self,
                      start_offset: ::libc::c_int,
                      end_offset: ::libc::c_int,
                      text: &::qt_core::string::String) {
    unsafe { ::ffi::qt_gui_c_QAccessibleEditableTextInterface_replaceText(self as *mut ::accessible_editable_text_interface::AccessibleEditableTextInterface, start_offset, end_offset, text as *const ::qt_core::string::String) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_editable_text_interface::AccessibleEditableTextInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleEditableTextInterface_delete
  }
}
