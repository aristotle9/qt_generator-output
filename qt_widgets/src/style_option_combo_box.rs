/// C++ type: <span style='color: green;'>```QStyleOptionComboBox```</span>
#[repr(C)]
pub struct StyleOptionComboBox(u8);

impl StyleOptionComboBox {
  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionComboBox::currentIcon() const```</span>
  ///
  ///
  pub fn current_icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_currentIcon(self as *const ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionComboBox::currentIcon_mut()```</span>
  ///
  ///
  pub fn current_icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_currentIcon_mut(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionComboBox::currentText() const```</span>
  ///
  ///
  pub fn current_text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_currentText(self as *const ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionComboBox::currentText_mut()```</span>
  ///
  ///
  pub fn current_text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_currentText_mut(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionComboBox::editable() const```</span>
  ///
  ///
  pub fn editable(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_editable(self as *const ::style_option_combo_box::StyleOptionComboBox)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionComboBox::frame() const```</span>
  ///
  ///
  pub fn frame(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_frame(self as *const ::style_option_combo_box::StyleOptionComboBox)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionComboBox::iconSize() const```</span>
  ///
  ///
  pub fn icon_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionComboBox_iconSize(self as *const ::style_option_combo_box::StyleOptionComboBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionComboBox::iconSize_mut()```</span>
  ///
  ///
  pub fn icon_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_iconSize_mut(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionComboBox::QStyleOptionComboBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComboBox::QStyleOptionComboBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_combo_box::StyleOptionComboBox) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComboBox::QStyleOptionComboBox(const QStyleOptionComboBox& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox>
    where Args: overloading::StyleOptionComboBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionComboBox::popupRect() const```</span>
  ///
  ///
  pub fn popup_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionComboBox_popupRect(self as *const ::style_option_combo_box::StyleOptionComboBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionComboBox::popupRect_mut()```</span>
  ///
  ///
  pub fn popup_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_popupRect_mut(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_currentIcon(QIcon value)```</span>
  ///
  ///
  pub fn set_current_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_set_currentIcon(self as *mut ::style_option_combo_box::StyleOptionComboBox, value as *const ::qt_gui::icon::Icon) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_currentText(QString value)```</span>
  ///
  ///
  pub fn set_current_text(&mut self, value: &::qt_core::string::String) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_set_currentText(self as *mut ::style_option_combo_box::StyleOptionComboBox, value as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_editable(bool value)```</span>
  ///
  ///
  pub fn set_editable(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_set_editable(self as *mut ::style_option_combo_box::StyleOptionComboBox, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_frame(bool value)```</span>
  ///
  ///
  pub fn set_frame(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_set_frame(self as *mut ::style_option_combo_box::StyleOptionComboBox,
                                                         value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_iconSize(QSize value)```</span>
  ///
  ///
  pub fn set_icon_size(&mut self, value: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_set_iconSize(self as *mut ::style_option_combo_box::StyleOptionComboBox, value as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionComboBox::set_popupRect(QRect value)```</span>
  ///
  ///
  pub fn set_popup_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionComboBox_set_popupRect(self as *mut ::style_option_combo_box::StyleOptionComboBox, value as *const ::qt_core::rect::Rect)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_combo_box::StyleOptionComboBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionComboBox_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionComboBox::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983044```</span>
  Type = 983044,
}

/// C++ type: <span style='color: green;'>```QStyleOptionComboBox::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_combo_box::StyleOptionComboBox {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOption_ptr(self as *const ::style_option_combo_box::StyleOptionComboBox as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_combo_box::StyleOptionComboBox {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_combo_box::StyleOptionComboBox as *mut ::style_option_combo_box::StyleOptionComboBox) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_combo_box::StyleOptionComboBox> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_combo_box::StyleOptionComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_combo_box::StyleOptionComboBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_combo_box::StyleOptionComboBox> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_combo_box::StyleOptionComboBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_combo_box::StyleOptionComboBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComboBox_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_combo_box::StyleOptionComboBox {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_combo_box::StyleOptionComboBox as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_combo_box::StyleOptionComboBox {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_combo_box::StyleOptionComboBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionComboBox::new](../struct.StyleOptionComboBox.html#method.new) method.
  pub trait StyleOptionComboBoxNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox>;
  }
  impl StyleOptionComboBoxNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionComboBoxNewArgs for &'a ::style_option_combo_box::StyleOptionComboBox {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_combo_box::StyleOptionComboBox> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComboBox_new_other(other as *const ::style_option_combo_box::StyleOptionComboBox) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
