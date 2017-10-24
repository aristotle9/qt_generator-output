/// C++ type: <span style='color: green;'>```QChar::Category```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Category {
  /// C++ enum variant: <span style='color: green;'>```Mark_NonSpacing = 0```</span>
  MarkNonSpacing = 0,
  /// C++ enum variant: <span style='color: green;'>```Mark_SpacingCombining = 1```</span>
  MarkSpacingCombining = 1,
  /// C++ enum variant: <span style='color: green;'>```Mark_Enclosing = 2```</span>
  MarkEnclosing = 2,
  /// C++ enum variant: <span style='color: green;'>```Number_DecimalDigit = 3```</span>
  NumberDecimalDigit = 3,
  /// C++ enum variant: <span style='color: green;'>```Number_Letter = 4```</span>
  NumberLetter = 4,
  /// C++ enum variant: <span style='color: green;'>```Number_Other = 5```</span>
  NumberOther = 5,
  /// C++ enum variant: <span style='color: green;'>```Separator_Space = 6```</span>
  SeparatorSpace = 6,
  /// C++ enum variant: <span style='color: green;'>```Separator_Line = 7```</span>
  SeparatorLine = 7,
  /// C++ enum variant: <span style='color: green;'>```Separator_Paragraph = 8```</span>
  SeparatorParagraph = 8,
  /// C++ enum variant: <span style='color: green;'>```Other_Control = 9```</span>
  OtherControl = 9,
  /// C++ enum variant: <span style='color: green;'>```Other_Format = 10```</span>
  OtherFormat = 10,
  /// C++ enum variant: <span style='color: green;'>```Other_Surrogate = 11```</span>
  OtherSurrogate = 11,
  /// C++ enum variant: <span style='color: green;'>```Other_PrivateUse = 12```</span>
  OtherPrivateUse = 12,
  /// C++ enum variant: <span style='color: green;'>```Other_NotAssigned = 13```</span>
  OtherNotAssigned = 13,
  /// C++ enum variant: <span style='color: green;'>```Letter_Uppercase = 14```</span>
  LetterUppercase = 14,
  /// C++ enum variant: <span style='color: green;'>```Letter_Lowercase = 15```</span>
  LetterLowercase = 15,
  /// C++ enum variant: <span style='color: green;'>```Letter_Titlecase = 16```</span>
  LetterTitlecase = 16,
  /// C++ enum variant: <span style='color: green;'>```Letter_Modifier = 17```</span>
  LetterModifier = 17,
  /// C++ enum variant: <span style='color: green;'>```Letter_Other = 18```</span>
  LetterOther = 18,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_Connector = 19```</span>
  PunctuationConnector = 19,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_Dash = 20```</span>
  PunctuationDash = 20,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_Open = 21```</span>
  PunctuationOpen = 21,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_Close = 22```</span>
  PunctuationClose = 22,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_InitialQuote = 23```</span>
  PunctuationInitialQuote = 23,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_FinalQuote = 24```</span>
  PunctuationFinalQuote = 24,
  /// C++ enum variant: <span style='color: green;'>```Punctuation_Other = 25```</span>
  PunctuationOther = 25,
  /// C++ enum variant: <span style='color: green;'>```Symbol_Math = 26```</span>
  SymbolMath = 26,
  /// C++ enum variant: <span style='color: green;'>```Symbol_Currency = 27```</span>
  SymbolCurrency = 27,
  /// C++ enum variant: <span style='color: green;'>```Symbol_Modifier = 28```</span>
  SymbolModifier = 28,
  /// C++ enum variant: <span style='color: green;'>```Symbol_Other = 29```</span>
  SymbolOther = 29,
}

