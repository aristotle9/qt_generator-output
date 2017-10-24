/// C++ type: <span style='color: green;'>```QGraphicsLineItem```</span>
#[repr(C)]
pub struct GraphicsLineItem(u8);

impl GraphicsLineItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsLineItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_boundingRect_to_output(self as *const ::graphics_line_item::GraphicsLineItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsLineItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLineItem_contains(self as *const ::graphics_line_item::GraphicsLineItem,
                                                     point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsLineItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsLineItem_isObscuredBy(self as *const ::graphics_line_item::GraphicsLineItem, item)
  }

  /// C++ method: <span style='color: green;'>```QLineF QGraphicsLineItem::line() const```</span>
  ///
  ///
  pub fn line(&self) -> ::qt_core::line_f::LineF {
    {
      let mut object: ::qt_core::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_line_to_output(self as *const ::graphics_line_item::GraphicsLineItem,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLineItem::QGraphicsLineItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::line_f::LineF) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem(const QLineF& line)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>
    where Args: overloading::GraphicsLineItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsLineItem::QGraphicsLineItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::line_f::LineF, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem(const QLineF& line, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsLineItem::QGraphicsLineItem(double x1, double y1, double x2, double y2, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>
    where Args: overloading::GraphicsLineItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsLineItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_opaqueArea_to_output(self as *const ::graphics_line_item::GraphicsLineItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLineItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLineItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLineItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLineItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPen QGraphicsLineItem::pen() const```</span>
  ///
  ///
  pub fn pen(&self) -> ::qt_gui::pen::Pen {
    {
      let mut object: ::qt_gui::pen::Pen =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_pen_to_output(self as *const ::graphics_line_item::GraphicsLineItem,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsLineItem::setLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_line(&mut self, &::qt_core::line_f::LineF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLineItem::setLine(const QLineF& line)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_line(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsLineItem::setLine(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn set_line<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsLineItemSetLineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsLineItem::setPen(const QPen& pen)```</span>
  ///
  ///
  pub fn set_pen(&mut self, pen: &::qt_gui::pen::Pen) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLineItem_setPen(self as *mut ::graphics_line_item::GraphicsLineItem,
                                                   pen as *const ::qt_gui::pen::Pen)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsLineItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_shape_to_output(self as *const ::graphics_line_item::GraphicsLineItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsLineItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_type(self as *const ::graphics_line_item::GraphicsLineItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_line_item::GraphicsLineItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsLineItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_line_item::GraphicsLineItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_line_item::GraphicsLineItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_dynamic_cast_QGraphicsLineItem_ptr(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_line_item::GraphicsLineItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_dynamic_cast_QGraphicsLineItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_line_item::GraphicsLineItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_line_item::GraphicsLineItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_line_item::GraphicsLineItem as *mut ::graphics_line_item::GraphicsLineItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_line_item::GraphicsLineItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_line_item::GraphicsLineItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsLineItem_ptr(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_line_item::GraphicsLineItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsLineItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_line_item::GraphicsLineItem {
  type Target = ::graphics_item::GraphicsItem;
  fn deref(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_line_item::GraphicsLineItem as *mut ::graphics_line_item::GraphicsLineItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_line_item::GraphicsLineItem {
  fn deref_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_line_item::GraphicsLineItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsLineItem::new](../struct.GraphicsLineItem.html#method.new) method.
  pub trait GraphicsLineItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>;
  }
  impl<'a> GraphicsLineItemNewArgs for &'a ::qt_core::line_f::LineF {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {
      let line = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_new_line(line as *const ::qt_core::line_f::LineF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsLineItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsLineItemNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2(x1, y1, x2, y2) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLineItem::new_unsafe](../struct.GraphicsLineItem.html#method.new_unsafe) method.
  pub trait GraphicsLineItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem>;
  }
  impl<'a> GraphicsLineItemNewUnsafeArgs for (&'a ::qt_core::line_f::LineF, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {
      let line = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsLineItem_new_line_parent(line as *const ::qt_core::line_f::LineF,
                                                                             parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsLineItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsLineItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl GraphicsLineItemNewUnsafeArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_line_item::GraphicsLineItem> {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      let parent = self.4;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsLineItem_new_x1_y1_x2_y2_parent(x1, y1, x2, y2, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLineItem::paint](../struct.GraphicsLineItem.html#method.paint) method.
  pub trait GraphicsLineItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> ();
  }
  impl<'largs> GraphicsLineItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsLineItem_paint_painter_option(original_self as *mut ::graphics_line_item::GraphicsLineItem, painter, option)
    }
  }
  impl<'largs> GraphicsLineItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                          *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                          *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsLineItem_paint_painter_option_widget(original_self as *mut ::graphics_line_item::GraphicsLineItem, painter, option, widget)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsLineItem::set_line](../struct.GraphicsLineItem.html#method.set_line) method.
  pub trait GraphicsLineItemSetLineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> ();
  }
  impl<'largs> GraphicsLineItemSetLineArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> () {
      let line = self;
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsLineItem_setLine_line(original_self as *mut ::graphics_line_item::GraphicsLineItem, line as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> GraphicsLineItemSetLineArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_line_item::GraphicsLineItem) -> () {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      unsafe { ::ffi::qt_widgets_c_QGraphicsLineItem_setLine_x1_y1_x2_y2(original_self as *mut ::graphics_line_item::GraphicsLineItem, x1, y1, x2, y2) }
    }
  }
}
