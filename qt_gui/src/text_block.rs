/// C++ type: <span style='color: green;'>```QTextBlock::iterator```</span>
#[repr(C)]
pub struct Iterator([u8; ::type_sizes::QT_GUI_TEXT_BLOCK_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Iterator {
  unsafe fn new_uninitialized() -> Iterator {
    Iterator(::std::mem::uninitialized())
  }
}

impl Iterator {
  /// C++ method: <span style='color: green;'>```bool QTextBlock::iterator::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlock_iterator_atEnd(self as *const ::text_block::Iterator) }
  }

  /// C++ method: <span style='color: green;'>```QTextFragment QTextBlock::iterator::fragment() const```</span>
  ///
  ///
  pub fn fragment(&self) -> ::text_fragment::TextFragment {
    {
      let mut object: ::text_fragment::TextFragment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_iterator_fragment_to_output(self as *const ::text_block::Iterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::iterator::iterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_block::Iterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBlock::iterator::iterator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_block::Iterator) -> ::text_block::Iterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBlock::iterator::iterator(const QTextBlock::iterator& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_block::Iterator
    where Args: overloading::IteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextBlock::iterator& QTextBlock::iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::Iterator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlock_iterator_operator_dec(self as *mut ::text_block::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::iterator QTextBlock::iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::text_block::Iterator {
    {
      let mut object: ::text_block::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_iterator_operator_dec_postfix_to_output(self as *mut ::text_block::Iterator,
                                                                           arg1,
                                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::iterator::operator==(const QTextBlock::iterator& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::text_block::Iterator) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextBlock_iterator_operator_eq(self as *const ::text_block::Iterator,
                                                      o as *const ::text_block::Iterator)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::iterator& QTextBlock::iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::Iterator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlock_iterator_operator_inc(self as *mut ::text_block::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::iterator QTextBlock::iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::text_block::Iterator {
    {
      let mut object: ::text_block::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_iterator_operator_inc_postfix_to_output(self as *mut ::text_block::Iterator,
                                                                           arg1,
                                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::iterator::operator!=(const QTextBlock::iterator& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::text_block::Iterator) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextBlock_iterator_operator_neq(self as *const ::text_block::Iterator,
                                                       o as *const ::text_block::Iterator)
    }
  }
}

impl Drop for ::text_block::Iterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextBlock::iterator::~QTextBlock::iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_iterator_destructor(self as *mut ::text_block::Iterator) }
  }
}

/// C++ type: <span style='color: green;'>```QTextBlock```</span>
#[repr(C)]
pub struct TextBlock([u8; ::type_sizes::QT_GUI_TEXT_BLOCK_TEXT_BLOCK]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextBlock {
  unsafe fn new_uninitialized() -> TextBlock {
    TextBlock(::std::mem::uninitialized())
  }
}

impl TextBlock {
  /// C++ method: <span style='color: green;'>```QTextBlock::iterator QTextBlock::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::text_block::Iterator {
    {
      let mut object: ::text_block::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_begin_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlockFormat QTextBlock::blockFormat() const```</span>
  ///
  ///
  pub fn block_format(&self) -> ::text_block_format::TextBlockFormat {
    {
      let mut object: ::text_block_format::TextBlockFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_blockFormat_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::blockFormatIndex() const```</span>
  ///
  ///
  pub fn block_format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_blockFormatIndex(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::blockNumber() const```</span>
  ///
  ///
  pub fn block_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_blockNumber(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextBlock::charFormat() const```</span>
  ///
  ///
  pub fn char_format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_charFormat_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::charFormatIndex() const```</span>
  ///
  ///
  pub fn char_format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_charFormatIndex(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::clearLayout()```</span>
  ///
  ///
  pub fn clear_layout(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_clearLayout(self as *mut ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::contains(int position) const```</span>
  ///
  ///
  pub fn contains(&self, position: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlock_contains(self as *const ::text_block::TextBlock, position) }
  }

  /// C++ method: <span style='color: green;'>```const QTextDocument* QTextBlock::document() const```</span>
  ///
  ///
  pub fn document(&self) -> *const ::text_document::TextDocument {
    unsafe { ::ffi::qt_gui_c_QTextBlock_document(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::iterator QTextBlock::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::text_block::Iterator {
    {
      let mut object: ::text_block::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_end_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::firstLineNumber() const```</span>
  ///
  ///
  pub fn first_line_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_firstLineNumber(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::fragmentIndex() const```</span>
  ///
  ///
  pub fn fragment_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_fragmentIndex(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlock_isValid(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::isVisible() const```</span>
  ///
  ///
  pub fn is_visible(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlock_isVisible(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout* QTextBlock::layout() const```</span>
  ///
  ///
  pub fn layout(&self) -> *mut ::text_layout::TextLayout {
    unsafe { ::ffi::qt_gui_c_QTextBlock_layout(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_length(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::lineCount() const```</span>
  ///
  ///
  pub fn line_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_lineCount(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock::QTextBlock```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_block::TextBlock```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBlock::QTextBlock()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_block::TextBlock) -> ::text_block::TextBlock```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBlock::QTextBlock(const QTextBlock& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_block::TextBlock
    where Args: overloading::TextBlockNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextBlock QTextBlock::next() const```</span>
  ///
  ///
  pub fn next(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_next_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock& QTextBlock::operator=(const QTextBlock& o)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, o: &'l1 ::text_block::TextBlock) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextBlock_operator_assign(self as *mut ::text_block::TextBlock,
                                                 o as *const ::text_block::TextBlock)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::operator==(const QTextBlock& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextBlock_operator_eq(self as *const ::text_block::TextBlock,
                                             o as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::operator<(const QTextBlock& o) const```</span>
  ///
  ///
  pub fn op_lt(&self, o: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextBlock_operator_lt(self as *const ::text_block::TextBlock,
                                             o as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlock::operator!=(const QTextBlock& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextBlock_operator_neq(self as *const ::text_block::TextBlock,
                                              o as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_position(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QTextBlock::previous() const```</span>
  ///
  ///
  pub fn previous(&self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_previous_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::revision() const```</span>
  ///
  ///
  pub fn revision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_revision(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::setLineCount(int count)```</span>
  ///
  ///
  pub fn set_line_count(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_setLineCount(self as *mut ::text_block::TextBlock, count) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::setRevision(int rev)```</span>
  ///
  ///
  pub fn set_revision(&mut self, rev: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_setRevision(self as *mut ::text_block::TextBlock, rev) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::setUserData(QTextBlockUserData* data)```</span>
  ///
  ///
  pub unsafe fn set_user_data(&mut self, data: *mut ::text_block_user_data::TextBlockUserData) {
    ::ffi::qt_gui_c_QTextBlock_setUserData(self as *mut ::text_block::TextBlock, data)
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::setUserState(int state)```</span>
  ///
  ///
  pub fn set_user_state(&mut self, state: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_setUserState(self as *mut ::text_block::TextBlock, state) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlock::setVisible(bool visible)```</span>
  ///
  ///
  pub fn set_visible(&mut self, visible: bool) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_setVisible(self as *mut ::text_block::TextBlock, visible) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextBlock::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_text_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange> QTextBlock::textFormats() const```</span>
  ///
  ///
  pub fn text_formats(&self) -> ::vector::VectorTextLayoutFormatRange {
    {
      let mut object: ::vector::VectorTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlock_textFormats_to_output(self as *const ::text_block::TextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextList* QTextBlock::textList() const```</span>
  ///
  ///
  pub fn text_list(&self) -> *mut ::text_list::TextList {
    unsafe { ::ffi::qt_gui_c_QTextBlock_textList(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlockUserData* QTextBlock::userData() const```</span>
  ///
  ///
  pub fn user_data(&self) -> *mut ::text_block_user_data::TextBlockUserData {
    unsafe { ::ffi::qt_gui_c_QTextBlock_userData(self as *const ::text_block::TextBlock) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlock::userState() const```</span>
  ///
  ///
  pub fn user_state(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlock_userState(self as *const ::text_block::TextBlock) }
  }
}

impl Drop for ::text_block::TextBlock {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextBlock::~QTextBlock()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextBlock_destructor(self as *mut ::text_block::TextBlock) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Iterator::new](../struct.Iterator.html#method.new) method.
  pub trait IteratorNewArgs {
    fn exec(self) -> ::text_block::Iterator;
  }
  impl IteratorNewArgs for () {
    fn exec(self) -> ::text_block::Iterator {

      {
        let mut object: ::text_block::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextBlock_iterator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> IteratorNewArgs for &'a ::text_block::Iterator {
    fn exec(self) -> ::text_block::Iterator {
      let o = self;
      {
        let mut object: ::text_block::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextBlock_iterator_constructor_o(o as *const ::text_block::Iterator, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextBlock::new](../struct.TextBlock.html#method.new) method.
  pub trait TextBlockNewArgs {
    fn exec(self) -> ::text_block::TextBlock;
  }
  impl TextBlockNewArgs for () {
    fn exec(self) -> ::text_block::TextBlock {

      {
        let mut object: ::text_block::TextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextBlock_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextBlockNewArgs for &'a ::text_block::TextBlock {
    fn exec(self) -> ::text_block::TextBlock {
      let o = self;
      {
        let mut object: ::text_block::TextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextBlock_constructor_o(o as *const ::text_block::TextBlock, &mut object);
        }
        object
      }
    }
  }
}
