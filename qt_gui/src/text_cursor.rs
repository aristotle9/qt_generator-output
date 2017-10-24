/// C++ type: <span style='color: green;'>```QTextCursor::MoveMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MoveMode {
  /// C++ enum variant: <span style='color: green;'>```MoveAnchor = 0```</span>
  Move = 0,
  /// C++ enum variant: <span style='color: green;'>```KeepAnchor = 1```</span>
  Keep = 1,
}

/// C++ type: <span style='color: green;'>```QTextCursor::MoveOperation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MoveOperation {
  /// C++ enum variant: <span style='color: green;'>```NoMove = 0```</span>
  NoMove = 0,
  /// C++ enum variant: <span style='color: green;'>```Start = 1```</span>
  Start = 1,
  /// C++ enum variant: <span style='color: green;'>```Up = 2```</span>
  Up = 2,
  /// C++ enum variant: <span style='color: green;'>```StartOfLine = 3```</span>
  StartOfLine = 3,
  /// C++ enum variant: <span style='color: green;'>```StartOfBlock = 4```</span>
  StartOfBlock = 4,
  /// C++ enum variant: <span style='color: green;'>```StartOfWord = 5```</span>
  StartOfWord = 5,
  /// C++ enum variant: <span style='color: green;'>```PreviousBlock = 6```</span>
  PreviousBlock = 6,
  /// C++ enum variant: <span style='color: green;'>```PreviousCharacter = 7```</span>
  PreviousCharacter = 7,
  /// C++ enum variant: <span style='color: green;'>```PreviousWord = 8```</span>
  PreviousWord = 8,
  /// C++ enum variant: <span style='color: green;'>```Left = 9```</span>
  Left = 9,
  /// C++ enum variant: <span style='color: green;'>```WordLeft = 10```</span>
  WordLeft = 10,
  /// C++ enum variant: <span style='color: green;'>```End = 11```</span>
  End = 11,
  /// C++ enum variant: <span style='color: green;'>```Down = 12```</span>
  Down = 12,
  /// C++ enum variant: <span style='color: green;'>```EndOfLine = 13```</span>
  EndOfLine = 13,
  /// C++ enum variant: <span style='color: green;'>```EndOfWord = 14```</span>
  EndOfWord = 14,
  /// C++ enum variant: <span style='color: green;'>```EndOfBlock = 15```</span>
  EndOfBlock = 15,
  /// C++ enum variant: <span style='color: green;'>```NextBlock = 16```</span>
  NextBlock = 16,
  /// C++ enum variant: <span style='color: green;'>```NextCharacter = 17```</span>
  NextCharacter = 17,
  /// C++ enum variant: <span style='color: green;'>```NextWord = 18```</span>
  NextWord = 18,
  /// C++ enum variant: <span style='color: green;'>```Right = 19```</span>
  Right = 19,
  /// C++ enum variant: <span style='color: green;'>```WordRight = 20```</span>
  WordRight = 20,
  /// C++ enum variant: <span style='color: green;'>```NextCell = 21```</span>
  NextCell = 21,
  /// C++ enum variant: <span style='color: green;'>```PreviousCell = 22```</span>
  PreviousCell = 22,
  /// C++ enum variant: <span style='color: green;'>```NextRow = 23```</span>
  NextRow = 23,
  /// C++ enum variant: <span style='color: green;'>```PreviousRow = 24```</span>
  PreviousRow = 24,
}

/// C++ type: <span style='color: green;'>```QTextCursor::SelectionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectionType {
  /// C++ enum variant: <span style='color: green;'>```WordUnderCursor = 0```</span>
  WordUnderCursor = 0,
  /// C++ enum variant: <span style='color: green;'>```LineUnderCursor = 1```</span>
  LineUnderCursor = 1,
  /// C++ enum variant: <span style='color: green;'>```BlockUnderCursor = 2```</span>
  BlockUnderCursor = 2,
  /// C++ enum variant: <span style='color: green;'>```Document = 3```</span>
  Document = 3,
}

