/// C++ type: <span style='color: green;'>```QStyleOptionGroupBox```</span>
#[repr(C)]
pub struct StyleOptionGroupBox(u8);

impl StyleOptionGroupBox {
  /// C++ method: <span style='color: green;'>```int QStyleOptionGroupBox::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionGroupBox_lineWidth(self as *const ::style_option_group_box::StyleOptionGroupBox)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionGroupBox::midLineWidth() const```</span>
  ///
  ///
  pub fn mid_line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_midLineWidth(self as *const ::style_option_group_box::StyleOptionGroupBox) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionGroupBox::QStyleOptionGroupBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionGroupBox::QStyleOptionGroupBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_group_box::StyleOptionGroupBox) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionGroupBox::QStyleOptionGroupBox(const QStyleOptionGroupBox& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox>
    where Args: overloading::StyleOptionGroupBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionGroupBox::set_lineWidth(int value)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionGroupBox_set_lineWidth(self as *mut ::style_option_group_box::StyleOptionGroupBox, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionGroupBox::set_midLineWidth(int value)```</span>
  ///
  ///
  pub fn set_mid_line_width(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_set_midLineWidth(self as *mut ::style_option_group_box::StyleOptionGroupBox, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionGroupBox::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionGroupBox_set_text(self as *mut ::style_option_group_box::StyleOptionGroupBox,
                                                        value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionGroupBox::set_textColor(QColor value)```</span>
  ///
  ///
  pub fn set_text_color(&mut self, value: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionGroupBox_set_textColor(self as *mut ::style_option_group_box::StyleOptionGroupBox, value as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionGroupBox::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionGroupBox_text(self as *const ::style_option_group_box::StyleOptionGroupBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QColor& QStyleOptionGroupBox::textColor() const```</span>
  ///
  ///
  pub fn text_color<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionGroupBox_textColor(self as *const ::style_option_group_box::StyleOptionGroupBox)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QStyleOptionGroupBox::textColor_mut()```</span>
  ///
  ///
  pub fn text_color_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_textColor_mut(self as *mut ::style_option_group_box::StyleOptionGroupBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionGroupBox::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionGroupBox_text_mut(self as *mut ::style_option_group_box::StyleOptionGroupBox)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_group_box::StyleOptionGroupBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionGroupBox_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionGroupBox::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983046```</span>
  Type = 983046,
}

/// C++ type: <span style='color: green;'>```QStyleOptionGroupBox::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_group_box::StyleOptionGroupBox {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_group_box::StyleOptionGroupBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOption_ptr(self as *const ::style_option_group_box::StyleOptionGroupBox as *mut ::style_option_group_box::StyleOptionGroupBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_group_box::StyleOptionGroupBox {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_group_box::StyleOptionGroupBox) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_group_box::StyleOptionGroupBox as *mut ::style_option_group_box::StyleOptionGroupBox) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_group_box::StyleOptionGroupBox> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_group_box::StyleOptionGroupBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_group_box::StyleOptionGroupBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_group_box::StyleOptionGroupBox> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_group_box::StyleOptionGroupBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_group_box::StyleOptionGroupBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionGroupBox_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_group_box::StyleOptionGroupBox {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_group_box::StyleOptionGroupBox as *mut ::style_option_group_box::StyleOptionGroupBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_group_box::StyleOptionGroupBox {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_group_box::StyleOptionGroupBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionGroupBox::new](../struct.StyleOptionGroupBox.html#method.new) method.
  pub trait StyleOptionGroupBoxNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox>;
  }
  impl StyleOptionGroupBoxNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionGroupBoxNewArgs for &'a ::style_option_group_box::StyleOptionGroupBox {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_group_box::StyleOptionGroupBox> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGroupBox_new_other(other as *const ::style_option_group_box::StyleOptionGroupBox) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
