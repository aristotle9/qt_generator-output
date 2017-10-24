/// C++ type: <span style='color: green;'>```QStyleOptionViewItem::Position```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Position {
  /// C++ enum variant: <span style='color: green;'>```Left = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```Right = 1```</span>
  Right = 1,
  /// C++ enum variant: <span style='color: green;'>```Top = 2```</span>
  Top = 2,
  /// C++ enum variant: <span style='color: green;'>```Bottom = 3```</span>
  Bottom = 3,
}

/// C++ type: <span style='color: green;'>```QStyleOptionViewItem::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 10```</span>
  Type = 10,
}

/// C++ type: <span style='color: green;'>```QStyleOptionViewItem::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 4```</span>
  Version = 4,
}

/// C++ type: <span style='color: green;'>```QStyleOptionViewItem```</span>
#[repr(C)]
pub struct StyleOptionViewItem(u8);

impl StyleOptionViewItem {
  /// C++ method: <span style='color: green;'>```const QBrush& QStyleOptionViewItem::backgroundBrush() const```</span>
  ///
  ///
  pub fn background_brush<'l0>(&'l0 self) -> &'l0 ::qt_gui::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_backgroundBrush(self as *const ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QBrush& QStyleOptionViewItem::backgroundBrush_mut()```</span>
  ///
  ///
  pub fn background_brush_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_backgroundBrush_mut(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::CheckState& QStyleOptionViewItem::checkState() const```</span>
  ///
  ///
  pub fn check_state<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::CheckState {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_checkState(self as *const ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::CheckState& QStyleOptionViewItem::checkState_mut()```</span>
  ///
  ///
  pub fn check_state_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::CheckState {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_checkState_mut(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionViewItem::Position QStyleOptionViewItem::decorationPosition() const```</span>
  ///
  ///
  pub fn decoration_position(&self) -> ::style_option_view_item::Position {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_decorationPosition(self as *const ::style_option_view_item::StyleOptionViewItem) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionViewItem::decorationSize() const```</span>
  ///
  ///
  pub fn decoration_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_decorationSize(self as *const ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionViewItem::decorationSize_mut()```</span>
  ///
  ///
  pub fn decoration_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_decorationSize_mut(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFlags<QStyleOptionViewItem::ViewItemFeature>& QStyleOptionViewItem::features() const```</span>
  ///
  ///
  pub fn features(&self) -> ::qt_core::flags::Flags<::style_option_view_item::ViewItemFeature> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_features(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```const QFont& QStyleOptionViewItem::font() const```</span>
  ///
  ///
  pub fn font<'l0>(&'l0 self) -> &'l0 ::qt_gui::font::Font {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_font(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFont& QStyleOptionViewItem::font_mut()```</span>
  ///
  ///
  pub fn font_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::font::Font {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_font_mut(self as *mut ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionViewItem::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_icon(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionViewItem::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_icon_mut(self as *mut ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QStyleOptionViewItem::index() const```</span>
  ///
  ///
  pub fn index<'l0>(&'l0 self) -> &'l0 ::qt_core::model_index::ModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_index(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex& QStyleOptionViewItem::index_mut()```</span>
  ///
  ///
  pub fn index_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::model_index::ModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_index_mut(self as *mut ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QStyleOptionViewItem::locale() const```</span>
  ///
  ///
  pub fn locale<'l0>(&'l0 self) -> &'l0 ::qt_core::locale::Locale {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_locale(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale& QStyleOptionViewItem::locale_mut()```</span>
  ///
  ///
  pub fn locale_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::locale::Locale {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_locale_mut(self as *mut ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionViewItem::QStyleOptionViewItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionViewItem::QStyleOptionViewItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_view_item::StyleOptionViewItem) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionViewItem::QStyleOptionViewItem(const QStyleOptionViewItem& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem>
    where Args: overloading::StyleOptionViewItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_backgroundBrush(QBrush value)```</span>
  ///
  ///
  pub fn set_background_brush(&mut self, value: &::qt_gui::brush::Brush) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_backgroundBrush(self as *mut ::style_option_view_item::StyleOptionViewItem, value as *const ::qt_gui::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_checkState(Qt::CheckState value)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, value: &::qt_core::qt::CheckState) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_checkState(self as *mut ::style_option_view_item::StyleOptionViewItem, value as *const ::qt_core::qt::CheckState) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_decorationPosition(QStyleOptionViewItem::Position value)```</span>
  ///
  ///
  pub fn set_decoration_position(&mut self, value: ::style_option_view_item::Position) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_decorationPosition(self as *mut ::style_option_view_item::StyleOptionViewItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_decorationSize(QSize value)```</span>
  ///
  ///
  pub fn set_decoration_size(&mut self, value: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_decorationSize(self as *mut ::style_option_view_item::StyleOptionViewItem, value as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_features(QFlags<QStyleOptionViewItem::ViewItemFeature> value)```</span>
  ///
  ///
  pub fn set_features(&mut self, value: ::qt_core::flags::Flags<::style_option_view_item::ViewItemFeature>) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_features(self as *mut ::style_option_view_item::StyleOptionViewItem, value.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_font(QFont value)```</span>
  ///
  ///
  pub fn set_font(&mut self, value: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_font(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                        value as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_icon(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                        value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_index(QModelIndex value)```</span>
  ///
  ///
  pub fn set_index(&mut self, value: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_index(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                         value as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_locale(QLocale value)```</span>
  ///
  ///
  pub fn set_locale(&mut self, value: &::qt_core::locale::Locale) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_locale(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                          value as *const ::qt_core::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_showDecorationSelected(bool value)```</span>
  ///
  ///
  pub fn set_show_decoration_selected(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_showDecorationSelected(self as *mut ::style_option_view_item::StyleOptionViewItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_set_text(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                        value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_textElideMode(Qt::TextElideMode value)```</span>
  ///
  ///
  pub fn set_text_elide_mode(&mut self, value: &::qt_core::qt::TextElideMode) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_textElideMode(self as *mut ::style_option_view_item::StyleOptionViewItem, value as *const ::qt_core::qt::TextElideMode) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_viewItemPosition(QStyleOptionViewItem::ViewItemPosition value)```</span>
  ///
  ///
  pub fn set_view_item_position(&mut self, value: ::style_option_view_item::ViewItemPosition) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_set_viewItemPosition(self as *mut ::style_option_view_item::StyleOptionViewItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionViewItem::set_widget(const QWidget* value)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, value: *const ::widget::Widget) {
    ::ffi::qt_widgets_c_QStyleOptionViewItem_set_widget(self as *mut ::style_option_view_item::StyleOptionViewItem,
                                                        value)
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionViewItem::showDecorationSelected() const```</span>
  ///
  ///
  pub fn show_decoration_selected(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_showDecorationSelected(self as *const ::style_option_view_item::StyleOptionViewItem) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionViewItem::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_text(self as *const ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::TextElideMode& QStyleOptionViewItem::textElideMode() const```</span>
  ///
  ///
  pub fn text_elide_mode<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::TextElideMode {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_textElideMode(self as *const ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::TextElideMode& QStyleOptionViewItem::textElideMode_mut()```</span>
  ///
  ///
  pub fn text_elide_mode_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::TextElideMode {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_textElideMode_mut(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionViewItem::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionViewItem_text_mut(self as *mut ::style_option_view_item::StyleOptionViewItem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionViewItem::ViewItemPosition QStyleOptionViewItem::viewItemPosition() const```</span>
  ///
  ///
  pub fn view_item_position(&self) -> ::style_option_view_item::ViewItemPosition {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_viewItemPosition(self as *const ::style_option_view_item::StyleOptionViewItem) }
  }

  /// C++ method: <span style='color: green;'>```const QWidget* QStyleOptionViewItem::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *const ::widget::Widget {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionViewItem_widget(self as *const ::style_option_view_item::StyleOptionViewItem)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_view_item::StyleOptionViewItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionViewItem_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionViewItem::ViewItemFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewItemFeature {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```WrapText = 1```</span>
  WrapText = 1,
  /// C++ enum variant: <span style='color: green;'>```Alternate = 2```</span>
  Alternate = 2,
  /// C++ enum variant: <span style='color: green;'>```HasCheckIndicator = 4```</span>
  HasCheckIndicator = 4,
  /// C++ enum variant: <span style='color: green;'>```HasDisplay = 8```</span>
  HasDisplay = 8,
  /// C++ enum variant: <span style='color: green;'>```HasDecoration = 16```</span>
  HasDecoration = 16,
}

impl ::qt_core::flags::FlaggableEnum for ViewItemFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ViewItemFeature"
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionViewItem::ViewItemPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ViewItemPosition {
  /// C++ enum variant: <span style='color: green;'>```Invalid = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Beginning = 1```</span>
  Beginning = 1,
  /// C++ enum variant: <span style='color: green;'>```Middle = 2```</span>
  Middle = 2,
  /// C++ enum variant: <span style='color: green;'>```End = 3```</span>
  End = 3,
  /// C++ enum variant: <span style='color: green;'>```OnlyOne = 4```</span>
  OnlyOne = 4,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_view_item::StyleOptionViewItem {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_view_item::StyleOptionViewItem as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_view_item::StyleOptionViewItem> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_view_item::StyleOptionViewItem {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOptionViewItem_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_view_item::StyleOptionViewItem {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOptionViewItem_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_view_item::StyleOptionViewItem {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_view_item::StyleOptionViewItem as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_view_item::StyleOptionViewItem {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_view_item::StyleOptionViewItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionViewItem::new](../struct.StyleOptionViewItem.html#method.new) method.
  pub trait StyleOptionViewItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem>;
  }
  impl StyleOptionViewItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionViewItemNewArgs for &'a ::style_option_view_item::StyleOptionViewItem {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_view_item::StyleOptionViewItem> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionViewItem_new_other(other as *const ::style_option_view_item::StyleOptionViewItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
