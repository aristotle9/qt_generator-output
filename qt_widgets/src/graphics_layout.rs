/// C++ type: <span style='color: green;'>```QGraphicsLayout```</span>
#[repr(C)]
pub struct GraphicsLayout(u8);

impl GraphicsLayout {
  /// C++ method: <span style='color: green;'>```void QGraphicsLayout::activate()```</span>
  ///
  ///
  pub fn activate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_activate(self as *mut ::graphics_layout::GraphicsLayout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual int QGraphicsLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_count(self as *const ::graphics_layout::GraphicsLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayout::getContentsMargins(double* left, double* top, double* right, double* bottom) const```</span>
  ///
  ///
  pub unsafe fn get_contents_margins(&self,
                                     left: *mut ::libc::c_double,
                                     top: *mut ::libc::c_double,
                                     right: *mut ::libc::c_double,
                                     bottom: *mut ::libc::c_double) {
    ::ffi::qt_widgets_c_QGraphicsLayout_getContentsMargins(self as *const ::graphics_layout::GraphicsLayout,
                                                           left,
                                                           top,
                                                           right,
                                                           bottom)
  }

  /// C++ method: <span style='color: green;'>```static bool QGraphicsLayout::instantInvalidatePropagation()```</span>
  ///
  ///
  pub fn instant_invalidate_propagation() -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_instantInvalidatePropagation() }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_invalidate(self as *mut ::graphics_layout::GraphicsLayout) }
  }

  /// C++ method: <span style='color: green;'>```bool QGraphicsLayout::isActivated() const```</span>
  ///
  ///
  pub fn is_activated(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_isActivated(self as *const ::graphics_layout::GraphicsLayout) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QGraphicsLayoutItem* QGraphicsLayout::itemAt(int i) const```</span>
  ///
  ///
  pub fn item_at(&self, i: ::libc::c_int) -> *mut ::graphics_layout_item::GraphicsLayoutItem {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_itemAt(self as *const ::graphics_layout::GraphicsLayout, i) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QGraphicsLayout::removeAt(int index)```</span>
  ///
  ///
  pub fn remove_at(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_removeAt(self as *mut ::graphics_layout::GraphicsLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsLayout::setContentsMargins(double left, double top, double right, double bottom)```</span>
  ///
  ///
  pub fn set_contents_margins(&mut self,
                              left: ::libc::c_double,
                              top: ::libc::c_double,
                              right: ::libc::c_double,
                              bottom: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsLayout_setContentsMargins(self as *mut ::graphics_layout::GraphicsLayout,
                                                             left,
                                                             top,
                                                             right,
                                                             bottom)
    }
  }

  /// C++ method: <span style='color: green;'>```static void QGraphicsLayout::setInstantInvalidatePropagation(bool enable)```</span>
  ///
  ///
  pub fn set_instant_invalidate_propagation(enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_setInstantInvalidatePropagation(enable) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayout::updateGeometry()```</span>
  ///
  ///
  pub fn update_geometry(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_updateGeometry(self as *mut ::graphics_layout::GraphicsLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsLayout::widgetEvent(QEvent* e)```</span>
  ///
  ///
  pub unsafe fn widget_event(&mut self, e: *mut ::qt_core::event::Event) {
    ::ffi::qt_widgets_c_QGraphicsLayout_widgetEvent(self as *mut ::graphics_layout::GraphicsLayout, e)
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_layout::GraphicsLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_layout::GraphicsLayout> for ::graphics_layout_item::GraphicsLayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_layout::GraphicsLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_dynamic_cast_QGraphicsLayout_ptr(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_layout::GraphicsLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_dynamic_cast_QGraphicsLayout_ptr(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_layout::GraphicsLayout {
  fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_layout::GraphicsLayout> for ::graphics_layout_item::GraphicsLayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_layout::GraphicsLayout {
  type Target = ::graphics_layout_item::GraphicsLayoutItem;
  fn deref(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_layout::GraphicsLayout {
  fn deref_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
