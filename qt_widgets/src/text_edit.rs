/// C++ type: <span style='color: green;'>```QTextEdit::AutoFormattingFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AutoFormattingFlag {
  /// C++ enum variant: <span style='color: green;'>```AutoAll = -1```</span>
  All = -1,
  /// C++ enum variant: <span style='color: green;'>```AutoNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```AutoBulletList = 1```</span>
  BulletList = 1,
}

impl ::qt_core::flags::FlaggableEnum for AutoFormattingFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "AutoFormattingFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextEdit::ExtraSelection```</span>
#[repr(C)]
pub struct ExtraSelection([u8; ::type_sizes::QT_WIDGETS_TEXT_EDIT_EXTRA_SELECTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ExtraSelection {
  unsafe fn new_uninitialized() -> ExtraSelection {
    ExtraSelection(::std::mem::uninitialized())
  }
}

impl ExtraSelection {
  /// C++ method: <span style='color: green;'>```const QTextCursor& QTextEdit::ExtraSelection::cursor() const```</span>
  ///
  ///
  pub fn cursor<'l0>(&'l0 self) -> &'l0 ::qt_gui::text_cursor::TextCursor {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_cursor(self as *const ::text_edit::ExtraSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCursor& QTextEdit::ExtraSelection::cursor_mut()```</span>
  ///
  ///
  pub fn cursor_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::text_cursor::TextCursor {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_cursor_mut(self as *mut ::text_edit::ExtraSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextCharFormat& QTextEdit::ExtraSelection::format() const```</span>
  ///
  ///
  pub fn format<'l0>(&'l0 self) -> &'l0 ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_format(self as *const ::text_edit::ExtraSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QTextEdit::ExtraSelection::format_mut()```</span>
  ///
  ///
  pub fn format_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_format_mut(self as *mut ::text_edit::ExtraSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::ExtraSelection::set_cursor(QTextCursor value)```</span>
  ///
  ///
  pub fn set_cursor(&mut self, value: &::qt_gui::text_cursor::TextCursor) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_set_cursor(self as *mut ::text_edit::ExtraSelection,
                                                              value as *const ::qt_gui::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::ExtraSelection::set_format(QTextCharFormat value)```</span>
  ///
  ///
  pub fn set_format(&mut self, value: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_set_format(self as *mut ::text_edit::ExtraSelection, value as *const ::qt_gui::text_char_format::TextCharFormat) }
  }
}

impl Drop for ::text_edit::ExtraSelection {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextEdit::ExtraSelection::~QTextEdit::ExtraSelection()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_ExtraSelection_destructor(self as *mut ::text_edit::ExtraSelection) }
  }
}

/// C++ type: <span style='color: green;'>```QTextEdit::LineWrapMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LineWrapMode {
  /// C++ enum variant: <span style='color: green;'>```NoWrap = 0```</span>
  NoWrap = 0,
  /// C++ enum variant: <span style='color: green;'>```WidgetWidth = 1```</span>
  WidgetWidth = 1,
  /// C++ enum variant: <span style='color: green;'>```FixedPixelWidth = 2```</span>
  FixedPixelWidth = 2,
  /// C++ enum variant: <span style='color: green;'>```FixedColumnWidth = 3```</span>
  FixedColumnWidth = 3,
}

/// C++ type: <span style='color: green;'>```QTextEdit```</span>
#[repr(C)]
pub struct TextEdit(u8);

