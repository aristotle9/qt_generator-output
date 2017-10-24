/// C++ type: <span style='color: green;'>```QStyleHintReturnMask```</span>
#[repr(C)]
pub struct StyleHintReturnMask(u8);

impl StyleHintReturnMask {
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleHintReturnMask::QStyleHintReturnMask()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::style_hint_return_mask::StyleHintReturnMask> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnMask_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QRegion& QStyleHintReturnMask::region() const```</span>
  ///
  ///
  pub fn region<'l0>(&'l0 self) -> &'l0 ::qt_gui::region::Region {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleHintReturnMask_region(self as *const ::style_hint_return_mask::StyleHintReturnMask)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRegion& QStyleHintReturnMask::region_mut()```</span>
  ///
  ///
  pub fn region_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::region::Region {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleHintReturnMask_region_mut(self as *mut ::style_hint_return_mask::StyleHintReturnMask)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleHintReturnMask::set_region(QRegion value)```</span>
  ///
  ///
  pub fn set_region(&mut self, value: &::qt_gui::region::Region) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleHintReturnMask_set_region(self as *mut ::style_hint_return_mask::StyleHintReturnMask,
                                                          value as *const ::qt_gui::region::Region)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::style_hint_return_mask::StyleHintReturnMask {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleHintReturnMask_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleHintReturnMask::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 61441```</span>
  Type = 61441,
}

/// C++ type: <span style='color: green;'>```QStyleHintReturnMask::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_hint_return::StyleHintReturn> for ::style_hint_return_mask::StyleHintReturnMask {
  fn static_cast_mut(&mut self) -> &mut ::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturn_ptr(self as *mut ::style_hint_return_mask::StyleHintReturnMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturn_ptr(self as *const ::style_hint_return_mask::StyleHintReturnMask as *mut ::style_hint_return_mask::StyleHintReturnMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_hint_return_mask::StyleHintReturnMask> for ::style_hint_return::StyleHintReturn {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_hint_return_mask::StyleHintReturnMask {
let ffi_result = ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturnMask_ptr(self as *mut ::style_hint_return::StyleHintReturn);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_hint_return_mask::StyleHintReturnMask {
let ffi_result = ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturnMask_ptr(self as *const ::style_hint_return::StyleHintReturn as *mut ::style_hint_return::StyleHintReturn);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_hint_return_mask::StyleHintReturnMask {
  type Target = ::style_hint_return::StyleHintReturn;
  fn deref(&self) -> &::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturn_ptr(self as *const ::style_hint_return_mask::StyleHintReturnMask as *mut ::style_hint_return_mask::StyleHintReturnMask) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_hint_return_mask::StyleHintReturnMask {
  fn deref_mut(&mut self) -> &mut ::style_hint_return::StyleHintReturn {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturnMask_G_static_cast_QStyleHintReturn_ptr(self as *mut ::style_hint_return_mask::StyleHintReturnMask) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
