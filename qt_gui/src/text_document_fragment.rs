/// C++ type: <span style='color: green;'>```QTextDocumentFragment```</span>
#[repr(C)]
pub struct TextDocumentFragment([u8; ::type_sizes::QT_GUI_TEXT_DOCUMENT_FRAGMENT_TEXT_DOCUMENT_FRAGMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextDocumentFragment {
  unsafe fn new_uninitialized() -> TextDocumentFragment {
    TextDocumentFragment(::std::mem::uninitialized())
  }
}

impl TextDocumentFragment {
  /// C++ method: <span style='color: green;'>```static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString& html)```</span>
  ///
  ///
  pub fn from_html(html: &::qt_core::string::String) -> ::text_document_fragment::TextDocumentFragment {
    {
      let mut object: ::text_document_fragment::TextDocumentFragment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html(html as *const ::qt_core::string::String,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QTextDocumentFragment QTextDocumentFragment::fromHtml(const QString& html, const QTextDocument* resourceProvider)```</span>
  ///
  ///
  pub unsafe fn from_html_unsafe(html: &::qt_core::string::String,
                                 resource_provider: *const ::text_document::TextDocument)
                                 -> ::text_document_fragment::TextDocumentFragment {
    {
      let mut object: ::text_document_fragment::TextDocumentFragment =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextDocumentFragment_fromHtml_to_output_html_resourceProvider(html as *const ::qt_core::string::String, resource_provider, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QTextDocumentFragment QTextDocumentFragment::fromPlainText(const QString& plainText)```</span>
  ///
  ///
  pub fn from_plain_text(plain_text: &::qt_core::string::String) -> ::text_document_fragment::TextDocumentFragment {
    {
      let mut object: ::text_document_fragment::TextDocumentFragment =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentFragment_fromPlainText_to_output(plain_text as *const ::qt_core::string::String,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextDocumentFragment::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextDocumentFragment_isEmpty(self as *const ::text_document_fragment::TextDocumentFragment)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocumentFragment::QTextDocumentFragment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_document_fragment::TextDocumentFragment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentFragment::QTextDocumentFragment()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_cursor::TextCursor) -> ::text_document_fragment::TextDocumentFragment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentFragment::QTextDocumentFragment(const QTextCursor& range)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::text_document_fragment::TextDocumentFragment) -> ::text_document_fragment::TextDocumentFragment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentFragment::QTextDocumentFragment(const QTextDocumentFragment& rhs)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_document_fragment::TextDocumentFragment
    where Args: overloading::TextDocumentFragmentNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QTextDocumentFragment::QTextDocumentFragment(const QTextDocument* document)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(document: *const ::text_document::TextDocument)
                           -> ::text_document_fragment::TextDocumentFragment {
    {
      let mut object: ::text_document_fragment::TextDocumentFragment =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QTextDocumentFragment_constructor_document(document, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextDocumentFragment& QTextDocumentFragment::operator=(const QTextDocumentFragment& rhs)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             rhs: &'l1 ::text_document_fragment::TextDocumentFragment)
                             -> &'l0 mut ::text_document_fragment::TextDocumentFragment {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextDocumentFragment_operator_assign(self as *mut ::text_document_fragment::TextDocumentFragment, rhs as *const ::text_document_fragment::TextDocumentFragment) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextDocumentFragment::toHtml```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_html(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextDocumentFragment::toHtml() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_html(&self, &::qt_core::byte_array::ByteArray) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextDocumentFragment::toHtml(const QByteArray& encoding = ?) const```</span>
  ///
  ///
  pub fn to_html<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::TextDocumentFragmentToHtmlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QTextDocumentFragment::toPlainText() const```</span>
  ///
  ///
  pub fn to_plain_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextDocumentFragment_toPlainText_to_output(self as *const ::text_document_fragment::TextDocumentFragment, &mut object);
      }
      object
    }
  }
}

impl Drop for ::text_document_fragment::TextDocumentFragment {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextDocumentFragment::~QTextDocumentFragment()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QTextDocumentFragment_destructor(self as *mut ::text_document_fragment::TextDocumentFragment)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextDocumentFragment::new](../struct.TextDocumentFragment.html#method.new) method.
  pub trait TextDocumentFragmentNewArgs {
    fn exec(self) -> ::text_document_fragment::TextDocumentFragment;
  }
  impl TextDocumentFragmentNewArgs for () {
    fn exec(self) -> ::text_document_fragment::TextDocumentFragment {

      {
        let mut object: ::text_document_fragment::TextDocumentFragment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentFragment_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextDocumentFragmentNewArgs for &'a ::text_cursor::TextCursor {
    fn exec(self) -> ::text_document_fragment::TextDocumentFragment {
      let range = self;
      {
        let mut object: ::text_document_fragment::TextDocumentFragment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentFragment_constructor_range(range as *const ::text_cursor::TextCursor,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> TextDocumentFragmentNewArgs for &'a ::text_document_fragment::TextDocumentFragment {
    fn exec(self) -> ::text_document_fragment::TextDocumentFragment {
      let rhs = self;
      {
        let mut object: ::text_document_fragment::TextDocumentFragment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentFragment_constructor_rhs(rhs as *const ::text_document_fragment::TextDocumentFragment, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextDocumentFragment::to_html](../struct.TextDocumentFragment.html#method.to_html) method.
  pub trait TextDocumentFragmentToHtmlArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_document_fragment::TextDocumentFragment) -> ::qt_core::string::String;
  }
  impl<'largs> TextDocumentFragmentToHtmlArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::text_document_fragment::TextDocumentFragment) -> ::qt_core::string::String {
      let encoding = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentFragment_toHtml_to_output_encoding(original_self as *const ::text_document_fragment::TextDocumentFragment, encoding as *const ::qt_core::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextDocumentFragmentToHtmlArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_document_fragment::TextDocumentFragment) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextDocumentFragment_toHtml_to_output_no_args(original_self as *const ::text_document_fragment::TextDocumentFragment, &mut object);
        }
        object
      }
    }
  }
}
