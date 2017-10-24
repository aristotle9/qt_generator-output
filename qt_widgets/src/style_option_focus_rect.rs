/// C++ type: <span style='color: green;'>```QStyleOptionFocusRect```</span>
#[repr(C)]
pub struct StyleOptionFocusRect([u8; ::type_sizes::QT_WIDGETS_STYLE_OPTION_FOCUS_RECT_STYLE_OPTION_FOCUS_RECT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StyleOptionFocusRect {
  unsafe fn new_uninitialized() -> StyleOptionFocusRect {
    StyleOptionFocusRect(::std::mem::uninitialized())
  }
}

impl StyleOptionFocusRect {
  /// C++ method: <span style='color: green;'>```const QColor& QStyleOptionFocusRect::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_backgroundColor(self as *const ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QStyleOptionFocusRect::backgroundColor_mut()```</span>
  ///
  ///
  pub fn background_color_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_backgroundColor_mut(self as *mut ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionFocusRect::QStyleOptionFocusRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::style_option_focus_rect::StyleOptionFocusRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionFocusRect::QStyleOptionFocusRect()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_focus_rect::StyleOptionFocusRect) -> ::style_option_focus_rect::StyleOptionFocusRect```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionFocusRect::QStyleOptionFocusRect(const QStyleOptionFocusRect& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::style_option_focus_rect::StyleOptionFocusRect
    where Args: overloading::StyleOptionFocusRectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionFocusRect::set_backgroundColor(QColor value)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, value: &::qt_gui::color::Color) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_set_backgroundColor(self as *mut ::style_option_focus_rect::StyleOptionFocusRect, value as *const ::qt_gui::color::Color) }
  }
}

impl Drop for ::style_option_focus_rect::StyleOptionFocusRect {
  /// C++ method: <span style='color: green;'>```[destructor] void QStyleOptionFocusRect::~QStyleOptionFocusRect()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QStyleOptionFocusRect_destructor(self as *mut ::style_option_focus_rect::StyleOptionFocusRect)
    }
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionFocusRect::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 1```</span>
  Type = 1,
}

/// C++ type: <span style='color: green;'>```QStyleOptionFocusRect::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_focus_rect::StyleOptionFocusRect {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOption_ptr(self as *const ::style_option_focus_rect::StyleOptionFocusRect as *mut ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_focus_rect::StyleOptionFocusRect> for ::style_option::StyleOption {
  unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_focus_rect::StyleOptionFocusRect {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOptionFocusRect_ptr(self as *mut ::style_option::StyleOption);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::style_option_focus_rect::StyleOptionFocusRect {
    let ffi_result = ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOptionFocusRect_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::style_option_focus_rect::StyleOptionFocusRect {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOption_ptr(self as *const ::style_option_focus_rect::StyleOptionFocusRect as *mut ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_focus_rect::StyleOptionFocusRect {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionFocusRect_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_focus_rect::StyleOptionFocusRect) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionFocusRect::new](../struct.StyleOptionFocusRect.html#method.new) method.
  pub trait StyleOptionFocusRectNewArgs {
    fn exec(self) -> ::style_option_focus_rect::StyleOptionFocusRect;
  }
  impl StyleOptionFocusRectNewArgs for () {
    fn exec(self) -> ::style_option_focus_rect::StyleOptionFocusRect {

      {
        let mut object: ::style_option_focus_rect::StyleOptionFocusRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionFocusRect_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StyleOptionFocusRectNewArgs for &'a ::style_option_focus_rect::StyleOptionFocusRect {
    fn exec(self) -> ::style_option_focus_rect::StyleOptionFocusRect {
      let other = self;
      {
        let mut object: ::style_option_focus_rect::StyleOptionFocusRect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionFocusRect_constructor_other(other as *const ::style_option_focus_rect::StyleOptionFocusRect, &mut object);
        }
        object
      }
    }
  }
}
