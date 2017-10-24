/// C++ type: <span style='color: green;'>```QGraphicsItemGroup```</span>
#[repr(C)]
pub struct GraphicsItemGroup(u8);

impl GraphicsItemGroup {
  /// C++ method: <span style='color: green;'>```void QGraphicsItemGroup::addToGroup(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn add_to_group(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItemGroup_addToGroup(self as *mut ::graphics_item_group::GraphicsItemGroup, item)
  }

  /// C++ method: <span style='color: green;'>```virtual QRectF QGraphicsItemGroup::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemGroup_boundingRect_to_output(self as *const ::graphics_item_group::GraphicsItemGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGraphicsItemGroup::isObscuredBy(const QGraphicsItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_obscured_by(&self, item: *const ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QGraphicsItemGroup_isObscuredBy(self as *const ::graphics_item_group::GraphicsItemGroup,
                                                        item)
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsItemGroup::QGraphicsItemGroup()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_item_group::GraphicsItemGroup> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsItemGroup::QGraphicsItemGroup(QGraphicsItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::graphics_item::GraphicsItem)
                           -> ::cpp_utils::CppBox<::graphics_item_group::GraphicsItemGroup> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemGroup_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual QPainterPath QGraphicsItemGroup::opaqueArea() const```</span>
  ///
  ///
  pub fn opaque_area(&self) -> ::qt_gui::painter_path::PainterPath {
    {
      let mut object: ::qt_gui::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsItemGroup_opaqueArea_to_output(self as *const ::graphics_item_group::GraphicsItemGroup, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItemGroup::paint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsItemGroup::paint(QPainter* painter, const QStyleOptionGraphicsItem* option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn paint(&mut self, (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem, *mut ::widget::Widget)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QGraphicsItemGroup::paint(QPainter* painter, const QStyleOptionGraphicsItem* option, QWidget* widget = ?)```</span>
  ///
  ///
  pub unsafe fn paint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::GraphicsItemGroupPaintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QGraphicsItemGroup::removeFromGroup(QGraphicsItem* item)```</span>
  ///
  ///
  pub unsafe fn remove_from_group(&mut self, item: *mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QGraphicsItemGroup_removeFromGroup(self as *mut ::graphics_item_group::GraphicsItemGroup, item)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsItemGroup::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_type(self as *const ::graphics_item_group::GraphicsItemGroup) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_item_group::GraphicsItemGroup {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsItemGroup_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_item_group::GraphicsItemGroup> for ::graphics_item::GraphicsItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_item_group::GraphicsItemGroup> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_dynamic_cast_QGraphicsItemGroup_ptr(self as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_item_group::GraphicsItemGroup> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_dynamic_cast_QGraphicsItemGroup_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::graphics_item::GraphicsItem> for ::graphics_item_group::GraphicsItemGroup {
  fn static_cast_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_item_group::GraphicsItemGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_item_group::GraphicsItemGroup as *mut ::graphics_item_group::GraphicsItemGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_item_group::GraphicsItemGroup> for ::graphics_item::GraphicsItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_item_group::GraphicsItemGroup {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItemGroup_ptr(self as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_item_group::GraphicsItemGroup {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItemGroup_ptr(self as *const ::graphics_item::GraphicsItem as *mut ::graphics_item::GraphicsItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_item_group::GraphicsItemGroup {
  type Target = ::graphics_item::GraphicsItem;
  fn deref(&self) -> &::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(self as *const ::graphics_item_group::GraphicsItemGroup as *mut ::graphics_item_group::GraphicsItemGroup) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_item_group::GraphicsItemGroup {
  fn deref_mut(&mut self) -> &mut ::graphics_item::GraphicsItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsItemGroup_G_static_cast_QGraphicsItem_ptr(self as *mut ::graphics_item_group::GraphicsItemGroup) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsItemGroup::paint](../struct.GraphicsItemGroup.html#method.paint) method.
  pub trait GraphicsItemGroupPaintArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item_group::GraphicsItemGroup) -> ();
  }
  impl<'largs> GraphicsItemGroupPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter, *const ::style_option_graphics_item::StyleOptionGraphicsItem) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item_group::GraphicsItemGroup) -> () {
      let painter = self.0;
      let option = self.1;
      ::ffi::qt_widgets_c_QGraphicsItemGroup_paint_painter_option(original_self as *mut ::graphics_item_group::GraphicsItemGroup, painter, option)
    }
  }
  impl<'largs> GraphicsItemGroupPaintArgs<'largs>
    for (*mut ::qt_gui::painter::Painter,
                                                           *const ::style_option_graphics_item::StyleOptionGraphicsItem,
                                                           *mut ::widget::Widget) {
    unsafe fn exec(self, original_self: &'largs mut ::graphics_item_group::GraphicsItemGroup) -> () {
      let painter = self.0;
      let option = self.1;
      let widget = self.2;
      ::ffi::qt_widgets_c_QGraphicsItemGroup_paint_painter_option_widget(original_self as *mut ::graphics_item_group::GraphicsItemGroup, painter, option, widget)
    }
  }
}
