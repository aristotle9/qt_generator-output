/// C++ type: <span style='color: green;'>```QGraphicsAnchorLayout```</span>
#[repr(C)]
pub struct GraphicsAnchorLayout(u8);

impl GraphicsAnchorLayout {
  /// C++ method: <span style='color: green;'>```QGraphicsAnchor* QGraphicsAnchorLayout::addAnchor(QGraphicsLayoutItem* firstItem, Qt::AnchorPoint firstEdge, QGraphicsLayoutItem* secondItem, Qt::AnchorPoint secondEdge)```</span>
  ///
  ///
  pub unsafe fn add_anchor(&mut self,
                           first_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                           first_edge: &::qt_core::qt::AnchorPoint,
                           second_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                           second_edge: &::qt_core::qt::AnchorPoint)
                           -> *mut ::graphics_anchor::GraphicsAnchor {
    ::ffi::qt_widgets_c_QGraphicsAnchorLayout_addAnchor(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout,
                                                        first_item,
                                                        first_edge as *const ::qt_core::qt::AnchorPoint,
                                                        second_item,
                                                        second_edge as *const ::qt_core::qt::AnchorPoint)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchorLayout::addCornerAnchors(QGraphicsLayoutItem* firstItem, Qt::Corner firstCorner, QGraphicsLayoutItem* secondItem, Qt::Corner secondCorner)```</span>
  ///
  ///
  pub unsafe fn add_corner_anchors(&mut self,
                                   first_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                                   first_corner: &::qt_core::qt::Corner,
                                   second_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                                   second_corner: &::qt_core::qt::Corner) {
    ::ffi::qt_widgets_c_QGraphicsAnchorLayout_addCornerAnchors(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout, first_item, first_corner as *const ::qt_core::qt::Corner, second_item, second_corner as *const ::qt_core::qt::Corner)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsAnchor* QGraphicsAnchorLayout::anchor(QGraphicsLayoutItem* firstItem, Qt::AnchorPoint firstEdge, QGraphicsLayoutItem* secondItem, Qt::AnchorPoint secondEdge)```</span>
  ///
  ///
  pub unsafe fn anchor(&mut self,
                       first_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                       first_edge: &::qt_core::qt::AnchorPoint,
                       second_item: *mut ::graphics_layout_item::GraphicsLayoutItem,
                       second_edge: &::qt_core::qt::AnchorPoint)
                       -> *mut ::graphics_anchor::GraphicsAnchor {
    ::ffi::qt_widgets_c_QGraphicsAnchorLayout_anchor(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout,
                                                     first_item,
                                                     first_edge as *const ::qt_core::qt::AnchorPoint,
                                                     second_item,
                                                     second_edge as *const ::qt_core::qt::AnchorPoint)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsAnchorLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_count(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsAnchorLayout::horizontalSpacing() const```</span>
  ///
  ///
  pub fn horizontal_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_horizontalSpacing(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsAnchorLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_invalidate(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QGraphicsLayoutItem* QGraphicsAnchorLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at(&self, index: ::libc::c_int) -> *mut ::graphics_layout_item::GraphicsLayoutItem {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_itemAt(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout,
                                                       index)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsAnchorLayout::QGraphicsAnchorLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_anchor_layout::GraphicsAnchorLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::graphics_layout_item::GraphicsLayoutItem)
                           -> ::cpp_utils::CppBox<::graphics_anchor_layout::GraphicsAnchorLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchorLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsAnchorLayout::removeAt(int index)```</span>
  ///
  ///
  pub fn remove_at(&mut self, index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_removeAt(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout,
                                                         index)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsAnchorLayout::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_setGeometry(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout, rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchorLayout::setHorizontalSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_horizontal_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_setHorizontalSpacing(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchorLayout::setSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsAnchorLayout_setSpacing(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsAnchorLayout::setVerticalSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_vertical_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_setVerticalSpacing(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsAnchorLayout::verticalSpacing() const```</span>
  ///
  ///
  pub fn vertical_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_verticalSpacing(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_anchor_layout::GraphicsAnchorLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsAnchorLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_anchor_layout::GraphicsAnchorLayout> for ::graphics_layout::GraphicsLayout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_anchor_layout::GraphicsAnchorLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_anchor_layout::GraphicsAnchorLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_anchor_layout::GraphicsAnchorLayout> for ::graphics_layout_item::GraphicsLayoutItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_anchor_layout::GraphicsAnchorLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_anchor_layout::GraphicsAnchorLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_dynamic_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::graphics_layout::GraphicsLayout> for ::graphics_anchor_layout::GraphicsAnchorLayout {
  fn static_cast_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_anchor_layout::GraphicsAnchorLayout {
fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_anchor_layout::GraphicsAnchorLayout> for ::graphics_layout::GraphicsLayout {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_anchor_layout::GraphicsAnchorLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_anchor_layout::GraphicsAnchorLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_anchor_layout::GraphicsAnchorLayout> for ::graphics_layout_item::GraphicsLayoutItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_anchor_layout::GraphicsAnchorLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_anchor_layout::GraphicsAnchorLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsAnchorLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_anchor_layout::GraphicsAnchorLayout {
  type Target = ::graphics_layout::GraphicsLayout;
  fn deref(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_anchor_layout::GraphicsAnchorLayout as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_anchor_layout::GraphicsAnchorLayout {
  fn deref_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsAnchorLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_anchor_layout::GraphicsAnchorLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
