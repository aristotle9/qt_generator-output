/// C++ type: <span style='color: green;'>```QPlainTextEdit::LineWrapMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LineWrapMode {
  /// C++ enum variant: <span style='color: green;'>```NoWrap = 0```</span>
  NoWrap = 0,
  /// C++ enum variant: <span style='color: green;'>```WidgetWidth = 1```</span>
  WidgetWidth = 1,
}

/// C++ type: <span style='color: green;'>```QPlainTextEdit```</span>
#[repr(C)]
pub struct PlainTextEdit(u8);

impl PlainTextEdit {
  /// C++ method: <span style='color: green;'>```QString QPlainTextEdit::anchorAt(const QPoint& pos) const```</span>
  ///
  ///
  pub fn anchor_at(&self, pos: &::qt_core::point::Point) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_anchorAt_to_output(self as *const ::plain_text_edit::PlainTextEdit,
                                                              pos as *const ::qt_core::point::Point,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::appendHtml(const QString& html)```</span>
  ///
  ///
  pub fn append_html(&mut self, html: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_appendHtml(self as *mut ::plain_text_edit::PlainTextEdit,
                                                    html as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::appendPlainText(const QString& text)```</span>
  ///
  ///
  pub fn append_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_appendPlainText(self as *mut ::plain_text_edit::PlainTextEdit,
                                                         text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::backgroundVisible() const```</span>
  ///
  ///
  pub fn background_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_backgroundVisible(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QPlainTextEdit::blockCount() const```</span>
  ///
  ///
  pub fn block_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_blockCount(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::canPaste() const```</span>
  ///
  ///
  pub fn can_paste(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_canPaste(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::centerCursor()```</span>
  ///
  ///
  pub fn center_cursor(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_centerCursor(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::centerOnScroll() const```</span>
  ///
  ///
  pub fn center_on_scroll(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_centerOnScroll(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_clear(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::copy()```</span>
  ///
  ///
  pub fn copy(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_copy(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::createStandardContextMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_standard_context_menu(&mut self, ()) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QPlainTextEdit::createStandardContextMenu()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_standard_context_menu(&mut self, &::qt_core::point::Point) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QPlainTextEdit::createStandardContextMenu(const QPoint& position)```</span>
  ///
  ///
  pub fn create_standard_context_menu<'largs, Args>(&'largs mut self, args: Args) -> *mut ::menu::Menu
    where Args: overloading::PlainTextEditCreateStandardContextMenuArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCharFormat QPlainTextEdit::currentCharFormat() const```</span>
  ///
  ///
  pub fn current_char_format(&self) -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_currentCharFormat_to_output(self as *const ::plain_text_edit::PlainTextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QPlainTextEdit::cursorForPosition(const QPoint& pos) const```</span>
  ///
  ///
  pub fn cursor_for_position(&self,
                             pos: &::qt_core::point::Point)
                             -> ::cpp_utils::CppBox<::qt_gui::text_cursor::TextCursor> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_cursorForPosition_as_ptr(self as *const ::plain_text_edit::PlainTextEdit,
                                                                    pos as *const ::qt_core::point::Point)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::cursorRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_rect(&self, ()) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QPlainTextEdit::cursorRect() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_rect(&self, &::qt_gui::text_cursor::TextCursor) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QPlainTextEdit::cursorRect(const QTextCursor& cursor) const```</span>
  ///
  ///
  pub fn cursor_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::PlainTextEditCursorRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QPlainTextEdit::cursorWidth() const```</span>
  ///
  ///
  pub fn cursor_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_cursorWidth(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::cut()```</span>
  ///
  ///
  pub fn cut(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_cut(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QPlainTextEdit::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::qt_gui::text_document::TextDocument {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_document(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QPlainTextEdit::documentTitle() const```</span>
  ///
  ///
  pub fn document_title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_documentTitle_to_output(self as *const ::plain_text_edit::PlainTextEdit,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::ensureCursorVisible()```</span>
  ///
  ///
  pub fn ensure_cursor_visible(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_ensureCursorVisible(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection> QPlainTextEdit::extraSelections() const```</span>
  ///
  ///
  pub fn extra_selections(&self) -> ::list::ListTextEditExtraSelection {
    {
      let mut object: ::list::ListTextEditExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_extraSelections_to_output(self as *const ::plain_text_edit::PlainTextEdit,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::inputMethodQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn input_method_query(&self, (&::qt_core::qt::InputMethodQuery, &::qt_core::variant::Variant)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QPlainTextEdit::inputMethodQuery(Qt::InputMethodQuery query, QVariant argument) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn input_method_query(&self, &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QPlainTextEdit::inputMethodQuery(Qt::InputMethodQuery property) const```</span>
  ///
  ///
  pub fn input_method_query<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::PlainTextEditInputMethodQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::insertPlainText(const QString& text)```</span>
  ///
  ///
  pub fn insert_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_insertPlainText(self as *mut ::plain_text_edit::PlainTextEdit,
                                                         text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_isReadOnly(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::isUndoRedoEnabled() const```</span>
  ///
  ///
  pub fn is_undo_redo_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_isUndoRedoEnabled(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::LineWrapMode QPlainTextEdit::lineWrapMode() const```</span>
  ///
  ///
  pub fn line_wrap_mode(&self) -> ::plain_text_edit::LineWrapMode {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_lineWrapMode(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QPlainTextEdit::loadResource(int type, const QUrl& name)```</span>
  ///
  ///
  pub fn load_resource(&mut self, type_: ::libc::c_int, name: &::qt_core::url::Url) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_loadResource_to_output(self as *mut ::plain_text_edit::PlainTextEdit,
                                                                  type_,
                                                                  name as *const ::qt_core::url::Url,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPlainTextEdit::maximumBlockCount() const```</span>
  ///
  ///
  pub fn maximum_block_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_maximumBlockCount(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::mergeCurrentCharFormat(const QTextCharFormat& modifier)```</span>
  ///
  ///
  pub fn merge_current_char_format(&mut self, modifier: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_mergeCurrentCharFormat(self as *mut ::plain_text_edit::PlainTextEdit, modifier as *const ::qt_gui::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPlainTextEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_metaObject(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::moveCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_cursor(&mut self, &::qt_gui::text_cursor::MoveOperation) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::moveCursor(QTextCursor::MoveOperation operation)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_cursor(&mut self, (&::qt_gui::text_cursor::MoveOperation, &::qt_gui::text_cursor::MoveMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::moveCursor(QTextCursor::MoveOperation operation, QTextCursor::MoveMode mode = ?)```</span>
  ///
  ///
  pub fn move_cursor<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PlainTextEditMoveCursorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPlainTextEdit::QPlainTextEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPlainTextEdit::QPlainTextEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPlainTextEdit::QPlainTextEdit(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>
    where Args: overloading::PlainTextEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPlainTextEdit::QPlainTextEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPlainTextEdit::QPlainTextEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPlainTextEdit::QPlainTextEdit(const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>
    where Args: overloading::PlainTextEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::overwriteMode() const```</span>
  ///
  ///
  pub fn overwrite_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_overwriteMode(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::paste()```</span>
  ///
  ///
  pub fn paste(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_paste(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QPlainTextEdit::placeholderText() const```</span>
  ///
  ///
  pub fn placeholder_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_placeholderText_to_output(self as *const ::plain_text_edit::PlainTextEdit,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::print(QPagedPaintDevice* printer) const```</span>
  ///
  ///
  pub unsafe fn print(&self, printer: *mut ::qt_gui::paged_paint_device::PagedPaintDevice) {
    ::ffi::qt_widgets_c_QPlainTextEdit_print(self as *const ::plain_text_edit::PlainTextEdit, printer)
  }

  /// C++ method: <span style='color: green;'>```virtual int QPlainTextEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QPlainTextEdit_qt_metacall(self as *mut ::plain_text_edit::PlainTextEdit,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QPlainTextEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QPlainTextEdit_qt_metacast(self as *mut ::plain_text_edit::PlainTextEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_redo(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_selectAll(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setBackgroundVisible(bool visible)```</span>
  ///
  ///
  pub fn set_background_visible(&mut self, visible: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setBackgroundVisible(self as *mut ::plain_text_edit::PlainTextEdit, visible)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setCenterOnScroll(bool enabled)```</span>
  ///
  ///
  pub fn set_center_on_scroll(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setCenterOnScroll(self as *mut ::plain_text_edit::PlainTextEdit, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setCurrentCharFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_current_char_format(&mut self, format: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setCurrentCharFormat(self as *mut ::plain_text_edit::PlainTextEdit, format as *const ::qt_gui::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setCursorWidth(int width)```</span>
  ///
  ///
  pub fn set_cursor_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setCursorWidth(self as *mut ::plain_text_edit::PlainTextEdit, width) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setDocument(QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn set_document(&mut self, document: *mut ::qt_gui::text_document::TextDocument) {
    ::ffi::qt_widgets_c_QPlainTextEdit_setDocument(self as *mut ::plain_text_edit::PlainTextEdit, document)
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setDocumentTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_document_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setDocumentTitle(self as *mut ::plain_text_edit::PlainTextEdit,
                                                          title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setExtraSelections(const QList<QTextEdit::ExtraSelection>& selections)```</span>
  ///
  ///
  pub fn set_extra_selections(&mut self, selections: &::list::ListTextEditExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setExtraSelections(self as *mut ::plain_text_edit::PlainTextEdit,
                                                            selections as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setLineWrapMode(QPlainTextEdit::LineWrapMode mode)```</span>
  ///
  ///
  pub fn set_line_wrap_mode(&mut self, mode: ::plain_text_edit::LineWrapMode) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setLineWrapMode(self as *mut ::plain_text_edit::PlainTextEdit, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setMaximumBlockCount(int maximum)```</span>
  ///
  ///
  pub fn set_maximum_block_count(&mut self, maximum: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setMaximumBlockCount(self as *mut ::plain_text_edit::PlainTextEdit, maximum)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setOverwriteMode(bool overwrite)```</span>
  ///
  ///
  pub fn set_overwrite_mode(&mut self, overwrite: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setOverwriteMode(self as *mut ::plain_text_edit::PlainTextEdit, overwrite)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setPlaceholderText(const QString& placeholderText)```</span>
  ///
  ///
  pub fn set_placeholder_text(&mut self, placeholder_text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setPlaceholderText(self as *mut ::plain_text_edit::PlainTextEdit,
                                                            placeholder_text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::setPlainText(const QString& text)```</span>
  ///
  ///
  pub fn set_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setPlainText(self as *mut ::plain_text_edit::PlainTextEdit,
                                                      text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setReadOnly(bool ro)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, ro: bool) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setReadOnly(self as *mut ::plain_text_edit::PlainTextEdit, ro) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setTabChangesFocus(bool b)```</span>
  ///
  ///
  pub fn set_tab_changes_focus(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setTabChangesFocus(self as *mut ::plain_text_edit::PlainTextEdit, b) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setTabStopWidth(int width)```</span>
  ///
  ///
  pub fn set_tab_stop_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_setTabStopWidth(self as *mut ::plain_text_edit::PlainTextEdit, width) }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setTextCursor(const QTextCursor& cursor)```</span>
  ///
  ///
  pub fn set_text_cursor(&mut self, cursor: &::qt_gui::text_cursor::TextCursor) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setTextCursor(self as *mut ::plain_text_edit::PlainTextEdit,
                                                       cursor as *const ::qt_gui::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setUndoRedoEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_undo_redo_enabled(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setUndoRedoEnabled(self as *mut ::plain_text_edit::PlainTextEdit, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPlainTextEdit::setWordWrapMode(QTextOption::WrapMode policy)```</span>
  ///
  ///
  pub fn set_word_wrap_mode(&mut self, policy: &::qt_gui::text_option::WrapMode) {
    unsafe {
      ::ffi::qt_widgets_c_QPlainTextEdit_setWordWrapMode(self as *mut ::plain_text_edit::PlainTextEdit,
                                                         policy as *const ::qt_gui::text_option::WrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPlainTextEdit::tabChangesFocus() const```</span>
  ///
  ///
  pub fn tab_changes_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_tabChangesFocus(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QPlainTextEdit::tabStopWidth() const```</span>
  ///
  ///
  pub fn tab_stop_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_tabStopWidth(self as *const ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QPlainTextEdit::textCursor() const```</span>
  ///
  ///
  pub fn text_cursor(&self) -> ::cpp_utils::CppBox<::qt_gui::text_cursor::TextCursor> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_textCursor_as_ptr(self as *const ::plain_text_edit::PlainTextEdit) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QString QPlainTextEdit::toPlainText() const```</span>
  ///
  ///
  pub fn to_plain_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_toPlainText_to_output(self as *const ::plain_text_edit::PlainTextEdit,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPlainTextEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPlainTextEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPlainTextEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QPlainTextEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_undo(self as *mut ::plain_text_edit::PlainTextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QPlainTextEdit::zoomIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn zoom_in(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::zoomIn()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn zoom_in(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::zoomIn(int range = ?)```</span>
  ///
  ///
  pub fn zoom_in<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PlainTextEditZoomInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPlainTextEdit::zoomOut```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn zoom_out(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::zoomOut()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn zoom_out(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QPlainTextEdit::zoomOut(int range = ?)```</span>
  ///
  ///
  pub fn zoom_out<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PlainTextEditZoomOutArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::plain_text_edit::PlainTextEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QPlainTextEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PlainTextEdit`.
  pub struct Signals<'a>(&'a ::plain_text_edit::PlainTextEdit);
  /// Represents a built-in Qt signal `QPlainTextEdit::selectionChanged`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct SelectionChanged<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::modificationChanged`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().modification_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct ModificationChanged<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for ModificationChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modificationChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModificationChanged<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::textChanged`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct TextChanged<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::undoAvailable`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().undo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct UndoAvailable<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for UndoAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2undoAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UndoAvailable<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::redoAvailable`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().redo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct RedoAvailable<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for RedoAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2redoAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RedoAvailable<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::cursorPositionChanged`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().cursor_position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct CursorPositionChanged<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for CursorPositionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorPositionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorPositionChanged<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::blockCountChanged`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().block_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct BlockCountChanged<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for BlockCountChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2blockCountChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BlockCountChanged<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::updateRequest`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().update_request()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct UpdateRequest<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for UpdateRequest<'a> {
    type Arguments = (&'static ::qt_core::rect::Rect, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2updateRequest(const QRect&,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpdateRequest<'a> {}
  /// Represents a built-in Qt signal `QPlainTextEdit::copyAvailable`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.signals().copy_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct CopyAvailable<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for CopyAvailable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2copyAvailable(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CopyAvailable<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::modificationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn modification_changed(&self) -> ModificationChanged {
      ModificationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::undoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_available(&self) -> UndoAvailable {
      UndoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::redoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_available(&self) -> RedoAvailable {
      RedoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::cursorPositionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_position_changed(&self) -> CursorPositionChanged {
      CursorPositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::blockCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn block_count_changed(&self) -> BlockCountChanged {
      BlockCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::updateRequest`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn update_request(&self) -> UpdateRequest {
      UpdateRequest(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QPlainTextEdit::copyAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy_available(&self) -> CopyAvailable {
      CopyAvailable(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PlainTextEdit`.
  pub struct Slots<'a>(&'a ::plain_text_edit::PlainTextEdit);
  /// Represents a built-in Qt slot `QPlainTextEdit::copy`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().copy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Copy<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Copy<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1copy()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::cut`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().cut()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Cut<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Cut<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cut()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::clear`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Clear<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::setPlainText`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().set_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct SetPlainText<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::zoomIn`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().zoom_in()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct ZoomIn<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for ZoomIn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomIn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::selectAll`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct SelectAll<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::centerCursor`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().center_cursor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct CenterCursor<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for CenterCursor<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1centerCursor()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::appendHtml`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().append_html()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct AppendHtml<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for AppendHtml<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1appendHtml(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::redo`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Redo<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::undo`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Undo<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::zoomOut`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().zoom_out()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct ZoomOut<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for ZoomOut<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomOut(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::paste`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().paste()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct Paste<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for Paste<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1paste()\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::appendPlainText`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().append_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct AppendPlainText<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for AppendPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1appendPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QPlainTextEdit::insertPlainText`.
  ///
  /// An object of this type can be created from `PlainTextEdit` with `object.slots().insert_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlainTextEdit` object.
  pub struct InsertPlainText<'a>(&'a ::plain_text_edit::PlainTextEdit);
  impl<'a> ::qt_core::connection::Receiver for InsertPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertPlainText(const QString&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::copy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy(&self) -> Copy {
      Copy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::cut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cut(&self) -> Cut {
      Cut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::setPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_plain_text(&self) -> SetPlainText {
      SetPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::zoomIn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_in(&self) -> ZoomIn {
      ZoomIn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::centerCursor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn center_cursor(&self) -> CenterCursor {
      CenterCursor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::appendHtml`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn append_html(&self) -> AppendHtml {
      AppendHtml(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::zoomOut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_out(&self) -> ZoomOut {
      ZoomOut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::paste`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn paste(&self) -> Paste {
      Paste(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::appendPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn append_plain_text(&self) -> AppendPlainText {
      AppendPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QPlainTextEdit::insertPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_plain_text(&self) -> InsertPlainText {
      InsertPlainText(self.0)
    }
  }
  impl ::plain_text_edit::PlainTextEdit {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::plain_text_edit::PlainTextEdit> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::plain_text_edit::PlainTextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::plain_text_edit::PlainTextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::plain_text_edit::PlainTextEdit> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::plain_text_edit::PlainTextEdit> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QFrame(self as *mut ::frame::Frame)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::plain_text_edit::PlainTextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::plain_text_edit::PlainTextEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::plain_text_edit::PlainTextEdit> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::plain_text_edit::PlainTextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_dynamic_cast_QPlainTextEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::plain_text_edit::PlainTextEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QObject_ptr(self as *mut ::plain_text_edit::PlainTextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QObject_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::plain_text_edit::PlainTextEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::plain_text_edit::PlainTextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPaintDevice_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::plain_text_edit::PlainTextEdit {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::plain_text_edit::PlainTextEdit {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QFrame_ptr(self as *mut ::plain_text_edit::PlainTextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QFrame_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::plain_text_edit::PlainTextEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QWidget_ptr(self as *mut ::plain_text_edit::PlainTextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QWidget_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_edit::PlainTextEdit> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_edit::PlainTextEdit> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_edit::PlainTextEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_edit::PlainTextEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_edit::PlainTextEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plain_text_edit::PlainTextEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plain_text_edit::PlainTextEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plain_text_edit::PlainTextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QPlainTextEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::plain_text_edit::PlainTextEdit {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *const ::plain_text_edit::PlainTextEdit as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::plain_text_edit::PlainTextEdit {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::plain_text_edit::PlainTextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PlainTextEdit::create_standard_context_menu](../struct.PlainTextEdit.html#method.create_standard_context_menu) method.
  pub trait PlainTextEditCreateStandardContextMenuArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> *mut ::menu::Menu;
  }
  impl<'largs> PlainTextEditCreateStandardContextMenuArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> *mut ::menu::Menu {

      unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_createStandardContextMenu_no_args(original_self as *mut ::plain_text_edit::PlainTextEdit) }
    }
  }
  impl<'largs> PlainTextEditCreateStandardContextMenuArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> *mut ::menu::Menu {
      let position = self;
      unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_createStandardContextMenu_position(original_self as *mut ::plain_text_edit::PlainTextEdit, position as *const ::qt_core::point::Point) }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::cursor_rect](../struct.PlainTextEdit.html#method.cursor_rect) method.
  pub trait PlainTextEditCursorRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::rect::Rect;
  }
  impl<'largs> PlainTextEditCursorRectArgs<'largs> for &'largs ::qt_gui::text_cursor::TextCursor {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::rect::Rect {
      let cursor = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPlainTextEdit_cursorRect_to_output_cursor(original_self as *const ::plain_text_edit::PlainTextEdit, cursor as *const ::qt_gui::text_cursor::TextCursor, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PlainTextEditCursorRectArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::rect::Rect {

      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPlainTextEdit_cursorRect_to_output_no_args(original_self as *const ::plain_text_edit::PlainTextEdit, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::input_method_query](../struct.PlainTextEdit.html#method.input_method_query) method.
  pub trait PlainTextEditInputMethodQueryArgs<'largs> {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::variant::Variant;
  }
  impl<'largs> PlainTextEditInputMethodQueryArgs<'largs> for &'largs ::qt_core::qt::InputMethodQuery {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::variant::Variant {
      let property = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_property(original_self as *const ::plain_text_edit::PlainTextEdit, property as *const ::qt_core::qt::InputMethodQuery, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PlainTextEditInputMethodQueryArgs<'largs>
    for (&'largs ::qt_core::qt::InputMethodQuery, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs ::plain_text_edit::PlainTextEdit) -> ::qt_core::variant::Variant {
      let query = self.0;
      let argument = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QPlainTextEdit_inputMethodQuery_to_output_query_argument(original_self as *const ::plain_text_edit::PlainTextEdit, query as *const ::qt_core::qt::InputMethodQuery, argument as *const ::qt_core::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::move_cursor](../struct.PlainTextEdit.html#method.move_cursor) method.
  pub trait PlainTextEditMoveCursorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> ();
  }
  impl<'largs> PlainTextEditMoveCursorArgs<'largs> for &'largs ::qt_gui::text_cursor::MoveOperation {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {
      let operation = self;
      unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_moveCursor_operation(original_self as *mut ::plain_text_edit::PlainTextEdit, operation as *const ::qt_gui::text_cursor::MoveOperation) }
    }
  }
  impl<'largs> PlainTextEditMoveCursorArgs<'largs>
    for (&'largs ::qt_gui::text_cursor::MoveOperation, &'largs ::qt_gui::text_cursor::MoveMode) {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {
      let operation = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_moveCursor_operation_mode(original_self as *mut ::plain_text_edit::PlainTextEdit, operation as *const ::qt_gui::text_cursor::MoveOperation, mode as *const ::qt_gui::text_cursor::MoveMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::new](../struct.PlainTextEdit.html#method.new) method.
  pub trait PlainTextEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>;
  }
  impl PlainTextEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PlainTextEditNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QPlainTextEdit_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::new_unsafe](../struct.PlainTextEdit.html#method.new_unsafe) method.
  pub trait PlainTextEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit>;
  }
  impl PlainTextEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> PlainTextEditNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plain_text_edit::PlainTextEdit> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QPlainTextEdit_new_text_parent(text as *const ::qt_core::string::String,
                                                                          parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::zoom_in](../struct.PlainTextEdit.html#method.zoom_in) method.
  pub trait PlainTextEditZoomInArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> ();
  }
  impl<'largs> PlainTextEditZoomInArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_zoomIn_no_args(original_self as *mut ::plain_text_edit::PlainTextEdit)
      }
    }
  }
  impl<'largs> PlainTextEditZoomInArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {
      let range = self;
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_zoomIn_range(original_self as *mut ::plain_text_edit::PlainTextEdit,
                                                        range)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PlainTextEdit::zoom_out](../struct.PlainTextEdit.html#method.zoom_out) method.
  pub trait PlainTextEditZoomOutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> ();
  }
  impl<'largs> PlainTextEditZoomOutArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {

      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_zoomOut_no_args(original_self as *mut ::plain_text_edit::PlainTextEdit)
      }
    }
  }
  impl<'largs> PlainTextEditZoomOutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::plain_text_edit::PlainTextEdit) -> () {
      let range = self;
      unsafe {
        ::ffi::qt_widgets_c_QPlainTextEdit_zoomOut_range(original_self as *mut ::plain_text_edit::PlainTextEdit,
                                                         range)
      }
    }
  }
}
