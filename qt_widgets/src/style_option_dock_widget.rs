/// C++ type: <span style='color: green;'>```QStyleOptionDockWidget```</span>
#[repr(C)]
pub struct StyleOptionDockWidget(u8);

impl StyleOptionDockWidget {
  /// C++ method: <span style='color: green;'>```bool QStyleOptionDockWidget::closable() const```</span>
  ///
  ///
  pub fn closable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_closable(self as *const ::style_option_dock_widget::StyleOptionDockWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionDockWidget::floatable() const```</span>
  ///
  ///
  pub fn floatable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_floatable(self as *const ::style_option_dock_widget::StyleOptionDockWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionDockWidget::movable() const```</span>
  ///
  ///
  pub fn movable(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_movable(self as *const ::style_option_dock_widget::StyleOptionDockWidget) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionDockWidget::QStyleOptionDockWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionDockWidget::QStyleOptionDockWidget()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_dock_widget::StyleOptionDockWidget) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionDockWidget::QStyleOptionDockWidget(const QStyleOptionDockWidget& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget>
    where Args: overloading::StyleOptionDockWidgetNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionDockWidget::set_closable(bool value)```</span>
  ///
  ///
  pub fn set_closable(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_set_closable(self as *mut ::style_option_dock_widget::StyleOptionDockWidget, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionDockWidget::set_floatable(bool value)```</span>
  ///
  ///
  pub fn set_floatable(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_set_floatable(self as *mut ::style_option_dock_widget::StyleOptionDockWidget, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionDockWidget::set_movable(bool value)```</span>
  ///
  ///
  pub fn set_movable(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_set_movable(self as *mut ::style_option_dock_widget::StyleOptionDockWidget, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionDockWidget::set_title(QString value)```</span>
  ///
  ///
  pub fn set_title(&mut self, value: &::qt_core::string::String) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_set_title(self as *mut ::style_option_dock_widget::StyleOptionDockWidget, value as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionDockWidget::set_verticalTitleBar(bool value)```</span>
  ///
  ///
  pub fn set_vertical_title_bar(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_set_verticalTitleBar(self as *mut ::style_option_dock_widget::StyleOptionDockWidget, value) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionDockWidget::title() const```</span>
  ///
  ///
  pub fn title<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_title(self as *const ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionDockWidget::title_mut()```</span>
  ///
  ///
  pub fn title_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_title_mut(self as *mut ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionDockWidget::verticalTitleBar() const```</span>
  ///
  ///
  pub fn vertical_title_bar(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_verticalTitleBar(self as *const ::style_option_dock_widget::StyleOptionDockWidget) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_dock_widget::StyleOptionDockWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionDockWidget_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionDockWidget::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 9```</span>
  Type = 9,
}

/// C++ type: <span style='color: green;'>```QStyleOptionDockWidget::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 2```</span>
  Version = 2,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_dock_widget::StyleOptionDockWidget {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOption_ptr(self as *const ::style_option_dock_widget::StyleOptionDockWidget as *mut ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_dock_widget::StyleOptionDockWidget> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_dock_widget::StyleOptionDockWidget {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOptionDockWidget_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_dock_widget::StyleOptionDockWidget {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOptionDockWidget_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_dock_widget::StyleOptionDockWidget {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOption_ptr(self as *const ::style_option_dock_widget::StyleOptionDockWidget as *mut ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_dock_widget::StyleOptionDockWidget {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_dock_widget::StyleOptionDockWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionDockWidget::new](../struct.StyleOptionDockWidget.html#method.new) method.
  pub trait StyleOptionDockWidgetNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget>;
  }
  impl StyleOptionDockWidgetNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionDockWidgetNewArgs for &'a ::style_option_dock_widget::StyleOptionDockWidget {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_dock_widget::StyleOptionDockWidget> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionDockWidget_new_other(other as *const ::style_option_dock_widget::StyleOptionDockWidget) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
