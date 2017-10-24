/// C++ type: <span style='color: green;'>```QStyleOptionTab::CornerWidget```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CornerWidget {
  /// C++ enum variant: <span style='color: green;'>```NoCornerWidgets = 0```</span>
  NoCornerWidgets = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftCornerWidget = 1```</span>
  LeftCornerWidget = 1,
  /// C++ enum variant: <span style='color: green;'>```RightCornerWidget = 2```</span>
  RightCornerWidget = 2,
}

impl ::qt_core::flags::FlaggableEnum for CornerWidget {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "CornerWidget"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab::SelectedPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SelectedPosition {
  /// C++ enum variant: <span style='color: green;'>```NotAdjacent = 0```</span>
  NotAdjacent = 0,
  /// C++ enum variant: <span style='color: green;'>```NextIsSelected = 1```</span>
  NextIsSelected = 1,
  /// C++ enum variant: <span style='color: green;'>```PreviousIsSelected = 2```</span>
  PreviousIsSelected = 2,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab```</span>
#[repr(C)]
pub struct StyleOptionTab(u8);

impl StyleOptionTab {
  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionTab::CornerWidget>& QStyleOptionTab::cornerWidgets() const```</span>
  ///
  ///
  pub fn corner_widgets(&self) -> ::qt_core::flags::Flags<::style_option_tab::CornerWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_cornerWidgets(self as *const ::style_option_tab::StyleOptionTab) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionTab::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_documentMode(self as *const ::style_option_tab::StyleOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionTab::TabFeature>& QStyleOptionTab::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_tab::TabFeature> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_features(self as *const ::style_option_tab::StyleOptionTab) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionTab::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_icon(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionTab::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_icon_mut(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTab::iconSize() const```</span>
  ///
  ///
  pub fn icon_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_iconSize(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTab::iconSize_mut()```</span>
  ///
  ///
  pub fn icon_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_iconSize_mut(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTab::leftButtonSize() const```</span>
  ///
  ///
  pub fn left_button_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_leftButtonSize(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTab::leftButtonSize_mut()```</span>
  ///
  ///
  pub fn left_button_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTab_leftButtonSize_mut(self as *mut ::style_option_tab::StyleOptionTab)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionTab::QStyleOptionTab```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTab::QStyleOptionTab()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tab::StyleOptionTab) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTab::QStyleOptionTab(const QStyleOptionTab& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab>
    where Args: overloading::StyleOptionTabNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStyleOptionTab::TabPosition QStyleOptionTab::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::style_option_tab::TabPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_position(self as *const ::style_option_tab::StyleOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTab::rightButtonSize() const```</span>
  ///
  ///
  pub fn right_button_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_rightButtonSize(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTab::rightButtonSize_mut()```</span>
  ///
  ///
  pub fn right_button_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTab_rightButtonSize_mut(self as *mut ::style_option_tab::StyleOptionTab)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionTab::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_row(self as *const ::style_option_tab::StyleOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionTab::SelectedPosition QStyleOptionTab::selectedPosition() const```</span>
  ///
  ///
  pub fn selected_position(&self) -> ::style_option_tab::SelectedPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_selectedPosition(self as *const ::style_option_tab::StyleOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_cornerWidgets(QFlags<QStyleOptionTab::CornerWidget> value)```</span>
  ///
  ///
  pub fn set_corner_widgets(&mut self, value: ::qt_core::flags::Flags<::style_option_tab::CornerWidget>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_cornerWidgets(self as *mut ::style_option_tab::StyleOptionTab,
                                                            value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_documentMode(bool value)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_documentMode(self as *mut ::style_option_tab::StyleOptionTab, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_features(QFlags<QStyleOptionTab::TabFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_tab::TabFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_features(self as *mut ::style_option_tab::StyleOptionTab,
                                                       value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_icon(self as *mut ::style_option_tab::StyleOptionTab,
                                                   value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_iconSize(QSize value)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_iconSize(self as *mut ::style_option_tab::StyleOptionTab,
                                                       value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_leftButtonSize(QSize value)```</span>
  ///
  ///
  pub fn set_left_button_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_leftButtonSize(self as *mut ::style_option_tab::StyleOptionTab,
                                                             value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_position(QStyleOptionTab::TabPosition value)```</span>
  ///
  ///
  pub fn set_position(&mut self, value: ::style_option_tab::TabPosition) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_set_position(self as *mut ::style_option_tab::StyleOptionTab, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_rightButtonSize(QSize value)```</span>
  ///
  ///
  pub fn set_right_button_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_rightButtonSize(self as *mut ::style_option_tab::StyleOptionTab,
                                                              value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_row(int value)```</span>
  ///
  ///
  pub fn set_row(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_set_row(self as *mut ::style_option_tab::StyleOptionTab, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_selectedPosition(QStyleOptionTab::SelectedPosition value)```</span>
  ///
  ///
  pub fn set_selected_position(&mut self, value: ::style_option_tab::SelectedPosition) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_selectedPosition(self as *mut ::style_option_tab::StyleOptionTab, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_shape(QTabBar::Shape value)```</span>
  ///
  ///
  pub fn set_shape(&mut self, value: &::tab_bar::Shape) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_shape(self as *mut ::style_option_tab::StyleOptionTab,
                                                    value as *const ::tab_bar::Shape)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTab::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTab_set_text(self as *mut ::style_option_tab::StyleOptionTab,
                                                   value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTabBar::Shape& QStyleOptionTab::shape() const```</span>
  ///
  ///
  pub fn shape<'l0>(&'l0 self) -> &'l0 ::tab_bar::Shape {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_shape(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTabBar::Shape& QStyleOptionTab::shape_mut()```</span>
  ///
  ///
  pub fn shape_mut<'l0>(&'l0 mut self) -> &'l0 mut ::tab_bar::Shape {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_shape_mut(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionTab::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_text(self as *const ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionTab::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_text_mut(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_tab::StyleOptionTab {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionTab_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 3```</span>
  Type = 3,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 3```</span>
  Version = 3,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab::TabFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```HasFrame = 1```</span>
  HasFrame = 1,
}

impl ::qt_core::flags::FlaggableEnum for TabFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "TabFeature"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTab::TabPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TabPosition {
  /// C++ enum variant: <span style='color: green;'>```Beginning = 0```</span>
  Beginning = 0,
  /// C++ enum variant: <span style='color: green;'>```Middle = 1```</span>
  Middle = 1,
  /// C++ enum variant: <span style='color: green;'>```End = 2```</span>
  End = 2,
  /// C++ enum variant: <span style='color: green;'>```OnlyOneTab = 3```</span>
  OnlyOneTab = 3,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tab::StyleOptionTab {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab::StyleOptionTab as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tab::StyleOptionTab> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tab::StyleOptionTab {
    let ffi_result =
      ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOptionTab_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_tab::StyleOptionTab {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOptionTab_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_tab::StyleOptionTab {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab::StyleOptionTab as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tab::StyleOptionTab {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab::StyleOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionTab::new](../struct.StyleOptionTab.html#method.new) method.
  pub trait StyleOptionTabNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab>;
  }
  impl StyleOptionTabNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionTabNewArgs for &'a ::style_option_tab::StyleOptionTab {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab::StyleOptionTab> {
      let other = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QStyleOptionTab_new_other(other as *const ::style_option_tab::StyleOptionTab) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
