/// C++ type: <span style='color: green;'>```QTreeWidget```</span>
#[repr(C)]
pub struct TreeWidget(u8);

impl TreeWidget {
  /// C++ method: <span style='color: green;'>```void QTreeWidget::addTopLevelItem(QTreeWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn add_top_level_item(&mut self, item: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidget_addTopLevelItem(self as *mut ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::addTopLevelItems(const QList<QTreeWidgetItem*>& items)```</span>
  ///
  ///
  pub fn add_top_level_items(&mut self, items: &::list::ListTreeWidgetItemMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidget_addTopLevelItems(self as *mut ::tree_widget::TreeWidget,
                                                       items as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTreeWidget::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_clear(self as *mut ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::closePersistentEditor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn close_persistent_editor(&mut self, *mut ::tree_widget_item::TreeWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::closePersistentEditor(QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn close_persistent_editor(&mut self, (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::closePersistentEditor(QTreeWidgetItem* item, int column = ?)```</span>
  ///
  ///
  pub unsafe fn close_persistent_editor<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeWidgetClosePersistentEditorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTreeWidget::collapseItem(const QTreeWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn collapse_item(&mut self, item: *const ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidget_collapseItem(self as *mut ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidget::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_columnCount(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidget::currentColumn() const```</span>
  ///
  ///
  pub fn current_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_currentColumn(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::currentItem() const```</span>
  ///
  ///
  pub fn current_item(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_currentItem(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::editItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn edit_item(&mut self, *mut ::tree_widget_item::TreeWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::editItem(QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn edit_item(&mut self, (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::editItem(QTreeWidgetItem* item, int column = ?)```</span>
  ///
  ///
  pub unsafe fn edit_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeWidgetEditItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QTreeWidget::expandItem(const QTreeWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn expand_item(&mut self, item: *const ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidget_expandItem(self as *mut ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::headerItem() const```</span>
  ///
  ///
  pub fn header_item(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_headerItem(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidget::indexOfTopLevelItem(QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn index_of_top_level_item(&self, item: *mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTreeWidget_indexOfTopLevelItem(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::insertTopLevelItem(int index, QTreeWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn insert_top_level_item(&mut self, index: ::libc::c_int, item: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidget_insertTopLevelItem(self as *mut ::tree_widget::TreeWidget, index, item)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::insertTopLevelItems(int index, const QList<QTreeWidgetItem*>& items)```</span>
  ///
  ///
  pub fn insert_top_level_items(&mut self, index: ::libc::c_int, items: &::list::ListTreeWidgetItemMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidget_insertTopLevelItems(self as *mut ::tree_widget::TreeWidget,
                                                          index,
                                                          items as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::invisibleRootItem() const```</span>
  ///
  ///
  pub fn invisible_root_item(&self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_invisibleRootItem(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidget::isFirstItemColumnSpanned(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_first_item_column_spanned(&self, item: *const ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QTreeWidget_isFirstItemColumnSpanned(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidget::isItemExpanded(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_expanded(&self, item: *const ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QTreeWidget_isItemExpanded(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidget::isItemHidden(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_hidden(&self, item: *const ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QTreeWidget_isItemHidden(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QTreeWidget::isItemSelected(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_selected(&self, item: *const ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QTreeWidget_isItemSelected(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::itemAbove(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn item_above(&self,
                           item: *const ::tree_widget_item::TreeWidgetItem)
                           -> *mut ::tree_widget_item::TreeWidgetItem {
    ::ffi::qt_widgets_c_QTreeWidget_itemAbove(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, &::qt_core::point::Point) -> *mut ::tree_widget_item::TreeWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::itemAt(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::tree_widget_item::TreeWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::itemAt(int x, int y) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::tree_widget_item::TreeWidgetItem
    where Args: overloading::TreeWidgetItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::itemBelow(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn item_below(&self,
                           item: *const ::tree_widget_item::TreeWidgetItem)
                           -> *mut ::tree_widget_item::TreeWidgetItem {
    ::ffi::qt_widgets_c_QTreeWidget_itemBelow(self as *const ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QTreeWidget::itemWidget(QTreeWidgetItem* item, int column) const```</span>
  ///
  ///
  pub unsafe fn item_widget(&self,
                            item: *mut ::tree_widget_item::TreeWidgetItem,
                            column: ::libc::c_int)
                            -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QTreeWidget_itemWidget(self as *const ::tree_widget::TreeWidget, item, column)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTreeWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_metaObject(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidget::QTreeWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTreeWidget::QTreeWidget(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::tree_widget::TreeWidget> {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::openPersistentEditor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn open_persistent_editor(&mut self, *mut ::tree_widget_item::TreeWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::openPersistentEditor(QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn open_persistent_editor(&mut self, (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::openPersistentEditor(QTreeWidgetItem* item, int column = ?)```</span>
  ///
  ///
  pub unsafe fn open_persistent_editor<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeWidgetOpenPersistentEditorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QTreeWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QTreeWidget_qt_metacall(self as *mut ::tree_widget::TreeWidget,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QTreeWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QTreeWidget_qt_metacast(self as *mut ::tree_widget::TreeWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::removeItemWidget(QTreeWidgetItem* item, int column)```</span>
  ///
  ///
  pub unsafe fn remove_item_widget(&mut self, item: *mut ::tree_widget_item::TreeWidgetItem, column: ::libc::c_int) {
    ::ffi::qt_widgets_c_QTreeWidget_removeItemWidget(self as *mut ::tree_widget::TreeWidget, item, column)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::scrollToItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, *const ::tree_widget_item::TreeWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTreeWidget::scrollToItem(const QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, (*const ::tree_widget_item::TreeWidgetItem, &::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QTreeWidget::scrollToItem(const QTreeWidgetItem* item, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub unsafe fn scroll_to_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeWidgetScrollToItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*> QTreeWidget::selectedItems() const```</span>
  ///
  ///
  pub fn selected_items(&self) -> ::list::ListTreeWidgetItemMutPtr {
    {
      let mut object: ::list::ListTreeWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidget_selectedItems_to_output(self as *const ::tree_widget::TreeWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setColumnCount(int columns)```</span>
  ///
  ///
  pub fn set_column_count(&mut self, columns: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_setColumnCount(self as *mut ::tree_widget::TreeWidget, columns) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidget::setCurrentItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_current_item(&mut self, *mut ::tree_widget_item::TreeWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::setCurrentItem(QTreeWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_current_item(&mut self, (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTreeWidget::setCurrentItem(QTreeWidgetItem* item, int column)```</span>
  ///
  ///
  pub unsafe fn set_current_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TreeWidgetSetCurrentItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTreeWidget::setFirstItemColumnSpanned(const QTreeWidgetItem* item, bool span)```</span>
  ///
  ///
  pub unsafe fn set_first_item_column_spanned(&mut self, item: *const ::tree_widget_item::TreeWidgetItem, span: bool) {
    ::ffi::qt_widgets_c_QTreeWidget_setFirstItemColumnSpanned(self as *mut ::tree_widget::TreeWidget, item, span)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setHeaderItem(QTreeWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_header_item(&mut self, item: *mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QTreeWidget_setHeaderItem(self as *mut ::tree_widget::TreeWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setHeaderLabel(const QString& label)```</span>
  ///
  ///
  pub fn set_header_label(&mut self, label: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidget_setHeaderLabel(self as *mut ::tree_widget::TreeWidget,
                                                     label as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setHeaderLabels(const QStringList& labels)```</span>
  ///
  ///
  pub fn set_header_labels(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidget_setHeaderLabels(self as *mut ::tree_widget::TreeWidget,
                                                      labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setItemExpanded(const QTreeWidgetItem* item, bool expand)```</span>
  ///
  ///
  pub unsafe fn set_item_expanded(&mut self, item: *const ::tree_widget_item::TreeWidgetItem, expand: bool) {
    ::ffi::qt_widgets_c_QTreeWidget_setItemExpanded(self as *mut ::tree_widget::TreeWidget, item, expand)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setItemHidden(const QTreeWidgetItem* item, bool hide)```</span>
  ///
  ///
  pub unsafe fn set_item_hidden(&mut self, item: *const ::tree_widget_item::TreeWidgetItem, hide: bool) {
    ::ffi::qt_widgets_c_QTreeWidget_setItemHidden(self as *mut ::tree_widget::TreeWidget, item, hide)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setItemSelected(const QTreeWidgetItem* item, bool select)```</span>
  ///
  ///
  pub unsafe fn set_item_selected(&mut self, item: *const ::tree_widget_item::TreeWidgetItem, select: bool) {
    ::ffi::qt_widgets_c_QTreeWidget_setItemSelected(self as *mut ::tree_widget::TreeWidget, item, select)
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::setItemWidget(QTreeWidgetItem* item, int column, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_item_widget(&mut self,
                                item: *mut ::tree_widget_item::TreeWidgetItem,
                                column: ::libc::c_int,
                                widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QTreeWidget_setItemWidget(self as *mut ::tree_widget::TreeWidget, item, column, widget)
  }

  /// C++ method: <span style='color: green;'>```virtual void QTreeWidget::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QTreeWidget_setSelectionModel(self as *mut ::tree_widget::TreeWidget, selection_model)
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidget::sortColumn() const```</span>
  ///
  ///
  pub fn sort_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_sortColumn(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QTreeWidget::sortItems(int column, Qt::SortOrder order)```</span>
  ///
  ///
  pub fn sort_items(&mut self, column: ::libc::c_int, order: &::qt_core::qt::SortOrder) {
    unsafe {
      ::ffi::qt_widgets_c_QTreeWidget_sortItems(self as *mut ::tree_widget::TreeWidget,
                                                column,
                                                order as *const ::qt_core::qt::SortOrder)
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::takeTopLevelItem(int index)```</span>
  ///
  ///
  pub fn take_top_level_item(&mut self, index: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_takeTopLevelItem(self as *mut ::tree_widget::TreeWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QTreeWidget::topLevelItem(int index) const```</span>
  ///
  ///
  pub fn top_level_item(&self, index: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_topLevelItem(self as *const ::tree_widget::TreeWidget, index) }
  }

  /// C++ method: <span style='color: green;'>```int QTreeWidget::topLevelItemCount() const```</span>
  ///
  ///
  pub fn top_level_item_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QTreeWidget_topLevelItemCount(self as *const ::tree_widget::TreeWidget) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTreeWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTreeWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTreeWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTreeWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QTreeWidget::visualItemRect(const QTreeWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn visual_item_rect(&self, item: *const ::tree_widget_item::TreeWidgetItem) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QTreeWidget_visualItemRect_to_output(self as *const ::tree_widget::TreeWidget,
                                                               item,
                                                               &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::tree_widget::TreeWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QTreeWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TreeWidget`.
  pub struct Signals<'a>(&'a ::tree_widget::TreeWidget);
  /// Represents a built-in Qt signal `QTreeWidget::itemEntered`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemEntered<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemEntered<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemEntered(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemEntered<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::collapsed`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().collapsed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Collapsed<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Collapsed<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2collapsed(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Collapsed<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemClicked`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemClicked<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemClicked<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemClicked(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemClicked<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemPressed`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemPressed<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemPressed<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemPressed(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemPressed<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemDoubleClicked`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemDoubleClicked<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemDoubleClicked<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemDoubleClicked(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::currentItemChanged`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().current_item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct CurrentItemChanged<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentItemChanged<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentItemChanged(QTreeWidgetItem*,QTreeWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentItemChanged<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemCollapsed`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_collapsed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemCollapsed<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemCollapsed<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemCollapsed(QTreeWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemCollapsed<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemActivated`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemActivated<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemActivated<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemActivated(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemActivated<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::expanded`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().expanded()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Expanded<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Expanded<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2expanded(const QModelIndex&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Expanded<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemSelectionChanged`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemSelectionChanged<'a>(&'a ::tree_widget::TreeWidget);
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
  /// Represents a built-in Qt signal `QTreeWidget::itemChanged`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemChanged<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemChanged<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemChanged(QTreeWidgetItem*,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemChanged<'a> {}
  /// Represents a built-in Qt signal `QTreeWidget::itemExpanded`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.signals().item_expanded()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ItemExpanded<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemExpanded<'a> {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemExpanded(QTreeWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemExpanded<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_entered(&self) -> ItemEntered {
      ItemEntered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::collapsed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapsed(&self) -> Collapsed {
      Collapsed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_clicked(&self) -> ItemClicked {
      ItemClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_pressed(&self) -> ItemPressed {
      ItemPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_double_clicked(&self) -> ItemDoubleClicked {
      ItemDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::currentItemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_item_changed(&self) -> CurrentItemChanged {
      CurrentItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemCollapsed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_collapsed(&self) -> ItemCollapsed {
      ItemCollapsed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_activated(&self) -> ItemActivated {
      ItemActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::expanded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expanded(&self) -> Expanded {
      Expanded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemSelectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_selection_changed(&self) -> ItemSelectionChanged {
      ItemSelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_changed(&self) -> ItemChanged {
      ItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTreeWidget::itemExpanded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_expanded(&self) -> ItemExpanded {
      ItemExpanded(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TreeWidget`.
  pub struct Slots<'a>(&'a ::tree_widget::TreeWidget);
  /// Represents a built-in Qt slot `QTreeWidget::sortByColumn`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().sort_by_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct SortByColumn<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for SortByColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1sortByColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::expandItem`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().expand_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ExpandItem<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ExpandItem<'a> {
    type Arguments = (*const ::tree_widget_item::TreeWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expandItem(const QTreeWidgetItem*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::scrollToItem`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().scroll_to_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ScrollToItem<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ScrollToItem<'a> {
    type Arguments = (*const ::tree_widget_item::TreeWidgetItem, &'static ::abstract_item_view::ScrollHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToItem(const QTreeWidgetItem*,QAbstractItemView::ScrollHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::columnResized`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().column_resized()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ColumnResized<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnResized<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnResized(int,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::reexpand`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().reexpand()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Reexpand<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Reexpand<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1reexpand()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::expandToDepth`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().expand_to_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ExpandToDepth<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ExpandToDepth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expandToDepth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::hideColumn`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().hide_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct HideColumn<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for HideColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1hideColumn(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::expand`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().expand()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Expand<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Expand<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expand(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::rowsRemoved`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct RowsRemoved<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for RowsRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1rowsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::collapseAll`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().collapse_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct CollapseAll<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for CollapseAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1collapseAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::clear`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Clear<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::collapse`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().collapse()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct Collapse<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for Collapse<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1collapse(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::columnMoved`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().column_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ColumnMoved<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnMoved<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnMoved()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::collapseItem`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().collapse_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct CollapseItem<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for CollapseItem<'a> {
    type Arguments = (*const ::tree_widget_item::TreeWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1collapseItem(const QTreeWidgetItem*)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::expandAll`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().expand_all()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ExpandAll<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ExpandAll<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1expandAll()\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::columnCountChanged`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().column_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ColumnCountChanged<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ColumnCountChanged<'a> {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1columnCountChanged(int,int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::resizeColumnToContents`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().resize_column_to_contents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ResizeColumnToContents<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ResizeColumnToContents<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resizeColumnToContents(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTreeWidget::showColumn`.
  ///
  /// An object of this type can be created from `TreeWidget` with `object.slots().show_column()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TreeWidget` object.
  pub struct ShowColumn<'a>(&'a ::tree_widget::TreeWidget);
  impl<'a> ::qt_core::connection::Receiver for ShowColumn<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1showColumn(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTreeWidget::sortByColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn sort_by_column(&self) -> SortByColumn {
      SortByColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::expandItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand_item(&self) -> ExpandItem {
      ExpandItem(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::scrollToItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_item(&self) -> ScrollToItem {
      ScrollToItem(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::columnResized`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_resized(&self) -> ColumnResized {
      ColumnResized(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::reexpand`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reexpand(&self) -> Reexpand {
      Reexpand(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::expandToDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand_to_depth(&self) -> ExpandToDepth {
      ExpandToDepth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::hideColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hide_column(&self) -> HideColumn {
      HideColumn(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::expand`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand(&self) -> Expand {
      Expand(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::collapseAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapse_all(&self) -> CollapseAll {
      CollapseAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::collapse`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapse(&self) -> Collapse {
      Collapse(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::columnMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_moved(&self) -> ColumnMoved {
      ColumnMoved(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::collapseItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn collapse_item(&self) -> CollapseItem {
      CollapseItem(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::expandAll`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn expand_all(&self) -> ExpandAll {
      ExpandAll(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::columnCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn column_count_changed(&self) -> ColumnCountChanged {
      ColumnCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::resizeColumnToContents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resize_column_to_contents(&self) -> ResizeColumnToContents {
      ResizeColumnToContents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTreeWidget::showColumn`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn show_column(&self) -> ShowColumn {
      ShowColumn(self.0)
    }
  }
  impl ::tree_widget::TreeWidget {
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

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QTreeWidgetItem& item)```</span>
///
///
pub fn op_shl<'l0, 'l1>(out: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 ::tree_widget_item::TreeWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QTreeWidget_G_operator_shl(out as *mut ::qt_core::data_stream::DataStream,
                                                   item as *const ::tree_widget_item::TreeWidgetItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QTreeWidgetItem& item)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 mut ::tree_widget_item::TreeWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QTreeWidget_G_operator_shr(in_ as *mut ::qt_core::data_stream::DataStream,
                                                   item as *mut ::tree_widget_item::TreeWidgetItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

impl ::cpp_utils::DynamicCast<::tree_widget::TreeWidget> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_widget::TreeWidget> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_widget::TreeWidget> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_widget::TreeWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_widget::TreeWidget> for ::tree_view::TreeView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_widget::TreeWidget> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QTreeView(self as *mut ::tree_view::TreeView)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QTreeView(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::tree_widget::TreeWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::tree_widget::TreeWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::tree_widget::TreeWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_dynamic_cast_QTreeWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QObject_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QObject_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QPaintDevice_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QAbstractItemView_ptr(self as *mut ::tree_widget::TreeWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QAbstractItemView_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::tree_widget::TreeWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QAbstractScrollArea_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QFrame_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QFrame_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::tree_view::TreeView> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::tree_view::TreeView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::tree_widget::TreeWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QWidget_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QWidget_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::tree_view::TreeView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QTreeView(self as *mut ::tree_view::TreeView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QTreeView(self as *const ::tree_view::TreeView as *mut ::tree_view::TreeView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tree_widget::TreeWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tree_widget::TreeWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tree_widget::TreeWidget {
    let ffi_result = ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tree_widget::TreeWidget {
  type Target = ::tree_view::TreeView;
  fn deref(&self) -> &::tree_view::TreeView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(self as *const ::tree_widget::TreeWidget as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tree_widget::TreeWidget {
  fn deref_mut(&mut self) -> &mut ::tree_view::TreeView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_G_static_cast_QTreeView_ptr(self as *mut ::tree_widget::TreeWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TreeWidget::close_persistent_editor](../struct.TreeWidget.html#method.close_persistent_editor) method.
  pub trait TreeWidgetClosePersistentEditorArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> ();
  }
  impl<'largs> TreeWidgetClosePersistentEditorArgs<'largs> for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTreeWidget_closePersistentEditor_item(original_self as *mut ::tree_widget::TreeWidget, item)
    }
  }
  impl<'largs> TreeWidgetClosePersistentEditorArgs<'largs> for (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self.0;
      let column = self.1;
      ::ffi::qt_widgets_c_QTreeWidget_closePersistentEditor_item_column(original_self as *mut ::tree_widget::TreeWidget, item, column)
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidget::edit_item](../struct.TreeWidget.html#method.edit_item) method.
  pub trait TreeWidgetEditItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> ();
  }
  impl<'largs> TreeWidgetEditItemArgs<'largs> for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTreeWidget_editItem_item(original_self as *mut ::tree_widget::TreeWidget, item)
    }
  }
  impl<'largs> TreeWidgetEditItemArgs<'largs> for (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self.0;
      let column = self.1;
      ::ffi::qt_widgets_c_QTreeWidget_editItem_item_column(original_self as *mut ::tree_widget::TreeWidget,
                                                           item,
                                                           column)
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidget::item_at](../struct.TreeWidget.html#method.item_at) method.
  pub trait TreeWidgetItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::tree_widget::TreeWidget) -> *mut ::tree_widget_item::TreeWidgetItem;
  }
  impl<'largs> TreeWidgetItemAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::tree_widget::TreeWidget) -> *mut ::tree_widget_item::TreeWidgetItem {
      let p = self;
      unsafe {
        ::ffi::qt_widgets_c_QTreeWidget_itemAt_p(original_self as *const ::tree_widget::TreeWidget,
                                                 p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> TreeWidgetItemAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::tree_widget::TreeWidget) -> *mut ::tree_widget_item::TreeWidgetItem {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QTreeWidget_itemAt_x_y(original_self as *const ::tree_widget::TreeWidget, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidget::open_persistent_editor](../struct.TreeWidget.html#method.open_persistent_editor) method.
  pub trait TreeWidgetOpenPersistentEditorArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> ();
  }
  impl<'largs> TreeWidgetOpenPersistentEditorArgs<'largs> for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTreeWidget_openPersistentEditor_item(original_self as *mut ::tree_widget::TreeWidget, item)
    }
  }
  impl<'largs> TreeWidgetOpenPersistentEditorArgs<'largs> for (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self.0;
      let column = self.1;
      ::ffi::qt_widgets_c_QTreeWidget_openPersistentEditor_item_column(original_self as *mut ::tree_widget::TreeWidget, item, column)
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidget::scroll_to_item](../struct.TreeWidget.html#method.scroll_to_item) method.
  pub trait TreeWidgetScrollToItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> ();
  }
  impl<'largs> TreeWidgetScrollToItemArgs<'largs> for *const ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTreeWidget_scrollToItem_item(original_self as *mut ::tree_widget::TreeWidget, item)
    }
  }
  impl<'largs> TreeWidgetScrollToItemArgs<'largs>
    for (*const ::tree_widget_item::TreeWidgetItem, &'largs ::abstract_item_view::ScrollHint) {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self.0;
      let hint = self.1;
      ::ffi::qt_widgets_c_QTreeWidget_scrollToItem_item_hint(original_self as *mut ::tree_widget::TreeWidget,
                                                             item,
                                                             hint as *const ::abstract_item_view::ScrollHint)
    }
  }
  /// This trait represents a set of arguments accepted by [TreeWidget::set_current_item](../struct.TreeWidget.html#method.set_current_item) method.
  pub trait TreeWidgetSetCurrentItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> ();
  }
  impl<'largs> TreeWidgetSetCurrentItemArgs<'largs> for *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QTreeWidget_setCurrentItem_item(original_self as *mut ::tree_widget::TreeWidget, item)
    }
  }
  impl<'largs> TreeWidgetSetCurrentItemArgs<'largs> for (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::tree_widget::TreeWidget) -> () {
      let item = self.0;
      let column = self.1;
      ::ffi::qt_widgets_c_QTreeWidget_setCurrentItem_item_column(original_self as *mut ::tree_widget::TreeWidget,
                                                                 item,
                                                                 column)
    }
  }
}
