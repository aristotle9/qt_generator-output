/// C++ type: <span style='color: green;'>```QTextDocument::FindFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FindFlag {
  /// C++ enum variant: <span style='color: green;'>```FindBackward = 1```</span>
  Backward = 1,
  /// C++ enum variant: <span style='color: green;'>```FindCaseSensitively = 2```</span>
  CaseSensitively = 2,
  /// C++ enum variant: <span style='color: green;'>```FindWholeWords = 4```</span>
  WholeWords = 4,
}

impl ::qt_core::flags::FlaggableEnum for FindFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FindFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextDocument::MetaInformation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MetaInformation {
  /// C++ enum variant: <span style='color: green;'>```DocumentTitle = 0```</span>
  Title = 0,
  /// C++ enum variant: <span style='color: green;'>```DocumentUrl = 1```</span>
  Url = 1,
}

/// C++ type: <span style='color: green;'>```QTextDocument::ResourceType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ResourceType {
  /// C++ enum variant: <span style='color: green;'>```HtmlResource = 1```</span>
  Html = 1,
  /// C++ enum variant: <span style='color: green;'>```ImageResource = 2```</span>
  Image = 2,
  /// C++ enum variant: <span style='color: green;'>```StyleSheetResource = 3```</span>
  StyleSheet = 3,
  /// C++ enum variant: <span style='color: green;'>```UserResource = 100```</span>
  User = 100,
}

/// C++ type: <span style='color: green;'>```QTextDocument::Stacks```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Stacks {
  /// C++ enum variant: <span style='color: green;'>```UndoStack = 1```</span>
  UndoStack = 1,
  /// C++ enum variant: <span style='color: green;'>```RedoStack = 2```</span>
  RedoStack = 2,
  /// C++ enum variant: <span style='color: green;'>```UndoAndRedoStacks = 3```</span>
  UndoAndRedoStacks = 3,
}

/// C++ type: <span style='color: green;'>```QTextDocument```</span>
#[repr(C)]
pub struct TextDocument(u8);

