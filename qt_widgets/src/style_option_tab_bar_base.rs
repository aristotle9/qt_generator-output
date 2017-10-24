/// C++ type: <span style='color: green;'>```QStyleOptionTabBarBase```</span>
#[repr(C)]
pub struct StyleOptionTabBarBase([u8; ::type_sizes::QT_WIDGETS_STYLE_OPTION_TAB_BAR_BASE_STYLE_OPTION_TAB_BAR_BASE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StyleOptionTabBarBase {
  unsafe fn new_uninitialized() -> StyleOptionTabBarBase {
    StyleOptionTabBarBase(::std::mem::uninitialized())
  }
}

impl StyleOptionTabBarBase {
  /// C++ method: <span style='color: green;'>```bool QStyleOptionTabBarBase::documentMode() const```</span>
  ///
  ///
  pub fn document_mode(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_documentMode(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionTabBarBase::QStyleOptionTabBarBase```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::style_option_tab_bar_base::StyleOptionTabBarBase```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTabBarBase::QStyleOptionTabBarBase()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tab_bar_base::StyleOptionTabBarBase) -> ::style_option_tab_bar_base::StyleOptionTabBarBase```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTabBarBase::QStyleOptionTabBarBase(const QStyleOptionTabBarBase& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::style_option_tab_bar_base::StyleOptionTabBarBase
    where Args: overloading::StyleOptionTabBarBaseNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionTabBarBase::selectedTabRect() const```</span>
  ///
  ///
  pub fn selected_tab_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_selectedTabRect(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionTabBarBase::selectedTabRect_mut()```</span>
  ///
  ///
  pub fn selected_tab_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_selectedTabRect_mut(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabBarBase::set_documentMode(bool value)```</span>
  ///
  ///
  pub fn set_document_mode(&mut self, value: bool) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_set_documentMode(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabBarBase::set_selectedTabRect(QRect value)```</span>
  ///
  ///
  pub fn set_selected_tab_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_set_selectedTabRect(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase, value as *const ::qt_core::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabBarBase::set_shape(QTabBar::Shape value)```</span>
  ///
  ///
  pub fn set_shape(&mut self, value: &::tab_bar::Shape) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_set_shape(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase, value as *const ::tab_bar::Shape) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabBarBase::set_tabBarRect(QRect value)```</span>
  ///
  ///
  pub fn set_tab_bar_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_set_tabBarRect(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase, value as *const ::qt_core::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```const QTabBar::Shape& QStyleOptionTabBarBase::shape() const```</span>
  ///
  ///
  pub fn shape<'l0>(&'l0 self) -> &'l0 ::tab_bar::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_shape(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTabBar::Shape& QStyleOptionTabBarBase::shape_mut()```</span>
  ///
  ///
  pub fn shape_mut<'l0>(&'l0 mut self) -> &'l0 mut ::tab_bar::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_shape_mut(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionTabBarBase::tabBarRect() const```</span>
  ///
  ///
  pub fn tab_bar_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_tabBarRect(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionTabBarBase::tabBarRect_mut()```</span>
  ///
  ///
  pub fn tab_bar_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_tabBarRect_mut(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::style_option_tab_bar_base::StyleOptionTabBarBase {
  /// C++ method: <span style='color: green;'>```[destructor] void QStyleOptionTabBarBase::~QStyleOptionTabBarBase()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_destructor(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) }
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTabBarBase::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 12```</span>
  Type = 12,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTabBarBase::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 2```</span>
  Version = 2,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tab_bar_base::StyleOptionTabBarBase {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tab_bar_base::StyleOptionTabBarBase> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tab_bar_base::StyleOptionTabBarBase {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOptionTabBarBase_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_tab_bar_base::StyleOptionTabBarBase {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOptionTabBarBase_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_tab_bar_base::StyleOptionTabBarBase {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab_bar_base::StyleOptionTabBarBase as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tab_bar_base::StyleOptionTabBarBase {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabBarBase_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab_bar_base::StyleOptionTabBarBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionTabBarBase::new](../struct.StyleOptionTabBarBase.html#method.new) method.
  pub trait StyleOptionTabBarBaseNewArgs {
    fn exec(self) -> ::style_option_tab_bar_base::StyleOptionTabBarBase;
  }
  impl StyleOptionTabBarBaseNewArgs for () {
    fn exec(self) -> ::style_option_tab_bar_base::StyleOptionTabBarBase {

      {
        let mut object: ::style_option_tab_bar_base::StyleOptionTabBarBase =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionTabBarBase_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StyleOptionTabBarBaseNewArgs for &'a ::style_option_tab_bar_base::StyleOptionTabBarBase {
    fn exec(self) -> ::style_option_tab_bar_base::StyleOptionTabBarBase {
      let other = self;
      {
        let mut object: ::style_option_tab_bar_base::StyleOptionTabBarBase =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QStyleOptionTabBarBase_constructor_other(other as *const ::style_option_tab_bar_base::StyleOptionTabBarBase, &mut object);
        }
        object
      }
    }
  }
}
