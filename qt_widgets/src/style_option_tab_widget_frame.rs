/// C++ type: <span style='color: green;'>```QStyleOptionTabWidgetFrame```</span>
#[repr(C)]
pub struct StyleOptionTabWidgetFrame(u8);

impl StyleOptionTabWidgetFrame {
  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTabWidgetFrame::leftCornerWidgetSize() const```</span>
  ///
  ///
  pub fn left_corner_widget_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_leftCornerWidgetSize(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTabWidgetFrame::leftCornerWidgetSize_mut()```</span>
  ///
  ///
  pub fn left_corner_widget_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_leftCornerWidgetSize_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionTabWidgetFrame::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_lineWidth(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) }
  }

  /// C++ method: <span style='color: green;'>```int QStyleOptionTabWidgetFrame::midLineWidth() const```</span>
  ///
  ///
  pub fn mid_line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_midLineWidth(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) }
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame>
    where Args: overloading::StyleOptionTabWidgetFrameNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTabWidgetFrame::rightCornerWidgetSize() const```</span>
  ///
  ///
  pub fn right_corner_widget_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_rightCornerWidgetSize(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTabWidgetFrame::rightCornerWidgetSize_mut()```</span>
  ///
  ///
  pub fn right_corner_widget_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_rightCornerWidgetSize_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionTabWidgetFrame::selectedTabRect() const```</span>
  ///
  ///
  pub fn selected_tab_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_selectedTabRect(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionTabWidgetFrame::selectedTabRect_mut()```</span>
  ///
  ///
  pub fn selected_tab_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_selectedTabRect_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_leftCornerWidgetSize(QSize value)```</span>
  ///
  ///
  pub fn set_left_corner_widget_size(&mut self, value: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_leftCornerWidgetSize(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_lineWidth(int value)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_lineWidth(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_midLineWidth(int value)```</span>
  ///
  ///
  pub fn set_mid_line_width(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_midLineWidth(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_rightCornerWidgetSize(QSize value)```</span>
  ///
  ///
  pub fn set_right_corner_widget_size(&mut self, value: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_rightCornerWidgetSize(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_selectedTabRect(QRect value)```</span>
  ///
  ///
  pub fn set_selected_tab_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_selectedTabRect(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::qt_core::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_shape(QTabBar::Shape value)```</span>
  ///
  ///
  pub fn set_shape(&mut self, value: &::tab_bar::Shape) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_shape(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::tab_bar::Shape) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_tabBarRect(QRect value)```</span>
  ///
  ///
  pub fn set_tab_bar_rect(&mut self, value: &::qt_core::rect::Rect) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_tabBarRect(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::qt_core::rect::Rect) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionTabWidgetFrame::set_tabBarSize(QSize value)```</span>
  ///
  ///
  pub fn set_tab_bar_size(&mut self, value: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_set_tabBarSize(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame, value as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```const QTabBar::Shape& QStyleOptionTabWidgetFrame::shape() const```</span>
  ///
  ///
  pub fn shape<'l0>(&'l0 self) -> &'l0 ::tab_bar::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_shape(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTabBar::Shape& QStyleOptionTabWidgetFrame::shape_mut()```</span>
  ///
  ///
  pub fn shape_mut<'l0>(&'l0 mut self) -> &'l0 mut ::tab_bar::Shape {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_shape_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRect& QStyleOptionTabWidgetFrame::tabBarRect() const```</span>
  ///
  ///
  pub fn tab_bar_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarRect(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRect& QStyleOptionTabWidgetFrame::tabBarRect_mut()```</span>
  ///
  ///
  pub fn tab_bar_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarRect_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QStyleOptionTabWidgetFrame::tabBarSize() const```</span>
  ///
  ///
  pub fn tab_bar_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarSize(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QStyleOptionTabWidgetFrame::tabBarSize_mut()```</span>
  ///
  ///
  pub fn tab_bar_size_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_tabBarSize_mut(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionTabWidgetFrame::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 11```</span>
  Type = 11,
}

/// C++ type: <span style='color: green;'>```QStyleOptionTabWidgetFrame::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 2```</span>
  Version = 2,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::style_option::StyleOption {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOptionTabWidgetFrame_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOptionTabWidgetFrame_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOption_ptr(self as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionTabWidgetFrame::new](../struct.StyleOptionTabWidgetFrame.html#method.new) method.
  pub trait StyleOptionTabWidgetFrameNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame>;
  }
  impl StyleOptionTabWidgetFrameNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionTabWidgetFrameNewArgs for &'a ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_tab_widget_frame::StyleOptionTabWidgetFrame> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionTabWidgetFrame_new_other(other as *const ::style_option_tab_widget_frame::StyleOptionTabWidgetFrame) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
