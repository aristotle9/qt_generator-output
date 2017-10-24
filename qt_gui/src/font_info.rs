/// C++ type: <span style='color: green;'>```QFontInfo```</span>
#[repr(C)]
pub struct FontInfo([u8; ::type_sizes::QT_GUI_FONT_INFO_FONT_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for FontInfo {
  unsafe fn new_uninitialized() -> FontInfo {
    FontInfo(::std::mem::uninitialized())
  }
}

impl FontInfo {
  /// C++ method: <span style='color: green;'>```bool QFontInfo::bold() const```</span>
  ///
  ///
  pub fn bold(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_bold(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::exactMatch() const```</span>
  ///
  ///
  pub fn exact_match(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_exactMatch(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```QString QFontInfo::family() const```</span>
  ///
  ///
  pub fn family(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontInfo_family_to_output(self as *const ::font_info::FontInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::fixedPitch() const```</span>
  ///
  ///
  pub fn fixed_pitch(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_fixedPitch(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::italic() const```</span>
  ///
  ///
  pub fn italic(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_italic(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```QFontInfo::QFontInfo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::font::Font) -> ::font_info::FontInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontInfo::QFontInfo(const QFont& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::font_info::FontInfo) -> ::font_info::FontInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontInfo::QFontInfo(const QFontInfo& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::font_info::FontInfo
    where Args: overloading::FontInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFontInfo& QFontInfo::operator=(const QFontInfo& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::font_info::FontInfo) -> &'l0 mut ::font_info::FontInfo {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QFontInfo_operator_assign(self as *mut ::font_info::FontInfo,
                                                arg1 as *const ::font_info::FontInfo)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::overline() const```</span>
  ///
  ///
  pub fn overline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_overline(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```int QFontInfo::pixelSize() const```</span>
  ///
  ///
  pub fn pixel_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontInfo_pixelSize(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```int QFontInfo::pointSize() const```</span>
  ///
  ///
  pub fn point_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontInfo_pointSize(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```double QFontInfo::pointSizeF() const```</span>
  ///
  ///
  pub fn point_size_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontInfo_pointSizeF(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::rawMode() const```</span>
  ///
  ///
  pub fn raw_mode(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_rawMode(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::strikeOut() const```</span>
  ///
  ///
  pub fn strike_out(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_strikeOut(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```QString QFontInfo::styleName() const```</span>
  ///
  ///
  pub fn style_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontInfo_styleName_to_output(self as *const ::font_info::FontInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFontInfo::swap(QFontInfo& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::font_info::FontInfo) {
    unsafe {
      ::ffi::qt_gui_c_QFontInfo_swap(self as *mut ::font_info::FontInfo,
                                     other as *mut ::font_info::FontInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontInfo::underline() const```</span>
  ///
  ///
  pub fn underline(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontInfo_underline(self as *const ::font_info::FontInfo) }
  }

  /// C++ method: <span style='color: green;'>```int QFontInfo::weight() const```</span>
  ///
  ///
  pub fn weight(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontInfo_weight(self as *const ::font_info::FontInfo) }
  }
}

impl Drop for ::font_info::FontInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QFontInfo::~QFontInfo()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QFontInfo_destructor(self as *mut ::font_info::FontInfo) }
  }
}

/// C++ method: <span style='color: green;'>```void swap(QFontInfo& value1, QFontInfo& value2)```</span>
///
///
pub fn swap(value1: &mut ::font_info::FontInfo, value2: &mut ::font_info::FontInfo) {
  unsafe {
    ::ffi::qt_gui_c_QFontInfo_G_swap(value1 as *mut ::font_info::FontInfo,
                                     value2 as *mut ::font_info::FontInfo)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FontInfo::new](../struct.FontInfo.html#method.new) method.
  pub trait FontInfoNewArgs {
    fn exec(self) -> ::font_info::FontInfo;
  }
  impl<'a> FontInfoNewArgs for &'a ::font::Font {
    fn exec(self) -> ::font_info::FontInfo {
      let arg1 = self;
      {
        let mut object: ::font_info::FontInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontInfo_constructor_QFont(arg1 as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontInfoNewArgs for &'a ::font_info::FontInfo {
    fn exec(self) -> ::font_info::FontInfo {
      let arg1 = self;
      {
        let mut object: ::font_info::FontInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontInfo_constructor_QFontInfo(arg1 as *const ::font_info::FontInfo, &mut object);
        }
        object
      }
    }
  }
}