impl TextDocument {
  /// C++ method: <span style='color: green;'>```void QTextDocument::addResource(int type, const QUrl& name, const QVariant& resource)```</span>
  ///
  ///
  pub fn add_resource(&mut self,
                      type_: ::libc::c_int,
                      name: &::qt_core::url::Url,
                      resource: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_addResource(self as *mut ::text_document::TextDocument,
                                                type_,
                                                name as *const ::qt_core::url::Url,
                                                resource as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::adjustSize()```</span>
  ///
  ///
  pub fn adjust_size(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_adjustSize(self as *mut ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextFormat> QTextDocument::allFormats() const```</span>
  ///
  ///
  pub fn all_formats(&self) -> ::vector::VectorTextFormat {
    {
      let mut object: ::vector::VectorTextFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_allFormats_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextDocument::appendUndoItem(QAbstractUndoItem* arg1)```</span>
  ///
  ///
  pub unsafe fn append_undo_item(&mut self, arg1: *mut ::abstract_undo_item::AbstractUndoItem) {
    ::ffi::qt_gui_c_QTextDocument_appendUndoItem(self as *mut ::text_document::TextDocument, arg1)
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::availableRedoSteps() const```</span>
  ///
  ///
  pub fn available_redo_steps(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_availableRedoSteps(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::availableUndoSteps() const```</span>
  ///
  ///
  pub fn available_undo_steps(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_availableUndoSteps(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QUrl QTextDocument::baseUrl() const```</span>
  ///
  ///
  pub fn base_url(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_baseUrl_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_begin_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::blockCount() const```</span>
  ///
  ///
  pub fn block_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_blockCount(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QChar QTextDocument::characterAt(int pos) const```</span>
  ///
  ///
  pub fn character_at(&self, pos: ::libc::c_int) -> ::qt_core::char::Char {
    {
      let mut object: ::qt_core::char::Char =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_characterAt_to_output(self as *const ::text_document::TextDocument,
                                                            pos,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::characterCount() const```</span>
  ///
  ///
  pub fn character_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_characterCount(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QTextDocument::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_clear(self as *mut ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::clearUndoRedoStacks```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn clear_undo_redo_stacks(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextDocument::clearUndoRedoStacks()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn clear_undo_redo_stacks(&mut self, ::text_document::Stacks) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextDocument::clearUndoRedoStacks(QTextDocument::Stacks historyToClear = ?)```</span>
  ///
  ///
  pub fn clear_undo_redo_stacks<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextDocumentClearUndoRedoStacksArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextDocument* QTextDocument::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QTextDocument_clone_no_args(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QTextDocument::clone(QObject* parent = ?) const```</span>
  ///
  ///
  pub unsafe fn clone_unsafe(&self, parent: *mut ::qt_core::object::Object) -> *mut ::text_document::TextDocument {
    ::ffi::qt_gui_c_QTextDocument_clone_parent(self as *const ::text_document::TextDocument, parent)
  }

  /// C++ method: <span style='color: green;'>```QFont QTextDocument::defaultFont() const```</span>
  ///
  ///
  pub fn default_font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_defaultFont_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextDocument::defaultStyleSheet() const```</span>
  ///
  ///
  pub fn default_style_sheet(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_defaultStyleSheet_to_output(self as *const ::text_document::TextDocument,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextOption QTextDocument::defaultTextOption() const```</span>
  ///
  ///
  pub fn default_text_option(&self) -> ::text_option::TextOption {
    {
      let mut object: ::text_option::TextOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_defaultTextOption_to_output(self as *const ::text_document::TextDocument,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTextDocumentLayout* QTextDocument::documentLayout() const```</span>
  ///
  ///
  pub fn document_layout(&self) -> *mut ::abstract_text_document_layout::AbstractTextDocumentLayout {
    unsafe { ::ffi::qt_gui_c_QTextDocument_documentLayout(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```double QTextDocument::documentMargin() const```</span>
  ///
  ///
  pub fn document_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextDocument_documentMargin(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::drawContents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_contents(&mut self, *mut ::painter::Painter) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextDocument::drawContents(QPainter* painter)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_contents(&mut self, (*mut ::painter::Painter, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextDocument::drawContents(QPainter* painter, const QRectF& rect = ?)```</span>
  ///
  ///
  pub unsafe fn draw_contents<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextDocumentDrawContentsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_end_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::find```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find(&self, &::qt_core::reg_exp::RegExp) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegExp& expr) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::reg_exp::RegExp, &::text_cursor::TextCursor)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegExp& expr, const QTextCursor& cursor) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::reg_exp::RegExp, &::text_cursor::TextCursor, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegExp& expr, const QTextCursor& cursor, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::reg_exp::RegExp, ::libc::c_int)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegExp& expr, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::reg_exp::RegExp, ::libc::c_int, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegExp& expr, int from = ?, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn find(&self, &::qt_core::regular_expression::RegularExpression) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegularExpression& expr) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::regular_expression::RegularExpression, &::text_cursor::TextCursor)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegularExpression& expr, const QTextCursor& cursor) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::regular_expression::RegularExpression, &::text_cursor::TextCursor, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegularExpression& expr, const QTextCursor& cursor, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::regular_expression::RegularExpression, ::libc::c_int)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegularExpression& expr, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::regular_expression::RegularExpression, ::libc::c_int, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QRegularExpression& expr, int from = ?, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn find(&self, &::qt_core::string::String) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QString& subString) const```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::string::String, &::text_cursor::TextCursor)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QString& subString, const QTextCursor& cursor) const```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::string::String, &::text_cursor::TextCursor, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QString& subString, const QTextCursor& cursor, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::string::String, ::libc::c_int)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QString& subString, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn find(&self, (&::qt_core::string::String, ::libc::c_int, ::qt_core::flags::Flags<::text_document::FindFlag>)) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```QTextCursor QTextDocument::find(const QString& subString, int from = ?, QFlags<QTextDocument::FindFlag> options = ?) const```</span>
  ///
  ///
  pub fn find<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>
    where Args: overloading::TextDocumentFindArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::findBlock(int pos) const```</span>
  ///
  ///
  pub fn find_block(&self, pos: ::libc::c_int) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_findBlock_to_output(self as *const ::text_document::TextDocument,
                                                          pos,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber) const```</span>
  ///
  ///
  pub fn find_block_by_line_number(&self, block_number: ::libc::c_int) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_findBlockByLineNumber_to_output(self as *const ::text_document::TextDocument,
                                                                      block_number,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::findBlockByNumber(int blockNumber) const```</span>
  ///
  ///
  pub fn find_block_by_number(&self, block_number: ::libc::c_int) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_findBlockByNumber_to_output(self as *const ::text_document::TextDocument,
                                                                  block_number,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::firstBlock() const```</span>
  ///
  ///
  pub fn first_block(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_firstBlock_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextDocument::frameAt(int pos) const```</span>
  ///
  ///
  pub fn frame_at(&self, pos: ::libc::c_int) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextDocument_frameAt(self as *const ::text_document::TextDocument, pos) }
  }

  /// C++ method: <span style='color: green;'>```double QTextDocument::idealWidth() const```</span>
  ///
  ///
  pub fn ideal_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextDocument_idealWidth(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```double QTextDocument::indentWidth() const```</span>
  ///
  ///
  pub fn indent_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextDocument_indentWidth(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_isEmpty(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::isModified() const```</span>
  ///
  ///
  pub fn is_modified(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_isModified(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::isRedoAvailable() const```</span>
  ///
  ///
  pub fn is_redo_available(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_isRedoAvailable(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::isUndoAvailable() const```</span>
  ///
  ///
  pub fn is_undo_available(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_isUndoAvailable(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::isUndoRedoEnabled() const```</span>
  ///
  ///
  pub fn is_undo_redo_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_isUndoRedoEnabled(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextDocument::lastBlock() const```</span>
  ///
  ///
  pub fn last_block(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_lastBlock_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::lineCount() const```</span>
  ///
  ///
  pub fn line_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_lineCount(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::markContentsDirty(int from, int length)```</span>
  ///
  ///
  pub fn mark_contents_dirty(&mut self, from: ::libc::c_int, length: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_markContentsDirty(self as *mut ::text_document::TextDocument, from, length) }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::maximumBlockCount() const```</span>
  ///
  ///
  pub fn maximum_block_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_maximumBlockCount(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextDocument::metaInformation(QTextDocument::MetaInformation info) const```</span>
  ///
  ///
  pub fn meta_information(&self, info: ::text_document::MetaInformation) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_metaInformation_to_output(self as *const ::text_document::TextDocument,
                                                                info,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTextDocument::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QTextDocument_metaObject(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::QTextDocument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::text_document::TextDocument>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocument::QTextDocument()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::text_document::TextDocument>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocument::QTextDocument(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::text_document::TextDocument>
    where Args: overloading::TextDocumentNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextDocument::QTextDocument```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::text_document::TextDocument>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocument::QTextDocument(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::text_document::TextDocument>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocument::QTextDocument(const QString& text, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::text_document::TextDocument>
    where Args: overloading::TextDocumentNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextObject* QTextDocument::object(int objectIndex) const```</span>
  ///
  ///
  pub fn object(&self, object_index: ::libc::c_int) -> *mut ::text_object::TextObject {
    unsafe { ::ffi::qt_gui_c_QTextDocument_object(self as *const ::text_document::TextDocument, object_index) }
  }

  /// C++ method: <span style='color: green;'>```QTextObject* QTextDocument::objectForFormat(const QTextFormat& arg1) const```</span>
  ///
  ///
  pub fn object_for_format(&self, arg1: &::text_format::TextFormat) -> *mut ::text_object::TextObject {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_objectForFormat(self as *const ::text_document::TextDocument,
                                                    arg1 as *const ::text_format::TextFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::pageCount() const```</span>
  ///
  ///
  pub fn page_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_pageCount(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QTextDocument::pageSize() const```</span>
  ///
  ///
  pub fn page_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_pageSize_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::print(QPagedPaintDevice* printer) const```</span>
  ///
  ///
  pub unsafe fn print(&self, printer: *mut ::paged_paint_device::PagedPaintDevice) {
    ::ffi::qt_gui_c_QTextDocument_print(self as *const ::text_document::TextDocument, printer)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTextDocument::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QTextDocument_qt_metacall(self as *mut ::text_document::TextDocument,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTextDocument::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QTextDocument_qt_metacast(self as *mut ::text_document::TextDocument, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextDocument::redo()```</span>
  ///
  ///
  pub fn redo(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_redo_no_args(self as *mut ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::redo(QTextCursor* cursor)```</span>
  ///
  ///
  pub unsafe fn redo_unsafe(&mut self, cursor: *mut ::text_cursor::TextCursor) {
    ::ffi::qt_gui_c_QTextDocument_redo_cursor(self as *mut ::text_document::TextDocument, cursor)
  }

  /// C++ method: <span style='color: green;'>```QVariant QTextDocument::resource(int type, const QUrl& name) const```</span>
  ///
  ///
  pub fn resource(&self, type_: ::libc::c_int, name: &::qt_core::url::Url) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_resource_to_output(self as *const ::text_document::TextDocument,
                                                         type_,
                                                         name as *const ::qt_core::url::Url,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextDocument::revision() const```</span>
  ///
  ///
  pub fn revision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextDocument_revision(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextDocument::rootFrame() const```</span>
  ///
  ///
  pub fn root_frame(&self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextDocument_rootFrame(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setBaseUrl(const QUrl& url)```</span>
  ///
  ///
  pub fn set_base_url(&mut self, url: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setBaseUrl(self as *mut ::text_document::TextDocument,
                                               url as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDefaultCursorMoveStyle(Qt::CursorMoveStyle style)```</span>
  ///
  ///
  pub fn set_default_cursor_move_style(&mut self, style: &::qt_core::qt::CursorMoveStyle) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setDefaultCursorMoveStyle(self as *mut ::text_document::TextDocument,
                                                              style as *const ::qt_core::qt::CursorMoveStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDefaultFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_default_font(&mut self, font: &::font::Font) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setDefaultFont(self as *mut ::text_document::TextDocument,
                                                   font as *const ::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDefaultStyleSheet(const QString& sheet)```</span>
  ///
  ///
  pub fn set_default_style_sheet(&mut self, sheet: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setDefaultStyleSheet(self as *mut ::text_document::TextDocument,
                                                         sheet as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDefaultTextOption(const QTextOption& option)```</span>
  ///
  ///
  pub fn set_default_text_option(&mut self, option: &::text_option::TextOption) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setDefaultTextOption(self as *mut ::text_document::TextDocument,
                                                         option as *const ::text_option::TextOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDocumentLayout(QAbstractTextDocumentLayout* layout)```</span>
  ///
  ///
  pub unsafe fn set_document_layout(&mut self,
                                    layout: *mut ::abstract_text_document_layout::AbstractTextDocumentLayout) {
    ::ffi::qt_gui_c_QTextDocument_setDocumentLayout(self as *mut ::text_document::TextDocument, layout)
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setDocumentMargin(double margin)```</span>
  ///
  ///
  pub fn set_document_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setDocumentMargin(self as *mut ::text_document::TextDocument, margin) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setHtml(const QString& html)```</span>
  ///
  ///
  pub fn set_html(&mut self, html: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setHtml(self as *mut ::text_document::TextDocument,
                                            html as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setIndentWidth(double width)```</span>
  ///
  ///
  pub fn set_indent_width(&mut self, width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setIndentWidth(self as *mut ::text_document::TextDocument, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setMaximumBlockCount(int maximum)```</span>
  ///
  ///
  pub fn set_maximum_block_count(&mut self, maximum: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setMaximumBlockCount(self as *mut ::text_document::TextDocument, maximum) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setMetaInformation(QTextDocument::MetaInformation info, const QString& arg2)```</span>
  ///
  ///
  pub fn set_meta_information(&mut self, info: ::text_document::MetaInformation, arg2: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setMetaInformation(self as *mut ::text_document::TextDocument,
                                                       info,
                                                       arg2 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::setModified```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_modified(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextDocument::setModified()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_modified(&mut self, bool) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTextDocument::setModified(bool m = ?)```</span>
  ///
  ///
  pub fn set_modified<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextDocumentSetModifiedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextDocument::setPageSize(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_page_size(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setPageSize(self as *mut ::text_document::TextDocument,
                                                size as *const ::qt_core::size_f::SizeF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setPlainText(const QString& text)```</span>
  ///
  ///
  pub fn set_plain_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocument_setPlainText(self as *mut ::text_document::TextDocument,
                                                 text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setTextWidth(double width)```</span>
  ///
  ///
  pub fn set_text_width(&mut self, width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setTextWidth(self as *mut ::text_document::TextDocument, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setUndoRedoEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_undo_redo_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setUndoRedoEnabled(self as *mut ::text_document::TextDocument, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::setUseDesignMetrics(bool b)```</span>
  ///
  ///
  pub fn set_use_design_metrics(&mut self, b: bool) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_setUseDesignMetrics(self as *mut ::text_document::TextDocument, b) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QTextDocument::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_size_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextDocument::textWidth() const```</span>
  ///
  ///
  pub fn text_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextDocument_textWidth(self as *const ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument::toHtml```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_html(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextDocument::toHtml() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_html(&self, &::qt_core::byte_array::ByteArray) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextDocument::toHtml(const QByteArray& encoding = ?) const```</span>
  ///
  ///
  pub fn to_html<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::TextDocumentToHtmlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QTextDocument::toPlainText() const```</span>
  ///
  ///
  pub fn to_plain_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_toPlainText_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextDocument::toRawText() const```</span>
  ///
  ///
  pub fn to_raw_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocument_toRawText_to_output(self as *const ::text_document::TextDocument, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextDocument::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextDocument_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTextDocument::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextDocument_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTextDocument::undo()```</span>
  ///
  ///
  pub fn undo(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextDocument_undo_no_args(self as *mut ::text_document::TextDocument) }
  }

  /// C++ method: <span style='color: green;'>```void QTextDocument::undo(QTextCursor* cursor)```</span>
  ///
  ///
  pub unsafe fn undo_unsafe(&mut self, cursor: *mut ::text_cursor::TextCursor) {
    ::ffi::qt_gui_c_QTextDocument_undo_cursor(self as *mut ::text_document::TextDocument, cursor)
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocument::useDesignMetrics() const```</span>
  ///
  ///
  pub fn use_design_metrics(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_useDesignMetrics(self as *const ::text_document::TextDocument) }
  }
}

impl ::cpp_utils::CppDeletable for ::text_document::TextDocument {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextDocument_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextDocument`.
  pub struct Signals<'a>(&'a ::text_document::TextDocument);
  /// Represents a built-in Qt signal `QTextDocument::modificationChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().modification_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct ModificationChanged<'a>(&'a ::text_document::TextDocument);
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
  /// Represents a built-in Qt signal `QTextDocument::undoAvailable`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().undo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct UndoAvailable<'a>(&'a ::text_document::TextDocument);
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
  /// Represents a built-in Qt signal `QTextDocument::contentsChange`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().contents_change()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct ContentsChange<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for ContentsChange<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2contentsChange(int,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContentsChange<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::objectNameChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct ObjectNameChanged<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::redoAvailable`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().redo_available()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct RedoAvailable<'a>(&'a ::text_document::TextDocument);
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
  /// Represents a built-in Qt signal `QTextDocument::cursorPositionChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().cursor_position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct CursorPositionChanged<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for CursorPositionChanged<'a> {
    type Arguments = (&'static ::text_cursor::TextCursor,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cursorPositionChanged(const QTextCursor&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CursorPositionChanged<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::blockCountChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().block_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct BlockCountChanged<'a>(&'a ::text_document::TextDocument);
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
  /// Represents a built-in Qt signal `QTextDocument::documentLayoutChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().document_layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct DocumentLayoutChanged<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for DocumentLayoutChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2documentLayoutChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DocumentLayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::contentsChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().contents_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct ContentsChanged<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for ContentsChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2contentsChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContentsChanged<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::baseUrlChanged`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().base_url_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct BaseUrlChanged<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for BaseUrlChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2baseUrlChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BaseUrlChanged<'a> {}
  /// Represents a built-in Qt signal `QTextDocument::undoCommandAdded`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.signals().undo_command_added()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct UndoCommandAdded<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for UndoCommandAdded<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2undoCommandAdded()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UndoCommandAdded<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTextDocument::modificationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn modification_changed(&self) -> ModificationChanged {
      ModificationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::undoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_available(&self) -> UndoAvailable {
      UndoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::contentsChange`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn contents_change(&self) -> ContentsChange {
      ContentsChange(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::redoAvailable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo_available(&self) -> RedoAvailable {
      RedoAvailable(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::cursorPositionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cursor_position_changed(&self) -> CursorPositionChanged {
      CursorPositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::blockCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn block_count_changed(&self) -> BlockCountChanged {
      BlockCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::documentLayoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn document_layout_changed(&self) -> DocumentLayoutChanged {
      DocumentLayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::contentsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn contents_changed(&self) -> ContentsChanged {
      ContentsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::baseUrlChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn base_url_changed(&self) -> BaseUrlChanged {
      BaseUrlChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTextDocument::undoCommandAdded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo_command_added(&self) -> UndoCommandAdded {
      UndoCommandAdded(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextDocument`.
  pub struct Slots<'a>(&'a ::text_document::TextDocument);
  /// Represents a built-in Qt slot `QTextDocument::setModified`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.slots().set_modified()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct SetModified<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for SetModified<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setModified(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextDocument::redo`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.slots().redo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct Redo<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for Redo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1redo()\0"
    }
  }
  /// Represents a built-in Qt slot `QTextDocument::appendUndoItem`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.slots().append_undo_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct AppendUndoItem<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for AppendUndoItem<'a> {
    type Arguments = (*mut ::abstract_undo_item::AbstractUndoItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1appendUndoItem(QAbstractUndoItem*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTextDocument::undo`.
  ///
  /// An object of this type can be created from `TextDocument` with `object.slots().undo()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextDocument` object.
  pub struct Undo<'a>(&'a ::text_document::TextDocument);
  impl<'a> ::qt_core::connection::Receiver for Undo<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1undo()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTextDocument::setModified`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_modified(&self) -> SetModified {
      SetModified(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextDocument::redo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn redo(&self) -> Redo {
      Redo(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextDocument::appendUndoItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn append_undo_item(&self) -> AppendUndoItem {
      AppendUndoItem(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTextDocument::undo`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn undo(&self) -> Undo {
      Undo(self.0)
    }
  }
  impl ::text_document::TextDocument {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::text_document::TextDocument {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(self as *mut ::text_document::TextDocument) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(self as *const ::text_document::TextDocument as *mut ::text_document::TextDocument) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_document::TextDocument> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_document::TextDocument {
    let ffi_result =
      ::ffi::qt_gui_c_QTextDocument_G_static_cast_QTextDocument_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_document::TextDocument {
    let ffi_result = ::ffi::qt_gui_c_QTextDocument_G_static_cast_QTextDocument_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_document::TextDocument {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(self as *const ::text_document::TextDocument as *mut ::text_document::TextDocument) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_document::TextDocument {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextDocument_G_static_cast_QObject_ptr(self as *mut ::text_document::TextDocument) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextDocument::clear_undo_redo_stacks](../struct.TextDocument.html#method.clear_undo_redo_stacks) method.
  pub trait TextDocumentClearUndoRedoStacksArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> ();
  }
  impl<'largs> TextDocumentClearUndoRedoStacksArgs<'largs> for ::text_document::Stacks {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {
      let history_to_clear = self;
      unsafe { ::ffi::qt_gui_c_QTextDocument_clearUndoRedoStacks_historyToClear(original_self as *mut ::text_document::TextDocument, history_to_clear) }
    }
  }
  impl<'largs> TextDocumentClearUndoRedoStacksArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {

      unsafe {
        ::ffi::qt_gui_c_QTextDocument_clearUndoRedoStacks_no_args(original_self as *mut ::text_document::TextDocument)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::draw_contents](../struct.TextDocument.html#method.draw_contents) method.
  pub trait TextDocumentDrawContentsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> ();
  }
  impl<'largs> TextDocumentDrawContentsArgs<'largs> for *mut ::painter::Painter {
    unsafe fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {
      let painter = self;
      ::ffi::qt_gui_c_QTextDocument_drawContents_painter(original_self as *mut ::text_document::TextDocument, painter)
    }
  }
  impl<'largs> TextDocumentDrawContentsArgs<'largs> for (*mut ::painter::Painter, &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {
      let painter = self.0;
      let rect = self.1;
      ::ffi::qt_gui_c_QTextDocument_drawContents_painter_rect(original_self as *mut ::text_document::TextDocument,
                                                              painter,
                                                              rect as *const ::qt_core::rect_f::RectF)
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::find](../struct.TextDocument.html#method.find) method.
  pub trait TextDocumentFindArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor>;
  }
  impl<'largs> TextDocumentFindArgs<'largs> for &'largs ::qt_core::reg_exp::RegExp {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegExp(original_self as *const ::text_document::TextDocument,
                                                            expr as *const ::qt_core::reg_exp::RegExp)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for (&'largs ::qt_core::reg_exp::RegExp, &'largs ::text_cursor::TextCursor) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let cursor = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::reg_exp::RegExp, cursor as *const ::text_cursor::TextCursor) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::reg_exp::RegExp,
                                                     &'largs ::text_cursor::TextCursor,
                                                     ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let cursor = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegExp_QTextCursor_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::reg_exp::RegExp, cursor as *const ::text_cursor::TextCursor, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for (&'largs ::qt_core::reg_exp::RegExp, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let from = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int(original_self as *const ::text_document::TextDocument,
                                                                expr as *const ::qt_core::reg_exp::RegExp,
                                                                from)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::reg_exp::RegExp, ::libc::c_int, ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let from = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegExp_int_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::reg_exp::RegExp, from, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for &'largs ::qt_core::regular_expression::RegularExpression {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::regular_expression::RegularExpression) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::regular_expression::RegularExpression, &'largs ::text_cursor::TextCursor) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let cursor = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::regular_expression::RegularExpression, cursor as *const ::text_cursor::TextCursor) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::regular_expression::RegularExpression,
                                                     &'largs ::text_cursor::TextCursor,
                                                     ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let cursor = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_QTextCursor_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::regular_expression::RegularExpression, cursor as *const ::text_cursor::TextCursor, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::regular_expression::RegularExpression, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let from = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::regular_expression::RegularExpression, from) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::regular_expression::RegularExpression,
                                                     ::libc::c_int,
                                                     ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let expr = self.0;
      let from = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QRegularExpression_int_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, expr as *const ::qt_core::regular_expression::RegularExpression, from, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let sub_string = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QString(original_self as *const ::text_document::TextDocument,
                                                            sub_string as *const ::qt_core::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for (&'largs ::qt_core::string::String, &'largs ::text_cursor::TextCursor) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let sub_string = self.0;
      let cursor = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor(original_self as *const ::text_document::TextDocument, sub_string as *const ::qt_core::string::String, cursor as *const ::text_cursor::TextCursor) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::string::String,
                                                     &'largs ::text_cursor::TextCursor,
                                                     ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let sub_string = self.0;
      let cursor = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QString_QTextCursor_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, sub_string as *const ::qt_core::string::String, cursor as *const ::text_cursor::TextCursor, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let sub_string = self.0;
      let from = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QString_int(original_self as *const ::text_document::TextDocument,
                                                                sub_string as *const ::qt_core::string::String,
                                                                from)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> TextDocumentFindArgs<'largs>
    for (&'largs ::qt_core::string::String, ::libc::c_int, ::qt_core::flags::Flags<::text_document::FindFlag>) {
    fn exec(self,
            original_self: &'largs ::text_document::TextDocument)
            -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let sub_string = self.0;
      let from = self.1;
      let options = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_find_as_ptr_QString_int_QFlags_QTextDocument_FindFlag(original_self as *const ::text_document::TextDocument, sub_string as *const ::qt_core::string::String, from, options.to_int() as ::libc::c_uint) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::new](../struct.TextDocument.html#method.new) method.
  pub trait TextDocumentNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument>;
  }
  impl TextDocumentNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TextDocumentNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocument_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::new_unsafe](../struct.TextDocument.html#method.new_unsafe) method.
  pub trait TextDocumentNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument>;
  }
  impl TextDocumentNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QTextDocument_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TextDocumentNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_document::TextDocument> {
      let text = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_gui_c_QTextDocument_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::set_modified](../struct.TextDocument.html#method.set_modified) method.
  pub trait TextDocumentSetModifiedArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> ();
  }
  impl<'largs> TextDocumentSetModifiedArgs<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {
      let m = self;
      unsafe { ::ffi::qt_gui_c_QTextDocument_setModified_m(original_self as *mut ::text_document::TextDocument, m) }
    }
  }
  impl<'largs> TextDocumentSetModifiedArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_document::TextDocument) -> () {

      unsafe { ::ffi::qt_gui_c_QTextDocument_setModified_no_args(original_self as *mut ::text_document::TextDocument) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocument::to_html](../struct.TextDocument.html#method.to_html) method.
  pub trait TextDocumentToHtmlArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_document::TextDocument) -> ::qt_core::string::String;
  }
  impl<'largs> TextDocumentToHtmlArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::text_document::TextDocument) -> ::qt_core::string::String {
      let encoding = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_toHtml_to_output_encoding(original_self as *const ::text_document::TextDocument, encoding as *const ::qt_core::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextDocumentToHtmlArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_document::TextDocument) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocument_toHtml_to_output_no_args(original_self as *const ::text_document::TextDocument, &mut object);
        }
        object
      }
    }
  }
}

pub mod qt {
  /// C++ method: <span style='color: green;'>```QTextCodec* Qt::codecForHtml(const QByteArray& ba)```</span>
  ///
  ///
  pub fn codec_for_html(ba: &::qt_core::byte_array::ByteArray) -> *mut ::qt_core::text_codec::TextCodec {
    unsafe { ::ffi::qt_gui_c_QTextDocument_G_Qt_codecForHtml(ba as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```Qt::convertFromPlainText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn convert_from_plain_text(&::qt_core::string::String) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString Qt::convertFromPlainText(const QString& plain)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn convert_from_plain_text((&::qt_core::string::String, ::qt_core::qt::WhiteSpaceMode)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString Qt::convertFromPlainText(const QString& plain, Qt::WhiteSpaceMode mode = ?)```</span>
  ///
  ///
  pub fn convert_from_plain_text<Args>(args: Args) -> ::qt_core::string::String
    where Args: overloading::ConvertFromPlainTextArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool Qt::mightBeRichText(const QString& arg1)```</span>
  ///
  ///
  pub fn might_be_rich_text(arg1: &::qt_core::string::String) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextDocument_G_Qt_mightBeRichText(arg1 as *const ::qt_core::string::String) }
  }

  /// Types for emulating overloading for overloaded functions in this module
  pub mod overloading {
    /// This trait represents a set of arguments accepted by [convert_from_plain_text](../fn.convert_from_plain_text.html) method.
    pub trait ConvertFromPlainTextArgs {
      fn exec(self) -> ::qt_core::string::String;
    }
    impl<'a> ConvertFromPlainTextArgs for &'a ::qt_core::string::String {
      fn exec(self) -> ::qt_core::string::String {
        let plain = self;
        {
          let mut object: ::qt_core::string::String =
            unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
          unsafe {
            ::ffi::qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain(plain as *const ::qt_core::string::String, &mut object);
          }
          object
        }
      }
    }
    impl<'a> ConvertFromPlainTextArgs for (&'a ::qt_core::string::String, ::qt_core::qt::WhiteSpaceMode) {
      fn exec(self) -> ::qt_core::string::String {
        let plain = self.0;
        let mode = self.1;
        {
          let mut object: ::qt_core::string::String =
            unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
          unsafe {
            ::ffi::qt_gui_c_QTextDocument_G_Qt_convertFromPlainText_to_output_plain_mode(plain as *const ::qt_core::string::String, mode, &mut object);
          }
          object
        }
      }
    }
  }

}
