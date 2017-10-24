/// C++ type: <span style='color: green;'>```QTextImageFormat```</span>
#[repr(C)]
pub struct TextImageFormat([u8; ::type_sizes::QT_GUI_TEXT_IMAGE_FORMAT_TEXT_IMAGE_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextImageFormat {
  unsafe fn new_uninitialized() -> TextImageFormat {
    TextImageFormat(::std::mem::uninitialized())
  }
}

impl TextImageFormat {
  /// C++ method: <span style='color: green;'>```double QTextImageFormat::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_height(self as *const ::text_image_format::TextImageFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextImageFormat::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_isValid(self as *const ::text_image_format::TextImageFormat) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextImageFormat::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextImageFormat_name_to_output(self as *const ::text_image_format::TextImageFormat,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextImageFormat::QTextImageFormat()```</span>
  ///
  ///
  pub fn new() -> ::text_image_format::TextImageFormat {
    {
      let mut object: ::text_image_format::TextImageFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextImageFormat_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextImageFormat::setHeight(double height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_setHeight(self as *mut ::text_image_format::TextImageFormat, height) }
  }

  /// C++ method: <span style='color: green;'>```void QTextImageFormat::setName(const QString& name)```</span>
  ///
  ///
  pub fn set_name(&mut self, name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextImageFormat_setName(self as *mut ::text_image_format::TextImageFormat,
                                               name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextImageFormat::setWidth(double width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_setWidth(self as *mut ::text_image_format::TextImageFormat, width) }
  }

  /// C++ method: <span style='color: green;'>```double QTextImageFormat::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_width(self as *const ::text_image_format::TextImageFormat) }
  }
}

impl Drop for ::text_image_format::TextImageFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextImageFormat::~QTextImageFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextImageFormat_destructor(self as *mut ::text_image_format::TextImageFormat) }
  }
}

impl ::cpp_utils::StaticCast<::text_char_format::TextCharFormat> for ::text_image_format::TextImageFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(self as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(self as *const ::text_image_format::TextImageFormat as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::text_format::TextFormat> for ::text_image_format::TextImageFormat {
  fn static_cast_mut(&mut self) -> &mut ::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextFormat_ptr(self as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::text_format::TextFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextFormat_ptr(self as *const ::text_image_format::TextImageFormat as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_image_format::TextImageFormat> for ::text_char_format::TextCharFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_image_format::TextImageFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextCharFormat(self as *mut ::text_char_format::TextCharFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_image_format::TextImageFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextCharFormat(self as *const ::text_char_format::TextCharFormat as *mut ::text_char_format::TextCharFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::text_image_format::TextImageFormat> for ::text_format::TextFormat {
  unsafe fn static_cast_mut(&mut self) -> &mut ::text_image_format::TextImageFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextFormat(self as *mut ::text_format::TextFormat);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::text_image_format::TextImageFormat {
    let ffi_result = ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextImageFormat_ptr_QTextFormat(self as *const ::text_format::TextFormat as *mut ::text_format::TextFormat);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::text_image_format::TextImageFormat {
  type Target = ::text_char_format::TextCharFormat;
  fn deref(&self) -> &::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(self as *const ::text_image_format::TextImageFormat as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::text_image_format::TextImageFormat {
  fn deref_mut(&mut self) -> &mut ::text_char_format::TextCharFormat {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextImageFormat_G_static_cast_QTextCharFormat_ptr(self as *mut ::text_image_format::TextImageFormat) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
