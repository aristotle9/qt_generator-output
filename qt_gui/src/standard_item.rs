/// C++ type: <span style='color: green;'>```QStandardItem::ItemType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ItemType {
  /// C++ enum variant: <span style='color: green;'>```Type = 0```</span>
  Type = 0,
  /// C++ enum variant: <span style='color: green;'>```UserType = 1000```</span>
  UserType = 1000,
}

/// C++ type: <span style='color: green;'>```QStandardItem```</span>
#[repr(C)]
pub struct StandardItem(u8);

impl StandardItem {
  /// C++ method: <span style='color: green;'>```QString QStandardItem::accessibleDescription() const```</span>
  ///
  ///
  pub fn accessible_description(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_accessibleDescription_to_output(self as *const ::standard_item::StandardItem,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStandardItem::accessibleText() const```</span>
  ///
  ///
  pub fn accessible_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_accessibleText_to_output(self as *const ::standard_item::StandardItem,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::appendColumn(const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn append_column(&mut self, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_appendColumn(self as *mut ::standard_item::StandardItem,
                                                 items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::appendRow(const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn append_row(&mut self, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_appendRow_items(self as *mut ::standard_item::StandardItem,
                                                    items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::appendRow(QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn append_row_unsafe(&mut self, item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItem_appendRow_item(self as *mut ::standard_item::StandardItem, item)
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::appendRows(const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn append_rows(&mut self, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_appendRows(self as *mut ::standard_item::StandardItem,
                                               items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QStandardItem::background() const```</span>
  ///
  ///
  pub fn background(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_background_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::child```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn child(&self, ::libc::c_int) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItem::child(int row) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn child(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItem::child(int row, int column = ?) const```</span>
  ///
  ///
  pub fn child<'largs, Args>(&'largs self, args: Args) -> *mut ::standard_item::StandardItem
    where Args: overloading::StandardItemChildArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QStandardItem* QStandardItem::clone() const```</span>
  ///
  ///
  pub fn clone(&self) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QStandardItem_clone(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```int QStandardItem::column() const```</span>
  ///
  ///
  pub fn column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItem_column(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```int QStandardItem::columnCount() const```</span>
  ///
  ///
  pub fn column_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItem_columnCount(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, ()) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItem::data() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, ::libc::c_int) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItem::data(int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::StandardItemDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFont QStandardItem::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_font_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QBrush QStandardItem::foreground() const```</span>
  ///
  ///
  pub fn foreground(&self) -> ::brush::Brush {
    {
      let mut object: ::brush::Brush = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_foreground_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::hasChildren() const```</span>
  ///
  ///
  pub fn has_children(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_hasChildren(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```QIcon QStandardItem::icon() const```</span>
  ///
  ///
  pub fn icon(&self) -> ::icon::Icon {
    {
      let mut object: ::icon::Icon = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_icon_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QStandardItem::index() const```</span>
  ///
  ///
  pub fn index(&self) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_index_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::insertColumn(int column, const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn insert_column(&mut self, column: ::libc::c_int, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_insertColumn(self as *mut ::standard_item::StandardItem,
                                                 column,
                                                 items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::insertColumns(int column, int count)```</span>
  ///
  ///
  pub fn insert_columns(&mut self, column: ::libc::c_int, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_insertColumns(self as *mut ::standard_item::StandardItem, column, count) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::insertRow(int row, const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn insert_row(&mut self, row: ::libc::c_int, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_insertRow_row_items(self as *mut ::standard_item::StandardItem,
                                                        row,
                                                        items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::insertRow(int row, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn insert_row_unsafe(&mut self, row: ::libc::c_int, item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItem_insertRow_row_item(self as *mut ::standard_item::StandardItem, row, item)
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, &::list::ListStandardItemMutPtr)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::insertRows(int row, const QList<QStandardItem*>& items)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::insertRows(int row, int count)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QStandardItem::isAutoTristate() const```</span>
  ///
  ///
  pub fn is_auto_tristate(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isAutoTristate(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isCheckable() const```</span>
  ///
  ///
  pub fn is_checkable(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isCheckable(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isDragEnabled() const```</span>
  ///
  ///
  pub fn is_drag_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isDragEnabled(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isDropEnabled() const```</span>
  ///
  ///
  pub fn is_drop_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isDropEnabled(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isEditable() const```</span>
  ///
  ///
  pub fn is_editable(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isEditable(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isEnabled(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isSelectable() const```</span>
  ///
  ///
  pub fn is_selectable(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isSelectable(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isTristate() const```</span>
  ///
  ///
  pub fn is_tristate(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isTristate(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```bool QStandardItem::isUserTristate() const```</span>
  ///
  ///
  pub fn is_user_tristate(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QStandardItem_isUserTristate(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel* QStandardItem::model() const```</span>
  ///
  ///
  pub fn model(&self) -> *mut ::standard_item_model::StandardItemModel {
    unsafe { ::ffi::qt_gui_c_QStandardItem_model(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::QStandardItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::standard_item::StandardItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItem::QStandardItem()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::icon::Icon, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::standard_item::StandardItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItem::QStandardItem(const QIcon& icon, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::standard_item::StandardItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItem::QStandardItem(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::standard_item::StandardItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItem::QStandardItem(int rows)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::standard_item::StandardItem>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItem::QStandardItem(int rows, int columns = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::standard_item::StandardItem>
    where Args: overloading::StandardItemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItem::operator<(const QStandardItem& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::standard_item::StandardItem) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_operator_lt(self as *const ::standard_item::StandardItem,
                                                other as *const ::standard_item::StandardItem)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItem::parent() const```</span>
  ///
  ///
  pub fn parent(&self) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QStandardItem_parent(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QStandardItem::read(QDataStream& in)```</span>
  ///
  ///
  pub fn read(&mut self, in_: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_read(self as *mut ::standard_item::StandardItem,
                                         in_ as *mut ::qt_core::data_stream::DataStream)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::removeColumn(int column)```</span>
  ///
  ///
  pub fn remove_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_removeColumn(self as *mut ::standard_item::StandardItem, column) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::removeColumns(int column, int count)```</span>
  ///
  ///
  pub fn remove_columns(&mut self, column: ::libc::c_int, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_removeColumns(self as *mut ::standard_item::StandardItem, column, count) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::removeRow(int row)```</span>
  ///
  ///
  pub fn remove_row(&mut self, row: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_removeRow(self as *mut ::standard_item::StandardItem, row) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::removeRows(int row, int count)```</span>
  ///
  ///
  pub fn remove_rows(&mut self, row: ::libc::c_int, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_removeRows(self as *mut ::standard_item::StandardItem, row, count) }
  }

  /// C++ method: <span style='color: green;'>```int QStandardItem::row() const```</span>
  ///
  ///
  pub fn row(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItem_row(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```int QStandardItem::rowCount() const```</span>
  ///
  ///
  pub fn row_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItem_rowCount(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setAccessibleDescription(const QString& accessibleDescription)```</span>
  ///
  ///
  pub fn set_accessible_description(&mut self, accessible_description: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setAccessibleDescription(self as *mut ::standard_item::StandardItem, accessible_description as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setAccessibleText(const QString& accessibleText)```</span>
  ///
  ///
  pub fn set_accessible_text(&mut self, accessible_text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setAccessibleText(self as *mut ::standard_item::StandardItem,
                                                      accessible_text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setAutoTristate(bool tristate)```</span>
  ///
  ///
  pub fn set_auto_tristate(&mut self, tristate: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setAutoTristate(self as *mut ::standard_item::StandardItem, tristate) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setBackground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_background(&mut self, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setBackground(self as *mut ::standard_item::StandardItem,
                                                  brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setCheckState(Qt::CheckState checkState)```</span>
  ///
  ///
  pub fn set_check_state(&mut self, check_state: &::qt_core::qt::CheckState) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setCheckState(self as *mut ::standard_item::StandardItem,
                                                  check_state as *const ::qt_core::qt::CheckState)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setCheckable(bool checkable)```</span>
  ///
  ///
  pub fn set_checkable(&mut self, checkable: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setCheckable(self as *mut ::standard_item::StandardItem, checkable) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::setChild```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_child(&mut self, (::libc::c_int, *mut ::standard_item::StandardItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::setChild(int row, QStandardItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_child(&mut self, (::libc::c_int, ::libc::c_int, *mut ::standard_item::StandardItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::setChild(int row, int column, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn set_child<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemSetChildArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStandardItem::setColumnCount(int columns)```</span>
  ///
  ///
  pub fn set_column_count(&mut self, columns: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setColumnCount(self as *mut ::standard_item::StandardItem, columns) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, &::qt_core::variant::Variant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStandardItem::setData(const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::variant::Variant, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStandardItem::setData(const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStandardItem::setDragEnabled(bool dragEnabled)```</span>
  ///
  ///
  pub fn set_drag_enabled(&mut self, drag_enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setDragEnabled(self as *mut ::standard_item::StandardItem, drag_enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setDropEnabled(bool dropEnabled)```</span>
  ///
  ///
  pub fn set_drop_enabled(&mut self, drop_enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setDropEnabled(self as *mut ::standard_item::StandardItem, drop_enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setEditable(bool editable)```</span>
  ///
  ///
  pub fn set_editable(&mut self, editable: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setEditable(self as *mut ::standard_item::StandardItem, editable) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setEnabled(self as *mut ::standard_item::StandardItem, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::font::Font) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setFont(self as *mut ::standard_item::StandardItem,
                                            font as *const ::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setForeground(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_foreground(&mut self, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setForeground(self as *mut ::standard_item::StandardItem,
                                                  brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setIcon(const QIcon& icon)```</span>
  ///
  ///
  pub fn set_icon(&mut self, icon: &::icon::Icon) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setIcon(self as *mut ::standard_item::StandardItem,
                                            icon as *const ::icon::Icon)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setRowCount(int rows)```</span>
  ///
  ///
  pub fn set_row_count(&mut self, rows: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setRowCount(self as *mut ::standard_item::StandardItem, rows) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setSelectable(bool selectable)```</span>
  ///
  ///
  pub fn set_selectable(&mut self, selectable: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setSelectable(self as *mut ::standard_item::StandardItem, selectable) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setSizeHint(const QSize& sizeHint)```</span>
  ///
  ///
  pub fn set_size_hint(&mut self, size_hint: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setSizeHint(self as *mut ::standard_item::StandardItem,
                                                size_hint as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setStatusTip(const QString& statusTip)```</span>
  ///
  ///
  pub fn set_status_tip(&mut self, status_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setStatusTip(self as *mut ::standard_item::StandardItem,
                                                 status_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setText(self as *mut ::standard_item::StandardItem,
                                            text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setToolTip(const QString& toolTip)```</span>
  ///
  ///
  pub fn set_tool_tip(&mut self, tool_tip: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setToolTip(self as *mut ::standard_item::StandardItem,
                                               tool_tip as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setTristate(bool tristate)```</span>
  ///
  ///
  pub fn set_tristate(&mut self, tristate: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setTristate(self as *mut ::standard_item::StandardItem, tristate) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setUserTristate(bool tristate)```</span>
  ///
  ///
  pub fn set_user_tristate(&mut self, tristate: bool) {
    unsafe { ::ffi::qt_gui_c_QStandardItem_setUserTristate(self as *mut ::standard_item::StandardItem, tristate) }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItem::setWhatsThis(const QString& whatsThis)```</span>
  ///
  ///
  pub fn set_whats_this(&mut self, whats_this: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_setWhatsThis(self as *mut ::standard_item::StandardItem,
                                                 whats_this as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QStandardItem::sizeHint() const```</span>
  ///
  ///
  pub fn size_hint(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_sizeHint_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::sortChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort_children(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::sortChildren(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort_children(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItem::sortChildren(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort_children<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemSortChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QStandardItem::statusTip() const```</span>
  ///
  ///
  pub fn status_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_statusTip_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem::takeChild```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn take_child(&mut self, ::libc::c_int) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItem::takeChild(int row)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn take_child(&mut self, (::libc::c_int, ::libc::c_int)) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItem::takeChild(int row, int column = ?)```</span>
  ///
  ///
  pub fn take_child<'largs, Args>(&'largs mut self, args: Args) -> *mut ::standard_item::StandardItem
    where Args: overloading::StandardItemTakeChildArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QStandardItem::takeColumn(int column)```</span>
  ///
  ///
  pub fn take_column(&mut self, column: ::libc::c_int) -> ::list::ListStandardItemMutPtr {
    {
      let mut object: ::list::ListStandardItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_takeColumn_to_output(self as *mut ::standard_item::StandardItem,
                                                           column,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QStandardItem::takeRow(int row)```</span>
  ///
  ///
  pub fn take_row(&mut self, row: ::libc::c_int) -> ::list::ListStandardItemMutPtr {
    {
      let mut object: ::list::ListStandardItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_takeRow_to_output(self as *mut ::standard_item::StandardItem, row, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStandardItem::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_text_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStandardItem::toolTip() const```</span>
  ///
  ///
  pub fn tool_tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_toolTip_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStandardItem::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItem_type(self as *const ::standard_item::StandardItem) }
  }

  /// C++ method: <span style='color: green;'>```QString QStandardItem::whatsThis() const```</span>
  ///
  ///
  pub fn whats_this(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_whatsThis_to_output(self as *const ::standard_item::StandardItem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QStandardItem::write(QDataStream& out) const```</span>
  ///
  ///
  pub fn write(&self, out: &mut ::qt_core::data_stream::DataStream) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItem_write(self as *const ::standard_item::StandardItem,
                                          out as *mut ::qt_core::data_stream::DataStream)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::standard_item::StandardItem {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QStandardItem_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StandardItem::child](../struct.StandardItem.html#method.child) method.
  pub trait StandardItemChildArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem;
  }
  impl<'largs> StandardItemChildArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem {
      let row = self;
      unsafe { ::ffi::qt_gui_c_QStandardItem_child_row(original_self as *const ::standard_item::StandardItem, row) }
    }
  }
  impl<'largs> StandardItemChildArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem {
      let row = self.0;
      let column = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_child_row_column(original_self as *const ::standard_item::StandardItem,
                                                       row,
                                                       column)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::data](../struct.StandardItem.html#method.data) method.
  pub trait StandardItemDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> ::qt_core::variant::Variant;
  }
  impl<'largs> StandardItemDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> ::qt_core::variant::Variant {

      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItem_data_to_output_no_args(original_self as *const ::standard_item::StandardItem,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StandardItemDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::standard_item::StandardItem) -> ::qt_core::variant::Variant {
      let role = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItem_data_to_output_role(original_self as *const ::standard_item::StandardItem,
                                                            role,
                                                            &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::insert_rows](../struct.StandardItem.html#method.insert_rows) method.
  pub trait StandardItemInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> ();
  }
  impl<'largs> StandardItemInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let row = self.0;
      let count = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_insertRows_row_count(original_self as *mut ::standard_item::StandardItem,
                                                           row,
                                                           count)
      }
    }
  }
  impl<'largs> StandardItemInsertRowsArgs<'largs> for (::libc::c_int, &'largs ::list::ListStandardItemMutPtr) {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let row = self.0;
      let items = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_insertRows_row_items(original_self as *mut ::standard_item::StandardItem,
                                                           row,
                                                           items as *const ::list::ListStandardItemMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::new](../struct.StandardItem.html#method.new) method.
  pub trait StandardItemNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem>;
  }
  impl<'a> StandardItemNewArgs for (&'a ::icon::Icon, &'a ::qt_core::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem> {
      let icon = self.0;
      let text = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QStandardItem_new_icon_text(icon as *const ::icon::Icon,
                                                    text as *const ::qt_core::string::String)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StandardItemNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItem_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StandardItemNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem> {
      let rows = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItem_new_rows(rows) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StandardItemNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem> {
      let rows = self.0;
      let columns = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItem_new_rows_columns(rows, columns) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> StandardItemNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item::StandardItem> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItem_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::set_child](../struct.StandardItem.html#method.set_child) method.
  pub trait StandardItemSetChildArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> ();
  }
  impl<'largs> StandardItemSetChildArgs<'largs> for (::libc::c_int, ::libc::c_int, *mut ::standard_item::StandardItem) {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let row = self.0;
      let column = self.1;
      let item = self.2;
      ::ffi::qt_gui_c_QStandardItem_setChild_row_column_item(original_self as *mut ::standard_item::StandardItem,
                                                             row,
                                                             column,
                                                             item)
    }
  }
  impl<'largs> StandardItemSetChildArgs<'largs> for (::libc::c_int, *mut ::standard_item::StandardItem) {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let row = self.0;
      let item = self.1;
      ::ffi::qt_gui_c_QStandardItem_setChild_row_item(original_self as *mut ::standard_item::StandardItem,
                                                      row,
                                                      item)
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::set_data](../struct.StandardItem.html#method.set_data) method.
  pub trait StandardItemSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> ();
  }
  impl<'largs> StandardItemSetDataArgs<'largs> for &'largs ::qt_core::variant::Variant {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let value = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_setData_value(original_self as *mut ::standard_item::StandardItem,
                                                    value as *const ::qt_core::variant::Variant)
      }
    }
  }
  impl<'largs> StandardItemSetDataArgs<'largs> for (&'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let value = self.0;
      let role = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_setData_value_role(original_self as *mut ::standard_item::StandardItem,
                                                         value as *const ::qt_core::variant::Variant,
                                                         role)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::sort_children](../struct.StandardItem.html#method.sort_children) method.
  pub trait StandardItemSortChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> ();
  }
  impl<'largs> StandardItemSortChildrenArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_sortChildren_column(original_self as *mut ::standard_item::StandardItem, column)
      }
    }
  }
  impl<'largs> StandardItemSortChildrenArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_sortChildren_column_order(original_self as *mut ::standard_item::StandardItem,
                                                                column,
                                                                order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItem::take_child](../struct.StandardItem.html#method.take_child) method.
  pub trait StandardItemTakeChildArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem;
  }
  impl<'largs> StandardItemTakeChildArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem {
      let row = self;
      unsafe { ::ffi::qt_gui_c_QStandardItem_takeChild_row(original_self as *mut ::standard_item::StandardItem, row) }
    }
  }
  impl<'largs> StandardItemTakeChildArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item::StandardItem) -> *mut ::standard_item::StandardItem {
      let row = self.0;
      let column = self.1;
      unsafe {
        ::ffi::qt_gui_c_QStandardItem_takeChild_row_column(original_self as *mut ::standard_item::StandardItem,
                                                           row,
                                                           column)
      }
    }
  }
}