/// C++ type: <span style='color: green;'>```QChar```</span>
#[repr(C)]
pub struct Char([u8; ::type_sizes::QT_CORE_CHAR_CHAR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Char {
  unsafe fn new_uninitialized() -> Char {
    Char(::std::mem::uninitialized())
  }
}

impl Char {
  /// C++ method: <span style='color: green;'>```QChar::Category QChar::category() const```</span>
  ///
  ///
  pub fn category(&self) -> ::char::Category {
    unsafe { ::ffi::qt_core_c_QChar_category_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::Category QChar::category(unsigned int ucs4)```</span>
  ///
  ///
  pub fn category_static(ucs4: ::libc::c_uint) -> ::char::Category {
    unsafe { ::ffi::qt_core_c_QChar_category_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QChar::cell() const```</span>
  ///
  ///
  pub fn cell(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QChar_cell(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QChar::combiningClass() const```</span>
  ///
  ///
  pub fn combining_class(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QChar_combiningClass_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static unsigned char QChar::combiningClass(unsigned int ucs4)```</span>
  ///
  ///
  pub fn combining_class_static(ucs4: ::libc::c_uint) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QChar_combiningClass_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::UnicodeVersion QChar::currentUnicodeVersion()```</span>
  ///
  ///
  pub fn current_unicode_version() -> ::char::UnicodeVersion {
    unsafe { ::ffi::qt_core_c_QChar_currentUnicodeVersion() }
  }

  /// C++ method: <span style='color: green;'>```QString QChar::decomposition() const```</span>
  ///
  ///
  pub fn decomposition(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_decomposition_to_output_no_args(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QChar::decomposition(unsigned int ucs4)```</span>
  ///
  ///
  pub fn decomposition_static(ucs4: ::libc::c_uint) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_decomposition_to_output_ucs4(ucs4, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QChar::Decomposition QChar::decompositionTag() const```</span>
  ///
  ///
  pub fn decomposition_tag(&self) -> ::char::Decomposition {
    unsafe { ::ffi::qt_core_c_QChar_decompositionTag_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::Decomposition QChar::decompositionTag(unsigned int ucs4)```</span>
  ///
  ///
  pub fn decomposition_tag_static(ucs4: ::libc::c_uint) -> ::char::Decomposition {
    unsafe { ::ffi::qt_core_c_QChar_decompositionTag_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```int QChar::digitValue() const```</span>
  ///
  ///
  pub fn digit_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QChar_digitValue_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static int QChar::digitValue(unsigned int ucs4)```</span>
  ///
  ///
  pub fn digit_value_static(ucs4: ::libc::c_uint) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QChar_digitValue_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar::Direction QChar::direction() const```</span>
  ///
  ///
  pub fn direction(&self) -> ::char::Direction {
    unsafe { ::ffi::qt_core_c_QChar_direction_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::Direction QChar::direction(unsigned int ucs4)```</span>
  ///
  ///
  pub fn direction_static(ucs4: ::libc::c_uint) -> ::char::Direction {
    unsafe { ::ffi::qt_core_c_QChar_direction_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```static QChar QChar::fromLatin1(char c)```</span>
  ///
  ///
  pub fn from_latin1(c: ::libc::c_char) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_fromLatin1_to_output(c, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::hasMirrored() const```</span>
  ///
  ///
  pub fn has_mirrored(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_hasMirrored_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::hasMirrored(unsigned int ucs4)```</span>
  ///
  ///
  pub fn has_mirrored_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_hasMirrored_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```static unsigned short QChar::highSurrogate(unsigned int ucs4)```</span>
  ///
  ///
  pub fn high_surrogate(ucs4: ::libc::c_uint) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QChar_highSurrogate(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isDigit() const```</span>
  ///
  ///
  pub fn is_digit(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isDigit_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isDigit(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_digit_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isDigit_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isHighSurrogate() const```</span>
  ///
  ///
  pub fn is_high_surrogate(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isHighSurrogate_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isHighSurrogate(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_high_surrogate_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isHighSurrogate_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isLetter() const```</span>
  ///
  ///
  pub fn is_letter(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLetter_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isLetterOrNumber() const```</span>
  ///
  ///
  pub fn is_letter_or_number(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLetterOrNumber_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isLetterOrNumber(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_letter_or_number_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLetterOrNumber_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isLetter(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_letter_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLetter_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isLowSurrogate() const```</span>
  ///
  ///
  pub fn is_low_surrogate(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLowSurrogate_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isLowSurrogate(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_low_surrogate_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLowSurrogate_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isLower() const```</span>
  ///
  ///
  pub fn is_lower(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLower_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isLower(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_lower_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isLower_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isMark() const```</span>
  ///
  ///
  pub fn is_mark(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isMark_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isMark(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_mark_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isMark_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isNonCharacter() const```</span>
  ///
  ///
  pub fn is_non_character(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isNonCharacter_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isNonCharacter(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_non_character_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isNonCharacter_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isNull(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isNumber() const```</span>
  ///
  ///
  pub fn is_number(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isNumber_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isNumber(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_number_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isNumber_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isPrint() const```</span>
  ///
  ///
  pub fn is_print(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isPrint_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isPrint(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_print_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isPrint_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isPunct() const```</span>
  ///
  ///
  pub fn is_punct(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isPunct_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isPunct(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_punct_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isPunct_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isSpace() const```</span>
  ///
  ///
  pub fn is_space(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSpace_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isSpace(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_space_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSpace_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isSurrogate() const```</span>
  ///
  ///
  pub fn is_surrogate(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSurrogate_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isSurrogate(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_surrogate_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSurrogate_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isSymbol() const```</span>
  ///
  ///
  pub fn is_symbol(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSymbol_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isSymbol(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_symbol_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isSymbol_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isTitleCase() const```</span>
  ///
  ///
  pub fn is_title_case(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isTitleCase_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isTitleCase(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_title_case_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isTitleCase_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```bool QChar::isUpper() const```</span>
  ///
  ///
  pub fn is_upper(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isUpper_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::isUpper(unsigned int ucs4)```</span>
  ///
  ///
  pub fn is_upper_static(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_isUpper_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar::Joining QChar::joining() const```</span>
  ///
  ///
  pub fn joining(&self) -> ::char::Joining {
    unsafe { ::ffi::qt_core_c_QChar_joining_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::Joining QChar::joining(unsigned int ucs4)```</span>
  ///
  ///
  pub fn joining_static(ucs4: ::libc::c_uint) -> ::char::Joining {
    unsafe { ::ffi::qt_core_c_QChar_joining_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar::JoiningType QChar::joiningType() const```</span>
  ///
  ///
  pub fn joining_type(&self) -> ::char::JoiningType {
    unsafe { ::ffi::qt_core_c_QChar_joiningType_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::JoiningType QChar::joiningType(unsigned int ucs4)```</span>
  ///
  ///
  pub fn joining_type_static(ucs4: ::libc::c_uint) -> ::char::JoiningType {
    unsafe { ::ffi::qt_core_c_QChar_joiningType_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```static unsigned short QChar::lowSurrogate(unsigned int ucs4)```</span>
  ///
  ///
  pub fn low_surrogate(ucs4: ::libc::c_uint) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QChar_lowSurrogate(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar QChar::mirroredChar() const```</span>
  ///
  ///
  pub fn mirrored_char(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_mirroredChar_to_output(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QChar::mirroredChar(unsigned int ucs4)```</span>
  ///
  ///
  pub fn mirrored_char_static(ucs4: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QChar_mirroredChar(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar::QChar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new0(()) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new0(::char::SpecialCharacter) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(QChar::SpecialCharacter s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new0(&::latin1_char::Latin1Char) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(QLatin1Char ch)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new0(::libc::c_char) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new0((::libc::c_uchar, ::libc::c_uchar)) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(unsigned char c, unsigned char r)```</span>
  ///
  ///
  pub fn new0<Args>(args: Args) -> ::char::Char
    where Args: overloading::CharNew0Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QChar::QChar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new1(::libc::c_int) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(int rc)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new1(::libc::c_uchar) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(unsigned char c)```</span>
  ///
  ///
  pub fn new1<Args>(args: Args) -> ::char::Char
    where Args: overloading::CharNew1Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QChar::QChar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new2(::libc::c_short) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(short rc)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new2(::libc::c_uint) -> ::char::Char```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(unsigned int rc)```</span>
  ///
  ///
  pub fn new2<Args>(args: Args) -> ::char::Char
    where Args: overloading::CharNew2Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QChar::QChar(unsigned short rc)```</span>
  ///
  ///
  pub fn new3(rc: ::libc::c_ushort) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_constructor_unsigned_short(rc, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QChar::requiresSurrogates(unsigned int ucs4)```</span>
  ///
  ///
  pub fn requires_surrogates(ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_core_c_QChar_requiresSurrogates(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QChar::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QChar_row(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```QChar::Script QChar::script() const```</span>
  ///
  ///
  pub fn script(&self) -> ::char::Script {
    unsafe { ::ffi::qt_core_c_QChar_script_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::Script QChar::script(unsigned int ucs4)```</span>
  ///
  ///
  pub fn script_static(ucs4: ::libc::c_uint) -> ::char::Script {
    unsafe { ::ffi::qt_core_c_QChar_script_ucs4(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```void QChar::setCell(unsigned char acell)```</span>
  ///
  ///
  pub fn set_cell(&mut self, acell: ::libc::c_uchar) {
    unsafe { ::ffi::qt_core_c_QChar_setCell(self as *mut ::char::Char, acell) }
  }

  /// C++ method: <span style='color: green;'>```void QChar::setRow(unsigned char arow)```</span>
  ///
  ///
  pub fn set_row(&mut self, arow: ::libc::c_uchar) {
    unsafe { ::ffi::qt_core_c_QChar_setRow(self as *mut ::char::Char, arow) }
  }

  /// C++ method: <span style='color: green;'>```QChar::surrogateToUcs4```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn surrogate_to_ucs4((&::char::Char, &::char::Char)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QChar::surrogateToUcs4(QChar high, QChar low)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn surrogate_to_ucs4((::libc::c_ushort, ::libc::c_ushort)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```static unsigned int QChar::surrogateToUcs4(unsigned short high, unsigned short low)```</span>
  ///
  ///
  pub fn surrogate_to_ucs4<Args>(args: Args) -> ::libc::c_uint
    where Args: overloading::CharSurrogateToUcs4Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QChar QChar::toCaseFolded() const```</span>
  ///
  ///
  pub fn to_case_folded(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_toCaseFolded_to_output(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QChar::toCaseFolded(unsigned int ucs4)```</span>
  ///
  ///
  pub fn to_case_folded_static(ucs4: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QChar_toCaseFolded(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```char QChar::toLatin1() const```</span>
  ///
  ///
  pub fn to_latin1(&self) -> ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QChar_toLatin1(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```QChar QChar::toLower() const```</span>
  ///
  ///
  pub fn to_lower(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_toLower_to_output(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QChar::toLower(unsigned int ucs4)```</span>
  ///
  ///
  pub fn to_lower_static(ucs4: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QChar_toLower(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar QChar::toTitleCase() const```</span>
  ///
  ///
  pub fn to_title_case(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_toTitleCase_to_output(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QChar::toTitleCase(unsigned int ucs4)```</span>
  ///
  ///
  pub fn to_title_case_static(ucs4: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QChar_toTitleCase(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```QChar QChar::toUpper() const```</span>
  ///
  ///
  pub fn to_upper(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QChar_toUpper_to_output(self as *const ::char::Char, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static unsigned int QChar::toUpper(unsigned int ucs4)```</span>
  ///
  ///
  pub fn to_upper_static(ucs4: ::libc::c_uint) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QChar_toUpper(ucs4) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QChar::unicode() const```</span>
  ///
  ///
  pub fn unicode(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QChar_unicode_const(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short& QChar::unicode()```</span>
  ///
  ///
  pub fn unicode_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_ushort {
    let ffi_result = unsafe { ::ffi::qt_core_c_QChar_unicode(self as *mut ::char::Char) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QChar::UnicodeVersion QChar::unicodeVersion() const```</span>
  ///
  ///
  pub fn unicode_version(&self) -> ::char::UnicodeVersion {
    unsafe { ::ffi::qt_core_c_QChar_unicodeVersion_no_args(self as *const ::char::Char) }
  }

  /// C++ method: <span style='color: green;'>```static QChar::UnicodeVersion QChar::unicodeVersion(unsigned int ucs4)```</span>
  ///
  ///
  pub fn unicode_version_static(ucs4: ::libc::c_uint) -> ::char::UnicodeVersion {
    unsafe { ::ffi::qt_core_c_QChar_unicodeVersion_ucs4(ucs4) }
  }
}

impl Drop for ::char::Char {
  /// C++ method: <span style='color: green;'>```[destructor] void QChar::~QChar()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QChar_destructor(self as *mut ::char::Char) }
  }
}

/// C++ type: <span style='color: green;'>```QChar::CombiningClass```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CombiningClass {
  /// C++ enum variant: <span style='color: green;'>```Combining_BelowLeftAttached = 200```</span>
  BelowLeftAttached = 200,
  /// C++ enum variant: <span style='color: green;'>```Combining_BelowAttached = 202```</span>
  BelowAttached = 202,
  /// C++ enum variant: <span style='color: green;'>```Combining_BelowRightAttached = 204```</span>
  BelowRightAttached = 204,
  /// C++ enum variant: <span style='color: green;'>```Combining_LeftAttached = 208```</span>
  LeftAttached = 208,
  /// C++ enum variant: <span style='color: green;'>```Combining_RightAttached = 210```</span>
  RightAttached = 210,
  /// C++ enum variant: <span style='color: green;'>```Combining_AboveLeftAttached = 212```</span>
  AboveLeftAttached = 212,
  /// C++ enum variant: <span style='color: green;'>```Combining_AboveAttached = 214```</span>
  AboveAttached = 214,
  /// C++ enum variant: <span style='color: green;'>```Combining_AboveRightAttached = 216```</span>
  AboveRightAttached = 216,
  /// C++ enum variant: <span style='color: green;'>```Combining_BelowLeft = 218```</span>
  BelowLeft = 218,
  /// C++ enum variant: <span style='color: green;'>```Combining_Below = 220```</span>
  Below = 220,
  /// C++ enum variant: <span style='color: green;'>```Combining_BelowRight = 222```</span>
  BelowRight = 222,
  /// C++ enum variant: <span style='color: green;'>```Combining_Left = 224```</span>
  Left = 224,
  /// C++ enum variant: <span style='color: green;'>```Combining_Right = 226```</span>
  Right = 226,
  /// C++ enum variant: <span style='color: green;'>```Combining_AboveLeft = 228```</span>
  AboveLeft = 228,
  /// C++ enum variant: <span style='color: green;'>```Combining_Above = 230```</span>
  Above = 230,
  /// C++ enum variant: <span style='color: green;'>```Combining_AboveRight = 232```</span>
  AboveRight = 232,
  /// C++ enum variant: <span style='color: green;'>```Combining_DoubleBelow = 233```</span>
  DoubleBelow = 233,
  /// C++ enum variant: <span style='color: green;'>```Combining_DoubleAbove = 234```</span>
  DoubleAbove = 234,
  /// C++ enum variant: <span style='color: green;'>```Combining_IotaSubscript = 240```</span>
  IotaSubscript = 240,
}

/// C++ type: <span style='color: green;'>```QChar::Decomposition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Decomposition {
  /// C++ enum variant: <span style='color: green;'>```NoDecomposition = 0```</span>
  NoDecomposition = 0,
  /// C++ enum variant: <span style='color: green;'>```Canonical = 1```</span>
  Canonical = 1,
  /// C++ enum variant: <span style='color: green;'>```Font = 2```</span>
  Font = 2,
  /// C++ enum variant: <span style='color: green;'>```NoBreak = 3```</span>
  NoBreak = 3,
  /// C++ enum variant: <span style='color: green;'>```Initial = 4```</span>
  Initial = 4,
  /// C++ enum variant: <span style='color: green;'>```Medial = 5```</span>
  Medial = 5,
  /// C++ enum variant: <span style='color: green;'>```Final = 6```</span>
  Final = 6,
  /// C++ enum variant: <span style='color: green;'>```Isolated = 7```</span>
  Isolated = 7,
  /// C++ enum variant: <span style='color: green;'>```Circle = 8```</span>
  Circle = 8,
  /// C++ enum variant: <span style='color: green;'>```Super = 9```</span>
  Super = 9,
  /// C++ enum variant: <span style='color: green;'>```Sub = 10```</span>
  Sub = 10,
  /// C++ enum variant: <span style='color: green;'>```Vertical = 11```</span>
  Vertical = 11,
  /// C++ enum variant: <span style='color: green;'>```Wide = 12```</span>
  Wide = 12,
  /// C++ enum variant: <span style='color: green;'>```Narrow = 13```</span>
  Narrow = 13,
  /// C++ enum variant: <span style='color: green;'>```Small = 14```</span>
  Small = 14,
  /// C++ enum variant: <span style='color: green;'>```Square = 15```</span>
  Square = 15,
  /// C++ enum variant: <span style='color: green;'>```Compat = 16```</span>
  Compat = 16,
  /// C++ enum variant: <span style='color: green;'>```Fraction = 17```</span>
  Fraction = 17,
}

/// C++ type: <span style='color: green;'>```QChar::Direction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Direction {
  /// C++ enum variant: <span style='color: green;'>```DirL = 0```</span>
  L = 0,
  /// C++ enum variant: <span style='color: green;'>```DirR = 1```</span>
  R = 1,
  /// C++ enum variant: <span style='color: green;'>```DirEN = 2```</span>
  EN = 2,
  /// C++ enum variant: <span style='color: green;'>```DirES = 3```</span>
  ES = 3,
  /// C++ enum variant: <span style='color: green;'>```DirET = 4```</span>
  ET = 4,
  /// C++ enum variant: <span style='color: green;'>```DirAN = 5```</span>
  AN = 5,
  /// C++ enum variant: <span style='color: green;'>```DirCS = 6```</span>
  CS = 6,
  /// C++ enum variant: <span style='color: green;'>```DirB = 7```</span>
  B = 7,
  /// C++ enum variant: <span style='color: green;'>```DirS = 8```</span>
  S = 8,
  /// C++ enum variant: <span style='color: green;'>```DirWS = 9```</span>
  WS = 9,
  /// C++ enum variant: <span style='color: green;'>```DirON = 10```</span>
  ON = 10,
  /// C++ enum variant: <span style='color: green;'>```DirLRE = 11```</span>
  LRE = 11,
  /// C++ enum variant: <span style='color: green;'>```DirLRO = 12```</span>
  LRO = 12,
  /// C++ enum variant: <span style='color: green;'>```DirAL = 13```</span>
  AL = 13,
  /// C++ enum variant: <span style='color: green;'>```DirRLE = 14```</span>
  RLE = 14,
  /// C++ enum variant: <span style='color: green;'>```DirRLO = 15```</span>
  RLO = 15,
  /// C++ enum variant: <span style='color: green;'>```DirPDF = 16```</span>
  PDF = 16,
  /// C++ enum variant: <span style='color: green;'>```DirNSM = 17```</span>
  NSM = 17,
  /// C++ enum variant: <span style='color: green;'>```DirBN = 18```</span>
  BN = 18,
  /// C++ enum variant: <span style='color: green;'>```DirLRI = 19```</span>
  LRI = 19,
  /// C++ enum variant: <span style='color: green;'>```DirRLI = 20```</span>
  RLI = 20,
  /// C++ enum variant: <span style='color: green;'>```DirFSI = 21```</span>
  FSI = 21,
  /// C++ enum variant: <span style='color: green;'>```DirPDI = 22```</span>
  PDI = 22,
}

/// C++ type: <span style='color: green;'>```QChar::Joining```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Joining {
  /// C++ enum variant: <span style='color: green;'>```OtherJoining = 0```</span>
  OtherJoining = 0,
  /// C++ enum variant: <span style='color: green;'>```Dual = 1```</span>
  Dual = 1,
  /// C++ enum variant: <span style='color: green;'>```Right = 2```</span>
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```Center = 3```</span>
  Center = 3,
}

/// C++ type: <span style='color: green;'>```QChar::JoiningType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum JoiningType {
  /// C++ enum variant: <span style='color: green;'>```Joining_None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Joining_Causing = 1```</span>
  Causing = 1,
  /// C++ enum variant: <span style='color: green;'>```Joining_Dual = 2```</span>
  Dual = 2,
  /// C++ enum variant: <span style='color: green;'>```Joining_Right = 3```</span>
  Right = 3,
  /// C++ enum variant: <span style='color: green;'>```Joining_Left = 4```</span>
  Left = 4,
  /// C++ enum variant: <span style='color: green;'>```Joining_Transparent = 5```</span>
  Transparent = 5,
}

/// C++ type: <span style='color: green;'>```QChar::Script```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Script {
  /// C++ enum variant: <span style='color: green;'>```Script_Unknown = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```Script_Inherited = 1```</span>
  Inherited = 1,
  /// C++ enum variant: <span style='color: green;'>```Script_Common = 2```</span>
  Common = 2,
  /// C++ enum variant: <span style='color: green;'>```Script_Latin = 3```</span>
  Latin = 3,
  /// C++ enum variant: <span style='color: green;'>```Script_Greek = 4```</span>
  Greek = 4,
  /// C++ enum variant: <span style='color: green;'>```Script_Cyrillic = 5```</span>
  Cyrillic = 5,
  /// C++ enum variant: <span style='color: green;'>```Script_Armenian = 6```</span>
  Armenian = 6,
  /// C++ enum variant: <span style='color: green;'>```Script_Hebrew = 7```</span>
  Hebrew = 7,
  /// C++ enum variant: <span style='color: green;'>```Script_Arabic = 8```</span>
  Arabic = 8,
  /// C++ enum variant: <span style='color: green;'>```Script_Syriac = 9```</span>
  Syriac = 9,
  /// C++ enum variant: <span style='color: green;'>```Script_Thaana = 10```</span>
  Thaana = 10,
  /// C++ enum variant: <span style='color: green;'>```Script_Devanagari = 11```</span>
  Devanagari = 11,
  /// C++ enum variant: <span style='color: green;'>```Script_Bengali = 12```</span>
  Bengali = 12,
  /// C++ enum variant: <span style='color: green;'>```Script_Gurmukhi = 13```</span>
  Gurmukhi = 13,
  /// C++ enum variant: <span style='color: green;'>```Script_Gujarati = 14```</span>
  Gujarati = 14,
  /// C++ enum variant: <span style='color: green;'>```Script_Oriya = 15```</span>
  Oriya = 15,
  /// C++ enum variant: <span style='color: green;'>```Script_Tamil = 16```</span>
  Tamil = 16,
  /// C++ enum variant: <span style='color: green;'>```Script_Telugu = 17```</span>
  Telugu = 17,
  /// C++ enum variant: <span style='color: green;'>```Script_Kannada = 18```</span>
  Kannada = 18,
  /// C++ enum variant: <span style='color: green;'>```Script_Malayalam = 19```</span>
  Malayalam = 19,
  /// C++ enum variant: <span style='color: green;'>```Script_Sinhala = 20```</span>
  Sinhala = 20,
  /// C++ enum variant: <span style='color: green;'>```Script_Thai = 21```</span>
  Thai = 21,
  /// C++ enum variant: <span style='color: green;'>```Script_Lao = 22```</span>
  Lao = 22,
  /// C++ enum variant: <span style='color: green;'>```Script_Tibetan = 23```</span>
  Tibetan = 23,
  /// C++ enum variant: <span style='color: green;'>```Script_Myanmar = 24```</span>
  Myanmar = 24,
  /// C++ enum variant: <span style='color: green;'>```Script_Georgian = 25```</span>
  Georgian = 25,
  /// C++ enum variant: <span style='color: green;'>```Script_Hangul = 26```</span>
  Hangul = 26,
  /// C++ enum variant: <span style='color: green;'>```Script_Ethiopic = 27```</span>
  Ethiopic = 27,
  /// C++ enum variant: <span style='color: green;'>```Script_Cherokee = 28```</span>
  Cherokee = 28,
  /// C++ enum variant: <span style='color: green;'>```Script_CanadianAboriginal = 29```</span>
  CanadianAboriginal = 29,
  /// C++ enum variant: <span style='color: green;'>```Script_Ogham = 30```</span>
  Ogham = 30,
  /// C++ enum variant: <span style='color: green;'>```Script_Runic = 31```</span>
  Runic = 31,
  /// C++ enum variant: <span style='color: green;'>```Script_Khmer = 32```</span>
  Khmer = 32,
  /// C++ enum variant: <span style='color: green;'>```Script_Mongolian = 33```</span>
  Mongolian = 33,
  /// C++ enum variant: <span style='color: green;'>```Script_Hiragana = 34```</span>
  Hiragana = 34,
  /// C++ enum variant: <span style='color: green;'>```Script_Katakana = 35```</span>
  Katakana = 35,
  /// C++ enum variant: <span style='color: green;'>```Script_Bopomofo = 36```</span>
  Bopomofo = 36,
  /// C++ enum variant: <span style='color: green;'>```Script_Han = 37```</span>
  Han = 37,
  /// C++ enum variant: <span style='color: green;'>```Script_Yi = 38```</span>
  Yi = 38,
  /// C++ enum variant: <span style='color: green;'>```Script_OldItalic = 39```</span>
  OldItalic = 39,
  /// C++ enum variant: <span style='color: green;'>```Script_Gothic = 40```</span>
  Gothic = 40,
  /// C++ enum variant: <span style='color: green;'>```Script_Deseret = 41```</span>
  Deseret = 41,
  /// C++ enum variant: <span style='color: green;'>```Script_Tagalog = 42```</span>
  Tagalog = 42,
  /// C++ enum variant: <span style='color: green;'>```Script_Hanunoo = 43```</span>
  Hanunoo = 43,
  /// C++ enum variant: <span style='color: green;'>```Script_Buhid = 44```</span>
  Buhid = 44,
  /// C++ enum variant: <span style='color: green;'>```Script_Tagbanwa = 45```</span>
  Tagbanwa = 45,
  /// C++ enum variant: <span style='color: green;'>```Script_Coptic = 46```</span>
  Coptic = 46,
  /// C++ enum variant: <span style='color: green;'>```Script_Limbu = 47```</span>
  Limbu = 47,
  /// C++ enum variant: <span style='color: green;'>```Script_TaiLe = 48```</span>
  TaiLe = 48,
  /// C++ enum variant: <span style='color: green;'>```Script_LinearB = 49```</span>
  LinearB = 49,
  /// C++ enum variant: <span style='color: green;'>```Script_Ugaritic = 50```</span>
  Ugaritic = 50,
  /// C++ enum variant: <span style='color: green;'>```Script_Shavian = 51```</span>
  Shavian = 51,
  /// C++ enum variant: <span style='color: green;'>```Script_Osmanya = 52```</span>
  Osmanya = 52,
  /// C++ enum variant: <span style='color: green;'>```Script_Cypriot = 53```</span>
  Cypriot = 53,
  /// C++ enum variant: <span style='color: green;'>```Script_Braille = 54```</span>
  Braille = 54,
  /// C++ enum variant: <span style='color: green;'>```Script_Buginese = 55```</span>
  Buginese = 55,
  /// C++ enum variant: <span style='color: green;'>```Script_NewTaiLue = 56```</span>
  NewTaiLue = 56,
  /// C++ enum variant: <span style='color: green;'>```Script_Glagolitic = 57```</span>
  Glagolitic = 57,
  /// C++ enum variant: <span style='color: green;'>```Script_Tifinagh = 58```</span>
  Tifinagh = 58,
  /// C++ enum variant: <span style='color: green;'>```Script_SylotiNagri = 59```</span>
  SylotiNagri = 59,
  /// C++ enum variant: <span style='color: green;'>```Script_OldPersian = 60```</span>
  OldPersian = 60,
  /// C++ enum variant: <span style='color: green;'>```Script_Kharoshthi = 61```</span>
  Kharoshthi = 61,
  /// C++ enum variant: <span style='color: green;'>```Script_Balinese = 62```</span>
  Balinese = 62,
  /// C++ enum variant: <span style='color: green;'>```Script_Cuneiform = 63```</span>
  Cuneiform = 63,
  /// C++ enum variant: <span style='color: green;'>```Script_Phoenician = 64```</span>
  Phoenician = 64,
  /// C++ enum variant: <span style='color: green;'>```Script_PhagsPa = 65```</span>
  PhagsPa = 65,
  /// C++ enum variant: <span style='color: green;'>```Script_Nko = 66```</span>
  Nko = 66,
  /// C++ enum variant: <span style='color: green;'>```Script_Sundanese = 67```</span>
  Sundanese = 67,
  /// C++ enum variant: <span style='color: green;'>```Script_Lepcha = 68```</span>
  Lepcha = 68,
  /// C++ enum variant: <span style='color: green;'>```Script_OlChiki = 69```</span>
  OlChiki = 69,
  /// C++ enum variant: <span style='color: green;'>```Script_Vai = 70```</span>
  Vai = 70,
  /// C++ enum variant: <span style='color: green;'>```Script_Saurashtra = 71```</span>
  Saurashtra = 71,
  /// C++ enum variant: <span style='color: green;'>```Script_KayahLi = 72```</span>
  KayahLi = 72,
  /// C++ enum variant: <span style='color: green;'>```Script_Rejang = 73```</span>
  Rejang = 73,
  /// C++ enum variant: <span style='color: green;'>```Script_Lycian = 74```</span>
  Lycian = 74,
  /// C++ enum variant: <span style='color: green;'>```Script_Carian = 75```</span>
  Carian = 75,
  /// C++ enum variant: <span style='color: green;'>```Script_Lydian = 76```</span>
  Lydian = 76,
  /// C++ enum variant: <span style='color: green;'>```Script_Cham = 77```</span>
  Cham = 77,
  /// C++ enum variant: <span style='color: green;'>```Script_TaiTham = 78```</span>
  TaiTham = 78,
  /// C++ enum variant: <span style='color: green;'>```Script_TaiViet = 79```</span>
  TaiViet = 79,
  /// C++ enum variant: <span style='color: green;'>```Script_Avestan = 80```</span>
  Avestan = 80,
  /// C++ enum variant: <span style='color: green;'>```Script_EgyptianHieroglyphs = 81```</span>
  EgyptianHieroglyphs = 81,
  /// C++ enum variant: <span style='color: green;'>```Script_Samaritan = 82```</span>
  Samaritan = 82,
  /// C++ enum variant: <span style='color: green;'>```Script_Lisu = 83```</span>
  Lisu = 83,
  /// C++ enum variant: <span style='color: green;'>```Script_Bamum = 84```</span>
  Bamum = 84,
  /// C++ enum variant: <span style='color: green;'>```Script_Javanese = 85```</span>
  Javanese = 85,
  /// C++ enum variant: <span style='color: green;'>```Script_MeeteiMayek = 86```</span>
  MeeteiMayek = 86,
  /// C++ enum variant: <span style='color: green;'>```Script_ImperialAramaic = 87```</span>
  ImperialAramaic = 87,
  /// C++ enum variant: <span style='color: green;'>```Script_OldSouthArabian = 88```</span>
  OldSouthArabian = 88,
  /// C++ enum variant: <span style='color: green;'>```Script_InscriptionalParthian = 89```</span>
  InscriptionalParthian = 89,
  /// C++ enum variant: <span style='color: green;'>```Script_InscriptionalPahlavi = 90```</span>
  InscriptionalPahlavi = 90,
  /// C++ enum variant: <span style='color: green;'>```Script_OldTurkic = 91```</span>
  OldTurkic = 91,
  /// C++ enum variant: <span style='color: green;'>```Script_Kaithi = 92```</span>
  Kaithi = 92,
  /// C++ enum variant: <span style='color: green;'>```Script_Batak = 93```</span>
  Batak = 93,
  /// C++ enum variant: <span style='color: green;'>```Script_Brahmi = 94```</span>
  Brahmi = 94,
  /// C++ enum variant: <span style='color: green;'>```Script_Mandaic = 95```</span>
  Mandaic = 95,
  /// C++ enum variant: <span style='color: green;'>```Script_Chakma = 96```</span>
  Chakma = 96,
  /// C++ enum variant: <span style='color: green;'>```Script_MeroiticCursive = 97```</span>
  MeroiticCursive = 97,
  /// C++ enum variant: <span style='color: green;'>```Script_MeroiticHieroglyphs = 98```</span>
  MeroiticHieroglyphs = 98,
  /// C++ enum variant: <span style='color: green;'>```Script_Miao = 99```</span>
  Miao = 99,
  /// C++ enum variant: <span style='color: green;'>```Script_Sharada = 100```</span>
  Sharada = 100,
  /// C++ enum variant: <span style='color: green;'>```Script_SoraSompeng = 101```</span>
  SoraSompeng = 101,
  /// C++ enum variant: <span style='color: green;'>```Script_Takri = 102```</span>
  Takri = 102,
  /// C++ enum variant: <span style='color: green;'>```Script_CaucasianAlbanian = 103```</span>
  CaucasianAlbanian = 103,
  /// C++ enum variant: <span style='color: green;'>```Script_BassaVah = 104```</span>
  BassaVah = 104,
  /// C++ enum variant: <span style='color: green;'>```Script_Duployan = 105```</span>
  Duployan = 105,
  /// C++ enum variant: <span style='color: green;'>```Script_Elbasan = 106```</span>
  Elbasan = 106,
  /// C++ enum variant: <span style='color: green;'>```Script_Grantha = 107```</span>
  Grantha = 107,
  /// C++ enum variant: <span style='color: green;'>```Script_PahawhHmong = 108```</span>
  PahawhHmong = 108,
  /// C++ enum variant: <span style='color: green;'>```Script_Khojki = 109```</span>
  Khojki = 109,
  /// C++ enum variant: <span style='color: green;'>```Script_LinearA = 110```</span>
  LinearA = 110,
  /// C++ enum variant: <span style='color: green;'>```Script_Mahajani = 111```</span>
  Mahajani = 111,
  /// C++ enum variant: <span style='color: green;'>```Script_Manichaean = 112```</span>
  Manichaean = 112,
  /// C++ enum variant: <span style='color: green;'>```Script_MendeKikakui = 113```</span>
  MendeKikakui = 113,
  /// C++ enum variant: <span style='color: green;'>```Script_Modi = 114```</span>
  Modi = 114,
  /// C++ enum variant: <span style='color: green;'>```Script_Mro = 115```</span>
  Mro = 115,
  /// C++ enum variant: <span style='color: green;'>```Script_OldNorthArabian = 116```</span>
  OldNorthArabian = 116,
  /// C++ enum variant: <span style='color: green;'>```Script_Nabataean = 117```</span>
  Nabataean = 117,
  /// C++ enum variant: <span style='color: green;'>```Script_Palmyrene = 118```</span>
  Palmyrene = 118,
  /// C++ enum variant: <span style='color: green;'>```Script_PauCinHau = 119```</span>
  PauCinHau = 119,
  /// C++ enum variant: <span style='color: green;'>```Script_OldPermic = 120```</span>
  OldPermic = 120,
  /// C++ enum variant: <span style='color: green;'>```Script_PsalterPahlavi = 121```</span>
  PsalterPahlavi = 121,
  /// C++ enum variant: <span style='color: green;'>```Script_Siddham = 122```</span>
  Siddham = 122,
  /// C++ enum variant: <span style='color: green;'>```Script_Khudawadi = 123```</span>
  Khudawadi = 123,
  /// C++ enum variant: <span style='color: green;'>```Script_Tirhuta = 124```</span>
  Tirhuta = 124,
  /// C++ enum variant: <span style='color: green;'>```Script_WarangCiti = 125```</span>
  WarangCiti = 125,
  /// C++ enum variant: <span style='color: green;'>```Script_Ahom = 126```</span>
  Ahom = 126,
  /// C++ enum variant: <span style='color: green;'>```Script_AnatolianHieroglyphs = 127```</span>
  AnatolianHieroglyphs = 127,
  /// C++ enum variant: <span style='color: green;'>```Script_Hatran = 128```</span>
  Hatran = 128,
  /// C++ enum variant: <span style='color: green;'>```Script_Multani = 129```</span>
  Multani = 129,
  /// C++ enum variant: <span style='color: green;'>```Script_OldHungarian = 130```</span>
  OldHungarian = 130,
  /// C++ enum variant: <span style='color: green;'>```Script_SignWriting = 131```</span>
  SignWriting = 131,
  /// C++ enum variant: <span style='color: green;'>```ScriptCount = 132```</span>
  Count = 132,
}

/// C++ type: <span style='color: green;'>```QChar::SpecialCharacter```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SpecialCharacter {
  /// C++ enum variant: <span style='color: green;'>```Null = 0```</span>
  Null = 0,
  /// C++ enum variant: <span style='color: green;'>```Tabulation = 9```</span>
  Tabulation = 9,
  /// C++ enum variant: <span style='color: green;'>```LineFeed = 10```</span>
  LineFeed = 10,
  /// C++ enum variant: <span style='color: green;'>```CarriageReturn = 13```</span>
  CarriageReturn = 13,
  /// C++ enum variant: <span style='color: green;'>```Space = 32```</span>
  Space = 32,
  /// C++ enum variant: <span style='color: green;'>```Nbsp = 160```</span>
  Nbsp = 160,
  /// C++ enum variant: <span style='color: green;'>```SoftHyphen = 173```</span>
  SoftHyphen = 173,
  /// C++ enum variant: <span style='color: green;'>```LineSeparator = 8232```</span>
  LineSeparator = 8232,
  /// C++ enum variant: <span style='color: green;'>```ParagraphSeparator = 8233```</span>
  ParagraphSeparator = 8233,
  /// C++ enum variant: <span style='color: green;'>```ByteOrderMark = 65279```</span>
  ByteOrderMark = 65279,
  /// C++ enum variant: <span style='color: green;'>```ObjectReplacementCharacter = 65532```</span>
  ObjectReplacementCharacter = 65532,
  /// C++ enum variant: <span style='color: green;'>```ReplacementCharacter = 65533```</span>
  ReplacementCharacter = 65533,
  /// C++ enum variant: <span style='color: green;'>```ByteOrderSwapped = 65534```</span>
  ByteOrderSwapped = 65534,
  /// C++ enum variant: <span style='color: green;'>```LastValidCodePoint = 1114111```</span>
  LastValidCodePoint = 1114111,
}

/// C++ type: <span style='color: green;'>```QChar::UnicodeVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum UnicodeVersion {
  /// C++ enum variant: <span style='color: green;'>```Unicode_Unassigned = 0```</span>
  UnicodeUnassigned = 0,
  /// C++ enum variant: <span style='color: green;'>```Unicode_1_1 = 1```</span>
  Unicode11 = 1,
  /// C++ enum variant: <span style='color: green;'>```Unicode_2_0 = 2```</span>
  Unicode20 = 2,
  /// C++ enum variant: <span style='color: green;'>```Unicode_2_1_2 = 3```</span>
  Unicode212 = 3,
  /// C++ enum variant: <span style='color: green;'>```Unicode_3_0 = 4```</span>
  Unicode30 = 4,
  /// C++ enum variant: <span style='color: green;'>```Unicode_3_1 = 5```</span>
  Unicode31 = 5,
  /// C++ enum variant: <span style='color: green;'>```Unicode_3_2 = 6```</span>
  Unicode32 = 6,
  /// C++ enum variant: <span style='color: green;'>```Unicode_4_0 = 7```</span>
  Unicode40 = 7,
  /// C++ enum variant: <span style='color: green;'>```Unicode_4_1 = 8```</span>
  Unicode41 = 8,
  /// C++ enum variant: <span style='color: green;'>```Unicode_5_0 = 9```</span>
  Unicode50 = 9,
  /// C++ enum variant: <span style='color: green;'>```Unicode_5_1 = 10```</span>
  Unicode51 = 10,
  /// C++ enum variant: <span style='color: green;'>```Unicode_5_2 = 11```</span>
  Unicode52 = 11,
  /// C++ enum variant: <span style='color: green;'>```Unicode_6_0 = 12```</span>
  Unicode60 = 12,
  /// C++ enum variant: <span style='color: green;'>```Unicode_6_1 = 13```</span>
  Unicode61 = 13,
  /// C++ enum variant: <span style='color: green;'>```Unicode_6_2 = 14```</span>
  Unicode62 = 14,
  /// C++ enum variant: <span style='color: green;'>```Unicode_6_3 = 15```</span>
  Unicode63 = 15,
  /// C++ enum variant: <span style='color: green;'>```Unicode_7_0 = 16```</span>
  Unicode70 = 16,
  /// C++ enum variant: <span style='color: green;'>```Unicode_8_0 = 17```</span>
  Unicode80 = 17,
}

/// C++ method: <span style='color: green;'>```bool operator>=(QChar c1, QChar c2)```</span>
///
///
pub fn op_ge(c1: &::char::Char, c2: &::char::Char) -> bool {
  unsafe { ::ffi::qt_core_c_QChar_G_operator_ge(c1 as *const ::char::Char, c2 as *const ::char::Char) }
}

/// C++ method: <span style='color: green;'>```bool operator>(QChar c1, QChar c2)```</span>
///
///
pub fn op_gt(c1: &::char::Char, c2: &::char::Char) -> bool {
  unsafe { ::ffi::qt_core_c_QChar_G_operator_gt(c1 as *const ::char::Char, c2 as *const ::char::Char) }
}

/// C++ method: <span style='color: green;'>```bool operator<=(QChar c1, QChar c2)```</span>
///
///
pub fn op_le(c1: &::char::Char, c2: &::char::Char) -> bool {
  unsafe { ::ffi::qt_core_c_QChar_G_operator_le(c1 as *const ::char::Char, c2 as *const ::char::Char) }
}

/// C++ method: <span style='color: green;'>```bool operator!=(QChar c1, QChar c2)```</span>
///
///
pub fn op_neq(c1: &::char::Char, c2: &::char::Char) -> bool {
  unsafe { ::ffi::qt_core_c_QChar_G_operator_neq(c1 as *const ::char::Char, c2 as *const ::char::Char) }
}

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, QChar arg2)```</span>
///
///
pub fn op_shl<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 ::char::Char)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QChar_G_operator_shl(arg1 as *mut ::data_stream::DataStream,
                                          arg2 as *const ::char::Char)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QChar& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 mut ::char::Char)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QChar_G_operator_shr(arg1 as *mut ::data_stream::DataStream,
                                          arg2 as *mut ::char::Char)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Char::new0](../struct.Char.html#method.new0) method.
  pub trait CharNew0Args {
    fn exec(self) -> ::char::Char;
  }
  impl CharNew0Args for ::char::SpecialCharacter {
    fn exec(self) -> ::char::Char {
      let s = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_QChar_SpecialCharacter(s, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CharNew0Args for &'a ::latin1_char::Latin1Char {
    fn exec(self) -> ::char::Char {
      let ch = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_QLatin1Char(ch as *const ::latin1_char::Latin1Char, &mut object);
        }
        object
      }
    }
  }
  impl CharNew0Args for ::libc::c_char {
    fn exec(self) -> ::char::Char {
      let c = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_char(c, &mut object);
        }
        object
      }
    }
  }
  impl CharNew0Args for () {
    fn exec(self) -> ::char::Char {

      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl CharNew0Args for (::libc::c_uchar, ::libc::c_uchar) {
    fn exec(self) -> ::char::Char {
      let c = self.0;
      let r = self.1;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_unsigned_char_unsigned_char(c, r, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Char::new1](../struct.Char.html#method.new1) method.
  pub trait CharNew1Args {
    fn exec(self) -> ::char::Char;
  }
  impl CharNew1Args for ::libc::c_int {
    fn exec(self) -> ::char::Char {
      let rc = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_int(rc, &mut object);
        }
        object
      }
    }
  }
  impl CharNew1Args for ::libc::c_uchar {
    fn exec(self) -> ::char::Char {
      let c = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_unsigned_char(c, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Char::new2](../struct.Char.html#method.new2) method.
  pub trait CharNew2Args {
    fn exec(self) -> ::char::Char;
  }
  impl CharNew2Args for ::libc::c_short {
    fn exec(self) -> ::char::Char {
      let rc = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_short(rc, &mut object);
        }
        object
      }
    }
  }
  impl CharNew2Args for ::libc::c_uint {
    fn exec(self) -> ::char::Char {
      let rc = self;
      {
        let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QChar_constructor_unsigned_int(rc, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Char::surrogate_to_ucs4](../struct.Char.html#method.surrogate_to_ucs4) method.
  pub trait CharSurrogateToUcs4Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> CharSurrogateToUcs4Args for (&'a ::char::Char, &'a ::char::Char) {
    fn exec(self) -> ::libc::c_uint {
      let high = self.0;
      let low = self.1;
      unsafe {
        ::ffi::qt_core_c_QChar_surrogateToUcs4_QChar_QChar(high as *const ::char::Char, low as *const ::char::Char)
      }
    }
  }
  impl CharSurrogateToUcs4Args for (::libc::c_ushort, ::libc::c_ushort) {
    fn exec(self) -> ::libc::c_uint {
      let high = self.0;
      let low = self.1;
      unsafe { ::ffi::qt_core_c_QChar_surrogateToUcs4_unsigned_short_unsigned_short(high, low) }
    }
  }
}
