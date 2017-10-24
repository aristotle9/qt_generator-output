/// C++ type: <span style='color: green;'>```QTextBlockFormat::LineHeightTypes```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LineHeightTypes {
  /// C++ enum variant: <span style='color: green;'>```SingleHeight = 0```</span>
  Single = 0,
  /// C++ enum variant: <span style='color: green;'>```ProportionalHeight = 1```</span>
  Proportional = 1,
  /// C++ enum variant: <span style='color: green;'>```FixedHeight = 2```</span>
  Fixed = 2,
  /// C++ enum variant: <span style='color: green;'>```MinimumHeight = 3```</span>
  Minimum = 3,
  /// C++ enum variant: <span style='color: green;'>```LineDistanceHeight = 4```</span>
  LineDistance = 4,
}

/// C++ type: <span style='color: green;'>```QTextBlockFormat```</span>
#[repr(C)]
pub struct TextBlockFormat([u8; ::type_sizes::QT_GUI_TEXT_BLOCK_FORMAT_TEXT_BLOCK_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextBlockFormat {
  unsafe fn new_uninitialized() -> TextBlockFormat {
    TextBlockFormat(::std::mem::uninitialized())
  }
}

impl TextBlockFormat {
  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::bottomMargin() const```</span>
  ///
  ///
  pub fn bottom_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_bottomMargin(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBlockFormat::indent() const```</span>
  ///
  ///
  pub fn indent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_indent(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlockFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_isValid(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::leftMargin() const```</span>
  ///
  ///
  pub fn left_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_leftMargin(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextBlockFormat::lineHeight```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn line_height(&self, ()) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::lineHeight() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn line_height(&self, (::libc::c_double, ::libc::c_double)) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::lineHeight(double scriptLineHeight, double scaling) const```</span>
  ///
  ///
  pub fn line_height<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::TextBlockFormatLineHeightArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QTextBlockFormat::lineHeightType() const```</span>
  ///
  ///
  pub fn line_height_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_lineHeightType(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextBlockFormat::QTextBlockFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_block_format::TextBlockFormat {
    {
      let mut object: ::text_block_format::TextBlockFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlockFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBlockFormat::nonBreakableLines() const```</span>
  ///
  ///
  pub fn non_breakable_lines(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_nonBreakableLines(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTextFormat::PageBreakFlag> QTextBlockFormat::pageBreakPolicy() const```</span>
  ///
  ///
  pub fn page_break_policy(&self) -> ::qt_core::flags::Flags<::text_format::PageBreakFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextBlockFormat_pageBreakPolicy(self as *const ::text_block_format::TextBlockFormat) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::rightMargin() const```</span>
  ///
  ///
  pub fn right_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_rightMargin(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setBottomMargin(double margin)```</span>
  ///
  ///
  pub fn set_bottom_margin(&mut self, margin: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setBottomMargin(self as *mut ::text_block_format::TextBlockFormat, margin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setIndent(int indent)```</span>
  ///
  ///
  pub fn set_indent(&mut self, indent: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_setIndent(self as *mut ::text_block_format::TextBlockFormat, indent) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setLeftMargin(double margin)```</span>
  ///
  ///
  pub fn set_left_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_setLeftMargin(self as *mut ::text_block_format::TextBlockFormat, margin) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setLineHeight(double height, int heightType)```</span>
  ///
  ///
  pub fn set_line_height(&mut self, height: ::libc::c_double, height_type: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setLineHeight(self as *mut ::text_block_format::TextBlockFormat,
                                                     height,
                                                     height_type)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setNonBreakableLines(bool b)```</span>
  ///
  ///
  pub fn set_non_breakable_lines(&mut self, b: bool) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setNonBreakableLines(self as *mut ::text_block_format::TextBlockFormat, b)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setPageBreakPolicy(QFlags<QTextFormat::PageBreakFlag> flags)```</span>
  ///
  ///
  pub fn set_page_break_policy(&mut self, flags: ::qt_core::flags::Flags<::text_format::PageBreakFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setPageBreakPolicy(self as *mut ::text_block_format::TextBlockFormat,
                                                          flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setRightMargin(double margin)```</span>
  ///
  ///
  pub fn set_right_margin(&mut self, margin: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setRightMargin(self as *mut ::text_block_format::TextBlockFormat, margin)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setTabPositions(const QList<QTextOption::Tab>& tabs)```</span>
  ///
  ///
  pub fn set_tab_positions(&mut self, tabs: &::list::ListTextOptionTab) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setTabPositions(self as *mut ::text_block_format::TextBlockFormat,
                                                       tabs as *const ::list::ListTextOptionTab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setTextIndent(double aindent)```</span>
  ///
  ///
  pub fn set_text_indent(&mut self, aindent: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextBlockFormat_setTextIndent(self as *mut ::text_block_format::TextBlockFormat, aindent)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBlockFormat::setTopMargin(double margin)```</span>
  ///
  ///
  pub fn set_top_margin(&mut self, margin: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_setTopMargin(self as *mut ::text_block_format::TextBlockFormat, margin) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab> QTextBlockFormat::tabPositions() const```</span>
  ///
  ///
  pub fn tab_positions(&self) -> ::list::ListTextOptionTab {
    {
      let mut object: ::list::ListTextOptionTab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextBlockFormat_tabPositions_to_output(self as *const ::text_block_format::TextBlockFormat,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::textIndent() const```</span>
  ///
  ///
  pub fn text_indent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_textIndent(self as *const ::text_block_format::TextBlockFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextBlockFormat::topMargin() const```</span>
  ///
  ///
  pub fn top_margin(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_topMargin(self as *const ::text_block_format::TextBlockFormat) }
  }
}

impl Drop for ::text_block_format::TextBlockFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextBlockFormat::~QTextBlockFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextBlockFormat_destructor(self as *mut ::text_block_format::TextBlockFormat) }
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_block_format::TextBlockFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_block_format::TextBlockFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_block_format::TextBlockFormat as *mut ::text_block_format::TextBlockFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_block_format::TextBlockFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_block_format::TextBlockFormat {
    let ffi_result =
      ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextBlockFormat_ptr(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_block_format::TextBlockFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextBlockFormat_ptr(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_block_format::TextBlockFormat {
  type Target = ::text_format::TextFormat;
  fn deref(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_block_format::TextBlockFormat as *mut ::text_block_format::TextBlockFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_block_format::TextBlockFormat {
  fn deref_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextBlockFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_block_format::TextBlockFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextBlockFormat::line_height](../struct.TextBlockFormat.html#method.line_height) method.
  pub trait TextBlockFormatLineHeightArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_block_format::TextBlockFormat) -> ::libc::c_double;
  }
  impl<'largs> TextBlockFormatLineHeightArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_block_format::TextBlockFormat) -> ::libc::c_double {

      unsafe { ::ffi::qt_gui_c_QTextBlockFormat_lineHeight_no_args(original_self as *const ::text_block_format::TextBlockFormat) }
    }
  }
  impl<'largs> TextBlockFormatLineHeightArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::text_block_format::TextBlockFormat) -> ::libc::c_double {
      let script_line_height = self.0;
      let scaling = self.1;
      unsafe { ::ffi::qt_gui_c_QTextBlockFormat_lineHeight_scriptLineHeight_scaling(original_self as *const ::text_block_format::TextBlockFormat, script_line_height, scaling) }
    }
  }
}
