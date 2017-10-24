/// C++ type: <span style='color: green;'>```QStyleOptionGraphicsItem```</span>
#[repr(C)]
pub struct StyleOptionGraphicsItem(u8);

impl StyleOptionGraphicsItem {
  /// C++ method: <span style='color: green;'>```const QRectF& QStyleOptionGraphicsItem::exposedRect() const```</span>
  ///
  ///
  pub fn exposed_rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_exposedRect(self as *const ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QStyleOptionGraphicsItem::exposedRect_mut()```</span>
  ///
  ///
  pub fn exposed_rect_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_exposedRect_mut(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QStyleOptionGraphicsItem::levelOfDetail() const```</span>
  ///
  ///
  pub fn level_of_detail(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetail(self as *const ::style_option_graphics_item::StyleOptionGraphicsItem) }
  }

  /// C++ method: <span style='color: green;'>```static double QStyleOptionGraphicsItem::levelOfDetailFromTransform(const QTransform& worldTransform)```</span>
  ///
  ///
  pub fn level_of_detail_from_transform(world_transform: &::qt_gui::transform::Transform) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_levelOfDetailFromTransform(world_transform as *const ::qt_gui::transform::Transform) }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QStyleOptionGraphicsItem::matrix() const```</span>
  ///
  ///
  pub fn matrix<'l0>(&'l0 self) -> &'l0 ::qt_gui::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_matrix(self as *const ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMatrix& QStyleOptionGraphicsItem::matrix_mut()```</span>
  ///
  ///
  pub fn matrix_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_matrix_mut(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStyleOptionGraphicsItem::QStyleOptionGraphicsItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::style_option_graphics_item::StyleOptionGraphicsItem) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStyleOptionGraphicsItem::QStyleOptionGraphicsItem(const QStyleOptionGraphicsItem& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem>
    where Args: overloading::StyleOptionGraphicsItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QStyleOptionGraphicsItem::set_exposedRect(QRectF value)```</span>
  ///
  ///
  pub fn set_exposed_rect(&mut self, value: &::qt_core::rect_f::RectF) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_set_exposedRect(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem, value as *const ::qt_core::rect_f::RectF) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionGraphicsItem::set_levelOfDetail(double value)```</span>
  ///
  ///
  pub fn set_level_of_detail(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_set_levelOfDetail(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem, value) }
  }

  /// C++ method: <span style='color: green;'>```void QStyleOptionGraphicsItem::set_matrix(QMatrix value)```</span>
  ///
  ///
  pub fn set_matrix(&mut self, value: &::qt_gui::matrix::Matrix) {
    unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_set_matrix(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem, value as *const ::qt_gui::matrix::Matrix) }
  }
}

impl ::cpp_utils::CppDeletable for ::style_option_graphics_item::StyleOptionGraphicsItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_delete
  }
}

/// C++ type: <span style='color: green;'>```QStyleOptionGraphicsItem::StyleOptionType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionType {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Type = 15```</span>
  Type = 15,
}

/// C++ type: <span style='color: green;'>```QStyleOptionGraphicsItem::StyleOptionVersion```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StyleOptionVersion {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Version = 1```</span>
  Version = 1,
}

impl ::cpp_utils::StaticCast<::style_option::StyleOption> for ::style_option_graphics_item::StyleOptionGraphicsItem {
  fn static_cast_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_graphics_item::StyleOptionGraphicsItem as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::style_option_graphics_item::StyleOptionGraphicsItem> for ::style_option::StyleOption {
unsafe fn static_cast_mut(&mut self) -> &mut ::style_option_graphics_item::StyleOptionGraphicsItem {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOptionGraphicsItem_ptr(self as *mut ::style_option::StyleOption);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::style_option_graphics_item::StyleOptionGraphicsItem {
let ffi_result = ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOptionGraphicsItem_ptr(self as *const ::style_option::StyleOption as *mut ::style_option::StyleOption);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::style_option_graphics_item::StyleOptionGraphicsItem {
  type Target = ::style_option::StyleOption;
  fn deref(&self) -> &::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(self as *const ::style_option_graphics_item::StyleOptionGraphicsItem as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::style_option_graphics_item::StyleOptionGraphicsItem {
  fn deref_mut(&mut self) -> &mut ::style_option::StyleOption {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_G_static_cast_QStyleOption_ptr(self as *mut ::style_option_graphics_item::StyleOptionGraphicsItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StyleOptionGraphicsItem::new](../struct.StyleOptionGraphicsItem.html#method.new) method.
  pub trait StyleOptionGraphicsItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem>;
  }
  impl StyleOptionGraphicsItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StyleOptionGraphicsItemNewArgs for &'a ::style_option_graphics_item::StyleOptionGraphicsItem {
    fn exec(self) -> ::cpp_utils::CppBox<::style_option_graphics_item::StyleOptionGraphicsItem> {
      let other = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QStyleOptionGraphicsItem_new_other(other as *const ::style_option_graphics_item::StyleOptionGraphicsItem) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
