/// C++ type: <span style='color: green;'>```QTextCharFormat::FontPropertiesInheritanceBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FontPropertiesInheritanceBehavior {
  /// C++ enum variant: <span style='color: green;'>```FontPropertiesSpecifiedOnly = 0```</span>
  SpecifiedOnly = 0,
  /// C++ enum variant: <span style='color: green;'>```FontPropertiesAll = 1```</span>
  All = 1,
}

/// C++ type: <span style='color: green;'>```QTextCharFormat```</span>
#[repr(C)]
pub struct TextCharFormat([u8; ::type_sizes::QT_GUI_TEXT_CHAR_FORMAT_TEXT_CHAR_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextCharFormat {
  unsafe fn new_uninitialized() -> TextCharFormat {
    TextCharFormat(::std::mem::uninitialized())
  }
}

impl TextCharFormat {
  /// C++ method: <span style='color: green;'>```QString QTextCharFormat::anchorHref() const```</span>
  ///
  ///
  pub fn anchor_href(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_anchorHref_to_output(self as *const ::text_char_format::TextCharFormat,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextCharFormat::anchorName() const```</span>
  ///
  ///
  pub fn anchor_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_anchorName_to_output(self as *const ::text_char_format::TextCharFormat,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QTextCharFormat::anchorNames() const```</span>
  ///
  ///
  pub fn anchor_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_anchorNames_to_output(self as *const ::text_char_format::TextCharFormat,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFont QTextCharFormat::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_font_to_output(self as *const ::text_char_format::TextCharFormat,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextCharFormat::fontFamily() const```</span>
  ///
  ///
  pub fn font_family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_fontFamily_to_output(self as *const ::text_char_format::TextCharFormat,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontFixedPitch() const```</span>
  ///
  ///
  pub fn font_fixed_pitch(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontFixedPitch(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontItalic() const```</span>
  ///
  ///
  pub fn font_italic(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontItalic(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontKerning() const```</span>
  ///
  ///
  pub fn font_kerning(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontKerning(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextCharFormat::fontLetterSpacing() const```</span>
  ///
  ///
  pub fn font_letter_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontLetterSpacing(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontOverline() const```</span>
  ///
  ///
  pub fn font_overline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontOverline(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextCharFormat::fontPointSize() const```</span>
  ///
  ///
  pub fn font_point_size(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontPointSize(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCharFormat::fontStretch() const```</span>
  ///
  ///
  pub fn font_stretch(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontStretch(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontStrikeOut() const```</span>
  ///
  ///
  pub fn font_strike_out(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontStrikeOut(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::fontUnderline() const```</span>
  ///
  ///
  pub fn font_underline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontUnderline(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCharFormat::fontWeight() const```</span>
  ///
  ///
  pub fn font_weight(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontWeight(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```double QTextCharFormat::fontWordSpacing() const```</span>
  ///
  ///
  pub fn font_word_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_fontWordSpacing(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::isAnchor() const```</span>
  ///
  ///
  pub fn is_anchor(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_isAnchor(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextCharFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_isValid(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextCharFormat::QTextCharFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_char_format::TextCharFormat {
    {
      let mut object: ::text_char_format::TextCharFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setAnchor(bool anchor)```</span>
  ///
  ///
  pub fn set_anchor(&mut self, anchor: bool) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setAnchor(self as *mut ::text_char_format::TextCharFormat, anchor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setAnchorHref(const QString& value)```</span>
  ///
  ///
  pub fn set_anchor_href(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setAnchorHref(self as *mut ::text_char_format::TextCharFormat,
                                                    value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setAnchorName(const QString& name)```</span>
  ///
  ///
  pub fn set_anchor_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setAnchorName(self as *mut ::text_char_format::TextCharFormat,
                                                    name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setAnchorNames(const QStringList& names)```</span>
  ///
  ///
  pub fn set_anchor_names(&mut self, names: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setAnchorNames(self as *mut ::text_char_format::TextCharFormat,
                                                     names as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat::setFont```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_font(&mut self, &::font::Font) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFont(const QFont& font)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_font(&mut self, (&::font::Font, ::text_char_format::FontPropertiesInheritanceBehavior)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFont(const QFont& font, QTextCharFormat::FontPropertiesInheritanceBehavior behavior)```</span>
  ///
  ///
  pub fn set_font<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCharFormatSetFontArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontCapitalization(QFont::Capitalization capitalization)```</span>
  ///
  ///
  pub fn set_font_capitalization(&mut self, capitalization: &::font::Capitalization) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontCapitalization(self as *mut ::text_char_format::TextCharFormat,
                                                            capitalization as *const ::font::Capitalization)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontFamily(const QString& family)```</span>
  ///
  ///
  pub fn set_font_family(&mut self, family: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontFamily(self as *mut ::text_char_format::TextCharFormat,
                                                    family as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontFixedPitch(bool fixedPitch)```</span>
  ///
  ///
  pub fn set_font_fixed_pitch(&mut self, fixed_pitch: bool) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontFixedPitch(self as *mut ::text_char_format::TextCharFormat, fixed_pitch)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontHintingPreference(QFont::HintingPreference hintingPreference)```</span>
  ///
  ///
  pub fn set_font_hinting_preference(&mut self, hinting_preference: &::font::HintingPreference) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontHintingPreference(self as *mut ::text_char_format::TextCharFormat,
                                                               hinting_preference as *const ::font::HintingPreference)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontItalic(bool italic)```</span>
  ///
  ///
  pub fn set_font_italic(&mut self, italic: bool) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontItalic(self as *mut ::text_char_format::TextCharFormat, italic) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontKerning(bool enable)```</span>
  ///
  ///
  pub fn set_font_kerning(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontKerning(self as *mut ::text_char_format::TextCharFormat, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontLetterSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_font_letter_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontLetterSpacing(self as *mut ::text_char_format::TextCharFormat, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontLetterSpacingType(QFont::SpacingType letterSpacingType)```</span>
  ///
  ///
  pub fn set_font_letter_spacing_type(&mut self, letter_spacing_type: &::font::SpacingType) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontLetterSpacingType(self as *mut ::text_char_format::TextCharFormat,
                                                               letter_spacing_type as *const ::font::SpacingType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontOverline(bool overline)```</span>
  ///
  ///
  pub fn set_font_overline(&mut self, overline: bool) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontOverline(self as *mut ::text_char_format::TextCharFormat, overline)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontPointSize(double size)```</span>
  ///
  ///
  pub fn set_font_point_size(&mut self, size: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontPointSize(self as *mut ::text_char_format::TextCharFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontStretch(int factor)```</span>
  ///
  ///
  pub fn set_font_stretch(&mut self, factor: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontStretch(self as *mut ::text_char_format::TextCharFormat, factor) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontStrikeOut(bool strikeOut)```</span>
  ///
  ///
  pub fn set_font_strike_out(&mut self, strike_out: bool) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontStrikeOut(self as *mut ::text_char_format::TextCharFormat, strike_out)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat::setFontStyleHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_font_style_hint(&mut self, &::font::StyleHint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontStyleHint(QFont::StyleHint hint)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_font_style_hint(&mut self, (&::font::StyleHint, &::font::StyleStrategy)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontStyleHint(QFont::StyleHint hint, QFont::StyleStrategy strategy = ?)```</span>
  ///
  ///
  pub fn set_font_style_hint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextCharFormatSetFontStyleHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontStyleStrategy(QFont::StyleStrategy strategy)```</span>
  ///
  ///
  pub fn set_font_style_strategy(&mut self, strategy: &::font::StyleStrategy) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontStyleStrategy(self as *mut ::text_char_format::TextCharFormat,
                                                           strategy as *const ::font::StyleStrategy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontUnderline(bool underline)```</span>
  ///
  ///
  pub fn set_font_underline(&mut self, underline: bool) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontUnderline(self as *mut ::text_char_format::TextCharFormat, underline)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontWeight(int weight)```</span>
  ///
  ///
  pub fn set_font_weight(&mut self, weight: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontWeight(self as *mut ::text_char_format::TextCharFormat, weight) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setFontWordSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_font_word_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setFontWordSpacing(self as *mut ::text_char_format::TextCharFormat, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setTableCellColumnSpan(int tableCellColumnSpan)```</span>
  ///
  ///
  pub fn set_table_cell_column_span(&mut self, table_cell_column_span: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setTableCellColumnSpan(self as *mut ::text_char_format::TextCharFormat,
                                                             table_cell_column_span)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setTableCellRowSpan(int tableCellRowSpan)```</span>
  ///
  ///
  pub fn set_table_cell_row_span(&mut self, table_cell_row_span: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setTableCellRowSpan(self as *mut ::text_char_format::TextCharFormat,
                                                          table_cell_row_span)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setTextOutline(const QPen& pen)```</span>
  ///
  ///
  pub fn set_text_outline(&mut self, pen: &::pen::Pen) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setTextOutline(self as *mut ::text_char_format::TextCharFormat,
                                                     pen as *const ::pen::Pen)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setToolTip(const QString& tip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setToolTip(self as *mut ::text_char_format::TextCharFormat,
                                                 tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setUnderlineColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_underline_color(&mut self, color: &::color::Color) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setUnderlineColor(self as *mut ::text_char_format::TextCharFormat,
                                                        color as *const ::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setUnderlineStyle(QTextCharFormat::UnderlineStyle style)```</span>
  ///
  ///
  pub fn set_underline_style(&mut self, style: ::text_char_format::UnderlineStyle) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_setUnderlineStyle(self as *mut ::text_char_format::TextCharFormat, style) }
  }

  /// C++ method: <span style='color: green;'>```void QTextCharFormat::setVerticalAlignment(QTextCharFormat::VerticalAlignment alignment)```</span>
  ///
  ///
  pub fn set_vertical_alignment(&mut self, alignment: ::text_char_format::VerticalAlignment) {
    unsafe {
      ::ffi::qt_gui_c_QTextCharFormat_setVerticalAlignment(self as *mut ::text_char_format::TextCharFormat, alignment)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextCharFormat::tableCellColumnSpan() const```</span>
  ///
  ///
  pub fn table_cell_column_span(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_tableCellColumnSpan(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QTextCharFormat::tableCellRowSpan() const```</span>
  ///
  ///
  pub fn table_cell_row_span(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_tableCellRowSpan(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPen QTextCharFormat::textOutline() const```</span>
  ///
  ///
  pub fn text_outline(&self) -> ::pen::Pen {
    {
      let mut object: ::pen::Pen = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_textOutline_to_output(self as *const ::text_char_format::TextCharFormat,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextCharFormat::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_toolTip_to_output(self as *const ::text_char_format::TextCharFormat,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QTextCharFormat::underlineColor() const```</span>
  ///
  ///
  pub fn underline_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_underlineColor_to_output(self as *const ::text_char_format::TextCharFormat,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat::UnderlineStyle QTextCharFormat::underlineStyle() const```</span>
  ///
  ///
  pub fn underline_style(&self) -> ::text_char_format::UnderlineStyle {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_underlineStyle(self as *const ::text_char_format::TextCharFormat) }
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat::VerticalAlignment QTextCharFormat::verticalAlignment() const```</span>
  ///
  ///
  pub fn vertical_alignment(&self) -> ::text_char_format::VerticalAlignment {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_verticalAlignment(self as *const ::text_char_format::TextCharFormat) }
  }
}

impl Drop for ::text_char_format::TextCharFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextCharFormat::~QTextCharFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextCharFormat_destructor(self as *mut ::text_char_format::TextCharFormat) }
  }
}

/// C++ type: <span style='color: green;'>```QTextCharFormat::UnderlineStyle```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UnderlineStyle {
  /// C++ enum variant: <span style='color: green;'>```NoUnderline = 0```</span>
  NoUnderline = 0,
  /// C++ enum variant: <span style='color: green;'>```SingleUnderline = 1```</span>
  SingleUnderline = 1,
  /// C++ enum variant: <span style='color: green;'>```DashUnderline = 2```</span>
  DashUnderline = 2,
  /// C++ enum variant: <span style='color: green;'>```DotLine = 3```</span>
  DotLine = 3,
  /// C++ enum variant: <span style='color: green;'>```DashDotLine = 4```</span>
  DashDotLine = 4,
  /// C++ enum variant: <span style='color: green;'>```DashDotDotLine = 5```</span>
  DashDotDotLine = 5,
  /// C++ enum variant: <span style='color: green;'>```WaveUnderline = 6```</span>
  WaveUnderline = 6,
  /// C++ enum variant: <span style='color: green;'>```SpellCheckUnderline = 7```</span>
  SpellCheckUnderline = 7,
}

/// C++ type: <span style='color: green;'>```QTextCharFormat::VerticalAlignment```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum VerticalAlignment {
  /// C++ enum variant: <span style='color: green;'>```AlignNormal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```AlignSuperScript = 1```</span>
  SuperScript = 1,
  /// C++ enum variant: <span style='color: green;'>```AlignSubScript = 2```</span>
  SubScript = 2,
  /// C++ enum variant: <span style='color: green;'>```AlignMiddle = 3```</span>
  Middle = 3,
  /// C++ enum variant: <span style='color: green;'>```AlignTop = 4```</span>
  Top = 4,
  /// C++ enum variant: <span style='color: green;'>```AlignBottom = 5```</span>
  Bottom = 5,
  /// C++ enum variant: <span style='color: green;'>```AlignBaseline = 6```</span>
  Baseline = 6,
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_char_format::TextCharFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_char_format::TextCharFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_char_format::TextCharFormat as *mut ::text_char_format::TextCharFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_char_format::TextCharFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_char_format::TextCharFormat {
    let ffi_result =
      ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextCharFormat_ptr(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_char_format::TextCharFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextCharFormat_ptr(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_char_format::TextCharFormat {
  type Target = ::text_format::TextFormat;
  fn deref(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_char_format::TextCharFormat as *mut ::text_char_format::TextCharFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_char_format::TextCharFormat {
  fn deref_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_char_format::TextCharFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextCharFormat::set_font](../struct.TextCharFormat.html#method.set_font) method.
  pub trait TextCharFormatSetFontArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> ();
  }
  impl<'largs> TextCharFormatSetFontArgs<'largs> for &'largs ::font::Font {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> () {
      let font = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_setFont_font(original_self as *mut ::text_char_format::TextCharFormat,
                                                     font as *const ::font::Font)
      }
    }
  }
  impl<'largs> TextCharFormatSetFontArgs<'largs>
    for (&'largs ::font::Font, ::text_char_format::FontPropertiesInheritanceBehavior) {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> () {
      let font = self.0;
      let behavior = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_setFont_font_behavior(original_self as *mut ::text_char_format::TextCharFormat, font as *const ::font::Font, behavior)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextCharFormat::set_font_style_hint](../struct.TextCharFormat.html#method.set_font_style_hint) method.
  pub trait TextCharFormatSetFontStyleHintArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> ();
  }
  impl<'largs> TextCharFormatSetFontStyleHintArgs<'largs> for &'largs ::font::StyleHint {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> () {
      let hint = self;
      unsafe {
        ::ffi::qt_gui_c_QTextCharFormat_setFontStyleHint_hint(original_self as *mut ::text_char_format::TextCharFormat, hint as *const ::font::StyleHint)
      }
    }
  }
  impl<'largs> TextCharFormatSetFontStyleHintArgs<'largs> for (&'largs ::font::StyleHint, &'largs ::font::StyleStrategy) {
    fn exec(self, original_self: &'largs mut ::text_char_format::TextCharFormat) -> () {
      let hint = self.0;
      let strategy = self.1;
      unsafe { ::ffi::qt_gui_c_QTextCharFormat_setFontStyleHint_hint_strategy(original_self as *mut ::text_char_format::TextCharFormat, hint as *const ::font::StyleHint, strategy as *const ::font::StyleStrategy) }
    }
  }
}
