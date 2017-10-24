/// C++ type: <span style='color: green;'>```QStyleHintReturn::HintReturnType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum HintReturnType {
  /// C++ enum variant: <span style='color: green;'>```SH_Default = 61440```</span>
  Default = 61440,
  /// C++ enum variant: <span style='color: green;'>```SH_Mask = 61441```</span>
  Mask = 61441,
  /// C++ enum variant: <span style='color: green;'>```SH_Variant = 61442```</span>
  Variant = 61442,
}

/// C++ type: <span style='color: green;'>```QStyleHintReturn```</span>
#[repr(C)]
pub struct StyleHintReturn(u8);

impl StyleHintReturn {
  /// C++ method: <span style='color: green;'>```QStyleHintReturn::QStyleHintReturn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleHintReturn::QStyleHintReturn()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleHintReturn::QStyleHintReturn(int version = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleHintReturn::QStyleHintReturn(int version = ?, int type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn>
    where Args: overloading::StyleHintReturnNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleHintReturn::set_type(int value)```</span>
  ///
  ///
  pub fn set_type(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_set_type(self as *mut ::style_hint_return::StyleHintReturn, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleHintReturn::set_version(int value)```</span>
  ///
  ///
  pub fn set_version(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleHintReturn_set_version(self as *mut ::style_hint_return::StyleHintReturn, value)
    }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHintReturn::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_type(self as *const ::style_hint_return::StyleHintReturn) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleHintReturn::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_version(self as *const ::style_hint_return::StyleHintReturn) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_hint_return::StyleHintReturn {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleHintReturn_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleHintReturn::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 61440```</span>
  Type = 61440,
}

/// C++ type: <span style='color: green;'>```QStyleHintReturn::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleHintReturn::new](../struct.StyleHintReturn.html#method.new) method.
  pub trait StyleHintReturnNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn>;
  }
  impl StyleHintReturnNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleHintReturnNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn> {
      let version = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_new_version(version) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleHintReturnNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::style_hint_return::StyleHintReturn> {
      let version = self.0;
      let type_ = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleHintReturn_new_version_type(version, type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
