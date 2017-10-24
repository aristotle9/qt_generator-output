/// C++ type: <span style='color: green;'>```QLayoutItem```</span>
#[repr(C)]
pub struct LayoutItem(u8);

impl LayoutItem {
  /// C++ method: <span style='color: green;'>```pure virtual QRect QLayoutItem::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayoutItem_geometry_to_output(self as *const ::layout_item::LayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QLayoutItem::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_hasHeightForWidth(self as *const ::layout_item::LayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLayoutItem::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_heightForWidth(self as *const ::layout_item::LayoutItem, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QLayoutItem::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_invalidate(self as *mut ::layout_item::LayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QLayoutItem::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_isEmpty(self as *const ::layout_item::LayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayout* QLayoutItem::layout()```</span>
  ///
  ///
  pub fn layout(&mut self) -> *mut ::layout::Layout {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_layout(self as *mut ::layout_item::LayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSize QLayoutItem::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayoutItem_maximumSize_to_output(self as *const ::layout_item::LayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QLayoutItem::minimumHeightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn minimum_height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_minimumHeightForWidth(self as *const ::layout_item::LayoutItem, arg1) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSize QLayoutItem::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayoutItem_minimumSize_to_output(self as *const ::layout_item::LayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QLayoutItem::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QLayoutItem_setGeometry(self as *mut ::layout_item::LayoutItem,
                                                  arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QSize QLayoutItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QLayoutItem_sizeHint_to_output(self as *const ::layout_item::LayoutItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSpacerItem* QLayoutItem::spacerItem()```</span>
  ///
  ///
  pub fn spacer_item(&mut self) -> *mut ::spacer_item::SpacerItem {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_spacerItem(self as *mut ::layout_item::LayoutItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual QWidget* QLayoutItem::widget()```</span>
  ///
  ///
  pub fn widget(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QLayoutItem_widget(self as *mut ::layout_item::LayoutItem) }
  }
}

impl ::cpp_utils::CppDeletable for ::layout_item::LayoutItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QLayoutItem_delete
  }
}
