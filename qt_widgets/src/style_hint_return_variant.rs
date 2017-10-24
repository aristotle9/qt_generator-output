/// C++ type: <span style='color: green;'>```QStyleHintReturnVariant```</span>
#[repr(C)]
pub struct StyleHintReturnVariant(u8);

impl StyleHintReturnVariant {
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleHintReturnVariant::QStyleHintReturnVariant()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::style_hint_return_variant::StyleHintReturnVariant> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHintReturnVariant::set_variant(QVariant value)```</span>
  ///
  ///
  pub fn set_variant(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_set_variant(self as *mut ::style_hint_return_variant::StyleHintReturnVariant, value as *const ::qt_core::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QStyleHintReturnVariant::variant() const```</span>
  ///
  ///
  pub fn variant<'l0>(&'l0 self) -> &'l0 ::qt_core::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_variant(self as *const ::style_hint_return_variant::StyleHintReturnVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QStyleHintReturnVariant::variant_mut()```</span>
  ///
  ///
  pub fn variant_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_variant_mut(self as *mut ::style_hint_return_variant::StyleHintReturnVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_hint_return_variant::StyleHintReturnVariant {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleHintReturnVariant_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleHintReturnVariant::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 61442```</span>
  Type = 61442,
}

/// C++ type: <span style='color: green;'>```QStyleHintReturnVariant::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_hint_return::StyleHintReturn> for ::style_hint_return_variant::StyleHintReturnVariant {
fn static_cast_mut(&mut self) -> &mut ::style_hint_return::StyleHintReturn {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturn_ptr(self as *mut ::style_hint_return_variant::StyleHintReturnVariant) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_hint_return::StyleHintReturn {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturn_ptr(self as *const ::style_hint_return_variant::StyleHintReturnVariant as *mut ::style_hint_return_variant::StyleHintReturnVariant) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_hint_return_variant::StyleHintReturnVariant> for ::style_hint_return::StyleHintReturn {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_hint_return_variant::StyleHintReturnVariant {
let ffi_result = ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturnVariant_ptr(self as *mut ::style_hint_return::StyleHintReturn);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_hint_return_variant::StyleHintReturnVariant {
let ffi_result = ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturnVariant_ptr(self as *const ::style_hint_return::StyleHintReturn as *mut ::style_hint_return::StyleHintReturn);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_hint_return_variant::StyleHintReturnVariant {
  type Target = ::style_hint_return::StyleHintReturn;
  fn deref(&self) -> &::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturn_ptr(self as *const ::style_hint_return_variant::StyleHintReturnVariant as *mut ::style_hint_return_variant::StyleHintReturnVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_hint_return_variant::StyleHintReturnVariant {
  fn deref_mut(&mut self) -> &mut ::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnVariant_G_static_cast_QStyleHintReturn_ptr(self as *mut ::style_hint_return_variant::StyleHintReturnVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
