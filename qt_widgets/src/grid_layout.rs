/// C++ type: <span style='color: green;'>```QGridLayout```</span>
#[repr(C)]
pub struct GridLayout(u8);

impl GridLayout {
  /// C++ method: <span style='color: green;'>```void QGridLayout::addWidget(QWidget* w)```</span>
  ///
  ///
  pub unsafe fn add_widget(&mut self, w: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGridLayout_addWidget(self as *mut ::grid_layout::GridLayout, w)
  }

  /// C++ method: <span style='color: green;'>```QRect QGridLayout::cellRect(int row, int column) const```</span>
  ///
  ///
  pub fn cell_rect(&self, row: ::libc::c_int, column: ::libc::c_int) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGridLayout_cellRect_to_output(self as *const ::grid_layout::GridLayout,
                                                           row,
                                                           column,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_columnCount(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::columnMinimumWidth(int column) const```</span>
  ///
  ///
  pub fn column_minimum_width(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_columnMinimumWidth(self as *const ::grid_layout::GridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::columnStretch(int column) const```</span>
  ///
  ///
  pub fn column_stretch(&self, column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_columnStretch(self as *const ::grid_layout::GridLayout, column) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGridLayout::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_count(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::getItemPosition(int idx, int* row, int* column, int* rowSpan, int* columnSpan) const```</span>
  ///
  ///
  pub unsafe fn get_item_position(&self,
                                  idx: ::libc::c_int,
                                  row: *mut ::libc::c_int,
                                  column: *mut ::libc::c_int,
                                  row_span: *mut ::libc::c_int,
                                  column_span: *mut ::libc::c_int) {
    ::ffi::qt_widgets_c_QGridLayout_getItemPosition(self as *const ::grid_layout::GridLayout,
                                                    idx,
                                                    row,
                                                    column,
                                                    row_span,
                                                    column_span)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QGridLayout::hasHeightForWidth() const```</span>
  ///
  ///
  pub fn has_height_for_width(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_hasHeightForWidth(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGridLayout::heightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_heightForWidth(self as *const ::grid_layout::GridLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::horizontalSpacing() const```</span>
  ///
  ///
  pub fn horizontal_spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_horizontalSpacing(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGridLayout::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_invalidate(self as *mut ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QGridLayout::itemAt(int index) const```</span>
  ///
  ///
  pub fn item_at(&self, index: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_itemAt(self as *const ::grid_layout::GridLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```QLayoutItem* QGridLayout::itemAtPosition(int row, int column) const```</span>
  ///
  ///
  pub fn item_at_position(&self, row: ::libc::c_int, column: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_itemAtPosition(self as *const ::grid_layout::GridLayout, row, column) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QGridLayout::maximumSize() const```</span>
  ///
  ///
  pub fn maximum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGridLayout_maximumSize_to_output(self as *const ::grid_layout::GridLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QGridLayout::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_metaObject(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QGridLayout::minimumHeightForWidth(int arg1) const```</span>
  ///
  ///
  pub fn minimum_height_for_width(&self, arg1: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_minimumHeightForWidth(self as *const ::grid_layout::GridLayout, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QGridLayout::minimumSize() const```</span>
  ///
  ///
  pub fn minimum_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGridLayout_minimumSize_to_output(self as *const ::grid_layout::GridLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGridLayout::QGridLayout()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::grid_layout::GridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGridLayout::QGridLayout(QWidget* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::grid_layout::GridLayout> {
    let ffi_result = ::ffi::qt_widgets_c_QGridLayout_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int QGridLayout::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QGridLayout_qt_metacall(self as *mut ::grid_layout::GridLayout,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QGridLayout::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QGridLayout_qt_metacast(self as *mut ::grid_layout::GridLayout, arg1)
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_rowCount(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::rowMinimumHeight(int row) const```</span>
  ///
  ///
  pub fn row_minimum_height(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_rowMinimumHeight(self as *const ::grid_layout::GridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::rowStretch(int row) const```</span>
  ///
  ///
  pub fn row_stretch(&self, row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_rowStretch(self as *const ::grid_layout::GridLayout, row) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setColumnMinimumWidth(int column, int minSize)```</span>
  ///
  ///
  pub fn set_column_minimum_width(&mut self, column: ::libc::c_int, min_size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGridLayout_setColumnMinimumWidth(self as *mut ::grid_layout::GridLayout, column, min_size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setColumnStretch(int column, int stretch)```</span>
  ///
  ///
  pub fn set_column_stretch(&mut self, column: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_setColumnStretch(self as *mut ::grid_layout::GridLayout, column, stretch) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setDefaultPositioning(int n, Qt::Orientation orient)```</span>
  ///
  ///
  pub fn set_default_positioning(&mut self, n: ::libc::c_int, orient: &::qt_core::qt::Orientation) {
    unsafe {
      ::ffi::qt_widgets_c_QGridLayout_setDefaultPositioning(self as *mut ::grid_layout::GridLayout,
                                                            n,
                                                            orient as *const ::qt_core::qt::Orientation)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QGridLayout::setGeometry(const QRect& arg1)```</span>
  ///
  ///
  pub fn set_geometry(&mut self, arg1: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_widgets_c_QGridLayout_setGeometry(self as *mut ::grid_layout::GridLayout,
                                                  arg1 as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setHorizontalSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_horizontal_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_setHorizontalSpacing(self as *mut ::grid_layout::GridLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setOriginCorner(Qt::Corner arg1)```</span>
  ///
  ///
  pub fn set_origin_corner(&mut self, arg1: &::qt_core::qt::Corner) {
    unsafe {
      ::ffi::qt_widgets_c_QGridLayout_setOriginCorner(self as *mut ::grid_layout::GridLayout,
                                                      arg1 as *const ::qt_core::qt::Corner)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setRowMinimumHeight(int row, int minSize)```</span>
  ///
  ///
  pub fn set_row_minimum_height(&mut self, row: ::libc::c_int, min_size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QGridLayout_setRowMinimumHeight(self as *mut ::grid_layout::GridLayout, row, min_size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setRowStretch(int row, int stretch)```</span>
  ///
  ///
  pub fn set_row_stretch(&mut self, row: ::libc::c_int, stretch: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_setRowStretch(self as *mut ::grid_layout::GridLayout, row, stretch) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_setSpacing(self as *mut ::grid_layout::GridLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```void QGridLayout::setVerticalSpacing(int spacing)```</span>
  ///
  ///
  pub fn set_vertical_spacing(&mut self, spacing: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_setVerticalSpacing(self as *mut ::grid_layout::GridLayout, spacing) }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QGridLayout::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGridLayout_sizeHint_to_output(self as *const ::grid_layout::GridLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::spacing() const```</span>
  ///
  ///
  pub fn spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_spacing(self as *const ::grid_layout::GridLayout) }
  }

  /// C++ method: <span style='color: green;'>```virtual QLayoutItem* QGridLayout::takeAt(int index)```</span>
  ///
  ///
  pub fn take_at(&mut self, index: ::libc::c_int) -> *mut ::layout_item::LayoutItem {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_takeAt(self as *mut ::grid_layout::GridLayout, index) }
  }

  /// C++ method: <span style='color: green;'>```static QString QGridLayout::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGridLayout_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QGridLayout::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QGridLayout_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QGridLayout::verticalSpacing() const```</span>
  ///
  ///
  pub fn vertical_spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QGridLayout_verticalSpacing(self as *const ::grid_layout::GridLayout) }
  }
}

impl ::cpp_utils::CppDeletable for ::grid_layout::GridLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGridLayout_delete
  }
}

impl ::cpp_utils::DynamicCast<::grid_layout::GridLayout> for ::layout::Layout {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::grid_layout::GridLayout> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayout(self as *mut ::layout::Layout) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::grid_layout::GridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::grid_layout::GridLayout> for ::layout_item::LayoutItem {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::grid_layout::GridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::grid_layout::GridLayout> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_dynamic_cast_QGridLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::grid_layout::GridLayout {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QObject_ptr(self as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QObject_ptr(self as *const ::grid_layout::GridLayout as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout::Layout> for ::grid_layout::GridLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(self as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(self as *const ::grid_layout::GridLayout as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::layout_item::LayoutItem> for ::grid_layout::GridLayout {
  fn static_cast_mut(&mut self) -> &mut ::layout_item::LayoutItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayoutItem_ptr(self as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::layout_item::LayoutItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayoutItem_ptr(self as *const ::grid_layout::GridLayout as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::grid_layout::GridLayout> for ::layout::Layout {
  unsafe fn static_cast_mut(&mut self) -> &mut ::grid_layout::GridLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayout(self as *mut ::layout::Layout);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::grid_layout::GridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayout(self as *const ::layout::Layout as *mut ::layout::Layout);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::grid_layout::GridLayout> for ::layout_item::LayoutItem {
  unsafe fn static_cast_mut(&mut self) -> &mut ::grid_layout::GridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayoutItem(self as *mut ::layout_item::LayoutItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::grid_layout::GridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QLayoutItem(self as *const ::layout_item::LayoutItem as *mut ::layout_item::LayoutItem);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::grid_layout::GridLayout> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::grid_layout::GridLayout {
    let ffi_result =
      ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::grid_layout::GridLayout {
    let ffi_result = ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QGridLayout_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::grid_layout::GridLayout {
  type Target = ::layout::Layout;
  fn deref(&self) -> &::layout::Layout {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(self as *const ::grid_layout::GridLayout as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::grid_layout::GridLayout {
  fn deref_mut(&mut self) -> &mut ::layout::Layout {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGridLayout_G_static_cast_QLayout_ptr(self as *mut ::grid_layout::GridLayout) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
