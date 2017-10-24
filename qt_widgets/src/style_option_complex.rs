/// C++ type: <span style='color: green;'>```QStyleOptionComplex```</span>
#[repr(C)]
pub struct StyleOptionComplex(u8);

impl StyleOptionComplex {
  /// C++ method: <span style='color: green;'>```QStyleOptionComplex::QStyleOptionComplex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComplex::QStyleOptionComplex()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_complex::StyleOptionComplex) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComplex::QStyleOptionComplex(const QStyleOptionComplex& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComplex::QStyleOptionComplex(int version = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionComplex::QStyleOptionComplex(int version = ?, int type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>
    where Args: overloading::StyleOptionComplexNewArgs
  {
    args.exec()
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_complex::StyleOptionComplex {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionComplex_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionComplex::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983040```</span>
  Type = 983040,
}

/// C++ type: <span style='color: green;'>```QStyleOptionComplex::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_complex::StyleOptionComplex {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_complex::StyleOptionComplex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOption_ptr(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_complex::StyleOptionComplex> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_complex::StyleOptionComplex {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOption_ptr(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_complex::StyleOptionComplex {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_complex::StyleOptionComplex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionComplex::new](../struct.StyleOptionComplex.html#method.new) method.
  pub trait StyleOptionComplexNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex>;
  }
  impl StyleOptionComplexNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionComplexNewArgs for &'a ::style_option_complex::StyleOptionComplex {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex> {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionComplex_new_other(other as *const ::style_option_complex::StyleOptionComplex)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleOptionComplexNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex> {
      let version = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_new_version(version) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StyleOptionComplexNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_complex::StyleOptionComplex> {
      let version = self.0;
      let type_ = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionComplex_new_version_type(version, type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
