/// C++ type: <span style='color: green;'>```QAbstractGraphicsShapeItem```</span>
#[repr(C)]
pub struct AbstractGraphicsShapeItem(u8);

impl AbstractGraphicsShapeItem {
  /// C++ method: <span style='color: green;'>```QBrush QAbstractGraphicsShapeItem::brush() const```</span>
  ///
  ///
  pub fn brush(&self) -> ::qt_gui::brush::Brush {
    {
      let mut object: ::qt_gui::brush::Brush =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_brush_to_output(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractGraphicsShapeItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_isObscuredBy(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, item)
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QAbstractGraphicsShapeItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_opaqueArea_to_output(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPen QAbstractGraphicsShapeItem::pen() const```</span>
  ///
  ///
  pub fn pen(&self) -> ::qt_gui::pen::Pen {
    {
      let mut object: ::qt_gui::pen::Pen =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_pen_to_output(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractGraphicsShapeItem::setBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_brush(&mut self, brush: &::qt_gui::brush::Brush) {
    unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_setBrush(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, brush as *const ::qt_gui::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```void QAbstractGraphicsShapeItem::setPen(const QPen& pen)```</span>
  ///
  ///
  pub fn set_pen(&mut self, pen: &::qt_gui::pen::Pen) {
    unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_setPen(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem, pen as *const ::qt_gui::pen::Pen) }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_item::GraphicsItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_dynamic_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_item::GraphicsItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_graphics_shape_item::AbstractGraphicsShapeItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_dynamic_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_item::GraphicsItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QGraphicsItem_ptr(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_item::GraphicsItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_item::GraphicsItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
  type Target = ::graphics_item::GraphicsItem;
  fn deref(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QGraphicsItem_ptr(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
  fn deref_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QAbstractGraphicsShapeItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
