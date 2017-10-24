/// C++ type: <span style='color: green;'>```QAccessibleTextInterface```</span>
#[repr(C)]
pub struct AccessibleTextInterface(u8);

impl AccessibleTextInterface {
  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::addSelection(int startOffset, int endOffset)```</span>
  ///
  ///
  pub fn add_selection(&mut self, start_offset: ::libc::c_int, end_offset: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_addSelection(self as *mut ::accessible_text_interface::AccessibleTextInterface, start_offset, end_offset) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QString QAccessibleTextInterface::attributes(int offset, int* startOffset, int* endOffset) const```</span>
  ///
  ///
  pub unsafe fn attributes(&self,
                           offset: ::libc::c_int,
                           start_offset: *mut ::libc::c_int,
                           end_offset: *mut ::libc::c_int)
                           -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleTextInterface_attributes_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, offset, start_offset, end_offset, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTextInterface::characterCount() const```</span>
  ///
  ///
  pub fn character_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_characterCount(self as *const ::accessible_text_interface::AccessibleTextInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QRect QAccessibleTextInterface::characterRect(int offset) const```</span>
  ///
  ///
  pub fn character_rect(&self, offset: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTextInterface_characterRect_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, offset, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTextInterface::cursorPosition() const```</span>
  ///
  ///
  pub fn cursor_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_cursorPosition(self as *const ::accessible_text_interface::AccessibleTextInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTextInterface::offsetAtPoint(const QPoint& point) const```</span>
  ///
  ///
  pub fn offset_at_point(&self, point: &::qt_core::point::Point) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_offsetAtPoint(self as *const ::accessible_text_interface::AccessibleTextInterface, point as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::removeSelection(int selectionIndex)```</span>
  ///
  ///
  pub fn remove_selection(&mut self, selection_index: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_removeSelection(self as *mut ::accessible_text_interface::AccessibleTextInterface, selection_index) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::scrollToSubstring(int startIndex, int endIndex)```</span>
  ///
  ///
  pub fn scroll_to_substring(&mut self, start_index: ::libc::c_int, end_index: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_scrollToSubstring(self as *mut ::accessible_text_interface::AccessibleTextInterface, start_index, end_index) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::selection(int selectionIndex, int* startOffset, int* endOffset) const```</span>
  ///
  ///
  pub unsafe fn selection(&self,
                          selection_index: ::libc::c_int,
                          start_offset: *mut ::libc::c_int,
                          end_offset: *mut ::libc::c_int) {
    ::ffi::qt_gui_c_QAccessibleTextInterface_selection(self as *const ::accessible_text_interface::AccessibleTextInterface, selection_index, start_offset, end_offset)
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QAccessibleTextInterface::selectionCount() const```</span>
  ///
  ///
  pub fn selection_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_selectionCount(self as *const ::accessible_text_interface::AccessibleTextInterface) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::setCursorPosition(int position)```</span>
  ///
  ///
  pub fn set_cursor_position(&mut self, position: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_setCursorPosition(self as *mut ::accessible_text_interface::AccessibleTextInterface, position) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QAccessibleTextInterface::setSelection(int selectionIndex, int startOffset, int endOffset)```</span>
  ///
  ///
  pub fn set_selection(&mut self,
                       selection_index: ::libc::c_int,
                       start_offset: ::libc::c_int,
                       end_offset: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextInterface_setSelection(self as *mut ::accessible_text_interface::AccessibleTextInterface, selection_index, start_offset, end_offset) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QString QAccessibleTextInterface::text(int startOffset, int endOffset) const```</span>
  ///
  ///
  pub fn text(&self, start_offset: ::libc::c_int, end_offset: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTextInterface_text_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, start_offset, end_offset, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleTextInterface::textAfterOffset(int offset, QAccessible::TextBoundaryType boundaryType, int* startOffset, int* endOffset) const```</span>
  ///
  ///
  pub unsafe fn text_after_offset(&self,
                                  offset: ::libc::c_int,
                                  boundary_type: &::accessible::TextBoundaryType,
                                  start_offset: *mut ::libc::c_int,
                                  end_offset: *mut ::libc::c_int)
                                  -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleTextInterface_textAfterOffset_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, offset, boundary_type as *const ::accessible::TextBoundaryType, start_offset, end_offset, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleTextInterface::textAtOffset(int offset, QAccessible::TextBoundaryType boundaryType, int* startOffset, int* endOffset) const```</span>
  ///
  ///
  pub unsafe fn text_at_offset(&self,
                               offset: ::libc::c_int,
                               boundary_type: &::accessible::TextBoundaryType,
                               start_offset: *mut ::libc::c_int,
                               end_offset: *mut ::libc::c_int)
                               -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleTextInterface_textAtOffset_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, offset, boundary_type as *const ::accessible::TextBoundaryType, start_offset, end_offset, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QAccessibleTextInterface::textBeforeOffset(int offset, QAccessible::TextBoundaryType boundaryType, int* startOffset, int* endOffset) const```</span>
  ///
  ///
  pub unsafe fn text_before_offset(&self,
                                   offset: ::libc::c_int,
                                   boundary_type: &::accessible::TextBoundaryType,
                                   start_offset: *mut ::libc::c_int,
                                   end_offset: *mut ::libc::c_int)
                                   -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QAccessibleTextInterface_textBeforeOffset_to_output(self as *const ::accessible_text_interface::AccessibleTextInterface, offset, boundary_type as *const ::accessible::TextBoundaryType, start_offset, end_offset, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_text_interface::AccessibleTextInterface {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTextInterface_delete
  }
}
