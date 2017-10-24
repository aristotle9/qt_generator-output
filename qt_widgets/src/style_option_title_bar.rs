/// C++ type: <span style='color: green;'>```QStyleOptionTitleBar```</span>
#[repr(C)]
pub struct StyleOptionTitleBar([u8; ::type_sizes::QT_WIDGETS_STYLE_OPTION_TITLE_BAR_STYLE_OPTION_TITLE_BAR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StyleOptionTitleBar {
  unsafe fn new_uninitialized() -> StyleOptionTitleBar {
    StyleOptionTitleBar(::std::mem::uninitialized())
  }
}

impl StyleOptionTitleBar {
  /// C++ method: <span style='color: green;'>```const QIcon& QStyleOptionTitleBar::icon() const```</span>
  ///
  ///
  pub fn icon<'l0>(&'l0 self) -> &'l0 ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTitleBar_icon(self as *const ::style_option_title_bar::StyleOptionTitleBar)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QIcon& QStyleOptionTitleBar::icon_mut()```</span>
  ///
  ///
  pub fn icon_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::icon::Icon {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTitleBar_icon_mut(self as *mut ::style_option_title_bar::StyleOptionTitleBar)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionTitleBar::QStyleOptionTitleBar```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::style_option_title_bar::StyleOptionTitleBar```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTitleBar::QStyleOptionTitleBar()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_title_bar::StyleOptionTitleBar) -> ::style_option_title_bar::StyleOptionTitleBar```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTitleBar::QStyleOptionTitleBar(const QStyleOptionTitleBar& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::style_option_title_bar::StyleOptionTitleBar
    where Args: overloading::StyleOptionTitleBarNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionTitleBar::set_icon(QIcon value)```</span>
  ///
  ///
  pub fn set_icon(&mut self, value: &::qt_gui::icon::Icon) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTitleBar_set_icon(self as *mut ::style_option_title_bar::StyleOptionTitleBar,
                                                        value as *const ::qt_gui::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTitleBar::set_text(QString value)```</span>
  ///
  ///
  pub fn set_text(&mut self, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTitleBar_set_text(self as *mut ::style_option_title_bar::StyleOptionTitleBar,
                                                        value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTitleBar::set_titleBarState(int value)```</span>
  ///
  ///
  pub fn set_title_bar_state(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_set_titleBarState(self as *mut ::style_option_title_bar::StyleOptionTitleBar, value) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QStyleOptionTitleBar::text() const```</span>
  ///
  ///
  pub fn text<'l0>(&'l0 self) -> &'l0 ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTitleBar_text(self as *const ::style_option_title_bar::StyleOptionTitleBar)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QStyleOptionTitleBar::text_mut()```</span>
  ///
  ///
  pub fn text_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::string::String {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QStyleOptionTitleBar_text_mut(self as *mut ::style_option_title_bar::StyleOptionTitleBar)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionTitleBar::titleBarState() const```</span>
  ///
  ///
  pub fn title_bar_state(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_titleBarState(self as *const ::style_option_title_bar::StyleOptionTitleBar) }
  }
}

impl Drop for ::style_option_title_bar::StyleOptionTitleBar {
  /// C++ method: <span style='color: green;'>```[destructor] void QStyleOptionTitleBar::~QStyleOptionTitleBar()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionTitleBar_destructor(self as *mut ::style_option_title_bar::StyleOptionTitleBar)
    }
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTitleBar::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 983045```</span>
  Type = 983045,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTitleBar::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_title_bar::StyleOptionTitleBar {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_title_bar::StyleOptionTitleBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOption_ptr(self as *const ::style_option_title_bar::StyleOptionTitleBar as *mut ::style_option_title_bar::StyleOptionTitleBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::style_option_complex::StyleOptionComplex> for ::style_option_title_bar::StyleOptionTitleBar {
fn static_cast_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_title_bar::StyleOptionTitleBar) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option_complex::StyleOptionComplex {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_title_bar::StyleOptionTitleBar as *mut ::style_option_title_bar::StyleOptionTitleBar) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_title_bar::StyleOptionTitleBar> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_title_bar::StyleOptionTitleBar {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOption(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_title_bar::StyleOptionTitleBar {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOption(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_title_bar::StyleOptionTitleBar> for ::style_option_complex::StyleOptionComplex {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_title_bar::StyleOptionTitleBar {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOptionComplex(self as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_title_bar::StyleOptionTitleBar {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionTitleBar_ptr_QStyleOptionComplex(self as *const ::style_option_complex::StyleOptionComplex as *mut ::style_option_complex::StyleOptionComplex);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_title_bar::StyleOptionTitleBar {
  type Target = ::style_option_complex::StyleOptionComplex;
  fn deref(&self) -> &::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(self as *const ::style_option_title_bar::StyleOptionTitleBar as *mut ::style_option_title_bar::StyleOptionTitleBar) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_title_bar::StyleOptionTitleBar {
  fn deref_mut(&mut self) -> &mut ::style_option_complex::StyleOptionComplex {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTitleBar_G_static_cast_QStyleOptionComplex_ptr(self as *mut ::style_option_title_bar::StyleOptionTitleBar) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionTitleBar::new](../struct.StyleOptionTitleBar.html#method.new) method.
  pub trait StyleOptionTitleBarNewArgs {
    fn exec(self) -> ::style_option_title_bar::StyleOptionTitleBar;
  }
  impl StyleOptionTitleBarNewArgs for () {
    fn exec(self) -> ::style_option_title_bar::StyleOptionTitleBar {

      {
        let mut object: ::style_option_title_bar::StyleOptionTitleBar =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionTitleBar_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StyleOptionTitleBarNewArgs for &'a ::style_option_title_bar::StyleOptionTitleBar {
    fn exec(self) -> ::style_option_title_bar::StyleOptionTitleBar {
      let other = self;
      {
        let mut object: ::style_option_title_bar::StyleOptionTitleBar =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionTitleBar_constructor_other(other as *const ::style_option_title_bar::StyleOptionTitleBar, &mut object);
        }
        object
      }
    }
  }
}
