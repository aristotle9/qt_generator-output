/// C++ type: <span style='color: green;'>```QTextListFormat::Style```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Style {
  /// C++ enum variant: <span style='color: green;'>```ListUpperRoman = -8```</span>
  UpperRoman = -8,
  /// C++ enum variant: <span style='color: green;'>```ListLowerRoman = -7```</span>
  LowerRoman = -7,
  /// C++ enum variant: <span style='color: green;'>```ListUpperAlpha = -6```</span>
  UpperAlpha = -6,
  /// C++ enum variant: <span style='color: green;'>```ListLowerAlpha = -5```</span>
  LowerAlpha = -5,
  /// C++ enum variant: <span style='color: green;'>```ListDecimal = -4```</span>
  Decimal = -4,
  /// C++ enum variant: <span style='color: green;'>```ListSquare = -3```</span>
  Square = -3,
  /// C++ enum variant: <span style='color: green;'>```ListCircle = -2```</span>
  Circle = -2,
  /// C++ enum variant: <span style='color: green;'>```ListDisc = -1```</span>
  Disc = -1,
  /// C++ enum variant: <span style='color: green;'>```ListStyleUndefined = 0```</span>
  StyleUndefined = 0,
}

/// C++ type: <span style='color: green;'>```QTextListFormat```</span>
#[repr(C)]
pub struct TextListFormat([u8; ::type_sizes::QT_GUI_TEXT_LIST_FORMAT_TEXT_LIST_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextListFormat {
  unsafe fn new_uninitialized() -> TextListFormat {
    TextListFormat(::std::mem::uninitialized())
  }
}

impl TextListFormat {
  /// C++ method: <span style='color: green;'>```int QTextListFormat::indent() const```</span>
  ///
  ///
  pub fn indent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_indent(self as *const ::text_list_format::TextListFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextListFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_isValid(self as *const ::text_list_format::TextListFormat) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextListFormat::QTextListFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_list_format::TextListFormat {
    {
      let mut object: ::text_list_format::TextListFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextListFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextListFormat::numberPrefix() const```</span>
  ///
  ///
  pub fn number_prefix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextListFormat_numberPrefix_to_output(self as *const ::text_list_format::TextListFormat,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextListFormat::numberSuffix() const```</span>
  ///
  ///
  pub fn number_suffix(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextListFormat_numberSuffix_to_output(self as *const ::text_list_format::TextListFormat,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextListFormat::setIndent(int indent)```</span>
  ///
  ///
  pub fn set_indent(&mut self, indent: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_setIndent(self as *mut ::text_list_format::TextListFormat, indent) }
  }

  /// C++ method: <span style='color: green;'>```void QTextListFormat::setNumberPrefix(const QString& numberPrefix)```</span>
  ///
  ///
  pub fn set_number_prefix(&mut self, number_prefix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextListFormat_setNumberPrefix(self as *mut ::text_list_format::TextListFormat,
                                                      number_prefix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextListFormat::setNumberSuffix(const QString& numberSuffix)```</span>
  ///
  ///
  pub fn set_number_suffix(&mut self, number_suffix: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextListFormat_setNumberSuffix(self as *mut ::text_list_format::TextListFormat,
                                                      number_suffix as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextListFormat::setStyle(QTextListFormat::Style style)```</span>
  ///
  ///
  pub fn set_style(&mut self, style: ::text_list_format::Style) {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_setStyle(self as *mut ::text_list_format::TextListFormat, style) }
  }

  /// C++ method: <span style='color: green;'>```QTextListFormat::Style QTextListFormat::style() const```</span>
  ///
  ///
  pub fn style(&self) -> ::text_list_format::Style {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_style(self as *const ::text_list_format::TextListFormat) }
  }
}

impl Drop for ::text_list_format::TextListFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextListFormat::~QTextListFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextListFormat_destructor(self as *mut ::text_list_format::TextListFormat) }
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_list_format::TextListFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_list_format::TextListFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_list_format::TextListFormat as *mut ::text_list_format::TextListFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_list_format::TextListFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_list_format::TextListFormat {
    let ffi_result =
      ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextListFormat_ptr(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_list_format::TextListFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextListFormat_ptr(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_list_format::TextListFormat {
  type Target = ::text_format::TextFormat;
  fn deref(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_list_format::TextListFormat as *mut ::text_list_format::TextListFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_list_format::TextListFormat {
  fn deref_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QTextListFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_list_format::TextListFormat)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
