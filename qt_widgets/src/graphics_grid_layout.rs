/// C++ type: <span style='color: green;'>```QGraphicsGridLayout```</span>
#[repr(C)]
pub struct GraphicsGridLayout(u8);

impl GraphicsGridLayout {
  /// C++ method: <span style='color: green;'>```int QGraphicsGridLayout::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_columnCount(self as *const ::graphics_grid_layout::GraphicsGridLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::columnMaximumWidth(int column) const```</span>
  ///
  ///
  pub fn column_maximum_width(&self, column: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_columnMaximumWidth(self as *const ::graphics_grid_layout::GraphicsGridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::columnMinimumWidth(int column) const```</span>
  ///
  ///
  pub fn column_minimum_width(&self, column: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_columnMinimumWidth(self as *const ::graphics_grid_layout::GraphicsGridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::columnPreferredWidth(int column) const```</span>
  ///
  ///
  pub fn column_preferred_width(&self, column: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_columnPreferredWidth(self as *const ::graphics_grid_layout::GraphicsGridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::columnSpacing(int column) const```</span>
  ///
  ///
  pub fn column_spacing(&self, column: ::libc::c_int) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_columnSpacing(self as *const ::graphics_grid_layout::GraphicsGridLayout,
                                                            column)
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsGridLayout::columnStretchFactor(int column) const```</span>
  ///
  ///
  pub fn column_stretch_factor(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_columnStretchFactor(self as *const ::graphics_grid_layout::GraphicsGridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGraphicsGridLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_count(self as *const ::graphics_grid_layout::GraphicsGridLayout) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::horizontalSpacing() const```</span>
  ///
  ///
  pub fn horizontal_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_horizontalSpacing(self as *const ::graphics_grid_layout::GraphicsGridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsGridLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_invalidate(self as *mut ::graphics_grid_layout::GraphicsGridLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsGridLayout::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::graphics_layout_item::GraphicsLayoutItem```<br>
  /// C++ method: <span style='color: green;'>```QGraphicsLayoutItem* QGraphicsGridLayout::itemAt(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, ::libc::c_int) -> *mut ::graphics_layout_item::GraphicsLayoutItem```<br>
  /// C++ method: <span style='color: green;'>```virtual QGraphicsLayoutItem* QGraphicsGridLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::graphics_layout_item::GraphicsLayoutItem
    where Args: overloading::GraphicsGridLayoutItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsGridLayout::QGraphicsGridLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_grid_layout::GraphicsGridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsGridLayout::QGraphicsGridLayout(QGraphicsLayoutItem* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::graphics_layout_item::GraphicsLayoutItem)
                           -> ::cpp_utils::CppBox<::graphics_grid_layout::GraphicsGridLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsGridLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsGridLayout::removeAt(int index)```</span>
  ///
  ///
  pub fn remove_at(&mut self, index: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_removeAt(self as *mut ::graphics_grid_layout::GraphicsGridLayout,
                                                       index)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem* item)```</span>
  ///
  ///
  pub unsafe fn remove_item(&mut self, item: *mut ::graphics_layout_item::GraphicsLayoutItem) {
    ::ffi::qt_widgets_c_QGraphicsGridLayout_removeItem(self as *mut ::graphics_grid_layout::GraphicsGridLayout,
                                                       item)
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsGridLayout::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_rowCount(self as *const ::graphics_grid_layout::GraphicsGridLayout)
    }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::rowMaximumHeight(int row) const```</span>
  ///
  ///
  pub fn row_maximum_height(&self, row: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_rowMaximumHeight(self as *const ::graphics_grid_layout::GraphicsGridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::rowMinimumHeight(int row) const```</span>
  ///
  ///
  pub fn row_minimum_height(&self, row: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_rowMinimumHeight(self as *const ::graphics_grid_layout::GraphicsGridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::rowPreferredHeight(int row) const```</span>
  ///
  ///
  pub fn row_preferred_height(&self, row: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_rowPreferredHeight(self as *const ::graphics_grid_layout::GraphicsGridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::rowSpacing(int row) const```</span>
  ///
  ///
  pub fn row_spacing(&self, row: ::libc::c_int) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_rowSpacing(self as *const ::graphics_grid_layout::GraphicsGridLayout,
                                                         row)
    }
  }

  /// C++ method: <span style='color: green;'>```int QGraphicsGridLayout::rowStretchFactor(int row) const```</span>
  ///
  ///
  pub fn row_stretch_factor(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_rowStretchFactor(self as *const ::graphics_grid_layout::GraphicsGridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnFixedWidth(int column, double width)```</span>
  ///
  ///
  pub fn set_column_fixed_width(&mut self, column: ::libc::c_int, width: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnFixedWidth(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnMaximumWidth(int column, double width)```</span>
  ///
  ///
  pub fn set_column_maximum_width(&mut self, column: ::libc::c_int, width: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnMaximumWidth(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnMinimumWidth(int column, double width)```</span>
  ///
  ///
  pub fn set_column_minimum_width(&mut self, column: ::libc::c_int, width: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnMinimumWidth(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnPreferredWidth(int column, double width)```</span>
  ///
  ///
  pub fn set_column_preferred_width(&mut self, column: ::libc::c_int, width: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnPreferredWidth(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, width) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnSpacing(int column, double spacing)```</span>
  ///
  ///
  pub fn set_column_spacing(&mut self, column: ::libc::c_int, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnSpacing(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch)```</span>
  ///
  ///
  pub fn set_column_stretch_factor(&mut self, column: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setColumnStretchFactor(self as *mut ::graphics_grid_layout::GraphicsGridLayout, column, stretch) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGraphicsGridLayout::setGeometry(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_setGeometry(self as *mut ::graphics_grid_layout::GraphicsGridLayout,
                                                          rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setHorizontalSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_horizontal_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setHorizontalSpacing(self as *mut ::graphics_grid_layout::GraphicsGridLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowFixedHeight(int row, double height)```</span>
  ///
  ///
  pub fn set_row_fixed_height(&mut self, row: ::libc::c_int, height: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowFixedHeight(self as *mut ::graphics_grid_layout::GraphicsGridLayout, row, height)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowMaximumHeight(int row, double height)```</span>
  ///
  ///
  pub fn set_row_maximum_height(&mut self, row: ::libc::c_int, height: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowMaximumHeight(self as *mut ::graphics_grid_layout::GraphicsGridLayout, row, height) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowMinimumHeight(int row, double height)```</span>
  ///
  ///
  pub fn set_row_minimum_height(&mut self, row: ::libc::c_int, height: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowMinimumHeight(self as *mut ::graphics_grid_layout::GraphicsGridLayout, row, height) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowPreferredHeight(int row, double height)```</span>
  ///
  ///
  pub fn set_row_preferred_height(&mut self, row: ::libc::c_int, height: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowPreferredHeight(self as *mut ::graphics_grid_layout::GraphicsGridLayout, row, height) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowSpacing(int row, double spacing)```</span>
  ///
  ///
  pub fn set_row_spacing(&mut self, row: ::libc::c_int, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowSpacing(self as *mut ::graphics_grid_layout::GraphicsGridLayout,
                                                            row,
                                                            spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch)```</span>
  ///
  ///
  pub fn set_row_stretch_factor(&mut self, row: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setRowStretchFactor(self as *mut ::graphics_grid_layout::GraphicsGridLayout, row, stretch) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_setSpacing(self as *mut ::graphics_grid_layout::GraphicsGridLayout,
                                                         spacing)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsGridLayout::setVerticalSpacing(double spacing)```</span>
  ///
  ///
  pub fn set_vertical_spacing(&mut self, spacing: ::libc::c_double) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_setVerticalSpacing(self as *mut ::graphics_grid_layout::GraphicsGridLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsGridLayout::sizeHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size_hint(&self, &::qt_core::qt::SizeHint) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```virtual QSizeF QGraphicsGridLayout::sizeHint(Qt::SizeHint which) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size_hint(&self, (&::qt_core::qt::SizeHint, &::qt_core::size_f::SizeF)) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```virtual QSizeF QGraphicsGridLayout::sizeHint(Qt::SizeHint which, const QSizeF& constraint = ?) const```</span>
  ///
  ///
  pub fn size_hint<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size_f::SizeF
    where Args: overloading::GraphicsGridLayoutSizeHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QGraphicsGridLayout::verticalSpacing() const```</span>
  ///
  ///
  pub fn vertical_spacing(&self) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_widgets_c_QGraphicsGridLayout_verticalSpacing(self as *const ::graphics_grid_layout::GraphicsGridLayout)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_grid_layout::GraphicsGridLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsGridLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_grid_layout::GraphicsGridLayout> for ::graphics_layout::GraphicsLayout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_grid_layout::GraphicsGridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_grid_layout::GraphicsGridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::graphics_grid_layout::GraphicsGridLayout> for ::graphics_layout_item::GraphicsLayoutItem {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_grid_layout::GraphicsGridLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_grid_layout::GraphicsGridLayout> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_dynamic_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::graphics_layout::GraphicsLayout> for ::graphics_grid_layout::GraphicsGridLayout {
  fn static_cast_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_grid_layout::GraphicsGridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_grid_layout::GraphicsGridLayout as *mut ::graphics_grid_layout::GraphicsGridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_layout_item::GraphicsLayoutItem> for ::graphics_grid_layout::GraphicsGridLayout {
fn static_cast_mut(&mut self) -> &mut ::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *mut ::graphics_grid_layout::GraphicsGridLayout) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_layout_item::GraphicsLayoutItem {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayoutItem_ptr(self as *const ::graphics_grid_layout::GraphicsGridLayout as *mut ::graphics_grid_layout::GraphicsGridLayout) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_grid_layout::GraphicsGridLayout> for ::graphics_layout::GraphicsLayout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_grid_layout::GraphicsGridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(self as *mut ::graphics_layout::GraphicsLayout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_grid_layout::GraphicsGridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayout(self as *const ::graphics_layout::GraphicsLayout as *mut ::graphics_layout::GraphicsLayout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_grid_layout::GraphicsGridLayout> for ::graphics_layout_item::GraphicsLayoutItem {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_grid_layout::GraphicsGridLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(self as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_grid_layout::GraphicsGridLayout {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsGridLayout_ptr_QGraphicsLayoutItem(self as *const ::graphics_layout_item::GraphicsLayoutItem as *mut ::graphics_layout_item::GraphicsLayoutItem);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_grid_layout::GraphicsGridLayout {
  type Target = ::graphics_layout::GraphicsLayout;
  fn deref(&self) -> &::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayout_ptr(self as *const ::graphics_grid_layout::GraphicsGridLayout as *mut ::graphics_grid_layout::GraphicsGridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_grid_layout::GraphicsGridLayout {
  fn deref_mut(&mut self) -> &mut ::graphics_layout::GraphicsLayout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_G_static_cast_QGraphicsLayout_ptr(self as *mut ::graphics_grid_layout::GraphicsGridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsGridLayout::item_at](../struct.GraphicsGridLayout.html#method.item_at) method.
  pub trait GraphicsGridLayoutItemAtArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout)
            -> *mut ::graphics_layout_item::GraphicsLayoutItem;
  }
  impl<'largs> GraphicsGridLayoutItemAtArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout)
            -> *mut ::graphics_layout_item::GraphicsLayoutItem {
      let index = self;
      unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_itemAt_index(original_self as *const ::graphics_grid_layout::GraphicsGridLayout, index) }
    }
  }
  impl<'largs> GraphicsGridLayoutItemAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout)
            -> *mut ::graphics_layout_item::GraphicsLayoutItem {
      let row = self.0;
      let column = self.1;
      unsafe { ::ffi::qt_widgets_c_QGraphicsGridLayout_itemAt_row_column(original_self as *const ::graphics_grid_layout::GraphicsGridLayout, row, column) }
    }
  }
  /// This trait represents a set of arguments accepted by [GraphicsGridLayout::size_hint](../struct.GraphicsGridLayout.html#method.size_hint) method.
  pub trait GraphicsGridLayoutSizeHintArgs<'largs> {
    fn exec(self, original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout) -> ::qt_core::size_f::SizeF;
  }
  impl<'largs> GraphicsGridLayoutSizeHintArgs<'largs> for &'largs ::qt_core::qt::SizeHint {
    fn exec(self, original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout) -> ::qt_core::size_f::SizeF {
      let which = self;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsGridLayout_sizeHint_to_output_which(original_self as *const ::graphics_grid_layout::GraphicsGridLayout, which as *const ::qt_core::qt::SizeHint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> GraphicsGridLayoutSizeHintArgs<'largs>
    for (&'largs ::qt_core::qt::SizeHint, &'largs ::qt_core::size_f::SizeF) {
    fn exec(self, original_self: &'largs ::graphics_grid_layout::GraphicsGridLayout) -> ::qt_core::size_f::SizeF {
      let which = self.0;
      let constraint = self.1;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QGraphicsGridLayout_sizeHint_to_output_which_constraint(original_self as *const ::graphics_grid_layout::GraphicsGridLayout, which as *const ::qt_core::qt::SizeHint, constraint as *const ::qt_core::size_f::SizeF, &mut object);
        }
        object
      }
    }
  }
}
