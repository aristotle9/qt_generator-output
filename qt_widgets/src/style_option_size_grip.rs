/// C++ type: <span style='color: green;'>```QStyleOptionSizeGrip```</span>
#[repr(C)]
pub struct StyleOptionSizeGrip([u8; ::type_sizes::QT_WIDGETS_STYLE_OPTION_SIZE_GRIP_STYLE_OPTION_SIZE_GRIP]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StyleOptionSizeGrip {
  unsafe fn new_uninitialized() -> StyleOptionSizeGrip {
    StyleOptionSizeGrip(::std::mem::uninitialized())
  }
}

impl StyleOptionSizeGrip {
  /// C++ method: <span style='color: green;'>```const Qt::Corner& QStyleOptionSizeGrip::corner() const```</span>
  ///
  ///
  pub fn corner<'l0>(&'l0 self) -> &'l0 ::qt_core::qt::Corner {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSizeGrip_corner(self as *const ::style_option_size_grip::StyleOptionSizeGrip)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::Corner& QStyleOptionSizeGrip::corner_mut()```</span>
  ///
  ///
  pub fn corner_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::qt::Corner {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionSizeGrip_corner_mut(self as *mut ::style_option_size_grip::StyleOptionSizeGrip)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionSizeGrip::QStyleOptionSizeGrip```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::style_option_size_grip::StyleOptionSizeGrip```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSizeGrip::QStyleOptionSizeGrip()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_size_grip::StyleOptionSizeGrip) -> ::style_option_size_grip::StyleOptionSizeGrip```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionSizeGrip::QStyleOptionSizeGrip(const QStyleOptionSizeGrip& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::style_option_size_grip::StyleOptionSizeGrip
    where Args: overloading::StyleOptionSizeGripNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionSizeGrip::set_corner(Qt::Corner value)```</span>
  ///
  ///
  pub fn set_corner(&mut self, value: &::qt_core::qt::Corner) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSizeGrip_set_corner(self as *mut ::style_option_size_grip::StyleOptionSizeGrip,
                                                          value as *const ::qt_core::qt::Corner)
    }
  }
}

impl Drop for ::style_option_size_grip::StyleOptionSizeGrip {
  /// C++ method: <span style='color: green;'>```[destructor] void QStyleOptionSizeGrip::~QStyleOptionSizeGrip()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionSizeGrip_destructor(self as *mut ::style_option_size_grip::StyleOptionSizeGrip)
    }
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionSizeGrip::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983047```</span>
  Type = 983047,
}

/// C++ type: <span style='color: green;'>```QStyleOptionSizeGrip::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_size_grip::StyleOptionSizeGrip {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOption_ptr(self as *const ::style_option_size_grip::StyleOptionSizeGrip as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_size_grip::StyleOptionSizeGrip {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_size_grip::StyleOptionSizeGrip as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_size_grip::StyleOptionSizeGrip> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_size_grip::StyleOptionSizeGrip {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_size_grip::StyleOptionSizeGrip {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_size_grip::StyleOptionSizeGrip> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_size_grip::StyleOptionSizeGrip {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_size_grip::StyleOptionSizeGrip {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionSizeGrip_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_size_grip::StyleOptionSizeGrip {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_size_grip::StyleOptionSizeGrip as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_size_grip::StyleOptionSizeGrip {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionSizeGrip_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_size_grip::StyleOptionSizeGrip) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionSizeGrip::new](../struct.StyleOptionSizeGrip.html#method.new) method.
  pub trait StyleOptionSizeGripNewArgs {
    fn exec(self) -> ::style_option_size_grip::StyleOptionSizeGrip;
  }
  impl StyleOptionSizeGripNewArgs for () {
    fn exec(self) -> ::style_option_size_grip::StyleOptionSizeGrip {

      {
        let mut object: ::style_option_size_grip::StyleOptionSizeGrip =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionSizeGrip_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StyleOptionSizeGripNewArgs for &'a ::style_option_size_grip::StyleOptionSizeGrip {
    fn exec(self) -> ::style_option_size_grip::StyleOptionSizeGrip {
      let other = self;
      {
        let mut object: ::style_option_size_grip::StyleOptionSizeGrip =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionSizeGrip_constructor_other(other as *const ::style_option_size_grip::StyleOptionSizeGrip, &mut object);
        }
        object
      }
    }
  }
}
