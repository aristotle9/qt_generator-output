/// C++ type: <span style='color: green;'>```QStyleOptionToolBar```</span>
#[repr(C)]
pub struct StyleOptionToolBar(u8);

impl StyleOptionToolBar {
  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionToolBar::ToolBarFeature>& QStyleOptionToolBar::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_tool_bar::ToolBarFeature> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBar_features(self as *const ::style_option_tool_bar::StyleOptionToolBar)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionToolBar::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_lineWidth(self as *const ::style_option_tool_bar::StyleOptionToolBar)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionToolBar::midLineWidth() const```</span>
  ///
  ///
  pub fn mid_line_width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_midLineWidth(self as *const ::style_option_tool_bar::StyleOptionToolBar)
    }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionToolBar::QStyleOptionToolBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolBar::QStyleOptionToolBar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tool_bar::StyleOptionToolBar) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionToolBar::QStyleOptionToolBar(const QStyleOptionToolBar& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar>
    where Args: overloading::StyleOptionToolBarNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyleOptionToolBar::ToolBarPosition QStyleOptionToolBar::positionOfLine() const```</span>
  ///
  ///
  pub fn position_of_line(&self) -> ::style_option_tool_bar::ToolBarPosition {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_positionOfLine(self as *const ::style_option_tool_bar::StyleOptionToolBar)
    }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionToolBar::ToolBarPosition QStyleOptionToolBar::positionWithinLine() const```</span>
  ///
  ///
  pub fn position_within_line(&self) -> ::style_option_tool_bar::ToolBarPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_positionWithinLine(self as *const ::style_option_tool_bar::StyleOptionToolBar) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_features(QFlags<QStyleOptionToolBar::ToolBarFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_tool_bar::ToolBarFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_set_features(self as *mut ::style_option_tool_bar::StyleOptionToolBar,
                                                           value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_lineWidth(int value)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_set_lineWidth(self as *mut ::style_option_tool_bar::StyleOptionToolBar,
                                                            value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_midLineWidth(int value)```</span>
  ///
  ///
  pub fn set_mid_line_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_set_midLineWidth(self as *mut ::style_option_tool_bar::StyleOptionToolBar, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_positionOfLine(QStyleOptionToolBar::ToolBarPosition value)```</span>
  ///
  ///
  pub fn set_position_of_line(&mut self, value: ::style_option_tool_bar::ToolBarPosition) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_set_positionOfLine(self as *mut ::style_option_tool_bar::StyleOptionToolBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_positionWithinLine(QStyleOptionToolBar::ToolBarPosition value)```</span>
  ///
  ///
  pub fn set_position_within_line(&mut self, value: ::style_option_tool_bar::ToolBarPosition) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_set_positionWithinLine(self as *mut ::style_option_tool_bar::StyleOptionToolBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionToolBar::set_toolBarArea(Qt::ToolBarArea value)```</span>
  ///
  ///
  pub fn set_tool_bar_area(&mut self, value: &::qt_core::qt::ToolBarArea) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionToolBar_set_toolBarArea(self as *mut ::style_option_tool_bar::StyleOptionToolBar, value as *const ::qt_core::qt::ToolBarArea)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::ToolBarArea& QStyleOptionToolBar::toolBarArea() const```</span>
  ///
  ///
  pub fn tool_bar_area<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::ToolBarArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionToolBar_toolBarArea(self as *const ::style_option_tool_bar::StyleOptionToolBar)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::ToolBarArea& QStyleOptionToolBar::toolBarArea_mut()```</span>
  ///
  ///
  pub fn tool_bar_area_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::ToolBarArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_toolBarArea_mut(self as *mut ::style_option_tool_bar::StyleOptionToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_tool_bar::StyleOptionToolBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionToolBar_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBar::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 14```</span>
  Type = 14,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBar::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBar::ToolBarFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolBarFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Movable = 1```</span>
  Movable = 1,
}

impl ::qt_core::flags::FlaggableEnum for ToolBarFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ToolBarFeature"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionToolBar::ToolBarPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ToolBarPosition {
  /// C++ enum variant: <span style='color: green;'>```Beginning = 0```</span>
  Beginning = 0,
  /// C++ enum variant: <span style='color: green;'>```Middle = 1```</span>
  Middle = 1,
  /// C++ enum variant: <span style='color: green;'>```End = 2```</span>
  End = 2,
  /// C++ enum variant: <span style='color: green;'>```OnlyOne = 3```</span>
  OnlyOne = 3,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tool_bar::StyleOptionToolBar {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tool_bar::StyleOptionToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tool_bar::StyleOptionToolBar as *mut ::style_option_tool_bar::StyleOptionToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tool_bar::StyleOptionToolBar> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tool_bar::StyleOptionToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOptionToolBar_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_tool_bar::StyleOptionToolBar {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOptionToolBar_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_tool_bar::StyleOptionToolBar {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tool_bar::StyleOptionToolBar as *mut ::style_option_tool_bar::StyleOptionToolBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tool_bar::StyleOptionToolBar {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tool_bar::StyleOptionToolBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionToolBar::new](../struct.StyleOptionToolBar.html#method.new) method.
  pub trait StyleOptionToolBarNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar>;
  }
  impl StyleOptionToolBarNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionToolBar_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionToolBarNewArgs for &'a ::style_option_tool_bar::StyleOptionToolBar {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tool_bar::StyleOptionToolBar> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionToolBar_new_other(other as *const ::style_option_tool_bar::StyleOptionToolBar)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