/// C++ type: <span style='color: green;'>```QTextCursor```</span>
#[repr(C)]
pub struct TextCursor(u8);

impl TextCursor {
  /// C++ method: <span style='color: green;'>```int QTextCursor::anchor() const```</span>
  ///
  ///
  pub fn anchor(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_anchor(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::atBlockEnd() const```</span>
  ///
  ///
  pub fn at_block_end(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_atBlockEnd(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::atBlockStart() const```</span>
  ///
  ///
  pub fn at_block_start(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_atBlockStart(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_atEnd(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::atStart() const```</span>
  ///
  ///
  pub fn at_start(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_atStart(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::beginEditBlock()```</span>
  ///
  ///
  pub fn begin_edit_block(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_beginEditBlock(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextCursor::block() const```</span>
  ///
  ///
  pub fn block(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_block_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextCursor::blockCharFormat() const```</span>
  ///
  ///
  pub fn block_char_format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_blockCharFormat_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlockFormat QTextCursor::blockFormat() const```</span>
  ///
  ///
  pub fn block_format(&self) -> ::text_block_format::TextBlockFormat {
    {
      let mut object: ::text_block_format::TextBlockFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_blockFormat_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::blockNumber() const```</span>
  ///
  ///
  pub fn block_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_blockNumber(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextCursor::charFormat() const```</span>
  ///
  ///
  pub fn char_format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_charFormat_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::clearSelection()```</span>
  ///
  ///
  pub fn clear_selection(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_clearSelection(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::columnNumber() const```</span>
  ///
  ///
  pub fn column_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_columnNumber(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor::createList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_list(&mut self, &::text_list_format::Style) -> *mut ::text_list::TextList```<br>
  /// C++ method: <span style='color: green;'>```QTextList* QTextCursor::createList(QTextListFormat::Style style)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_list(&mut self, &::text_list_format::TextListFormat) -> *mut ::text_list::TextList```<br>
  /// C++ method: <span style='color: green;'>```QTextList* QTextCursor::createList(const QTextListFormat& format)```</span>
  ///
  ///
  pub fn create_list<'largs, Args>(&'largs mut self, args: Args) -> *mut ::text_list::TextList
    where Args: overloading::TextCursorCreateListArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextFrame* QTextCursor::currentFrame() const```</span>
  ///
  ///
  pub fn current_frame(&self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QTextCursor_currentFrame(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextList* QTextCursor::currentList() const```</span>
  ///
  ///
  pub fn current_list(&self) -> *mut ::text_list::TextList {
    unsafe { ::ffi::qt_gui_c_QTextCursor_currentList(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextTable* QTextCursor::currentTable() const```</span>
  ///
  ///
  pub fn current_table(&self) -> *mut ::text_table::TextTable {
    unsafe { ::ffi::qt_gui_c_QTextCursor_currentTable(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::deleteChar()```</span>
  ///
  ///
  pub fn delete_char(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_deleteChar(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::deletePreviousChar()```</span>
  ///
  ///
  pub fn delete_previous_char(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_deletePreviousChar(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextDocument* QTextCursor::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *mut ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QTextCursor_document(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::endEditBlock()```</span>
  ///
  ///
  pub fn end_edit_block(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_endEditBlock(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::hasComplexSelection() const```</span>
  ///
  ///
  pub fn has_complex_selection(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_hasComplexSelection(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::hasSelection() const```</span>
  ///
  ///
  pub fn has_selection(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_hasSelection(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor::insertBlock```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_block(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertBlock()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_block(&mut self, &::text_block_format::TextBlockFormat) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertBlock(const QTextBlockFormat& format)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_block(&mut self, (&::text_block_format::TextBlockFormat, &::text_char_format::TextCharFormat)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertBlock(const QTextBlockFormat& format, const QTextCharFormat& charFormat)```</span>
  ///
  ///
  pub fn insert_block<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCursorInsertBlockArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertFragment(const QTextDocumentFragment& fragment)```</span>
  ///
  ///
  pub fn insert_fragment(&mut self, fragment: &::text_document_fragment::TextDocumentFragment) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_insertFragment(self as *mut ::text_cursor::TextCursor,
                                                 fragment as *const ::text_document_fragment::TextDocumentFragment)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QTextCursor::insertFrame(const QTextFrameFormat& format)```</span>
  ///
  ///
  pub fn insert_frame(&mut self, format: &::text_frame_format::TextFrameFormat) -> *mut ::text_frame::TextFrame {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_insertFrame(self as *mut ::text_cursor::TextCursor,
                                              format as *const ::text_frame_format::TextFrameFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::insertHtml(const QString& html)```</span>
  ///
  ///
  pub fn insert_html(&mut self, html: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_insertHtml(self as *mut ::text_cursor::TextCursor,
                                             html as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor::insertImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_image(&mut self, &::image::Image) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertImage(const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_image(&mut self, (&::image::Image, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertImage(const QImage& image, const QString& name = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_image(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertImage(const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn insert_image(&mut self, &::text_image_format::TextImageFormat) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertImage(const QTextImageFormat& format)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn insert_image(&mut self, (&::text_image_format::TextImageFormat, &::text_frame_format::Position)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertImage(const QTextImageFormat& format, QTextFrameFormat::Position alignment)```</span>
  ///
  ///
  pub fn insert_image<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCursorInsertImageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCursor::insertList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_list(&mut self, &::text_list_format::Style) -> *mut ::text_list::TextList```<br>
  /// C++ method: <span style='color: green;'>```QTextList* QTextCursor::insertList(QTextListFormat::Style style)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_list(&mut self, &::text_list_format::TextListFormat) -> *mut ::text_list::TextList```<br>
  /// C++ method: <span style='color: green;'>```QTextList* QTextCursor::insertList(const QTextListFormat& format)```</span>
  ///
  ///
  pub fn insert_list<'largs, Args>(&'largs mut self, args: Args) -> *mut ::text_list::TextList
    where Args: overloading::TextCursorInsertListArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCursor::insertTable```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_table(&mut self, (::libc::c_int, ::libc::c_int)) -> *mut ::text_table::TextTable```<br>
  /// C++ method: <span style='color: green;'>```QTextTable* QTextCursor::insertTable(int rows, int cols)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_table(&mut self, (::libc::c_int, ::libc::c_int, &::text_table_format::TextTableFormat)) -> *mut ::text_table::TextTable```<br>
  /// C++ method: <span style='color: green;'>```QTextTable* QTextCursor::insertTable(int rows, int cols, const QTextTableFormat& format)```</span>
  ///
  ///
  pub fn insert_table<'largs, Args>(&'largs mut self, args: Args) -> *mut ::text_table::TextTable
    where Args: overloading::TextCursorInsertTableArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCursor::insertText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_text(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertText(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_text(&mut self, (&::qt_core::string::String, &::text_char_format::TextCharFormat)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::insertText(const QString& text, const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn insert_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCursorInsertTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTextCursor::isCopyOf(const QTextCursor& other) const```</span>
  ///
  ///
  pub fn is_copy_of(&self, other: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_isCopyOf(self as *const ::text_cursor::TextCursor,
                                           other as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_isNull(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::joinPreviousEditBlock()```</span>
  ///
  ///
  pub fn join_previous_edit_block(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_joinPreviousEditBlock(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::keepPositionOnInsert() const```</span>
  ///
  ///
  pub fn keep_position_on_insert(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_keepPositionOnInsert(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::mergeBlockCharFormat(const QTextCharFormat& modifier)```</span>
  ///
  ///
  pub fn merge_block_char_format(&mut self, modifier: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_mergeBlockCharFormat(self as *mut ::text_cursor::TextCursor,
                                                       modifier as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::mergeBlockFormat(const QTextBlockFormat& modifier)```</span>
  ///
  ///
  pub fn merge_block_format(&mut self, modifier: &::text_block_format::TextBlockFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_mergeBlockFormat(self as *mut ::text_cursor::TextCursor,
                                                   modifier as *const ::text_block_format::TextBlockFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::mergeCharFormat(const QTextCharFormat& modifier)```</span>
  ///
  ///
  pub fn merge_char_format(&mut self, modifier: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_mergeCharFormat(self as *mut ::text_cursor::TextCursor,
                                                  modifier as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor::movePosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_position(&mut self, ::text_cursor::MoveOperation) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextCursor::movePosition(QTextCursor::MoveOperation op)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_position(&mut self, (::text_cursor::MoveOperation, ::text_cursor::MoveMode)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextCursor::movePosition(QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn move_position(&mut self, (::text_cursor::MoveOperation, ::text_cursor::MoveMode, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextCursor::movePosition(QTextCursor::MoveOperation op, QTextCursor::MoveMode arg2 = ?, int n = ?)```</span>
  ///
  ///
  pub fn move_position<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::TextCursorMovePositionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextCursor::QTextCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCursor::QTextCursor()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_block::TextBlock) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCursor::QTextCursor(const QTextBlock& block)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::text_cursor::TextCursor) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCursor::QTextCursor(const QTextCursor& cursor)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>
    where Args: overloading::TextCursorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextCursor::QTextCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::text_document::TextDocument) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCursor::QTextCursor(QTextDocument* document)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::text_frame::TextFrame) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextCursor::QTextCursor(QTextFrame* frame)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>
    where Args: overloading::TextCursorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextCursor& QTextCursor::operator=(const QTextCursor& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::text_cursor::TextCursor)
                             -> &'l0 mut ::text_cursor::TextCursor {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_assign(self as *mut ::text_cursor::TextCursor,
                                                  other as *const ::text_cursor::TextCursor)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator==(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_eq(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_eq(self as *const ::text_cursor::TextCursor,
                                              rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator>=(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_ge(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_ge(self as *const ::text_cursor::TextCursor,
                                              rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator>(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_gt(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_gt(self as *const ::text_cursor::TextCursor,
                                              rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator<=(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_le(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_le(self as *const ::text_cursor::TextCursor,
                                              rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator<(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_lt(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_lt(self as *const ::text_cursor::TextCursor,
                                              rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::operator!=(const QTextCursor& rhs) const```</span>
  ///
  ///
  pub fn op_neq(&self, rhs: &::text_cursor::TextCursor) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_operator_neq(self as *const ::text_cursor::TextCursor,
                                               rhs as *const ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_position(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::positionInBlock() const```</span>
  ///
  ///
  pub fn position_in_block(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_positionInBlock(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::removeSelectedText()```</span>
  ///
  ///
  pub fn remove_selected_text(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_removeSelectedText(self as *mut ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::select(QTextCursor::SelectionType selection)```</span>
  ///
  ///
  pub fn select(&mut self, selection: ::text_cursor::SelectionType) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_select(self as *mut ::text_cursor::TextCursor, selection) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::selectedTableCells(int* firstRow, int* numRows, int* firstColumn, int* numColumns) const```</span>
  ///
  ///
  pub unsafe fn selected_table_cells(&self,
                                     first_row: *mut ::libc::c_int,
                                     num_rows: *mut ::libc::c_int,
                                     first_column: *mut ::libc::c_int,
                                     num_columns: *mut ::libc::c_int) {
    ::ffi::qt_gui_c_QTextCursor_selectedTableCells(self as *const ::text_cursor::TextCursor,
                                                   first_row,
                                                   num_rows,
                                                   first_column,
                                                   num_columns)
  }

  /// C++ method: <span style='color: green;'>```QString QTextCursor::selectedText() const```</span>
  ///
  ///
  pub fn selected_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_selectedText_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocumentFragment QTextCursor::selection() const```</span>
  ///
  ///
  pub fn selection(&self) -> ::text_document_fragment::TextDocumentFragment {
    {
      let mut object: ::text_document_fragment::TextDocumentFragment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_selection_to_output(self as *const ::text_cursor::TextCursor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::selectionEnd() const```</span>
  ///
  ///
  pub fn selection_end(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_selectionEnd(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::selectionStart() const```</span>
  ///
  ///
  pub fn selection_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_selectionStart(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::setBlockCharFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_block_char_format(&mut self, format: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_setBlockCharFormat(self as *mut ::text_cursor::TextCursor,
                                                     format as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::setBlockFormat(const QTextBlockFormat& format)```</span>
  ///
  ///
  pub fn set_block_format(&mut self, format: &::text_block_format::TextBlockFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_setBlockFormat(self as *mut ::text_cursor::TextCursor,
                                                 format as *const ::text_block_format::TextBlockFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::setCharFormat(const QTextCharFormat& format)```</span>
  ///
  ///
  pub fn set_char_format(&mut self, format: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_setCharFormat(self as *mut ::text_cursor::TextCursor,
                                                format as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::setKeepPositionOnInsert(bool b)```</span>
  ///
  ///
  pub fn set_keep_position_on_insert(&mut self, b: bool) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_setKeepPositionOnInsert(self as *mut ::text_cursor::TextCursor, b) }
  }

  /// C++ method: <span style='color: green;'>```QTextCursor::setPosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_position(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::setPosition(int pos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_position(&mut self, (::libc::c_int, ::text_cursor::MoveMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCursor::setPosition(int pos, QTextCursor::MoveMode mode = ?)```</span>
  ///
  ///
  pub fn set_position<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCursorSetPositionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextCursor::setVerticalMovementX(int x)```</span>
  ///
  ///
  pub fn set_vertical_movement_x(&mut self, x: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_setVerticalMovementX(self as *mut ::text_cursor::TextCursor, x) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::setVisualNavigation(bool b)```</span>
  ///
  ///
  pub fn set_visual_navigation(&mut self, b: bool) {
    unsafe { ::ffi::qt_gui_c_QTextCursor_setVisualNavigation(self as *mut ::text_cursor::TextCursor, b) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCursor::swap(QTextCursor& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::text_cursor::TextCursor) {
    unsafe {
      ::ffi::qt_gui_c_QTextCursor_swap(self as *mut ::text_cursor::TextCursor,
                                       other as *mut ::text_cursor::TextCursor)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextCursor::verticalMovementX() const```</span>
  ///
  ///
  pub fn vertical_movement_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCursor_verticalMovementX(self as *const ::text_cursor::TextCursor) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCursor::visualNavigation() const```</span>
  ///
  ///
  pub fn visual_navigation(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCursor_visualNavigation(self as *const ::text_cursor::TextCursor) }
  }
}

impl ::cpp_utils::CppDeletable for ::text_cursor::TextCursor {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextCursor_delete
  }
}

/// C++ method: <span style='color: green;'>```void swap(QTextCursor& value1, QTextCursor& value2)```</span>
///
///
pub fn swap(value1: &mut ::text_cursor::TextCursor, value2: &mut ::text_cursor::TextCursor) {
  unsafe {
    ::ffi::qt_gui_c_QTextCursor_G_swap(value1 as *mut ::text_cursor::TextCursor,
                                       value2 as *mut ::text_cursor::TextCursor)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextCursor::create_list](../struct.TextCursor.html#method.create_list) method.
  pub trait TextCursorCreateListArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList;
  }
  impl<'largs> TextCursorCreateListArgs<'largs> for &'largs ::text_list_format::TextListFormat {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList {
      let format = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_createList_format(original_self as *mut ::text_cursor::TextCursor,
                                                      format as *const ::text_list_format::TextListFormat)
      }
    }
  }
  impl<'largs> TextCursorCreateListArgs<'largs> for &'largs ::text_list_format::Style {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList {
      let style = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_createList_style(original_self as *mut ::text_cursor::TextCursor,
                                                     style as *const ::text_list_format::Style)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::insert_block](../struct.TextCursor.html#method.insert_block) method.
  pub trait TextCursorInsertBlockArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> ();
  }
  impl<'largs> TextCursorInsertBlockArgs<'largs> for &'largs ::text_block_format::TextBlockFormat {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let format = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertBlock_format(original_self as *mut ::text_cursor::TextCursor,
                                                       format as *const ::text_block_format::TextBlockFormat)
      }
    }
  }
  impl<'largs> TextCursorInsertBlockArgs<'largs>
    for (&'largs ::text_block_format::TextBlockFormat, &'largs ::text_char_format::TextCharFormat) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let format = self.0;
      let char_format = self.1;
      unsafe { ::ffi::qt_gui_c_QTextCursor_insertBlock_format_charFormat(original_self as *mut ::text_cursor::TextCursor, format as *const ::text_block_format::TextBlockFormat, char_format as *const ::text_char_format::TextCharFormat) }
    }
  }
  impl<'largs> TextCursorInsertBlockArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {

      unsafe { ::ffi::qt_gui_c_QTextCursor_insertBlock_no_args(original_self as *mut ::text_cursor::TextCursor) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::insert_image](../struct.TextCursor.html#method.insert_image) method.
  pub trait TextCursorInsertImageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> ();
  }
  impl<'largs> TextCursorInsertImageArgs<'largs> for &'largs ::text_image_format::TextImageFormat {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let format = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertImage_format(original_self as *mut ::text_cursor::TextCursor,
                                                       format as *const ::text_image_format::TextImageFormat)
      }
    }
  }
  impl<'largs> TextCursorInsertImageArgs<'largs>
    for (&'largs ::text_image_format::TextImageFormat, &'largs ::text_frame_format::Position) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let format = self.0;
      let alignment = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertImage_format_alignment(original_self as *mut ::text_cursor::TextCursor, format as *const ::text_image_format::TextImageFormat, alignment as *const ::text_frame_format::Position)
      }
    }
  }
  impl<'largs> TextCursorInsertImageArgs<'largs> for &'largs ::image::Image {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let image = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertImage_image(original_self as *mut ::text_cursor::TextCursor,
                                                      image as *const ::image::Image)
      }
    }
  }
  impl<'largs> TextCursorInsertImageArgs<'largs> for (&'largs ::image::Image, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let image = self.0;
      let name = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertImage_image_name(original_self as *mut ::text_cursor::TextCursor,
                                                           image as *const ::image::Image,
                                                           name as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> TextCursorInsertImageArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let name = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertImage_name(original_self as *mut ::text_cursor::TextCursor,
                                                     name as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::insert_list](../struct.TextCursor.html#method.insert_list) method.
  pub trait TextCursorInsertListArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList;
  }
  impl<'largs> TextCursorInsertListArgs<'largs> for &'largs ::text_list_format::TextListFormat {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList {
      let format = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertList_format(original_self as *mut ::text_cursor::TextCursor,
                                                      format as *const ::text_list_format::TextListFormat)
      }
    }
  }
  impl<'largs> TextCursorInsertListArgs<'largs> for &'largs ::text_list_format::Style {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_list::TextList {
      let style = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertList_style(original_self as *mut ::text_cursor::TextCursor,
                                                     style as *const ::text_list_format::Style)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::insert_table](../struct.TextCursor.html#method.insert_table) method.
  pub trait TextCursorInsertTableArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_table::TextTable;
  }
  impl<'largs> TextCursorInsertTableArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_table::TextTable {
      let rows = self.0;
      let cols = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertTable_rows_cols(original_self as *mut ::text_cursor::TextCursor, rows, cols)
      }
    }
  }
  impl<'largs> TextCursorInsertTableArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::text_table_format::TextTableFormat) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> *mut ::text_table::TextTable {
      let rows = self.0;
      let cols = self.1;
      let format = self.2;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertTable_rows_cols_format(original_self as *mut ::text_cursor::TextCursor, rows, cols, format as *const ::text_table_format::TextTableFormat)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::insert_text](../struct.TextCursor.html#method.insert_text) method.
  pub trait TextCursorInsertTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> ();
  }
  impl<'largs> TextCursorInsertTextArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let text = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertText_text(original_self as *mut ::text_cursor::TextCursor,
                                                    text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> TextCursorInsertTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::text_char_format::TextCharFormat) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let text = self.0;
      let format = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_insertText_text_format(original_self as *mut ::text_cursor::TextCursor,
                                                           text as *const ::qt_core::string::String,
                                                           format as *const ::text_char_format::TextCharFormat)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::move_position](../struct.TextCursor.html#method.move_position) method.
  pub trait TextCursorMovePositionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> bool;
  }
  impl<'largs> TextCursorMovePositionArgs<'largs> for ::text_cursor::MoveOperation {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> bool {
      let op = self;
      unsafe { ::ffi::qt_gui_c_QTextCursor_movePosition_op(original_self as *mut ::text_cursor::TextCursor, op) }
    }
  }
  impl<'largs> TextCursorMovePositionArgs<'largs> for (::text_cursor::MoveOperation, ::text_cursor::MoveMode) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> bool {
      let op = self.0;
      let arg2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_movePosition_op_arg2(original_self as *mut ::text_cursor::TextCursor, op, arg2)
      }
    }
  }
  impl<'largs> TextCursorMovePositionArgs<'largs>
    for (::text_cursor::MoveOperation, ::text_cursor::MoveMode, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> bool {
      let op = self.0;
      let arg2 = self.1;
      let n = self.2;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_movePosition_op_arg2_n(original_self as *mut ::text_cursor::TextCursor, op, arg2, n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::new](../struct.TextCursor.html#method.new) method.
  pub trait TextCursorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>;
  }
  impl<'a> TextCursorNewArgs for &'a ::text_block::TextBlock {
    fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let block = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextCursor_new_block(block as *const ::text_block::TextBlock) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TextCursorNewArgs for &'a ::text_cursor::TextCursor {
    fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let cursor = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextCursor_new_cursor(cursor as *const ::text_cursor::TextCursor) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TextCursorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextCursor_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::new_unsafe](../struct.TextCursor.html#method.new_unsafe) method.
  pub trait TextCursorNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor>;
  }
  impl TextCursorNewUnsafeArgs for *mut ::text_document::TextDocument {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let document = self;
      let ffi_result = ::ffi::qt_gui_c_QTextCursor_new_document(document);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TextCursorNewUnsafeArgs for *mut ::text_frame::TextFrame {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::text_cursor::TextCursor> {
      let frame = self;
      let ffi_result = ::ffi::qt_gui_c_QTextCursor_new_frame(frame);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [TextCursor::set_position](../struct.TextCursor.html#method.set_position) method.
  pub trait TextCursorSetPositionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> ();
  }
  impl<'largs> TextCursorSetPositionArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let pos = self;
      unsafe { ::ffi::qt_gui_c_QTextCursor_setPosition_pos(original_self as *mut ::text_cursor::TextCursor, pos) }
    }
  }
  impl<'largs> TextCursorSetPositionArgs<'largs> for (::libc::c_int, ::text_cursor::MoveMode) {
    fn exec(self, original_self: &'largs mut ::text_cursor::TextCursor) -> () {
      let pos = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCursor_setPosition_pos_mode(original_self as *mut ::text_cursor::TextCursor, pos, mode)
      }
    }
  }
}
