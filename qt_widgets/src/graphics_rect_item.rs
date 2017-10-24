/// C++ type: <span style='color: green;'>```QGraphicsRectItem```</span>
#[repr(C)]
pub struct GraphicsRectItem(u8);

impl GraphicsRectItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsRectItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRectItem_boundingRect_to_output(self as *const ::graphics_rect_item::GraphicsRectItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsRectItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsRectItem_contains(self as *const ::graphics_rect_item::GraphicsRectItem,
                                                     point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsRectItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsRectItem_isObscuredBy(self as *const ::graphics_rect_item::GraphicsRectItem, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsRectItem::QGraphicsRectItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect_f::RectF) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>
    where Args: overloading::GraphicsRectItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsRectItem::QGraphicsRectItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::rect_f::RectF, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem(const QRectF& rect, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsRectItem::QGraphicsRectItem(double x, double y, double w, double h, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>
    where Args: overloading::GraphicsRectItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsRectItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRectItem_opaqueArea_to_output(self as *const ::graphics_rect_item::GraphicsRectItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsRectItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsRectItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsRectItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsRectItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsRectItem::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRectItem_rect_to_output(self as *const ::graphics_rect_item::GraphicsRectItem,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsRectItem::setRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsRectItem::setRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsRectItem::setRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsRectItemSetRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsRectItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRectItem_shape_to_output(self as *const ::graphics_rect_item::GraphicsRectItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsRectItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_type(self as *const ::graphics_rect_item::GraphicsRectItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_rect_item::GraphicsRectItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsRectItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_rect_item::GraphicsRectItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_rect_item::GraphicsRectItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_rect_item::GraphicsRectItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_rect_item::GraphicsRectItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_rect_item::GraphicsRectItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_rect_item::GraphicsRectItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_dynamic_cast_QGraphicsRectItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_rect_item::GraphicsRectItem {
fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_rect_item::GraphicsRectItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_rect_item::GraphicsRectItem as *mut ::graphics_rect_item::GraphicsRectItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_rect_item::GraphicsRectItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_rect_item::GraphicsRectItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_rect_item::GraphicsRectItem as *mut ::graphics_rect_item::GraphicsRectItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_rect_item::GraphicsRectItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_rect_item::GraphicsRectItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_rect_item::GraphicsRectItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_rect_item::GraphicsRectItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_rect_item::GraphicsRectItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_rect_item::GraphicsRectItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QGraphicsRectItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_rect_item::GraphicsRectItem {
  type Target = ::abstract_graphics_shape_item::AbstractGraphicsShapeItem;
  fn deref(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_rect_item::GraphicsRectItem as *mut ::graphics_rect_item::GraphicsRectItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_rect_item::GraphicsRectItem {
  fn deref_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_rect_item::GraphicsRectItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsRectItem::new](../struct.GraphicsRectItem.html#method.new) method.
  pub trait GraphicsRectItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>;
  }
  impl GraphicsRectItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsRectItemNewArgs for &'a ::qt_core::rect_f::RectF {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {
      let rect = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_new_rect(rect as *const ::qt_core::rect_f::RectF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsRectItemNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_new_x_y_w_h(x, y, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsRectItem::new_unsafe](../struct.GraphicsRectItem.html#method.new_unsafe) method.
  pub trait GraphicsRectItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem>;
  }
  impl GraphicsRectItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsRectItemNewUnsafeArgs for (&'a ::qt_core::rect_f::RectF, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {
      let rect = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_new_rect_parent(rect as *const ::qt_core::rect_f::RectF,
                                                                             parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsRectItemNewUnsafeArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_rect_item::GraphicsRectItem> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let parent = self.4;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsRectItem_new_x_y_w_h_parent(x, y, w, h, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsRectItem::paint](../struct.GraphicsRectItem.html#method.paint) method.
  pub trait GraphicsRectItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> ();
  }
  impl<'largs> GraphicsRectItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsRectItem_paint_painter_option(original_self as *mut ::graphics_rect_item::GraphicsRectItem, painter, option)
    }
  }
  impl<'largs> GraphicsRectItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                          *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                          *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsRectItem_paint_painter_option_widget(original_self as *mut ::graphics_rect_item::GraphicsRectItem, painter, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsRectItem::set_rect](../struct.GraphicsRectItem.html#method.set_rect) method.
  pub trait GraphicsRectItemSetRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> ();
  }
  impl<'largs> GraphicsRectItemSetRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsRectItem_setRect_rect(original_self as *mut ::graphics_rect_item::GraphicsRectItem, rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> GraphicsRectItemSetRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_rect_item::GraphicsRectItem) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_widgets_c_QGraphicsRectItem_setRect_x_y_w_h(original_self as *mut ::graphics_rect_item::GraphicsRectItem, x, y, w, h) }
    }
  }
}
