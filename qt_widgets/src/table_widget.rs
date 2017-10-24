/// C++ type: <span style='color: green;'>```QTableWidget```</span>
#[repr(C)]
pub struct TableWidget(u8);

impl TableWidget {
  /// C++ method: <span style='color: green;'>```QWidget* QTableWidget::cellWidget(int row, int column) const```</span>
  ///
  ///
  pub fn cell_widget(&self, row: ::libc::c_int, column: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_cellWidget(self as *const ::table_widget::TableWidget, row, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_clear(self as *mut ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::clearContents()```</span>
  ///
  ///
  pub fn clear_contents(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_clearContents(self as *mut ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::closePersistentEditor(QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn close_persistent_editor(&mut self, item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_closePersistentEditor(self as *mut ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::column(const QTableWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn column(&self, item: *const ::table_widget_item::TableWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTableWidget_column(self as *const ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_columnCount(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::currentColumn() const```</span>
  ///
  ///
  pub fn current_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_currentColumn(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::currentItem() const```</span>
  ///
  ///
  pub fn current_item(&self) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_currentItem(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::currentRow() const```</span>
  ///
  ///
  pub fn current_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_currentRow(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::editItem(QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn edit_item(&mut self, item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_editItem(self as *mut ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::horizontalHeaderItem(int column) const```</span>
  ///
  ///
  pub fn horizontal_header_item(&self, column: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_horizontalHeaderItem(self as *const ::table_widget::TableWidget, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::insertColumn(int column)```</span>
  ///
  ///
  pub fn insert_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_insertColumn(self as *mut ::table_widget::TableWidget, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::insertRow(int row)```</span>
  ///
  ///
  pub fn insert_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_insertRow(self as *mut ::table_widget::TableWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```bool QTableWidget::isItemSelected(const QTableWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_selected(&self, item: *const ::table_widget_item::TableWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QTableWidget_isItemSelected(self as *const ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QTableWidget::isSortingEnabled() const```</span>
  ///
  ///
  pub fn is_sorting_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_isSortingEnabled(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::item(int row, int column) const```</span>
  ///
  ///
  pub fn item(&self, row: ::libc::c_int, column: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_item(self as *const ::table_widget::TableWidget, row, column) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidget::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, &::qt_core::point::Point) -> *mut ::table_widget_item::TableWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::itemAt(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::table_widget_item::TableWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::itemAt(int x, int y) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::table_widget_item::TableWidgetItem
    where Args: overloading::TableWidgetItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTableWidgetItem* QTableWidget::itemPrototype() const```</span>
  ///
  ///
  pub fn item_prototype(&self) -> *const ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_itemPrototype(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTableWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_metaObject(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidget::QTableWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::table_widget::TableWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidget::QTableWidget()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::table_widget::TableWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidget::QTableWidget(int rows, int columns)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::table_widget::TableWidget>
    where Args: overloading::TableWidgetNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTableWidget::QTableWidget```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::widget::Widget) -> ::cpp_utils::CppBox<::table_widget::TableWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidget::QTableWidget(QWidget* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_int, ::libc::c_int, *mut ::widget::Widget)) -> ::cpp_utils::CppBox<::table_widget::TableWidget>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTableWidget::QTableWidget(int rows, int columns, QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::table_widget::TableWidget>
    where Args: overloading::TableWidgetNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QTableWidget::openPersistentEditor(QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn open_persistent_editor(&mut self, item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_openPersistentEditor(self as *mut ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```virtual int QTableWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTableWidget_qt_metacall(self as *mut ::table_widget::TableWidget,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTableWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTableWidget_qt_metacast(self as *mut ::table_widget::TableWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::removeCellWidget(int row, int column)```</span>
  ///
  ///
  pub fn remove_cell_widget(&mut self, row: ::libc::c_int, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_removeCellWidget(self as *mut ::table_widget::TableWidget, row, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::removeColumn(int column)```</span>
  ///
  ///
  pub fn remove_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_removeColumn(self as *mut ::table_widget::TableWidget, column) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::removeRow(int row)```</span>
  ///
  ///
  pub fn remove_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_removeRow(self as *mut ::table_widget::TableWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::row(const QTableWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn row(&self, item: *const ::table_widget_item::TableWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTableWidget_row(self as *const ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_rowCount(self as *const ::table_widget::TableWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidget::scrollToItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, *const ::table_widget_item::TableWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::scrollToItem(const QTableWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, (*const ::table_widget_item::TableWidgetItem, &::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTableWidget::scrollToItem(const QTableWidgetItem* item, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub unsafe fn scroll_to_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TableWidgetScrollToItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*> QTableWidget::selectedItems() const```</span>
  ///
  ///
  pub fn selected_items(&self) -> ::list::ListTableWidgetItemMutPtr {
    {
      let mut object: ::list::ListTableWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_selectedItems_to_output(self as *const ::table_widget::TableWidget,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange> QTableWidget::selectedRanges() const```</span>
  ///
  ///
  pub fn selected_ranges(&self) -> ::list::ListTableWidgetSelectionRange {
    {
      let mut object: ::list::ListTableWidgetSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_selectedRanges_to_output(self as *const ::table_widget::TableWidget,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setCellWidget(int row, int column, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_cell_widget(&mut self, row: ::libc::c_int, column: ::libc::c_int, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QTableWidget_setCellWidget(self as *mut ::table_widget::TableWidget,
                                                   row,
                                                   column,
                                                   widget)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setColumnCount(int columns)```</span>
  ///
  ///
  pub fn set_column_count(&mut self, columns: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_setColumnCount(self as *mut ::table_widget::TableWidget, columns) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setCurrentCell(int row, int column)```</span>
  ///
  ///
  pub fn set_current_cell(&mut self, row: ::libc::c_int, column: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_setCurrentCell(self as *mut ::table_widget::TableWidget, row, column) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setCurrentItem(QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_current_item(&mut self, item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_setCurrentItem(self as *mut ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setHorizontalHeaderItem(int column, QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_horizontal_header_item(&mut self,
                                           column: ::libc::c_int,
                                           item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_setHorizontalHeaderItem(self as *mut ::table_widget::TableWidget, column, item)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setHorizontalHeaderLabels(const QStringList& labels)```</span>
  ///
  ///
  pub fn set_horizontal_header_labels(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidget_setHorizontalHeaderLabels(self as *mut ::table_widget::TableWidget,
                                                                 labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setItem(int row, int column, QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item(&mut self,
                         row: ::libc::c_int,
                         column: ::libc::c_int,
                         item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_setItem(self as *mut ::table_widget::TableWidget, row, column, item)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setItemPrototype(const QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item_prototype(&mut self, item: *const ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_setItemPrototype(self as *mut ::table_widget::TableWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setItemSelected(const QTableWidgetItem* item, bool select)```</span>
  ///
  ///
  pub unsafe fn set_item_selected(&mut self, item: *const ::table_widget_item::TableWidgetItem, select: bool) {
    ::ffi::qt_widgets_c_QTableWidget_setItemSelected(self as *mut ::table_widget::TableWidget, item, select)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setRangeSelected(const QTableWidgetSelectionRange& range, bool select)```</span>
  ///
  ///
  pub fn set_range_selected(&mut self,
                            range: &::table_widget_selection_range::TableWidgetSelectionRange,
                            select: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_setRangeSelected(self as *mut ::table_widget::TableWidget, range as *const ::table_widget_selection_range::TableWidgetSelectionRange, select) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setRowCount(int rows)```</span>
  ///
  ///
  pub fn set_row_count(&mut self, rows: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_setRowCount(self as *mut ::table_widget::TableWidget, rows) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setSortingEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_sorting_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_setSortingEnabled(self as *mut ::table_widget::TableWidget, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setVerticalHeaderItem(int row, QTableWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_vertical_header_item(&mut self,
                                         row: ::libc::c_int,
                                         item: *mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QTableWidget_setVerticalHeaderItem(self as *mut ::table_widget::TableWidget, row, item)
  }

  /// C++ method: <span style='color: green;'>```void QTableWidget::setVerticalHeaderLabels(const QStringList& labels)```</span>
  ///
  ///
  pub fn set_vertical_header_labels(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidget_setVerticalHeaderLabels(self as *mut ::table_widget::TableWidget,
                                                               labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidget::sortItems```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort_items(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTableWidget::sortItems(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort_items(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTableWidget::sortItems(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort_items<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TableWidgetSortItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::takeHorizontalHeaderItem(int column)```</span>
  ///
  ///
  pub fn take_horizontal_header_item(&mut self, column: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe {
      ::ffi::qt_widgets_c_QTableWidget_takeHorizontalHeaderItem(self as *mut ::table_widget::TableWidget, column)
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::takeItem(int row, int column)```</span>
  ///
  ///
  pub fn take_item(&mut self, row: ::libc::c_int, column: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_takeItem(self as *mut ::table_widget::TableWidget, row, column) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::takeVerticalHeaderItem(int row)```</span>
  ///
  ///
  pub fn take_vertical_header_item(&mut self, row: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_takeVerticalHeaderItem(self as *mut ::table_widget::TableWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTableWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTableWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTableWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTableWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QTableWidget::verticalHeaderItem(int row) const```</span>
  ///
  ///
  pub fn vertical_header_item(&self, row: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_verticalHeaderItem(self as *const ::table_widget::TableWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::visualColumn(int logicalColumn) const```</span>
  ///
  ///
  pub fn visual_column(&self, logical_column: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_visualColumn(self as *const ::table_widget::TableWidget, logical_column) }
  }

  /// C++ method: <span style='color: green;'>```QRect QTableWidget::visualItemRect(const QTableWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn visual_item_rect(&self, item: *const ::table_widget_item::TableWidgetItem) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTableWidget_visualItemRect_to_output(self as *const ::table_widget::TableWidget,
                                                                item,
                                                                &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTableWidget::visualRow(int logicalRow) const```</span>
  ///
  ///
  pub fn visual_row(&self, logical_row: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTableWidget_visualRow(self as *const ::table_widget::TableWidget, logical_row) }
  }
}

impl ::cpp_utils::CppDeletable for ::table_widget::TableWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTableWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TableWidget`.
  pub struct Signals<'a>(&'a ::table_widget::TableWidget);
  /// Represents a built-in Qt signal `QTableWidget::cellChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellChanged(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellChanged<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::cellDoubleClicked`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellDoubleClicked<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellDoubleClicked<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellDoubleClicked(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemClicked`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemClicked<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemClicked<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemClicked(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemClicked<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::currentCellChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().current_cell_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CurrentCellChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentCellChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentCellChanged(int,int,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentCellChanged<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemPressed`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemPressed<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemPressed<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemPressed(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemPressed<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::currentItemChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().current_item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CurrentItemChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentItemChanged<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem, *mut ::table_widget_item::TableWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentItemChanged(QTableWidgetItem*,QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentItemChanged<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::cellPressed`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellPressed<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellPressed<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellPressed(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellPressed<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::cellClicked`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellClicked<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellClicked<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellClicked(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellClicked<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::cellEntered`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellEntered<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellEntered<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellEntered(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellEntered<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemActivated`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemActivated<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemActivated<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemActivated(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemActivated<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemDoubleClicked`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemDoubleClicked<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemDoubleClicked<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemDoubleClicked(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemEntered`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemEntered<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemEntered<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemEntered(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemEntered<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::cellActivated`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().cell_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct CellActivated<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for CellActivated<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cellActivated(int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CellActivated<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemSelectionChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemSelectionChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemSelectionChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemSelectionChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemSelectionChanged<'a> {}
  /// Represents a built-in Qt signal `QTableWidget::itemChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.signals().item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ItemChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemChanged<'a> {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemChanged(QTableWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_changed(&self) -> CellChanged {
      CellChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_double_clicked(&self) -> CellDoubleClicked {
      CellDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_clicked(&self) -> ItemClicked {
      ItemClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::currentCellChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_cell_changed(&self) -> CurrentCellChanged {
      CurrentCellChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_pressed(&self) -> ItemPressed {
      ItemPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::currentItemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_item_changed(&self) -> CurrentItemChanged {
      CurrentItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_pressed(&self) -> CellPressed {
      CellPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_clicked(&self) -> CellClicked {
      CellClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_entered(&self) -> CellEntered {
      CellEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_activated(&self) -> ItemActivated {
      ItemActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_double_clicked(&self) -> ItemDoubleClicked {
      ItemDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_entered(&self) -> ItemEntered {
      ItemEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::cellActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cell_activated(&self) -> CellActivated {
      CellActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemSelectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_selection_changed(&self) -> ItemSelectionChanged {
      ItemSelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTableWidget::itemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_changed(&self) -> ItemChanged {
      ItemChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TableWidget`.
  pub struct Slots<'a>(&'a ::table_widget::TableWidget);
  /// Represents a built-in Qt slot `QTableWidget::columnMoved`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().column_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ColumnMoved<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnMoved(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::resizeColumnToContents`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().resize_column_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ResizeColumnToContents<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::resizeRowsToContents`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().resize_rows_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ResizeRowsToContents<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ResizeRowsToContents<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeRowsToContents()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::columnResized`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().column_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ColumnResized<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::setShowGrid`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().set_show_grid()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct SetShowGrid<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for SetShowGrid<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShowGrid(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::resizeColumnsToContents`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().resize_columns_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ResizeColumnsToContents<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnsToContents<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnsToContents()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::hideColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().hide_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct HideColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for HideColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::showRow`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().show_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ShowRow<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::selectRow`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().select_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct SelectRow<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for SelectRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::insertColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().insert_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct InsertColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for InsertColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::removeColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().remove_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct RemoveColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for RemoveColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1removeColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::rowResized`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().row_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct RowResized<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for RowResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::rowMoved`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().row_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct RowMoved<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for RowMoved<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowMoved(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::insertRow`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().insert_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct InsertRow<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for InsertRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1insertRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::resizeRowToContents`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().resize_row_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ResizeRowToContents<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ResizeRowToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeRowToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::sortByColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().sort_by_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct SortByColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for SortByColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sortByColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::rowCountChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().row_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct RowCountChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for RowCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::selectColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().select_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct SelectColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for SelectColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1selectColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::clearContents`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().clear_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ClearContents<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ClearContents<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clearContents()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::hideRow`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().hide_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct HideRow<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for HideRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::columnCountChanged`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().column_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ColumnCountChanged<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::clear`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct Clear<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::showColumn`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().show_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ShowColumn<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::removeRow`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().remove_row()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct RemoveRow<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for RemoveRow<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1removeRow(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTableWidget::scrollToItem`.
  ///
  /// An object of this type can be created from `TableWidget` with `object.slots().scroll_to_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TableWidget` object.
  pub struct ScrollToItem<'a>(&'a ::table_widget::TableWidget);
  impl<'a> ::qt_core::connection::Receiver for ScrollToItem<'a> {
    type Arguments = (*const ::table_widget_item::TableWidgetItem, &'static ::abstract_item_view::ScrollHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToItem(const QTableWidgetItem*,QAbstractItemView::ScrollHint)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTableWidget::columnMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_moved(&self) -> ColumnMoved {
      ColumnMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::resizeColumnToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_column_to_contents(&self) -> ResizeColumnToContents {
      ResizeColumnToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::resizeRowsToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_rows_to_contents(&self) -> ResizeRowsToContents {
      ResizeRowsToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::columnResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_resized(&self) -> ColumnResized {
      ColumnResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::setShowGrid`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_show_grid(&self) -> SetShowGrid {
      SetShowGrid(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::resizeColumnsToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_columns_to_contents(&self) -> ResizeColumnsToContents {
      ResizeColumnsToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::hideColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_column(&self) -> HideColumn {
      HideColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::showRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_row(&self) -> ShowRow {
      ShowRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::selectRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_row(&self) -> SelectRow {
      SelectRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::insertColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_column(&self) -> InsertColumn {
      InsertColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::removeColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn remove_column(&self) -> RemoveColumn {
      RemoveColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::rowResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_resized(&self) -> RowResized {
      RowResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::rowMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_moved(&self) -> RowMoved {
      RowMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::insertRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn insert_row(&self) -> InsertRow {
      InsertRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::resizeRowToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_row_to_contents(&self) -> ResizeRowToContents {
      ResizeRowToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::sortByColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_by_column(&self) -> SortByColumn {
      SortByColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::rowCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn row_count_changed(&self) -> RowCountChanged {
      RowCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::selectColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_column(&self) -> SelectColumn {
      SelectColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::clearContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear_contents(&self) -> ClearContents {
      ClearContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::hideRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_row(&self) -> HideRow {
      HideRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::columnCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_count_changed(&self) -> ColumnCountChanged {
      ColumnCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::showColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_column(&self) -> ShowColumn {
      ShowColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::removeRow`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn remove_row(&self) -> RemoveRow {
      RemoveRow(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTableWidget::scrollToItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_item(&self) -> ScrollToItem {
      ScrollToItem(self.0)
    }
  }
  impl ::table_widget::TableWidget {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QTableWidgetItem& item)```</span>
///
///
pub fn op_shl<'l0, 'l1>(out: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 ::table_widget_item::TableWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result =
    unsafe {
      ::ffi::qt_widgets_c_QTableWidget_G_operator_shl(out as *mut ::qt_core::data_stream::DataStream,
                                                      item as *const ::table_widget_item::TableWidgetItem)
    };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QTableWidgetItem& item)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 mut ::table_widget_item::TableWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QTableWidget_G_operator_shr(in_ as *mut ::qt_core::data_stream::DataStream,
                                                    item as *mut ::table_widget_item::TableWidgetItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

impl ::cpp_utils::DynamicCast<::table_widget::TableWidget> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_widget::TableWidget> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_widget::TableWidget> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_widget::TableWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_widget::TableWidget> for ::table_view::TableView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QTableView(self as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QTableView(self as *const ::table_view::TableView as *mut ::table_view::TableView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::table_widget::TableWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::table_widget::TableWidget> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QWidget(self as *mut ::widget::Widget)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::table_widget::TableWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_dynamic_cast_QTableWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QObject_ptr(self as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QObject_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::table_widget::TableWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QPaintDevice_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QAbstractItemView_ptr(self as *mut ::table_widget::TableWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QAbstractItemView_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::table_widget::TableWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QAbstractScrollArea_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QFrame_ptr(self as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QFrame_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::table_view::TableView> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(self as *mut ::table_widget::TableWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::table_view::TableView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::table_widget::TableWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QWidget_ptr(self as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QWidget_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::table_view::TableView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QTableView(self as *mut ::table_view::TableView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QTableView(self as *const ::table_view::TableView as *mut ::table_view::TableView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::table_widget::TableWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::table_widget::TableWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::table_widget::TableWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::table_widget::TableWidget {
  type Target = ::table_view::TableView;
  fn deref(&self) -> &::table_view::TableView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(self as *const ::table_widget::TableWidget as *mut ::table_widget::TableWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::table_widget::TableWidget {
  fn deref_mut(&mut self) -> &mut ::table_view::TableView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_G_static_cast_QTableView_ptr(self as *mut ::table_widget::TableWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TableWidget::item_at](../struct.TableWidget.html#method.item_at) method.
  pub trait TableWidgetItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::table_widget::TableWidget) -> *mut ::table_widget_item::TableWidgetItem;
  }
  impl<'largs> TableWidgetItemAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::table_widget::TableWidget) -> *mut ::table_widget_item::TableWidgetItem {
      let p = self;
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_itemAt_p(original_self as *const ::table_widget::TableWidget,
                                                  p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> TableWidgetItemAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::table_widget::TableWidget) -> *mut ::table_widget_item::TableWidgetItem {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QTableWidget_itemAt_x_y(original_self as *const ::table_widget::TableWidget, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [TableWidget::new](../struct.TableWidget.html#method.new) method.
  pub trait TableWidgetNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget>;
  }
  impl TableWidgetNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TableWidgetNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget> {
      let rows = self.0;
      let columns = self.1;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QTableWidget_new_rows_columns(rows, columns) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TableWidget::new_unsafe](../struct.TableWidget.html#method.new_unsafe) method.
  pub trait TableWidgetNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget>;
  }
  impl TableWidgetNewUnsafeArgs for *mut ::widget::Widget {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget> {
      let parent = self;
      let ffi_result = ::ffi::qt_widgets_c_QTableWidget_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl TableWidgetNewUnsafeArgs for (::libc::c_int, ::libc::c_int, *mut ::widget::Widget) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::table_widget::TableWidget> {
      let rows = self.0;
      let columns = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_widgets_c_QTableWidget_new_rows_columns_parent(rows, columns, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [TableWidget::scroll_to_item](../struct.TableWidget.html#method.scroll_to_item) method.
  pub trait TableWidgetScrollToItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> ();
  }
  impl<'largs> TableWidgetScrollToItemArgs<'largs> for *const ::table_widget_item::TableWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTableWidget_scrollToItem_item(original_self as *mut ::table_widget::TableWidget, item)
    }
  }
  impl<'largs> TableWidgetScrollToItemArgs<'largs>
    for (*const ::table_widget_item::TableWidgetItem, &'largs ::abstract_item_view::ScrollHint) {
    unsafe fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> () {
      let item = self.0;
      let hint = self.1;
      ::ffi::qt_widgets_c_QTableWidget_scrollToItem_item_hint(original_self as *mut ::table_widget::TableWidget,
                                                              item,
                                                              hint as *const ::abstract_item_view::ScrollHint)
    }
  }
  /// This trait represents a set of arguments accepted by [TableWidget::sort_items](../struct.TableWidget.html#method.sort_items) method.
  pub trait TableWidgetSortItemsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> ();
  }
  impl<'largs> TableWidgetSortItemsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_sortItems_column(original_self as *mut ::table_widget::TableWidget, column)
      }
    }
  }
  impl<'largs> TableWidgetSortItemsArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::table_widget::TableWidget) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QTableWidget_sortItems_column_order(original_self as *mut ::table_widget::TableWidget,
                                                                column,
                                                                order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
}
