/// C++ type: <span style='color: green;'>```QTextFragment```</span>
#[repr(C)]
pub struct TextFragment([u8; ::type_sizes::QT_GUI_TEXT_FRAGMENT_TEXT_FRAGMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextFragment {
  unsafe fn new_uninitialized() -> TextFragment {
    TextFragment(::std::mem::uninitialized())
  }
}

impl TextFragment {
  /// C++ method: <span style='color: green;'>```QTextCharFormat QTextFragment::charFormat() const```</span>
  ///
  ///
  pub fn char_format(&self) -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFragment_charFormat_to_output(self as *const ::text_fragment::TextFragment, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextFragment::charFormatIndex() const```</span>
  ///
  ///
  pub fn char_format_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFragment_charFormatIndex(self as *const ::text_fragment::TextFragment) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFragment::contains(int position) const```</span>
  ///
  ///
  pub fn contains(&self, position: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFragment_contains(self as *const ::text_fragment::TextFragment, position) }
  }

  /// C++ method: <span style='color: green;'>```QTextFragment::glyphRuns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ()) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextFragment::glyphRuns() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ::libc::c_int) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextFragment::glyphRuns(int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn glyph_runs(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextFragment::glyphRuns(int from = ?, int length = ?) const```</span>
  ///
  ///
  pub fn glyph_runs<'largs, Args>(&'largs self, args: Args) -> ::list::ListGlyphRun
    where Args: overloading::TextFragmentGlyphRunsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTextFragment::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextFragment_isValid(self as *const ::text_fragment::TextFragment) }
  }

  /// C++ method: <span style='color: green;'>```int QTextFragment::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFragment_length(self as *const ::text_fragment::TextFragment) }
  }

  /// C++ method: <span style='color: green;'>```QTextFragment::QTextFragment```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_fragment::TextFragment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFragment::QTextFragment()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::text_fragment::TextFragment) -> ::text_fragment::TextFragment```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextFragment::QTextFragment(const QTextFragment& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_fragment::TextFragment
    where Args: overloading::TextFragmentNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextFragment& QTextFragment::operator=(const QTextFragment& o)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             o: &'l1 ::text_fragment::TextFragment)
                             -> &'l0 mut ::text_fragment::TextFragment {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTextFragment_operator_assign(self as *mut ::text_fragment::TextFragment,
                                                    o as *const ::text_fragment::TextFragment)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QTextFragment::operator==(const QTextFragment& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::text_fragment::TextFragment) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFragment_operator_eq(self as *const ::text_fragment::TextFragment,
                                                o as *const ::text_fragment::TextFragment)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFragment::operator<(const QTextFragment& o) const```</span>
  ///
  ///
  pub fn op_lt(&self, o: &::text_fragment::TextFragment) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFragment_operator_lt(self as *const ::text_fragment::TextFragment,
                                                o as *const ::text_fragment::TextFragment)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextFragment::operator!=(const QTextFragment& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::text_fragment::TextFragment) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextFragment_operator_neq(self as *const ::text_fragment::TextFragment,
                                                 o as *const ::text_fragment::TextFragment)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextFragment::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextFragment_position(self as *const ::text_fragment::TextFragment) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextFragment::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextFragment_text_to_output(self as *const ::text_fragment::TextFragment, &mut object);
      }
      object
    }
  }
}

impl Drop for ::text_fragment::TextFragment {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextFragment::~QTextFragment()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextFragment_destructor(self as *mut ::text_fragment::TextFragment) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextFragment::glyph_runs](../struct.TextFragment.html#method.glyph_runs) method.
  pub trait TextFragmentGlyphRunsArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_fragment::TextFragment) -> ::list::ListGlyphRun;
  }
  impl<'largs> TextFragmentGlyphRunsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_fragment::TextFragment) -> ::list::ListGlyphRun {
      let from = self;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFragment_glyphRuns_to_output_from(original_self as *const ::text_fragment::TextFragment, from, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextFragmentGlyphRunsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::text_fragment::TextFragment) -> ::list::ListGlyphRun {
      let from = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFragment_glyphRuns_to_output_from_length(original_self as *const ::text_fragment::TextFragment, from, length, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextFragmentGlyphRunsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_fragment::TextFragment) -> ::list::ListGlyphRun {

      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFragment_glyphRuns_to_output_no_args(original_self as *const ::text_fragment::TextFragment, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextFragment::new](../struct.TextFragment.html#method.new) method.
  pub trait TextFragmentNewArgs {
    fn exec(self) -> ::text_fragment::TextFragment;
  }
  impl TextFragmentNewArgs for () {
    fn exec(self) -> ::text_fragment::TextFragment {

      {
        let mut object: ::text_fragment::TextFragment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFragment_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextFragmentNewArgs for &'a ::text_fragment::TextFragment {
    fn exec(self) -> ::text_fragment::TextFragment {
      let o = self;
      {
        let mut object: ::text_fragment::TextFragment =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextFragment_constructor_o(o as *const ::text_fragment::TextFragment, &mut object);
        }
        object
      }
    }
  }
}
