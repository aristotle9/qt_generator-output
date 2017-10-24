/// C++ type: <span style='color: green;'>```QStyleOptionToolButton```</span>
#[repr(C)]
pub struct StyleOptionToolButton(u8);

impl StyleOptionToolButton {
  /// C++ method: <span style='color: green;'>```const Qt::ArrowType& QStyleOptionToolButton::arrowType() const```</span>
  ///
  ///
  pub fn arrow_type<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::ArrowType {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_arrowType(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::ArrowType& QStyleOptionToolButton::arrowType_mut()```</span>
  ///
  ///
  pub fn arrow_type_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::ArrowType {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_arrowType_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionToolButton::ToolButtonFeature>& QStyleOptionToolButton::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_tool_button::ToolButtonFeature> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_features(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```const QFont& QStyleOptionToolButton::font() const```</span>
  ///
  ///
  pub fn font<'l0>(&'l0 self) -> &'l0 ::qt_gui::font::Font {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_font(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFont& QStyleOptionToolButton::font_mut()```</span>
  ///
  ///
  pub fn font_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::font::Font {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_font_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionToolButton::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_icon(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionToolButton::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_icon_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionToolButton::iconSize() const```</span>
  ///
  ///
  pub fn icon_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_iconSize(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionToolButton::iconSize_mut()```</span>
  ///
  ///
  pub fn icon_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_iconSize_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionToolButton::QStyleOptionToolButton```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolButton::QStyleOptionToolButton()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tool_button::StyleOptionToolButton) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolButton::QStyleOptionToolButton(const QStyleOptionToolButton& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton>
    where Args: overloading::StyleOptionToolButtonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QPoint& QStyleOptionToolButton::pos() const```</span>
  ///
  ///
  pub fn pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolButton_pos(self as *const ::style_option_tool_button::StyleOptionToolButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QStyleOptionToolButton::pos_mut()```</span>
  ///
  ///
  pub fn pos_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_pos_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_arrowType(Qt::ArrowType value)```</span>
  ///
  ///
  pub fn set_arrow_type(&mut self, value: &::qt_core::qt::ArrowType) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_arrowType(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_core::qt::ArrowType) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_features(QFlags<QStyleOptionToolButton::ToolButtonFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_tool_button::ToolButtonFeature>) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_features(self as *mut ::style_option_tool_button::StyleOptionToolButton, value.to_int() as ::libc::c_uint) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_font(QFont value)```</span>
  ///
  ///
  pub fn set_font(&mut self, value: &::qt_gui::font::Font) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_font(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_gui::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_icon(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_gui::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_iconSize(QSize value)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, value: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_iconSize(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_pos(QPoint value)```</span>
  ///
  ///
  pub fn set_pos(&mut self, value: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolButton_set_pos(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_text(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolButton::set_toolButtonStyle(Qt::ToolButtonStyle value)```</span>
  ///
  ///
  pub fn set_tool_button_style(&mut self, value: &::qt_core::qt::ToolButtonStyle) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_set_toolButtonStyle(self as *mut ::style_option_tool_button::StyleOptionToolButton, value as *const ::qt_core::qt::ToolButtonStyle) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionToolButton::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_text(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionToolButton::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_text_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::ToolButtonStyle& QStyleOptionToolButton::toolButtonStyle() const```</span>
  ///
  ///
  pub fn tool_button_style<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::ToolButtonStyle {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_toolButtonStyle(self as *const ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::ToolButtonStyle& QStyleOptionToolButton::toolButtonStyle_mut()```</span>
  ///
  ///
  pub fn tool_button_style_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::ToolButtonStyle {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_toolButtonStyle_mut(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_tool_button::StyleOptionToolButton {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionToolButton_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolButton::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983043```</span>
  Type = 983043,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolButton::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolButton::ToolButtonFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolButtonFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Arrow = 1```</span>
  Arrow = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Menu = 4```</span>
  /// - <span style='color: green;'>```MenuButtonPopup = 4```</span>
  ///
  Menu = 4,
  /// C++ enum variant: <span style='color: green;'>```PopupDelay = 8```</span>
  PopupDelay = 8,
  /// C++ enum variant: <span style='color: green;'>```HasMenu = 16```</span>
  HasMenu = 16,
}

impl ::qt_core::flags::FlaggableEnum for ToolButtonFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ToolButtonFeature"
  }
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tool_button::StyleOptionToolButton {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tool_button::StyleOptionToolButton as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_tool_button::StyleOptionToolButton {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_tool_button::StyleOptionToolButton as *mut ::style_option_tool_button::StyleOptionToolButton) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tool_button::StyleOptionToolButton> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tool_button::StyleOptionToolButton {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_tool_button::StyleOptionToolButton {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tool_button::StyleOptionToolButton> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tool_button::StyleOptionToolButton {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_tool_button::StyleOptionToolButton {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionToolButton_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_tool_button::StyleOptionToolButton {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_tool_button::StyleOptionToolButton as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tool_button::StyleOptionToolButton {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_tool_button::StyleOptionToolButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionToolButton::new](../struct.StyleOptionToolButton.html#method.new) method.
  pub trait StyleOptionToolButtonNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton>;
  }
  impl StyleOptionToolButtonNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionToolButtonNewArgs for &'a ::style_option_tool_button::StyleOptionToolButton {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_button::StyleOptionToolButton> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolButton_new_other(other as *const ::style_option_tool_button::StyleOptionToolButton) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
