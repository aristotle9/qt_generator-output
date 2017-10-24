/// C++ type: <span style='color: green;'>```QStyleOptionButton::ButtonFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ButtonFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Flat = 1```</span>
  Flat = 1,
  /// C++ enum variant: <span style='color: green;'>```HasMenu = 2```</span>
  HasMenu = 2,
  /// C++ enum variant: <span style='color: green;'>```DefaultButton = 4```</span>
  DefaultButton = 4,
  /// C++ enum variant: <span style='color: green;'>```AutoDefaultButton = 8```</span>
  AutoDefaultButton = 8,
  /// C++ enum variant: <span style='color: green;'>```CommandLinkButton = 16```</span>
  CommandLinkButton = 16,
}

impl ::qt_core::flags::FlaggableEnum for ButtonFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ButtonFeature"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionButton```</span>
#[repr(C)]
pub struct StyleOptionButton(u8);

impl StyleOptionButton {
  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionButton::ButtonFeature>& QStyleOptionButton::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_button::ButtonFeature> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionButton_features(self as *const ::style_option_button::StyleOptionButton)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionButton::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_icon(self as *const ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionButton::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_icon_mut(self as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionButton::iconSize() const```</span>
  ///
  ///
  pub fn icon_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionButton_iconSize(self as *const ::style_option_button::StyleOptionButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionButton::iconSize_mut()```</span>
  ///
  ///
  pub fn icon_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionButton_iconSize_mut(self as *mut ::style_option_button::StyleOptionButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionButton::QStyleOptionButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionButton::QStyleOptionButton()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_button::StyleOptionButton) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionButton::QStyleOptionButton(const QStyleOptionButton& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton>
    where Args: overloading::StyleOptionButtonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionButton::set_features(QFlags<QStyleOptionButton::ButtonFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_button::ButtonFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionButton_set_features(self as *mut ::style_option_button::StyleOptionButton,
                                                          value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionButton::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionButton_set_icon(self as *mut ::style_option_button::StyleOptionButton,
                                                      value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionButton::set_iconSize(QSize value)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionButton_set_iconSize(self as *mut ::style_option_button::StyleOptionButton,
                                                          value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionButton::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionButton_set_text(self as *mut ::style_option_button::StyleOptionButton,
                                                      value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionButton::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_text(self as *const ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionButton::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_text_mut(self as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_button::StyleOptionButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionButton_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionButton::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 2```</span>
  Type = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionButton::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_button::StyleOptionButton {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(self as *const ::style_option_button::StyleOptionButton as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_button::StyleOptionButton> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_button::StyleOptionButton {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOptionButton_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_button::StyleOptionButton {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOptionButton_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_button::StyleOptionButton {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(self as *const ::style_option_button::StyleOptionButton as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_button::StyleOptionButton {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_button::StyleOptionButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionButton::new](../struct.StyleOptionButton.html#method.new) method.
  pub trait StyleOptionButtonNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton>;
  }
  impl StyleOptionButtonNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionButton_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionButtonNewArgs for &'a ::style_option_button::StyleOptionButton {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_button::StyleOptionButton> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionButton_new_other(other as *const ::style_option_button::StyleOptionButton)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
