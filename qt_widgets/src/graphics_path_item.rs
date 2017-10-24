/// C++ type: <span style='color: green;'>```QGraphicsPathItem```</span>
#[repr(C)]
pub struct GraphicsPathItem(u8);

impl GraphicsPathItem {
  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsPathItem::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPathItem_boundingRect_to_output(self as *const ::graphics_path_item::GraphicsPathItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPathItem::contains(const QPointF& point) const```</span>
  ///
  ///
  pub fn contains(&self, point: &::qt_core::point_f::PointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPathItem_contains(self as *const ::graphics_path_item::GraphicsPathItem,
                                                     point as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsPathItem_isObscuredBy(self as *const ::graphics_path_item::GraphicsPathItem, item)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPathItem::QGraphicsPathItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPathItem::QGraphicsPathItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_gui::painter_path::PainterPath) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPathItem::QGraphicsPathItem(const QPainterPath& path)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>
    where Args: overloading::GraphicsPathItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QGraphicsPathItem::QGraphicsPathItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::graphics_item::GraphicsItem) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPathItem::QGraphicsPathItem(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_gui::painter_path::PainterPath, *mut ::graphics_item::GraphicsItem)) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsPathItem::QGraphicsPathItem(const QPainterPath& path, QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>
    where Args: overloading::GraphicsPathItemNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPathItem::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPathItem_opaqueArea_to_output(self as *const ::graphics_path_item::GraphicsPathItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsPathItem::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsPathItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsPathItem::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsPathItemPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath QGraphicsPathItem::path() const```</span>
  ///
  ///
  pub fn path(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPathItem_path_to_output(self as *const ::graphics_path_item::GraphicsPathItem,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsPathItem::setPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn set_path(&mut self, path: &::qt_gui::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsPathItem_setPath(self as *mut ::graphics_path_item::GraphicsPathItem,
                                                    path as *const ::qt_gui::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsPathItem::shape() const```</span>
  ///
  ///
  pub fn shape(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsPathItem_shape_to_output(self as *const ::graphics_path_item::GraphicsPathItem,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsPathItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_type(self as *const ::graphics_path_item::GraphicsPathItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_path_item::GraphicsPathItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsPathItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_path_item::GraphicsPathItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_path_item::GraphicsPathItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_path_item::GraphicsPathItem> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::graphics_path_item::GraphicsPathItem> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_path_item::GraphicsPathItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_path_item::GraphicsPathItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_dynamic_cast_QGraphicsPathItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_graphics_shape_item::AbstractGraphicsShapeItem> for ::graphics_path_item::GraphicsPathItem {
fn static_cast_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_path_item::GraphicsPathItem) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_path_item::GraphicsPathItem as *mut ::graphics_path_item::GraphicsPathItem) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_path_item::GraphicsPathItem {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_path_item::GraphicsPathItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_path_item::GraphicsPathItem as *mut ::graphics_path_item::GraphicsPathItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_path_item::GraphicsPathItem> for ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_path_item::GraphicsPathItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(self as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_path_item::GraphicsPathItem {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QAbstractGraphicsShapeItem(self as *const ::abstract_graphics_shape_item::AbstractGraphicsShapeItem as *mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_path_item::GraphicsPathItem> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_path_item::GraphicsPathItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QGraphicsItem(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_path_item::GraphicsPathItem {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QGraphicsPathItem_ptr_QGraphicsItem(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_path_item::GraphicsPathItem {
  type Target = ::abstract_graphics_shape_item::AbstractGraphicsShapeItem;
  fn deref(&self) -> &::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *const ::graphics_path_item::GraphicsPathItem as *mut ::graphics_path_item::GraphicsPathItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_path_item::GraphicsPathItem {
  fn deref_mut(&mut self) -> &mut ::abstract_graphics_shape_item::AbstractGraphicsShapeItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_G_static_cast_QAbstractGraphicsShapeItem_ptr(self as *mut ::graphics_path_item::GraphicsPathItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsPathItem::new](../struct.GraphicsPathItem.html#method.new) method.
  pub trait GraphicsPathItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>;
  }
  impl GraphicsPathItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> GraphicsPathItemNewArgs for &'a ::qt_gui::painter_path::PainterPath {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem> {
      let path = self;
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_QGraphicsPathItem_new_path(path as *const ::qt_gui::painter_path::PainterPath) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPathItem::new_unsafe](../struct.GraphicsPathItem.html#method.new_unsafe) method.
  pub trait GraphicsPathItemNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem>;
  }
  impl GraphicsPathItemNewUnsafeArgs for *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QGraphicsPathItem_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> GraphicsPathItemNewUnsafeArgs
    for (&'a ::qt_gui::painter_path::PainterPath, *mut ::graphics_item::GraphicsItem) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::graphics_path_item::GraphicsPathItem> {
      let path = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QGraphicsPathItem_new_path_parent(path as *const ::qt_gui::painter_path::PainterPath,
                                                              parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsPathItem::paint](../struct.GraphicsPathItem.html#method.paint) method.
  pub trait GraphicsPathItemPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_path_item::GraphicsPathItem) -> ();
  }
  impl<'largs> GraphicsPathItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_path_item::GraphicsPathItem) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsPathItem_paint_painter_option(original_self as *mut ::graphics_path_item::GraphicsPathItem, painter, option)
    }
  }
  impl<'largs> GraphicsPathItemPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                          *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                          *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_path_item::GraphicsPathItem) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsPathItem_paint_painter_option_widget(original_self as *mut ::graphics_path_item::GraphicsPathItem, painter, option, widget)
    }
  }
}
