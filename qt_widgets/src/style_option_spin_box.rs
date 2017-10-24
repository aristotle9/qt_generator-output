/// C++ type: <span style='color: green;'>```QStyleOptionSpinBox```</span>
#[repr(C)]
pub struct StyleOptionSpinBox(u8);

impl StyleOptionSpinBox {
  /// C++ method: <span style='color: green;'>```const QAbstractSpinBox::ButtonSymbols& QStyleOptionSpinBox::buttonSymbols() const```</span>
  ///
  ///
  pub fn button_symbols<'l0>(&'l0 self) -> &'l0 ::abstract_spin_box::ButtonSymbols {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_buttonSymbols(self as *const ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractSpinBox::ButtonSymbols& QStyleOptionSpinBox::buttonSymbols_mut()```</span>
  ///
  ///
  pub fn button_symbols_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_spin_box::ButtonSymbols {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_buttonSymbols_mut(self as *mut ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QStyleOptionSpinBox::frame() const```</span>
  ///
  ///
  pub fn frame(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_frame(self as *const ::style_option_spin_box::StyleOptionSpinBox) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionSpinBox::QStyleOptionSpinBox```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSpinBox::QStyleOptionSpinBox()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_spin_box::StyleOptionSpinBox) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSpinBox::QStyleOptionSpinBox(const QStyleOptionSpinBox& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox>
    where Args: overloading::StyleOptionSpinBoxNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionSpinBox::set_buttonSymbols(QAbstractSpinBox::ButtonSymbols value)```</span>
  ///
  ///
  pub fn set_button_symbols(&mut self, value: &::abstract_spin_box::ButtonSymbols) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_set_buttonSymbols(self as *mut ::style_option_spin_box::StyleOptionSpinBox, value as *const ::abstract_spin_box::ButtonSymbols) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionSpinBox::set_frame(bool value)```</span>
  ///
  ///
  pub fn set_frame(&mut self, value: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSpinBox_set_frame(self as *mut ::style_option_spin_box::StyleOptionSpinBox,
                                                        value)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_spin_box::StyleOptionSpinBox {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionSpinBox_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionSpinBox::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983042```</span>
  Type = 983042,
}

/// C++ type: <span style='color: green;'>```QStyleOptionSpinBox::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_spin_box::StyleOptionSpinBox {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOption_ptr(self as *const ::style_option_spin_box::StyleOptionSpinBox as *mut ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_spin_box::StyleOptionSpinBox {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_spin_box::StyleOptionSpinBox) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_spin_box::StyleOptionSpinBox as *mut ::style_option_spin_box::StyleOptionSpinBox) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_spin_box::StyleOptionSpinBox> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_spin_box::StyleOptionSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_spin_box::StyleOptionSpinBox {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_spin_box::StyleOptionSpinBox> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_spin_box::StyleOptionSpinBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_spin_box::StyleOptionSpinBox {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionSpinBox_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_spin_box::StyleOptionSpinBox {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_spin_box::StyleOptionSpinBox as *mut ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_spin_box::StyleOptionSpinBox {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_spin_box::StyleOptionSpinBox) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionSpinBox::new](../struct.StyleOptionSpinBox.html#method.new) method.
  pub trait StyleOptionSpinBoxNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox>;
  }
  impl StyleOptionSpinBoxNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSpinBox_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionSpinBoxNewArgs for &'a ::style_option_spin_box::StyleOptionSpinBox {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_spin_box::StyleOptionSpinBox> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionSpinBox_new_other(other as *const ::style_option_spin_box::StyleOptionSpinBox)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
