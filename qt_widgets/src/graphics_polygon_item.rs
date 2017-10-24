/// C++ type: <span style='color: green;'>```QGraphicsPolygonItem```</span>
#[repr(C)]
pub struct GraphicsPolygonItem(u8);

impl GraphicsPolygonItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsPolygonItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPolygonItem_boundingRect_to_output(self as *const ::graphics_polygon_item::GraphicsPolygonItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPolygonItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_contains(self as *const ::graphics_polygon_item::GraphicsPolygonItem,
                                                        point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPolygonItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsPolygonItem_isObscuredBy(self as *const ::graphics_polygon_item::GraphicsPolygonItem,
                                                          item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem::QGraphicsPolygonItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPolygonItem::QGraphicsPolygonItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::polygon_f::PolygonF) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF& polygon)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>
    where Args: overloading::GraphicsPolygonItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem::QGraphicsPolygonItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPolygonItem::QGraphicsPolygonItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::polygon_f::PolygonF, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPolygonItem::QGraphicsPolygonItem(const QPolygonF& polygon, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>
    where Args: overloading::GraphicsPolygonItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPolygonItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPolygonItem_opaqueArea_to_output(self as *const ::graphics_polygon_item::GraphicsPolygonItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPolygonItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsPolygonItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsPolygonItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsPolygonItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygonF QGraphicsPolygonItem::polygon() const```</span>
  ///
  ///
  pub fn polygon(&self) -> ::qt_gui::polygon_f::PolygonF {
    {
      let mut object: ::qt_gui::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPolygonItem_polygon_to_output(self as *const ::graphics_polygon_item::GraphicsPolygonItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsPolygonItem::setFillRule(Qt::FillRule rule)```</span>
  ///
  ///
  pub fn set_fill_rule(&mut self, rule: &::qt_core::qt::FillRule) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_setFillRule(self as *mut ::graphics_polygon_item::GraphicsPolygonItem,
                                                           rule as *const ::qt_core::qt::FillRule)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsPolygonItem::setPolygon(const QPolygonF& polygon)```</span>
  ///
  ///
  pub fn set_polygon(&mut self, polygon: &::qt_gui::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_setPolygon(self as *mut ::graphics_polygon_item::GraphicsPolygonItem,
                                                          polygon as *const ::qt_gui::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPolygonItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPolygonItem_shape_to_output(self as *const ::graphics_polygon_item::GraphicsPolygonItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsPolygonItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_type(self as *const ::graphics_polygon_item::GraphicsPolygonItem)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_polygon_item::GraphicsPolygonItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsPolygonItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_polygon_item::GraphicsPolygonItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_polygon_item::GraphicsPolygonItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_polygon_item::GraphicsPolygonItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_polygon_item::GraphicsPolygonItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_polygon_item::GraphicsPolygonItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_polygon_item::GraphicsPolygonItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_dynamic_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_polygon_item::GraphicsPolygonItem {
fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_polygon_item::GraphicsPolygonItem as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_polygon_item::GraphicsPolygonItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_polygon_item::GraphicsPolygonItem as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_polygon_item::GraphicsPolygonItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_polygon_item::GraphicsPolygonItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_polygon_item::GraphicsPolygonItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_polygon_item::GraphicsPolygonItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_polygon_item::GraphicsPolygonItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_polygon_item::GraphicsPolygonItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QGraphicsPolygonItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_polygon_item::GraphicsPolygonItem {
  type Target = ::abstract_graphics_shape_item::AbstractGraphicsShapeItem;
  fn deref(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_polygon_item::GraphicsPolygonItem as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_polygon_item::GraphicsPolygonItem {
  fn deref_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_polygon_item::GraphicsPolygonItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsPolygonItem::new](../struct.GraphicsPolygonItem.html#method.new) method.
  pub trait GraphicsPolygonItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>;
  }
  impl GraphicsPolygonItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPolygonItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsPolygonItemNewArgs for &'a ::qt_gui::polygon_f::PolygonF {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem> {
      let polygon = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsPolygonItem_new_polygon(polygon as *const ::qt_gui::polygon_f::PolygonF)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPolygonItem::new_unsafe](../struct.GraphicsPolygonItem.html#method.new_unsafe) method.
  pub trait GraphicsPolygonItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem>;
  }
  impl GraphicsPolygonItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsPolygonItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsPolygonItemNewUnsafeArgs for (&'a ::qt_gui::polygon_f::PolygonF, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_polygon_item::GraphicsPolygonItem> {
      let polygon = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsPolygonItem_new_polygon_parent(polygon as *const ::qt_gui::polygon_f::PolygonF,
                                                                    parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPolygonItem::paint](../struct.GraphicsPolygonItem.html#method.paint) method.
  pub trait GraphicsPolygonItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_polygon_item::GraphicsPolygonItem) -> ();
  }
  impl<'largs> GraphicsPolygonItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_polygon_item::GraphicsPolygonItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_paint_painter_option(original_self as *mut ::graphics_polygon_item::GraphicsPolygonItem, painter, option)
    }
  }
  impl<'largs> GraphicsPolygonItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                             *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                             *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_polygon_item::GraphicsPolygonItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsPolygonItem_paint_painter_option_widget(original_self as *mut ::graphics_polygon_item::GraphicsPolygonItem, painter, option, widget)
    }
  }
}
