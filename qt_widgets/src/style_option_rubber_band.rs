/// C++ type: <span style='color: green;'>```QStyleOptionRubberBand```</span>
#[repr(C)]
pub struct StyleOptionRubberBand(u8);

impl StyleOptionRubberBand {
  /// C++ method: <span style='color: green;'>```QStyleOptionRubberBand::QStyleOptionRubberBand```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionRubberBand::QStyleOptionRubberBand()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_rubber_band::StyleOptionRubberBand) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionRubberBand::QStyleOptionRubberBand(const QStyleOptionRubberBand& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand>
    where Args: overloading::StyleOptionRubberBandNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QStyleOptionRubberBand::opaque() const```</span>
  ///
  ///
  pub fn opaque(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_opaque(self as *const ::style_option_rubber_band::StyleOptionRubberBand) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionRubberBand::set_opaque(bool value)```</span>
  ///
  ///
  pub fn set_opaque(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_set_opaque(self as *mut ::style_option_rubber_band::StyleOptionRubberBand, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionRubberBand::set_shape(QRubberBand::Shape value)```</span>
  ///
  ///
  pub fn set_shape(&mut self, value: &::rubber_band::Shape) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_set_shape(self as *mut ::style_option_rubber_band::StyleOptionRubberBand, value as *const ::rubber_band::Shape) }
  }

  /// C++ method: <span style='color: green;'>```const QRubberBand::Shape& QStyleOptionRubberBand::shape() const```</span>
  ///
  ///
  pub fn shape<'l0>(&'l0 self) -> &'l0 ::rubber_band::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_shape(self as *const ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRubberBand::Shape& QStyleOptionRubberBand::shape_mut()```</span>
  ///
  ///
  pub fn shape_mut<'l0>(&'l0 mut self) -> &'l0 mut ::rubber_band::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_shape_mut(self as *mut ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_rubber_band::StyleOptionRubberBand {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionRubberBand_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionRubberBand::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 13```</span>
  Type = 13,
}

/// C++ type: <span style='color: green;'>```QStyleOptionRubberBand::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_rubber_band::StyleOptionRubberBand {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(self as *const ::style_option_rubber_band::StyleOptionRubberBand as *mut ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_rubber_band::StyleOptionRubberBand> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_rubber_band::StyleOptionRubberBand {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOptionRubberBand_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_rubber_band::StyleOptionRubberBand {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOptionRubberBand_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_rubber_band::StyleOptionRubberBand {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(self as *const ::style_option_rubber_band::StyleOptionRubberBand as *mut ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_rubber_band::StyleOptionRubberBand {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_rubber_band::StyleOptionRubberBand) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionRubberBand::new](../struct.StyleOptionRubberBand.html#method.new) method.
  pub trait StyleOptionRubberBandNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand>;
  }
  impl StyleOptionRubberBandNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionRubberBandNewArgs for &'a ::style_option_rubber_band::StyleOptionRubberBand {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_rubber_band::StyleOptionRubberBand> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionRubberBand_new_other(other as *const ::style_option_rubber_band::StyleOptionRubberBand) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
