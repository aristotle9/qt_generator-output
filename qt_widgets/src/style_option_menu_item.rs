/// C++ type: <span style='color: green;'>```QStyleOptionMenuItem::CheckType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CheckType {
  /// C++ enum variant: <span style='color: green;'>```NotCheckable = 0```</span>
  NotCheckable = 0,
  /// C++ enum variant: <span style='color: green;'>```Exclusive = 1```</span>
  Exclusive = 1,
  /// C++ enum variant: <span style='color: green;'>```NonExclusive = 2```</span>
  NonExclusive = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionMenuItem::MenuItemType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MenuItemType {
  /// C++ enum variant: <span style='color: green;'>```Normal = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```DefaultItem = 1```</span>
  DefaultItem = 1,
  /// C++ enum variant: <span style='color: green;'>```Separator = 2```</span>
  Separator = 2,
  /// C++ enum variant: <span style='color: green;'>```SubMenu = 3```</span>
  SubMenu = 3,
  /// C++ enum variant: <span style='color: green;'>```Scroller = 4```</span>
  Scroller = 4,
  /// C++ enum variant: <span style='color: green;'>```TearOff = 5```</span>
  TearOff = 5,
  /// C++ enum variant: <span style='color: green;'>```Margin = 6```</span>
  Margin = 6,
  /// C++ enum variant: <span style='color: green;'>```EmptyArea = 7```</span>
  EmptyArea = 7,
}

/// C++ type: <span style='color: green;'>```QStyleOptionMenuItem```</span>
#[repr(C)]
pub struct StyleOptionMenuItem(u8);

impl StyleOptionMenuItem {
  /// C++ method: <span style='color: green;'>```QStyleOptionMenuItem::CheckType QStyleOptionMenuItem::checkType() const```</span>
  ///
  ///
  pub fn check_type(&self) -> ::style_option_menu_item::CheckType {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_checkType(self as *const ::style_option_menu_item::StyleOptionMenuItem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionMenuItem::checked() const```</span>
  ///
  ///
  pub fn checked(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_checked(self as *const ::style_option_menu_item::StyleOptionMenuItem)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFont& QStyleOptionMenuItem::font() const```</span>
  ///
  ///
  pub fn font<'l0>(&'l0 self) -> &'l0 ::qt_gui::font::Font {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_font(self as *const ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFont& QStyleOptionMenuItem::font_mut()```</span>
  ///
  ///
  pub fn font_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::font::Font {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_font_mut(self as *mut ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionMenuItem::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_icon(self as *const ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionMenuItem::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_icon_mut(self as *mut ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionMenuItem::maxIconWidth() const```</span>
  ///
  ///
  pub fn max_icon_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_maxIconWidth(self as *const ::style_option_menu_item::StyleOptionMenuItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionMenuItem::menuHasCheckableItems() const```</span>
  ///
  ///
  pub fn menu_has_checkable_items(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_menuHasCheckableItems(self as *const ::style_option_menu_item::StyleOptionMenuItem) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionMenuItem::MenuItemType QStyleOptionMenuItem::menuItemType() const```</span>
  ///
  ///
  pub fn menu_item_type(&self) -> ::style_option_menu_item::MenuItemType {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_menuItemType(self as *const ::style_option_menu_item::StyleOptionMenuItem) }
  }

  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionMenuItem::menuRect() const```</span>
  ///
  ///
  pub fn menu_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_menuRect(self as *const ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionMenuItem::menuRect_mut()```</span>
  ///
  ///
  pub fn menu_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_menuRect_mut(self as *mut ::style_option_menu_item::StyleOptionMenuItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionMenuItem::QStyleOptionMenuItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionMenuItem::QStyleOptionMenuItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_menu_item::StyleOptionMenuItem) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionMenuItem::QStyleOptionMenuItem(const QStyleOptionMenuItem& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem>
    where Args: overloading::StyleOptionMenuItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_checkType(QStyleOptionMenuItem::CheckType value)```</span>
  ///
  ///
  pub fn set_check_type(&mut self, value: ::style_option_menu_item::CheckType) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_checkType(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_checked(bool value)```</span>
  ///
  ///
  pub fn set_checked(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_checked(self as *mut ::style_option_menu_item::StyleOptionMenuItem,
                                                           value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_font(QFont value)```</span>
  ///
  ///
  pub fn set_font(&mut self, value: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_font(self as *mut ::style_option_menu_item::StyleOptionMenuItem,
                                                        value as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_icon(self as *mut ::style_option_menu_item::StyleOptionMenuItem,
                                                        value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_maxIconWidth(int value)```</span>
  ///
  ///
  pub fn set_max_icon_width(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_maxIconWidth(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_menuHasCheckableItems(bool value)```</span>
  ///
  ///
  pub fn set_menu_has_checkable_items(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_menuHasCheckableItems(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_menuItemType(QStyleOptionMenuItem::MenuItemType value)```</span>
  ///
  ///
  pub fn set_menu_item_type(&mut self, value: ::style_option_menu_item::MenuItemType) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_menuItemType(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_menuRect(QRect value)```</span>
  ///
  ///
  pub fn set_menu_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_menuRect(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_tabWidth(int value)```</span>
  ///
  ///
  pub fn set_tab_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_tabWidth(self as *mut ::style_option_menu_item::StyleOptionMenuItem, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionMenuItem::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_set_text(self as *mut ::style_option_menu_item::StyleOptionMenuItem,
                                                        value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionMenuItem::tabWidth() const```</span>
  ///
  ///
  pub fn tab_width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionMenuItem_tabWidth(self as *const ::style_option_menu_item::StyleOptionMenuItem)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionMenuItem::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_text(self as *const ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionMenuItem::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionMenuItem_text_mut(self as *mut ::style_option_menu_item::StyleOptionMenuItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_menu_item::StyleOptionMenuItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionMenuItem_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionMenuItem::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 4```</span>
  Type = 4,
}

/// C++ type: <span style='color: green;'>```QStyleOptionMenuItem::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_menu_item::StyleOptionMenuItem {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_menu_item::StyleOptionMenuItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_menu_item::StyleOptionMenuItem as *mut ::style_option_menu_item::StyleOptionMenuItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_menu_item::StyleOptionMenuItem> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_menu_item::StyleOptionMenuItem {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOptionMenuItem_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_menu_item::StyleOptionMenuItem {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOptionMenuItem_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_menu_item::StyleOptionMenuItem {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_menu_item::StyleOptionMenuItem as *mut ::style_option_menu_item::StyleOptionMenuItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_menu_item::StyleOptionMenuItem {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_menu_item::StyleOptionMenuItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionMenuItem::new](../struct.StyleOptionMenuItem.html#method.new) method.
  pub trait StyleOptionMenuItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem>;
  }
  impl StyleOptionMenuItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionMenuItemNewArgs for &'a ::style_option_menu_item::StyleOptionMenuItem {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_menu_item::StyleOptionMenuItem> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionMenuItem_new_other(other as *const ::style_option_menu_item::StyleOptionMenuItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
