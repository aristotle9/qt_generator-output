/// C++ type: <span style='color: green;'>```QGraphicsEllipseItem```</span>
#[repr(C)]
pub struct GraphicsEllipseItem(u8);

impl GraphicsEllipseItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsEllipseItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEllipseItem_boundingRect_to_output(self as *const ::graphics_ellipse_item::GraphicsEllipseItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsEllipseItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_contains(self as *const ::graphics_ellipse_item::GraphicsEllipseItem,
                                                        point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsEllipseItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsEllipseItem_isObscuredBy(self as *const ::graphics_ellipse_item::GraphicsEllipseItem,
                                                          item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem::QGraphicsEllipseItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect_f::RectF) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>
    where Args: overloading::GraphicsEllipseItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem::QGraphicsEllipseItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::rect_f::RectF, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem(const QRectF& rect, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsEllipseItem::QGraphicsEllipseItem(double x, double y, double w, double h, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>
    where Args: overloading::GraphicsEllipseItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsEllipseItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEllipseItem_opaqueArea_to_output(self as *const ::graphics_ellipse_item::GraphicsEllipseItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsEllipseItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsEllipseItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsEllipseItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QGraphicsEllipseItem::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEllipseItem_rect_to_output(self as *const ::graphics_ellipse_item::GraphicsEllipseItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsEllipseItem::setRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsEllipseItem::setRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsEllipseItem::setRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn set_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsEllipseItemSetRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsEllipseItem::setSpanAngle(int angle)```</span>
  ///
  ///
  pub fn set_span_angle(&mut self, angle: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_setSpanAngle(self as *mut ::graphics_ellipse_item::GraphicsEllipseItem,
                                                            angle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsEllipseItem::setStartAngle(int angle)```</span>
  ///
  ///
  pub fn set_start_angle(&mut self, angle: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_setStartAngle(self as *mut ::graphics_ellipse_item::GraphicsEllipseItem, angle)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsEllipseItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsEllipseItem_shape_to_output(self as *const ::graphics_ellipse_item::GraphicsEllipseItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsEllipseItem::spanAngle() const```</span>
  ///
  ///
  pub fn span_angle(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_spanAngle(self as *const ::graphics_ellipse_item::GraphicsEllipseItem)
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsEllipseItem::startAngle() const```</span>
  ///
  ///
  pub fn start_angle(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_startAngle(self as *const ::graphics_ellipse_item::GraphicsEllipseItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsEllipseItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_type(self as *const ::graphics_ellipse_item::GraphicsEllipseItem)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_ellipse_item::GraphicsEllipseItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsEllipseItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_ellipse_item::GraphicsEllipseItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_ellipse_item::GraphicsEllipseItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_ellipse_item::GraphicsEllipseItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_ellipse_item::GraphicsEllipseItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_ellipse_item::GraphicsEllipseItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_ellipse_item::GraphicsEllipseItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_dynamic_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_ellipse_item::GraphicsEllipseItem {
fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_ellipse_item::GraphicsEllipseItem as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_ellipse_item::GraphicsEllipseItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_ellipse_item::GraphicsEllipseItem as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_ellipse_item::GraphicsEllipseItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_ellipse_item::GraphicsEllipseItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_ellipse_item::GraphicsEllipseItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_ellipse_item::GraphicsEllipseItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_ellipse_item::GraphicsEllipseItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_ellipse_item::GraphicsEllipseItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QGraphicsEllipseItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_ellipse_item::GraphicsEllipseItem {
  type Target = ::abstract_graphics_shape_item::AbstractGraphicsShapeItem;
  fn deref(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_ellipse_item::GraphicsEllipseItem as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_ellipse_item::GraphicsEllipseItem {
  fn deref_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_ellipse_item::GraphicsEllipseItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsEllipseItem::new](../struct.GraphicsEllipseItem.html#method.new) method.
  pub trait GraphicsEllipseItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>;
  }
  impl GraphicsEllipseItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsEllipseItemNewArgs for &'a ::qt_core::rect_f::RectF {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {
      let rect = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_rect(rect as *const ::qt_core::rect_f::RectF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsEllipseItemNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h(x, y, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsEllipseItem::new_unsafe](../struct.GraphicsEllipseItem.html#method.new_unsafe) method.
  pub trait GraphicsEllipseItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem>;
  }
  impl GraphicsEllipseItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsEllipseItemNewUnsafeArgs for (&'a ::qt_core::rect_f::RectF, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {
      let rect = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_rect_parent(rect as *const ::qt_core::rect_f::RectF, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsEllipseItemNewUnsafeArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_ellipse_item::GraphicsEllipseItem> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let parent = self.4;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsEllipseItem_new_x_y_w_h_parent(x, y, w, h, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsEllipseItem::paint](../struct.GraphicsEllipseItem.html#method.paint) method.
  pub trait GraphicsEllipseItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> ();
  }
  impl<'largs> GraphicsEllipseItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_paint_painter_option(original_self as *mut ::graphics_ellipse_item::GraphicsEllipseItem, painter, option)
    }
  }
  impl<'largs> GraphicsEllipseItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                             *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                             *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsEllipseItem_paint_painter_option_widget(original_self as *mut ::graphics_ellipse_item::GraphicsEllipseItem, painter, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsEllipseItem::set_rect](../struct.GraphicsEllipseItem.html#method.set_rect) method.
  pub trait GraphicsEllipseItemSetRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> ();
  }
  impl<'largs> GraphicsEllipseItemSetRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> () {
      let rect = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_setRect_rect(original_self as *mut ::graphics_ellipse_item::GraphicsEllipseItem, rect as *const ::qt_core::rect_f::RectF) }
    }
  }
  impl<'largs> GraphicsEllipseItemSetRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_ellipse_item::GraphicsEllipseItem) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_widgets_c_QGraphicsEllipseItem_setRect_x_y_w_h(original_self as *mut ::graphics_ellipse_item::GraphicsEllipseItem, x, y, w, h) }
    }
  }
}
