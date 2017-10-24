/// C++ type: <span style='color: green;'>```QWidgetItem```</span>
#[repr(C)]
pub struct WidgetItem(u8);

impl WidgetItem {
  /// C++ method: <span style='color: green;'>```virtual QRect QWidgetItem::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidgetItem_geometry_to_output(self as *const ::widget_item::WidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWidgetItem::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidgetItem_hasHeightForWidth(self as *const ::widget_item::WidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QWidgetItem::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QWidgetItem_heightForWidth(self as *const ::widget_item::WidgetItem, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QWidgetItem::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QWidgetItem_isEmpty(self as *const ::widget_item::WidgetItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWidgetItem::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidgetItem_maximumSize_to_output(self as *const ::widget_item::WidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWidgetItem::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidgetItem_minimumSize_to_output(self as *const ::widget_item::WidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWidgetItem::QWidgetItem(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn new(w: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::widget_item::WidgetItem> {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetItem_new(w);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual void QWidgetItem::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QWidgetItem_setGeometry(self as *mut ::widget_item::WidgetItem,
                                                  arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QWidgetItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QWidgetItem_sizeHint_to_output(self as *const ::widget_item::WidgetItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QWidget* QWidgetItem::widget()```</span>
  ///
  ///
  pub fn widget(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QWidgetItem_widget(self as *mut ::widget_item::WidgetItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::widget_item::WidgetItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QWidgetItem_delete
  }
}

impl ::cpp_utils::DynamicCast<::widget_item::WidgetItem> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::widget_item::WidgetItem> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_dynamic_cast_QWidgetItem_ptr(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::widget_item::WidgetItem> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_dynamic_cast_QWidgetItem_ptr(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::widget_item::WidgetItem {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(self as *mut ::widget_item::WidgetItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(self as *const ::widget_item::WidgetItem as *mut ::widget_item::WidgetItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::widget_item::WidgetItem> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::widget_item::WidgetItem {
    let ffi_result =
      ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QWidgetItem_ptr(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::widget_item::WidgetItem {
    let ffi_result = ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QWidgetItem_ptr(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::widget_item::WidgetItem {
  type Target = ::layout_item::LayoutItem;
  fn deref(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(self as *const ::widget_item::WidgetItem as *mut ::widget_item::WidgetItem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::widget_item::WidgetItem {
  fn deref_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QWidgetItem_G_static_cast_QLayoutItem_ptr(self as *mut ::widget_item::WidgetItem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
