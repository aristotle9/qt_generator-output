/// C++ type: <span style='color: green;'>```QGraphicsSimpleTextItem```</span>
#[repr(C)]
pub struct GraphicsSimpleTextItem(u8);

impl GraphicsSimpleTextItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsSimpleTextItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_boundingRect_to_output(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsSimpleTextItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_contains(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, point as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```QFont QGraphicsSimpleTextItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_font_to_output(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsSimpleTextItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_isObscuredBy(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsSimpleTextItem::QGraphicsSimpleTextItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(const QString& text)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>
    where Args: overloading::GraphicsSimpleTextItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsSimpleTextItem::QGraphicsSimpleTextItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSimpleTextItem::QGraphicsSimpleTextItem(const QString& text, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>
    where Args: overloading::GraphicsSimpleTextItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsSimpleTextItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_opaqueArea_to_output(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsSimpleTextItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn paint(&mut self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                      widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_paint(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem, painter, option, widget)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSimpleTextItem::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_setFont(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem, font as *const ::qt_gui::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSimpleTextItem::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_setText(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem, text as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsSimpleTextItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_shape_to_output(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QGraphicsSimpleTextItem::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_text_to_output(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsSimpleTextItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_type(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_simple_text_item::GraphicsSimpleTextItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_simple_text_item::GraphicsSimpleTextItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_simple_text_item::GraphicsSimpleTextItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_simple_text_item::GraphicsSimpleTextItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_simple_text_item::GraphicsSimpleTextItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_simple_text_item::GraphicsSimpleTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_simple_text_item::GraphicsSimpleTextItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_dynamic_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_simple_text_item::GraphicsSimpleTextItem {
fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_simple_text_item::GraphicsSimpleTextItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_simple_text_item::GraphicsSimpleTextItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_simple_text_item::GraphicsSimpleTextItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_simple_text_item::GraphicsSimpleTextItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_simple_text_item::GraphicsSimpleTextItem> for ::graphics_item::GraphicsItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_simple_text_item::GraphicsSimpleTextItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_simple_text_item::GraphicsSimpleTextItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QGraphicsSimpleTextItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_simple_text_item::GraphicsSimpleTextItem {
  type Target = ::abstract_graphics_shape_item::AbstractGraphicsShapeItem;
  fn deref(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_simple_text_item::GraphicsSimpleTextItem as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_simple_text_item::GraphicsSimpleTextItem {
  fn deref_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_simple_text_item::GraphicsSimpleTextItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSimpleTextItem::new](../struct.GraphicsSimpleTextItem.html#method.new) method.
  pub trait GraphicsSimpleTextItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>;
  }
  impl GraphicsSimpleTextItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsSimpleTextItemNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem> {
      let text = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsSimpleTextItem::new_unsafe](../struct.GraphicsSimpleTextItem.html#method.new_unsafe) method.
  pub trait GraphicsSimpleTextItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem>;
  }
  impl GraphicsSimpleTextItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsSimpleTextItemNewUnsafeArgs for (&'a ::qt_core::string::String, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_simple_text_item::GraphicsSimpleTextItem> {
      let text = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsSimpleTextItem_new_text_parent(text as *const ::qt_core::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