impl TextEdit {
  /// C++ method: <span style='color: green;'>```bool QTextEdit::acceptRichText() const```</span>
  ///
  ///
  pub fn accept_rich_text(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_acceptRichText(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::anchorAt(const QPoint& pos) const```</span>
  ///
  ///
  pub fn anchor_at(&self, pos: &::qt_core::point::Point) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_anchorAt_to_output(self as *const ::text_edit::TextEdit,
                                                         pos as *const ::qt_core::point::Point,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::append(const QString& text)```</span>
  ///
  ///
  pub fn append(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_append(self as *mut ::text_edit::TextEdit,
                                           text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTextEdit::AutoFormattingFlag> QTextEdit::autoFormatting() const```</span>
  ///
  ///
  pub fn auto_formatting(&self) -> ::qt_core::flags::Flags<::text_edit::AutoFormattingFlag> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_autoFormatting(self as *const ::text_edit::TextEdit) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::canPaste() const```</span>
  ///
  ///
  pub fn can_paste(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_canPaste(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_clear(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::copy()```</span>
  ///
  ///
  pub fn copy(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_copy(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::createStandardContextMenu```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_standard_context_menu(&mut self, ()) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QTextEdit::createStandardContextMenu()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_standard_context_menu(&mut self, &::qt_core::point::Point) -> *mut ::menu::Menu```<br>
  /// C++ method: <span style='color: green;'>```QMenu* QTextEdit::createStandardContextMenu(const QPoint& position)```</span>
  ///
  ///
  pub fn create_standard_context_menu<'largs, Args>(&'largs mut self, args: Args) -> *mut ::menu::Menu
    where Args: overloading::TextEditCreateStandardContextMenuArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextEdit::currentCharFormat() const```</span>
  ///
  ///
  pub fn current_char_format(&self) -> ::qt_gui::text_char_format::TextCharFormat {
    {
      let mut object: ::qt_gui::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_currentCharFormat_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QTextEdit::currentFont() const```</span>
  ///
  ///
  pub fn current_font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_currentFont_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextEdit::cursorForPosition(const QPoint& pos) const```</span>
  ///
  ///
  pub fn cursor_for_position(&self,
                             pos: &::qt_core::point::Point)
                             -> ::cpp_utils::CppBox<::qt_gui::text_cursor::TextCursor> {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QTextEdit_cursorForPosition_as_ptr(self as *const ::text_edit::TextEdit,
                                                             pos as *const ::qt_core::point::Point)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::cursorRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_rect(&self, ()) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QTextEdit::cursorRect() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_rect(&self, &::qt_gui::text_cursor::TextCursor) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QTextEdit::cursorRect(const QTextCursor& cursor) const```</span>
  ///
  ///
  pub fn cursor_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::TextEditCursorRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QTextEdit::cursorWidth() const```</span>
  ///
  ///
  pub fn cursor_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_cursorWidth(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::cut()```</span>
  ///
  ///
  pub fn cut(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_cut(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QTextEdit::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::qt_gui::text_document::TextDocument {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_document(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::documentTitle() const```</span>
  ///
  ///
  pub fn document_title(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_documentTitle_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::ensureCursorVisible()```</span>
  ///
  ///
  pub fn ensure_cursor_visible(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_ensureCursorVisible(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection> QTextEdit::extraSelections() const```</span>
  ///
  ///
  pub fn extra_selections(&self) -> ::list::ListTextEditExtraSelection {
    {
      let mut object: ::list::ListTextEditExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_extraSelections_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::fontFamily() const```</span>
  ///
  ///
  pub fn font_family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_fontFamily_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::fontItalic() const```</span>
  ///
  ///
  pub fn font_italic(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_fontItalic(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```double QTextEdit::fontPointSize() const```</span>
  ///
  ///
  pub fn font_point_size(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_fontPointSize(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::fontUnderline() const```</span>
  ///
  ///
  pub fn font_underline(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_fontUnderline(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QTextEdit::fontWeight() const```</span>
  ///
  ///
  pub fn font_weight(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_fontWeight(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::inputMethodQuery```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn input_method_query(&self, (&::qt_core::qt::InputMethodQuery, &::qt_core::variant::Variant)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QTextEdit::inputMethodQuery(Qt::InputMethodQuery query, QVariant argument) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn input_method_query(&self, &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QTextEdit::inputMethodQuery(Qt::InputMethodQuery property) const```</span>
  ///
  ///
  pub fn input_method_query<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::TextEditInputMethodQueryArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::insertHtml(const QString& text)```</span>
  ///
  ///
  pub fn insert_html(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_insertHtml(self as *mut ::text_edit::TextEdit,
                                               text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::insertPlainText(const QString& text)```</span>
  ///
  ///
  pub fn insert_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_insertPlainText(self as *mut ::text_edit::TextEdit,
                                                    text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_isReadOnly(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::isUndoRedoEnabled() const```</span>
  ///
  ///
  pub fn is_undo_redo_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_isUndoRedoEnabled(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QTextEdit::lineWrapColumnOrWidth() const```</span>
  ///
  ///
  pub fn line_wrap_column_or_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_lineWrapColumnOrWidth(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::LineWrapMode QTextEdit::lineWrapMode() const```</span>
  ///
  ///
  pub fn line_wrap_mode(&self) -> ::text_edit::LineWrapMode {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_lineWrapMode(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QTextEdit::loadResource(int type, const QUrl& name)```</span>
  ///
  ///
  pub fn load_resource(&mut self, type_: ::libc::c_int, name: &::qt_core::url::Url) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_loadResource_to_output(self as *mut ::text_edit::TextEdit,
                                                             type_,
                                                             name as *const ::qt_core::url::Url,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::mergeCurrentCharFormat(const QTextCharFormat& modifier)```</span>
  ///
  ///
  pub fn merge_current_char_format(&mut self, modifier: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_mergeCurrentCharFormat(self as *mut ::text_edit::TextEdit, modifier as *const ::qt_gui::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextEdit::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_metaObject(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::moveCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_cursor(&mut self, &::qt_gui::text_cursor::MoveOperation) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextEdit::moveCursor(QTextCursor::MoveOperation operation)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_cursor(&mut self, (&::qt_gui::text_cursor::MoveOperation, &::qt_gui::text_cursor::MoveMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextEdit::moveCursor(QTextCursor::MoveOperation operation, QTextCursor::MoveMode mode = ?)```</span>
  ///
  ///
  pub fn move_cursor<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextEditMoveCursorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextEdit::QTextEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::text_edit::TextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextEdit::QTextEdit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::text_edit::TextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextEdit::QTextEdit(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::text_edit::TextEdit>
    where Args: overloading::TextEditNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextEdit::QTextEdit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::text_edit::TextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextEdit::QTextEdit(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::text_edit::TextEdit>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextEdit::QTextEdit(const QString& text, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::text_edit::TextEdit>
    where Args: overloading::TextEditNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QTextEdit::overwriteMode() const```</span>
  ///
  ///
  pub fn overwrite_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_overwriteMode(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::paste()```</span>
  ///
  ///
  pub fn paste(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_paste(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::placeholderText() const```</span>
  ///
  ///
  pub fn placeholder_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_placeholderText_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::print(QPagedPaintDevice* printer) const```</span>
  ///
  ///
  pub unsafe fn print(&self, printer: *mut ::qt_gui::paged_paint_device::PagedPaintDevice) {
    ::ffi::qt_widgets_c_QTextEdit_print(self as *const ::text_edit::TextEdit, printer)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextEdit::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTextEdit_qt_metacall(self as *mut ::text_edit::TextEdit,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextEdit::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTextEdit_qt_metacast(self as *mut ::text_edit::TextEdit, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_redo(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::scrollToAnchor(const QString& name)```</span>
  ///
  ///
  pub fn scroll_to_anchor(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_scrollToAnchor(self as *mut ::text_edit::TextEdit,
                                                   name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::selectAll()```</span>
  ///
  ///
  pub fn select_all(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_selectAll(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setAcceptRichText(bool accept)```</span>
  ///
  ///
  pub fn set_accept_rich_text(&mut self, accept: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setAcceptRichText(self as *mut ::text_edit::TextEdit, accept) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setAutoFormatting(QFlags<QTextEdit::AutoFormattingFlag> features)```</span>
  ///
  ///
  pub fn set_auto_formatting(&mut self, features: ::qt_core::flags::Flags<::text_edit::AutoFormattingFlag>) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setAutoFormatting(self as *mut ::text_edit::TextEdit,
                                                      features.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setCurrentCharFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_current_char_format(&mut self, format: &::qt_gui::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setCurrentCharFormat(self as *mut ::text_edit::TextEdit,
                                                         format as *const ::qt_gui::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setCurrentFont(const QFont& f)```</span>
  ///
  ///
  pub fn set_current_font(&mut self, f: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setCurrentFont(self as *mut ::text_edit::TextEdit,
                                                   f as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setCursorWidth(int width)```</span>
  ///
  ///
  pub fn set_cursor_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setCursorWidth(self as *mut ::text_edit::TextEdit, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setDocument(QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn set_document(&mut self, document: *mut ::qt_gui::text_document::TextDocument) {
    ::ffi::qt_widgets_c_QTextEdit_setDocument(self as *mut ::text_edit::TextEdit, document)
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setDocumentTitle(const QString& title)```</span>
  ///
  ///
  pub fn set_document_title(&mut self, title: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setDocumentTitle(self as *mut ::text_edit::TextEdit,
                                                     title as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setExtraSelections(const QList<QTextEdit::ExtraSelection>& selections)```</span>
  ///
  ///
  pub fn set_extra_selections(&mut self, selections: &::list::ListTextEditExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setExtraSelections(self as *mut ::text_edit::TextEdit,
                                                       selections as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setFontFamily(const QString& fontFamily)```</span>
  ///
  ///
  pub fn set_font_family(&mut self, font_family: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setFontFamily(self as *mut ::text_edit::TextEdit,
                                                  font_family as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setFontItalic(bool b)```</span>
  ///
  ///
  pub fn set_font_italic(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setFontItalic(self as *mut ::text_edit::TextEdit, b) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setFontPointSize(double s)```</span>
  ///
  ///
  pub fn set_font_point_size(&mut self, s: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setFontPointSize(self as *mut ::text_edit::TextEdit, s) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setFontUnderline(bool b)```</span>
  ///
  ///
  pub fn set_font_underline(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setFontUnderline(self as *mut ::text_edit::TextEdit, b) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setFontWeight(int w)```</span>
  ///
  ///
  pub fn set_font_weight(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setFontWeight(self as *mut ::text_edit::TextEdit, w) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setHtml(const QString& text)```</span>
  ///
  ///
  pub fn set_html(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setHtml(self as *mut ::text_edit::TextEdit,
                                            text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setLineWrapColumnOrWidth(int w)```</span>
  ///
  ///
  pub fn set_line_wrap_column_or_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setLineWrapColumnOrWidth(self as *mut ::text_edit::TextEdit, w) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setLineWrapMode(QTextEdit::LineWrapMode mode)```</span>
  ///
  ///
  pub fn set_line_wrap_mode(&mut self, mode: ::text_edit::LineWrapMode) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setLineWrapMode(self as *mut ::text_edit::TextEdit, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setOverwriteMode(bool overwrite)```</span>
  ///
  ///
  pub fn set_overwrite_mode(&mut self, overwrite: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setOverwriteMode(self as *mut ::text_edit::TextEdit, overwrite) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setPlaceholderText(const QString& placeholderText)```</span>
  ///
  ///
  pub fn set_placeholder_text(&mut self, placeholder_text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setPlaceholderText(self as *mut ::text_edit::TextEdit,
                                                       placeholder_text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setPlainText(const QString& text)```</span>
  ///
  ///
  pub fn set_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setPlainText(self as *mut ::text_edit::TextEdit,
                                                 text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setReadOnly(bool ro)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, ro: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setReadOnly(self as *mut ::text_edit::TextEdit, ro) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setTabChangesFocus(bool b)```</span>
  ///
  ///
  pub fn set_tab_changes_focus(&mut self, b: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setTabChangesFocus(self as *mut ::text_edit::TextEdit, b) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setTabStopWidth(int width)```</span>
  ///
  ///
  pub fn set_tab_stop_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setTabStopWidth(self as *mut ::text_edit::TextEdit, width) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setText(self as *mut ::text_edit::TextEdit,
                                            text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setTextBackgroundColor(const QColor& c)```</span>
  ///
  ///
  pub fn set_text_background_color(&mut self, c: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setTextBackgroundColor(self as *mut ::text_edit::TextEdit,
                                                           c as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::setTextColor(const QColor& c)```</span>
  ///
  ///
  pub fn set_text_color(&mut self, c: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setTextColor(self as *mut ::text_edit::TextEdit,
                                                 c as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setTextCursor(const QTextCursor& cursor)```</span>
  ///
  ///
  pub fn set_text_cursor(&mut self, cursor: &::qt_gui::text_cursor::TextCursor) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setTextCursor(self as *mut ::text_edit::TextEdit,
                                                  cursor as *const ::qt_gui::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setUndoRedoEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_undo_redo_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_setUndoRedoEnabled(self as *mut ::text_edit::TextEdit, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTextEdit::setWordWrapMode(QTextOption::WrapMode policy)```</span>
  ///
  ///
  pub fn set_word_wrap_mode(&mut self, policy: &::qt_gui::text_option::WrapMode) {
    unsafe {
      ::ffi::qt_widgets_c_QTextEdit_setWordWrapMode(self as *mut ::text_edit::TextEdit,
                                                    policy as *const ::qt_gui::text_option::WrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextEdit::tabChangesFocus() const```</span>
  ///
  ///
  pub fn tab_changes_focus(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_tabChangesFocus(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```int QTextEdit::tabStopWidth() const```</span>
  ///
  ///
  pub fn tab_stop_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_tabStopWidth(self as *const ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QColor QTextEdit::textBackgroundColor() const```</span>
  ///
  ///
  pub fn text_background_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_textBackgroundColor_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTextEdit::textColor() const```</span>
  ///
  ///
  pub fn text_color(&self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_textColor_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor QTextEdit::textCursor() const```</span>
  ///
  ///
  pub fn text_cursor(&self) -> ::cpp_utils::CppBox<::qt_gui::text_cursor::TextCursor> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_textCursor_as_ptr(self as *const ::text_edit::TextEdit) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::toHtml() const```</span>
  ///
  ///
  pub fn to_html(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_toHtml_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextEdit::toPlainText() const```</span>
  ///
  ///
  pub fn to_plain_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_toPlainText_to_output(self as *const ::text_edit::TextEdit, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextEdit::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTextEdit_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextEdit::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTextEdit_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTextEdit_undo(self as *mut ::text_edit::TextEdit) }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::zoomIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn zoom_in(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::zoomIn()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn zoom_in(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::zoomIn(int range = ?)```</span>
  ///
  ///
  pub fn zoom_in<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextEditZoomInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextEdit::zoomOut```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn zoom_out(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::zoomOut()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn zoom_out(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextEdit::zoomOut(int range = ?)```</span>
  ///
  ///
  pub fn zoom_out<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextEditZoomOutArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::text_edit::TextEdit {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTextEdit_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextEdit`.
  pub struct Signals<'a>(&'a ::text_edit::TextEdit);
  /// Represents a built-in Qt signal `QTextEdit::redoAvailable`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().redo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct RedoAvailable<'a>(&'a ::text_edit::TextEdit);
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
  /// Represents a built-in Qt signal `QTextEdit::copyAvailable`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().copy_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct CopyAvailable<'a>(&'a ::text_edit::TextEdit);
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
  /// Represents a built-in Qt signal `QTextEdit::cursorPositionChanged`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().cursor_position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct CursorPositionChanged<'a>(&'a ::text_edit::TextEdit);
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
  /// Represents a built-in Qt signal `QTextEdit::currentCharFormatChanged`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().current_char_format_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct CurrentCharFormatChanged<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for CurrentCharFormatChanged<'a> {
    type Arguments = (&'static ::qt_gui::text_char_format::TextCharFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentCharFormatChanged(const QTextCharFormat&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentCharFormatChanged<'a> {}
  /// Represents a built-in Qt signal `QTextEdit::selectionChanged`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SelectionChanged<'a>(&'a ::text_edit::TextEdit);
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
  /// Represents a built-in Qt signal `QTextEdit::undoAvailable`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().undo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct UndoAvailable<'a>(&'a ::text_edit::TextEdit);
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
  /// Represents a built-in Qt signal `QTextEdit::textChanged`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct TextChanged<'a>(&'a ::text_edit::TextEdit);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTextEdit::redoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_available(&self) -> RedoAvailable {
      RedoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::copyAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy_available(&self) -> CopyAvailable {
      CopyAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::cursorPositionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_position_changed(&self) -> CursorPositionChanged {
      CursorPositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::currentCharFormatChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_char_format_changed(&self) -> CurrentCharFormatChanged {
      CurrentCharFormatChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::selectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn selection_changed(&self) -> SelectionChanged {
      SelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::undoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_available(&self) -> UndoAvailable {
      UndoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextEdit::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextEdit`.
  pub struct Slots<'a>(&'a ::text_edit::TextEdit);
  /// Represents a built-in Qt slot `QTextEdit::zoomOut`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().zoom_out()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct ZoomOut<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for ZoomOut<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomOut(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::clear`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Clear<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setFontPointSize`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_font_point_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetFontPointSize<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFontPointSize<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontPointSize(double)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::scrollToAnchor`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().scroll_to_anchor()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct ScrollToAnchor<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for ScrollToAnchor<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToAnchor(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::zoomIn`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().zoom_in()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct ZoomIn<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for ZoomIn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1zoomIn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setCurrentFont`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_current_font()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetCurrentFont<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetCurrentFont<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentFont(const QFont&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::copy`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().copy()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Copy<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Copy<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1copy()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::insertHtml`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().insert_html()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct InsertHtml<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for InsertHtml<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertHtml(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setTextBackgroundColor`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_text_background_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetTextBackgroundColor<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetTextBackgroundColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextBackgroundColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setFontFamily`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_font_family()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetFontFamily<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFontFamily<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontFamily(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setTextColor`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_text_color()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetTextColor<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetTextColor<'a> {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTextColor(const QColor&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setText`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetText<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::redo`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Redo<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setFontWeight`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_font_weight()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetFontWeight<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFontWeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontWeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setFontItalic`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_font_italic()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetFontItalic<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFontItalic<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontItalic(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::append`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().append()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Append<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Append<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1append(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setFontUnderline`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_font_underline()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetFontUnderline<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetFontUnderline<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFontUnderline(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setHtml`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_html()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetHtml<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetHtml<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHtml(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::selectAll`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().select_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SelectAll<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SelectAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::insertPlainText`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().insert_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct InsertPlainText<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for InsertPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::undo`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Undo<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::cut`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().cut()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Cut<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Cut<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1cut()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::setPlainText`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().set_plain_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct SetPlainText<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for SetPlainText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPlainText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextEdit::paste`.
  ///
  /// An object of this type can be created from `TextEdit` with `object.slots().paste()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextEdit` object.
  pub struct Paste<'a>(&'a ::text_edit::TextEdit);
  impl<'a> ::qt_core::connection::Receiver for Paste<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1paste()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTextEdit::zoomOut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_out(&self) -> ZoomOut {
      ZoomOut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setFontPointSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_point_size(&self) -> SetFontPointSize {
      SetFontPointSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::scrollToAnchor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_anchor(&self) -> ScrollToAnchor {
      ScrollToAnchor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::zoomIn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_in(&self) -> ZoomIn {
      ZoomIn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setCurrentFont`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_font(&self) -> SetCurrentFont {
      SetCurrentFont(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::copy`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn copy(&self) -> Copy {
      Copy(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::insertHtml`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_html(&self) -> InsertHtml {
      InsertHtml(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setTextBackgroundColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text_background_color(&self) -> SetTextBackgroundColor {
      SetTextBackgroundColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setFontFamily`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_family(&self) -> SetFontFamily {
      SetFontFamily(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setTextColor`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text_color(&self) -> SetTextColor {
      SetTextColor(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setFontWeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_weight(&self) -> SetFontWeight {
      SetFontWeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setFontItalic`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_italic(&self) -> SetFontItalic {
      SetFontItalic(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::append`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn append(&self) -> Append {
      Append(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setFontUnderline`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font_underline(&self) -> SetFontUnderline {
      SetFontUnderline(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setHtml`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_html(&self) -> SetHtml {
      SetHtml(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::selectAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_all(&self) -> SelectAll {
      SelectAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::insertPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_plain_text(&self) -> InsertPlainText {
      InsertPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::cut`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cut(&self) -> Cut {
      Cut(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::setPlainText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_plain_text(&self) -> SetPlainText {
      SetPlainText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextEdit::paste`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn paste(&self) -> Paste {
      Paste(self.0)
    }
  }
  impl ::text_edit::TextEdit {
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

impl ::cpp_utils::DynamicCast<::text_edit::TextEdit> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_edit::TextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_edit::TextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_edit::TextEdit> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_edit::TextEdit> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_edit::TextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::text_edit::TextEdit> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::text_edit::TextEdit> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::text_edit::TextEdit> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_dynamic_cast_QTextEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_edit::TextEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QObject_ptr(self as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QObject_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::text_edit::TextEdit {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QPaintDevice_ptr(self as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QPaintDevice_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::text_edit::TextEdit {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::text_edit::TextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::text_edit::TextEdit {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QFrame_ptr(self as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QFrame_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::text_edit::TextEdit {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QWidget_ptr(self as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QWidget_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_edit::TextEdit> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_edit::TextEdit> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_edit::TextEdit> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result =
      ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_edit::TextEdit> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_edit::TextEdit> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_edit::TextEdit {
    let ffi_result = ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QTextEdit_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_edit::TextEdit {
  type Target = ::abstract_scroll_area::AbstractScrollArea;
  fn deref(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *const ::text_edit::TextEdit as *mut ::text_edit::TextEdit) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_edit::TextEdit {
  fn deref_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::text_edit::TextEdit)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextEdit::create_standard_context_menu](../struct.TextEdit.html#method.create_standard_context_menu) method.
  pub trait TextEditCreateStandardContextMenuArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> *mut ::menu::Menu;
  }
  impl<'largs> TextEditCreateStandardContextMenuArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> *mut ::menu::Menu {

      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_createStandardContextMenu_no_args(original_self as *mut ::text_edit::TextEdit)
      }
    }
  }
  impl<'largs> TextEditCreateStandardContextMenuArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> *mut ::menu::Menu {
      let position = self;
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_createStandardContextMenu_position(original_self as *mut ::text_edit::TextEdit,
                                                                         position as *const ::qt_core::point::Point)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::cursor_rect](../struct.TextEdit.html#method.cursor_rect) method.
  pub trait TextEditCursorRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::rect::Rect;
  }
  impl<'largs> TextEditCursorRectArgs<'largs> for &'largs ::qt_gui::text_cursor::TextCursor {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::rect::Rect {
      let cursor = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTextEdit_cursorRect_to_output_cursor(original_self as *const ::text_edit::TextEdit, cursor as *const ::qt_gui::text_cursor::TextCursor, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextEditCursorRectArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::rect::Rect {

      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTextEdit_cursorRect_to_output_no_args(original_self as *const ::text_edit::TextEdit,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::input_method_query](../struct.TextEdit.html#method.input_method_query) method.
  pub trait TextEditInputMethodQueryArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::variant::Variant;
  }
  impl<'largs> TextEditInputMethodQueryArgs<'largs> for &'largs ::qt_core::qt::InputMethodQuery {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::variant::Variant {
      let property = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTextEdit_inputMethodQuery_to_output_property(original_self as *const ::text_edit::TextEdit, property as *const ::qt_core::qt::InputMethodQuery, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextEditInputMethodQueryArgs<'largs>
    for (&'largs ::qt_core::qt::InputMethodQuery, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs ::text_edit::TextEdit) -> ::qt_core::variant::Variant {
      let query = self.0;
      let argument = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QTextEdit_inputMethodQuery_to_output_query_argument(original_self as *const ::text_edit::TextEdit, query as *const ::qt_core::qt::InputMethodQuery, argument as *const ::qt_core::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::move_cursor](../struct.TextEdit.html#method.move_cursor) method.
  pub trait TextEditMoveCursorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> ();
  }
  impl<'largs> TextEditMoveCursorArgs<'largs> for &'largs ::qt_gui::text_cursor::MoveOperation {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {
      let operation = self;
      unsafe {
        ::ffi::qt_widgets_c_QTextEdit_moveCursor_operation(original_self as *mut ::text_edit::TextEdit,
                                                           operation as *const ::qt_gui::text_cursor::MoveOperation)
      }
    }
  }
  impl<'largs> TextEditMoveCursorArgs<'largs>
    for (&'largs ::qt_gui::text_cursor::MoveOperation, &'largs ::qt_gui::text_cursor::MoveMode) {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {
      let operation = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_widgets_c_QTextEdit_moveCursor_operation_mode(original_self as *mut ::text_edit::TextEdit, operation as *const ::qt_gui::text_cursor::MoveOperation, mode as *const ::qt_gui::text_cursor::MoveMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::new](../struct.TextEdit.html#method.new) method.
  pub trait TextEditNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit>;
  }
  impl TextEditNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TextEditNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTextEdit_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::new_unsafe](../struct.TextEdit.html#method.new_unsafe) method.
  pub trait TextEditNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit>;
  }
  impl TextEditNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QTextEdit_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TextEditNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_edit::TextEdit> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QTextEdit_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::zoom_in](../struct.TextEdit.html#method.zoom_in) method.
  pub trait TextEditZoomInArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> ();
  }
  impl<'largs> TextEditZoomInArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {

      unsafe { ::ffi::qt_widgets_c_QTextEdit_zoomIn_no_args(original_self as *mut ::text_edit::TextEdit) }
    }
  }
  impl<'largs> TextEditZoomInArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {
      let range = self;
      unsafe { ::ffi::qt_widgets_c_QTextEdit_zoomIn_range(original_self as *mut ::text_edit::TextEdit, range) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextEdit::zoom_out](../struct.TextEdit.html#method.zoom_out) method.
  pub trait TextEditZoomOutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> ();
  }
  impl<'largs> TextEditZoomOutArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {

      unsafe { ::ffi::qt_widgets_c_QTextEdit_zoomOut_no_args(original_self as *mut ::text_edit::TextEdit) }
    }
  }
  impl<'largs> TextEditZoomOutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_edit::TextEdit) -> () {
      let range = self;
      unsafe { ::ffi::qt_widgets_c_QTextEdit_zoomOut_range(original_self as *mut ::text_edit::TextEdit, range) }
    }
  }
}
