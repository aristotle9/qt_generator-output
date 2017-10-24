/// C++ type: <span style='color: green;'>```QListWidget```</span>
#[repr(C)]
pub struct ListWidget(u8);

impl ListWidget {
  /// C++ method: <span style='color: green;'>```void QListWidget::addItem(const QString& label)```</span>
  ///
  ///
  pub fn add_item(&mut self, label: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidget_addItem_label(self as *mut ::list_widget::ListWidget,
                                                    label as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::addItem(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn add_item_unsafe(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_addItem_item(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::addItems(const QStringList& labels)```</span>
  ///
  ///
  pub fn add_items(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidget_addItems(self as *mut ::list_widget::ListWidget,
                                               labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QListWidget::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QListWidget_clear(self as *mut ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::closePersistentEditor(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn close_persistent_editor(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_closePersistentEditor(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QListWidget::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListWidget_count(self as *const ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QListWidget::currentItem() const```</span>
  ///
  ///
  pub fn current_item(&self) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QListWidget_currentItem(self as *const ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```int QListWidget::currentRow() const```</span>
  ///
  ///
  pub fn current_row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QListWidget_currentRow(self as *const ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidget::dropEvent(QDropEvent* event)```</span>
  ///
  ///
  pub unsafe fn drop_event(&mut self, event: *mut ::qt_gui::drop_event::DropEvent) {
    ::ffi::qt_widgets_c_QListWidget_dropEvent(self as *mut ::list_widget::ListWidget, event)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::editItem(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn edit_item(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_editItem(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::insertItem(int row, const QString& label)```</span>
  ///
  ///
  pub fn insert_item(&mut self, row: ::libc::c_int, label: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidget_insertItem_row_label(self as *mut ::list_widget::ListWidget,
                                                           row,
                                                           label as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::insertItem(int row, QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn insert_item_unsafe(&mut self, row: ::libc::c_int, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_insertItem_row_item(self as *mut ::list_widget::ListWidget, row, item)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::insertItems(int row, const QStringList& labels)```</span>
  ///
  ///
  pub fn insert_items(&mut self, row: ::libc::c_int, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QListWidget_insertItems(self as *mut ::list_widget::ListWidget,
                                                  row,
                                                  labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QListWidget::isItemHidden(const QListWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_hidden(&self, item: *const ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QListWidget_isItemHidden(self as *const ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QListWidget::isItemSelected(const QListWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn is_item_selected(&self, item: *const ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QListWidget_isItemSelected(self as *const ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```bool QListWidget::isSortingEnabled() const```</span>
  ///
  ///
  pub fn is_sorting_enabled(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QListWidget_isSortingEnabled(self as *const ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QListWidget::item(int row) const```</span>
  ///
  ///
  pub fn item(&self, row: ::libc::c_int) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QListWidget_item(self as *const ::list_widget::ListWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```QListWidget::itemAt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item_at(&self, &::qt_core::point::Point) -> *mut ::list_widget_item::ListWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QListWidgetItem* QListWidget::itemAt(const QPoint& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item_at(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::list_widget_item::ListWidgetItem```<br>
  /// C++ method: <span style='color: green;'>```QListWidgetItem* QListWidget::itemAt(int x, int y) const```</span>
  ///
  ///
  pub fn item_at<'largs, Args>(&'largs self, args: Args) -> *mut ::list_widget_item::ListWidgetItem
    where Args: overloading::ListWidgetItemAtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget* QListWidget::itemWidget(QListWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn item_widget(&self, item: *mut ::list_widget_item::ListWidgetItem) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QListWidget_itemWidget(self as *const ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QListWidget::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QListWidget_metaObject(self as *const ::list_widget::ListWidget) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QListWidget::QListWidget()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QListWidget::QListWidget(QWidget* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::widget::Widget) -> ::cpp_utils::CppBox<::list_widget::ListWidget> {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::openPersistentEditor(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn open_persistent_editor(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_openPersistentEditor(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```virtual int QListWidget::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QListWidget_qt_metacall(self as *mut ::list_widget::ListWidget,
                                                arg1 as *const ::qt_core::meta_object::Call,
                                                arg2,
                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QListWidget::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QListWidget_qt_metacast(self as *mut ::list_widget::ListWidget, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::removeItemWidget(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn remove_item_widget(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_removeItemWidget(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```int QListWidget::row(const QListWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn row(&self, item: *const ::list_widget_item::ListWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QListWidget_row(self as *const ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```QListWidget::scrollToItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, *const ::list_widget_item::ListWidgetItem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QListWidget::scrollToItem(const QListWidgetItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scroll_to_item(&mut self, (*const ::list_widget_item::ListWidgetItem, &::abstract_item_view::ScrollHint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QListWidget::scrollToItem(const QListWidgetItem* item, QAbstractItemView::ScrollHint hint = ?)```</span>
  ///
  ///
  pub unsafe fn scroll_to_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWidgetScrollToItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*> QListWidget::selectedItems() const```</span>
  ///
  ///
  pub fn selected_items(&self) -> ::list::ListListWidgetItemMutPtr {
    {
      let mut object: ::list::ListListWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_selectedItems_to_output(self as *const ::list_widget::ListWidget, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setCurrentItem(QListWidgetItem* item)```</span>
  ///
  ///
  pub unsafe fn set_current_item(&mut self, item: *mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QListWidget_setCurrentItem(self as *mut ::list_widget::ListWidget, item)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setCurrentRow(int row)```</span>
  ///
  ///
  pub fn set_current_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QListWidget_setCurrentRow(self as *mut ::list_widget::ListWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setItemHidden(const QListWidgetItem* item, bool hide)```</span>
  ///
  ///
  pub unsafe fn set_item_hidden(&mut self, item: *const ::list_widget_item::ListWidgetItem, hide: bool) {
    ::ffi::qt_widgets_c_QListWidget_setItemHidden(self as *mut ::list_widget::ListWidget, item, hide)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setItemSelected(const QListWidgetItem* item, bool select)```</span>
  ///
  ///
  pub unsafe fn set_item_selected(&mut self, item: *const ::list_widget_item::ListWidgetItem, select: bool) {
    ::ffi::qt_widgets_c_QListWidget_setItemSelected(self as *mut ::list_widget::ListWidget, item, select)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setItemWidget(QListWidgetItem* item, QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_item_widget(&mut self,
                                item: *mut ::list_widget_item::ListWidgetItem,
                                widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QListWidget_setItemWidget(self as *mut ::list_widget::ListWidget, item, widget)
  }

  /// C++ method: <span style='color: green;'>```virtual void QListWidget::setSelectionModel(QItemSelectionModel* selectionModel)```</span>
  ///
  ///
  pub unsafe fn set_selection_model(&mut self,
                                    selection_model: *mut ::qt_core::item_selection_model::ItemSelectionModel) {
    ::ffi::qt_widgets_c_QListWidget_setSelectionModel(self as *mut ::list_widget::ListWidget, selection_model)
  }

  /// C++ method: <span style='color: green;'>```void QListWidget::setSortingEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_sorting_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QListWidget_setSortingEnabled(self as *mut ::list_widget::ListWidget, enable) }
  }

  /// C++ method: <span style='color: green;'>```QListWidget::sortItems```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort_items(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QListWidget::sortItems()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort_items(&mut self, &::qt_core::qt::SortOrder) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QListWidget::sortItems(Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort_items<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWidgetSortItemsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QListWidgetItem* QListWidget::takeItem(int row)```</span>
  ///
  ///
  pub fn take_item(&mut self, row: ::libc::c_int) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QListWidget_takeItem(self as *mut ::list_widget::ListWidget, row) }
  }

  /// C++ method: <span style='color: green;'>```static QString QListWidget::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QListWidget_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QListWidget::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QListWidget_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QListWidget::visualItemRect(const QListWidgetItem* item) const```</span>
  ///
  ///
  pub unsafe fn visual_item_rect(&self, item: *const ::list_widget_item::ListWidgetItem) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QListWidget_visualItemRect_to_output(self as *const ::list_widget::ListWidget,
                                                               item,
                                                               &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::list_widget::ListWidget {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QListWidget_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ListWidget`.
  pub struct Signals<'a>(&'a ::list_widget::ListWidget);
  /// Represents a built-in Qt signal `QListWidget::currentRowChanged`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().current_row_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct CurrentRowChanged<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentRowChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentRowChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentRowChanged<'a> {}
  /// Represents a built-in Qt signal `QListWidget::currentItemChanged`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().current_item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct CurrentItemChanged<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentItemChanged<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem, *mut ::list_widget_item::ListWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentItemChanged(QListWidgetItem*,QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentItemChanged<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemDoubleClicked`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemDoubleClicked<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemDoubleClicked<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemDoubleClicked(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemDoubleClicked<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemClicked`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemClicked<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemClicked<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemClicked(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemClicked<'a> {}
  /// Represents a built-in Qt signal `QListWidget::currentTextChanged`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().current_text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct CurrentTextChanged<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for CurrentTextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2currentTextChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CurrentTextChanged<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemChanged`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemChanged<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemChanged<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemChanged(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemChanged<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemActivated`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemActivated<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemActivated<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemActivated(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemActivated<'a> {}
  /// Represents a built-in Qt signal `QListWidget::indexesMoved`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().indexes_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct IndexesMoved<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for IndexesMoved<'a> {
    type Arguments = (&'static ::qt_core::list::ListModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2indexesMoved(const QList< QModelIndex >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IndexesMoved<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemPressed`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemPressed<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemPressed<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemPressed(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemPressed<'a> {}
  /// Represents a built-in Qt signal `QListWidget::itemSelectionChanged`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_selection_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemSelectionChanged<'a>(&'a ::list_widget::ListWidget);
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
  /// Represents a built-in Qt signal `QListWidget::itemEntered`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.signals().item_entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ItemEntered<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ItemEntered<'a> {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemEntered(QListWidgetItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemEntered<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QListWidget::currentRowChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_row_changed(&self) -> CurrentRowChanged {
      CurrentRowChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::currentItemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_item_changed(&self) -> CurrentItemChanged {
      CurrentItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemDoubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_double_clicked(&self) -> ItemDoubleClicked {
      ItemDoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_clicked(&self) -> ItemClicked {
      ItemClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::currentTextChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn current_text_changed(&self) -> CurrentTextChanged {
      CurrentTextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_changed(&self) -> ItemChanged {
      ItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemActivated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_activated(&self) -> ItemActivated {
      ItemActivated(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::indexesMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn indexes_moved(&self) -> IndexesMoved {
      IndexesMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_pressed(&self) -> ItemPressed {
      ItemPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemSelectionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_selection_changed(&self) -> ItemSelectionChanged {
      ItemSelectionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QListWidget::itemEntered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_entered(&self) -> ItemEntered {
      ItemEntered(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ListWidget`.
  pub struct Slots<'a>(&'a ::list_widget::ListWidget);
  /// Represents a built-in Qt slot `QListWidget::scrollToItem`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.slots().scroll_to_item()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct ScrollToItem<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for ScrollToItem<'a> {
    type Arguments = (*const ::list_widget_item::ListWidgetItem, &'static ::abstract_item_view::ScrollHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1scrollToItem(const QListWidgetItem*,QAbstractItemView::ScrollHint)\0"
    }
  }
  /// Represents a built-in Qt slot `QListWidget::clear`.
  ///
  /// An object of this type can be created from `ListWidget` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ListWidget` object.
  pub struct Clear<'a>(&'a ::list_widget::ListWidget);
  impl<'a> ::qt_core::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QListWidget::scrollToItem`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn scroll_to_item(&self) -> ScrollToItem {
      ScrollToItem(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QListWidget::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
  }
  impl ::list_widget::ListWidget {
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

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QListWidgetItem& item)```</span>
///
///
pub fn op_shl<'l0, 'l1>(out: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 ::list_widget_item::ListWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QListWidget_G_operator_shl(out as *mut ::qt_core::data_stream::DataStream,
                                                   item as *const ::list_widget_item::ListWidgetItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QListWidgetItem& item)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 mut ::list_widget_item::ListWidgetItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_widgets_c_QListWidget_G_operator_shr(in_ as *mut ::qt_core::data_stream::DataStream,
                                                   item as *mut ::list_widget_item::ListWidgetItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

impl ::cpp_utils::DynamicCast<::list_widget::ListWidget> for ::abstract_item_view::AbstractItemView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_widget::ListWidget> for ::abstract_scroll_area::AbstractScrollArea {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_widget::ListWidget> for ::frame::Frame {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_widget::ListWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QFrame(self as *mut ::frame::Frame) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_widget::ListWidget> for ::list_view::ListView {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_widget::ListWidget> {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QListView(self as *mut ::list_view::ListView)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QListView(self as *const ::list_view::ListView as *mut ::list_view::ListView) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::list_widget::ListWidget> for ::widget::Widget {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::list_widget::ListWidget> {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QWidget(self as *mut ::widget::Widget) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::list_widget::ListWidget> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_dynamic_cast_QListWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QObject_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QObject_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_gui::paint_device::PaintDevice> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::qt_gui::paint_device::PaintDevice {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QPaintDevice_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_gui::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QPaintDevice_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_view::AbstractItemView> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_item_view::AbstractItemView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_G_static_cast_QAbstractItemView_ptr(self as *mut ::list_widget::ListWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_item_view::AbstractItemView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QAbstractItemView_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_scroll_area::AbstractScrollArea> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::abstract_scroll_area::AbstractScrollArea {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_G_static_cast_QAbstractScrollArea_ptr(self as *mut ::list_widget::ListWidget)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_scroll_area::AbstractScrollArea {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QAbstractScrollArea_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::frame::Frame> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::frame::Frame {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QFrame_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::frame::Frame {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QFrame_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::list_view::ListView> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::list_view::ListView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::widget::Widget> for ::list_widget::ListWidget {
  fn static_cast_mut(&mut self) -> &mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QWidget_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QWidget_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::abstract_item_view::AbstractItemView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractItemView(self as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractItemView(self as *const ::abstract_item_view::AbstractItemView as *mut ::abstract_item_view::AbstractItemView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::abstract_scroll_area::AbstractScrollArea {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractScrollArea(self as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QAbstractScrollArea(self as *const ::abstract_scroll_area::AbstractScrollArea as *mut ::abstract_scroll_area::AbstractScrollArea);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::frame::Frame {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QFrame(self as *mut ::frame::Frame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QFrame(self as *const ::frame::Frame as *mut ::frame::Frame);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::list_view::ListView {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QListView(self as *mut ::list_view::ListView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QListView(self as *const ::list_view::ListView as *mut ::list_view::ListView);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::qt_gui::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QPaintDevice(self as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QPaintDevice(self as *const ::qt_gui::paint_device::PaintDevice as *mut ::qt_gui::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::list_widget::ListWidget> for ::widget::Widget {
  unsafe fn static_cast_mut(&mut self) -> &mut ::list_widget::ListWidget {
    let ffi_result =
      ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QWidget(self as *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::list_widget::ListWidget {
    let ffi_result = ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListWidget_ptr_QWidget(self as *const ::widget::Widget as *mut ::widget::Widget);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::list_widget::ListWidget {
  type Target = ::list_view::ListView;
  fn deref(&self) -> &::list_view::ListView {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(self as *const ::list_widget::ListWidget as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::list_widget::ListWidget {
  fn deref_mut(&mut self) -> &mut ::list_view::ListView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QListWidget_G_static_cast_QListView_ptr(self as *mut ::list_widget::ListWidget) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListWidget::item_at](../struct.ListWidget.html#method.item_at) method.
  pub trait ListWidgetItemAtArgs<'largs> {
    fn exec(self, original_self: &'largs ::list_widget::ListWidget) -> *mut ::list_widget_item::ListWidgetItem;
  }
  impl<'largs> ListWidgetItemAtArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::list_widget::ListWidget) -> *mut ::list_widget_item::ListWidgetItem {
      let p = self;
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_itemAt_p(original_self as *const ::list_widget::ListWidget,
                                                 p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> ListWidgetItemAtArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list_widget::ListWidget) -> *mut ::list_widget_item::ListWidgetItem {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_widgets_c_QListWidget_itemAt_x_y(original_self as *const ::list_widget::ListWidget, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidget::scroll_to_item](../struct.ListWidget.html#method.scroll_to_item) method.
  pub trait ListWidgetScrollToItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> ();
  }
  impl<'largs> ListWidgetScrollToItemArgs<'largs> for *const ::list_widget_item::ListWidgetItem {
    unsafe fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> () {
      let item = self;
      ::ffi::qt_widgets_c_QListWidget_scrollToItem_item(original_self as *mut ::list_widget::ListWidget, item)
    }
  }
  impl<'largs> ListWidgetScrollToItemArgs<'largs>
    for (*const ::list_widget_item::ListWidgetItem, &'largs ::abstract_item_view::ScrollHint) {
    unsafe fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> () {
      let item = self.0;
      let hint = self.1;
      ::ffi::qt_widgets_c_QListWidget_scrollToItem_item_hint(original_self as *mut ::list_widget::ListWidget,
                                                             item,
                                                             hint as *const ::abstract_item_view::ScrollHint)
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidget::sort_items](../struct.ListWidget.html#method.sort_items) method.
  pub trait ListWidgetSortItemsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> ();
  }
  impl<'largs> ListWidgetSortItemsArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> () {

      unsafe { ::ffi::qt_widgets_c_QListWidget_sortItems_no_args(original_self as *mut ::list_widget::ListWidget) }
    }
  }
  impl<'largs> ListWidgetSortItemsArgs<'largs> for &'largs ::qt_core::qt::SortOrder {
    fn exec(self, original_self: &'largs mut ::list_widget::ListWidget) -> () {
      let order = self;
      unsafe {
        ::ffi::qt_widgets_c_QListWidget_sortItems_order(original_self as *mut ::list_widget::ListWidget,
                                                        order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
}
