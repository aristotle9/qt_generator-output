/// C++ type: <span style='color: green;'>```QStyleOptionProgressBar```</span>
#[repr(C)]
pub struct StyleOptionProgressBar(u8);

impl StyleOptionProgressBar {
  /// C++ method: <span style='color: green;'>```bool QStyleOptionProgressBar::bottomToTop() const```</span>
  ///
  ///
  pub fn bottom_to_top(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_bottomToTop(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionProgressBar::invertedAppearance() const```</span>
  ///
  ///
  pub fn inverted_appearance(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_invertedAppearance(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionProgressBar::maximum() const```</span>
  ///
  ///
  pub fn maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_maximum(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionProgressBar::minimum() const```</span>
  ///
  ///
  pub fn minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_minimum(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionProgressBar::QStyleOptionProgressBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionProgressBar::QStyleOptionProgressBar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_progress_bar::StyleOptionProgressBar) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionProgressBar::QStyleOptionProgressBar(const QStyleOptionProgressBar& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar>
    where Args: overloading::StyleOptionProgressBarNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const Qt::Orientation& QStyleOptionProgressBar::orientation() const```</span>
  ///
  ///
  pub fn orientation<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::Orientation {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_orientation(self as *const ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::Orientation& QStyleOptionProgressBar::orientation_mut()```</span>
  ///
  ///
  pub fn orientation_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::Orientation {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_orientation_mut(self as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionProgressBar::progress() const```</span>
  ///
  ///
  pub fn progress(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_progress(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_bottomToTop(bool value)```</span>
  ///
  ///
  pub fn set_bottom_to_top(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_bottomToTop(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_invertedAppearance(bool value)```</span>
  ///
  ///
  pub fn set_inverted_appearance(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_invertedAppearance(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_maximum(int value)```</span>
  ///
  ///
  pub fn set_maximum(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_maximum(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_minimum(int value)```</span>
  ///
  ///
  pub fn set_minimum(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_minimum(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_orientation(Qt::Orientation value)```</span>
  ///
  ///
  pub fn set_orientation(&mut self, value: &::qt_core::qt::Orientation) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_orientation(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value as *const ::qt_core::qt::Orientation) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_progress(int value)```</span>
  ///
  ///
  pub fn set_progress(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_progress(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_text(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionProgressBar::set_textVisible(bool value)```</span>
  ///
  ///
  pub fn set_text_visible(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_set_textVisible(self as *mut ::style_option_progress_bar::StyleOptionProgressBar, value) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionProgressBar::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_text(self as *const ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionProgressBar::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_text_mut(self as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionProgressBar::textVisible() const```</span>
  ///
  ///
  pub fn text_visible(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_textVisible(self as *const ::style_option_progress_bar::StyleOptionProgressBar) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_progress_bar::StyleOptionProgressBar {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionProgressBar_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionProgressBar::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 6```</span>
  Type = 6,
}

/// C++ type: <span style='color: green;'>```QStyleOptionProgressBar::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 2```</span>
  Version = 2,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_progress_bar::StyleOptionProgressBar {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(self as *const ::style_option_progress_bar::StyleOptionProgressBar as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_progress_bar::StyleOptionProgressBar> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_progress_bar::StyleOptionProgressBar {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOptionProgressBar_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_progress_bar::StyleOptionProgressBar {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOptionProgressBar_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_progress_bar::StyleOptionProgressBar {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(self as *const ::style_option_progress_bar::StyleOptionProgressBar as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_progress_bar::StyleOptionProgressBar {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_progress_bar::StyleOptionProgressBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionProgressBar::new](../struct.StyleOptionProgressBar.html#method.new) method.
  pub trait StyleOptionProgressBarNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar>;
  }
  impl StyleOptionProgressBarNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionProgressBarNewArgs for &'a ::style_option_progress_bar::StyleOptionProgressBar {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_progress_bar::StyleOptionProgressBar> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionProgressBar_new_other(other as *const ::style_option_progress_bar::StyleOptionProgressBar) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
