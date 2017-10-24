/// C++ type: <span style='color: green;'>```QGraphicsPixmapItem```</span>
#[repr(C)]
pub struct GraphicsPixmapItem(u8);

impl GraphicsPixmapItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsPixmapItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_boundingRect_to_output(self as *const ::graphics_pixmap_item::GraphicsPixmapItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPixmapItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPixmapItem_contains(self as *const ::graphics_pixmap_item::GraphicsPixmapItem,
                                                       point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPixmapItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsPixmapItem_isObscuredBy(self as *const ::graphics_pixmap_item::GraphicsPixmapItem,
                                                         item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPixmapItem::QGraphicsPixmapItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPixmapItem::QGraphicsPixmapItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::pixmap::Pixmap) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPixmapItem::QGraphicsPixmapItem(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>
    where Args: overloading::GraphicsPixmapItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsPixmapItem::QGraphicsPixmapItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPixmapItem::QGraphicsPixmapItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::pixmap::Pixmap, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPixmapItem::QGraphicsPixmapItem(const QPixmap& pixmap, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>
    where Args: overloading::GraphicsPixmapItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsPixmapItem::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_offset_to_output(self as *const ::graphics_pixmap_item::GraphicsPixmapItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPixmapItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_opaqueArea_to_output(self as *const ::graphics_pixmap_item::GraphicsPixmapItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsPixmapItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn paint(&mut self,
                      painter: *mut ::qt_gui::painter::Painter,
                      option: *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                      widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsPixmapItem_paint(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem,
                                                  painter,
                                                  option,
                                                  widget)
  }

  /// C++ method: <span style='color: green;'>```QPixmap QGraphicsPixmapItem::pixmap() const```</span>
  ///
  ///
  pub fn pixmap(&self) -> ::cpp_utils::CppBox<::qt_gui::pixmap::Pixmap> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_pixmap_as_ptr(self as *const ::graphics_pixmap_item::GraphicsPixmapItem)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPixmapItem::setOffset```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_offset(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsPixmapItem::setOffset(const QPointF& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_offset(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QGraphicsPixmapItem::setOffset(double x, double y)```</span>
  ///
  ///
  pub fn set_offset<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsPixmapItemSetOffsetArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsPixmapItem::setPixmap(const QPixmap& pixmap)```</span>
  ///
  ///
  pub fn set_pixmap(&mut self, pixmap: &::qt_gui::pixmap::Pixmap) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPixmapItem_setPixmap(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem,
                                                        pixmap as *const ::qt_gui::pixmap::Pixmap)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsPixmapItem::setShapeMode(QGraphicsPixmapItem::ShapeMode mode)```</span>
  ///
  ///
  pub fn set_shape_mode(&mut self, mode: ::graphics_pixmap_item::ShapeMode) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPixmapItem_setShapeMode(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem,
                                                           mode)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsPixmapItem::setTransformationMode(Qt::TransformationMode mode)```</span>
  ///
  ///
  pub fn set_transformation_mode(&mut self, mode: &::qt_core::qt::TransformationMode) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_setTransformationMode(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem, mode as *const ::qt_core::qt::TransformationMode) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPixmapItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_shape_to_output(self as *const ::graphics_pixmap_item::GraphicsPixmapItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPixmapItem::ShapeMode QGraphicsPixmapItem::shapeMode() const```</span>
  ///
  ///
  pub fn shape_mode(&self) -> ::graphics_pixmap_item::ShapeMode {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPixmapItem_shapeMode(self as *const ::graphics_pixmap_item::GraphicsPixmapItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsPixmapItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_type(self as *const ::graphics_pixmap_item::GraphicsPixmapItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_pixmap_item::GraphicsPixmapItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsPixmapItem_delete
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsPixmapItem::ShapeMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ShapeMode {
  /// C++ enum variant: <span style='color: green;'>```MaskShape = 0```</span>
  Mask = 0,
  /// C++ enum variant: <span style='color: green;'>```BoundingRectShape = 1```</span>
  BoundingRect = 1,
  /// C++ enum variant: <span style='color: green;'>```HeuristicMaskShape = 2```</span>
  HeuristicMask = 2,
}

impl ::cpp_utils::DynamicCast<::graphics_pixmap_item::GraphicsPixmapItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_pixmap_item::GraphicsPixmapItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_dynamic_cast_QGraphicsPixmapItem_ptr(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_pixmap_item::GraphicsPixmapItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_dynamic_cast_QGraphicsPixmapItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_pixmap_item::GraphicsPixmapItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_pixmap_item::GraphicsPixmapItem as *mut ::graphics_pixmap_item::GraphicsPixmapItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_pixmap_item::GraphicsPixmapItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_pixmap_item::GraphicsPixmapItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsPixmapItem_ptr(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_pixmap_item::GraphicsPixmapItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsPixmapItem_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_pixmap_item::GraphicsPixmapItem {
  type Target = ::graphics_item::GraphicsItem;
  fn deref(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_pixmap_item::GraphicsPixmapItem as *mut ::graphics_pixmap_item::GraphicsPixmapItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_pixmap_item::GraphicsPixmapItem {
  fn deref_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_pixmap_item::GraphicsPixmapItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsPixmapItem::new](../struct.GraphicsPixmapItem.html#method.new) method.
  pub trait GraphicsPixmapItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>;
  }
  impl GraphicsPixmapItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsPixmapItemNewArgs for &'a ::qt_gui::pixmap::Pixmap {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem> {
      let pixmap = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_new_pixmap(pixmap as *const ::qt_gui::pixmap::Pixmap) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPixmapItem::new_unsafe](../struct.GraphicsPixmapItem.html#method.new_unsafe) method.
  pub trait GraphicsPixmapItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem>;
  }
  impl GraphicsPixmapItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsPixmapItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsPixmapItemNewUnsafeArgs for (&'a ::qt_gui::pixmap::Pixmap, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_pixmap_item::GraphicsPixmapItem> {
      let pixmap = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsPixmapItem_new_pixmap_parent(pixmap as *const ::qt_gui::pixmap::Pixmap, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPixmapItem::set_offset](../struct.GraphicsPixmapItem.html#method.set_offset) method.
  pub trait GraphicsPixmapItemSetOffsetArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::graphics_pixmap_item::GraphicsPixmapItem) -> ();
  }
  impl<'largs> GraphicsPixmapItemSetOffsetArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::graphics_pixmap_item::GraphicsPixmapItem) -> () {
      let offset = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_setOffset_offset(original_self as *mut ::graphics_pixmap_item::GraphicsPixmapItem, offset as *const ::qt_core::point_f::PointF) }
    }
  }
  impl<'largs> GraphicsPixmapItemSetOffsetArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::graphics_pixmap_item::GraphicsPixmapItem) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsPixmapItem_setOffset_x_y(original_self as *mut ::graphics_pixmap_item::GraphicsPixmapItem, x, y) }
    }
  }
}
