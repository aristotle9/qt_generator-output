/// C++ type: <span style='color: green;'>```QStandardItemModel```</span>
#[repr(C)]
pub struct StandardItemModel(u8);

impl StandardItemModel {
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::appendColumn(const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn append_column(&mut self, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_appendColumn(self as *mut ::standard_item_model::StandardItemModel,
                                                      items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::appendRow(const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn append_row(&mut self, items: &::list::ListStandardItemMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_appendRow_items(self as *mut ::standard_item_model::StandardItemModel,
                                                         items as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::appendRow(QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn append_row_unsafe(&mut self, item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItemModel_appendRow_item(self as *mut ::standard_item_model::StandardItemModel, item)
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QStandardItemModel_clear(self as *mut ::standard_item_model::StandardItemModel) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStandardItemModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStandardItemModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StandardItemModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::qt_core::model_index::ModelIndex) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItemModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::qt_core::model_index::ModelIndex, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItemModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::StandardItemModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::qt_core::mime_data::MimeData,
                               action: &::qt_core::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::qt_core::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_gui_c_QStandardItemModel_dropMimeData(self as *mut ::standard_item_model::StandardItemModel,
                                                    data,
                                                    action as *const ::qt_core::qt::DropAction,
                                                    row,
                                                    column,
                                                    parent as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::qt_core::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::hasChildren(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::StandardItemModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItemModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QStandardItemModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::StandardItemModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::horizontalHeaderItem(int column) const```</span>
  ///
  ///
  pub fn horizontal_header_item(&self, column: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_horizontalHeaderItem(self as *const ::standard_item_model::StandardItemModel,
                                                              column)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QStandardItemModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QStandardItemModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::qt_core::model_index::ModelIndex
    where Args: overloading::StandardItemModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QModelIndex QStandardItemModel::indexFromItem(const QStandardItem* item) const```</span>
  ///
  ///
  pub unsafe fn index_from_item(&self,
                                item: *const ::standard_item::StandardItem)
                                -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QStandardItemModel_indexFromItem_to_output(self as *const ::standard_item_model::StandardItemModel, item, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::insertColumn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_column(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStandardItemModel::insertColumn(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_column(&mut self, (::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStandardItemModel::insertColumn(int column, const QModelIndex& parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_column(&mut self, (::libc::c_int, &::list::ListStandardItemMutPtr)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::insertColumn(int column, const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn insert_column<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::StandardItemModelInsertColumnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::insertColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::insertColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::insertColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelInsertColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::insertRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_row(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStandardItemModel::insertRow(int row)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QStandardItemModel::insertRow(int row, const QModelIndex& parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, &::list::ListStandardItemMutPtr)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::insertRow(int row, const QList<QStandardItem*>& items)```</span>
  ///
  ///
  pub fn insert_row<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::StandardItemModelInsertRowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::insertRow(int row, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn insert_row_unsafe(&mut self, row: ::libc::c_int, item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItemModel_insertRow_row_item(self as *mut ::standard_item_model::StandardItemModel,
                                                          row,
                                                          item)
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::insertRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::insertRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::invisibleRootItem() const```</span>
  ///
  ///
  pub fn invisible_root_item(&self) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_invisibleRootItem(self as *const ::standard_item_model::StandardItemModel)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::item```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn item(&self, ::libc::c_int) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::item(int row) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn item(&self, (::libc::c_int, ::libc::c_int)) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::item(int row, int column = ?) const```</span>
  ///
  ///
  pub fn item<'largs, Args>(&'largs self, args: Args) -> *mut ::standard_item::StandardItem
    where Args: overloading::StandardItemModelItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QMap<int, QVariant> QStandardItemModel::itemData(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn item_data(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::map::MapCIntVariant {
    {
      let mut object: ::qt_core::map::MapCIntVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_itemData_to_output(self as *const ::standard_item_model::StandardItemModel, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::itemFromIndex(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn item_from_index(&self, index: &::qt_core::model_index::ModelIndex) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_itemFromIndex(self as *const ::standard_item_model::StandardItemModel,
                                                       index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QStandardItem* QStandardItemModel::itemPrototype() const```</span>
  ///
  ///
  pub fn item_prototype(&self) -> *const ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QStandardItemModel_itemPrototype(self as *const ::standard_item_model::StandardItemModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QStandardItemModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QStandardItemModel_metaObject(self as *const ::standard_item_model::StandardItemModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QStandardItemModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::qt_core::list::ListModelIndex) -> *mut ::qt_core::mime_data::MimeData {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_mimeData(self as *const ::standard_item_model::StandardItemModel,
                                                  indexes as *const ::qt_core::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QStandardItemModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_mimeTypes_to_output(self as *const ::standard_item_model::StandardItemModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::QStandardItemModel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItemModel::QStandardItemModel()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItemModel::QStandardItemModel(int rows, int columns)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>
    where Args: overloading::StandardItemModelNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::QStandardItemModel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItemModel::QStandardItemModel(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_int, ::libc::c_int, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStandardItemModel::QStandardItemModel(int rows, int columns, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>
    where Args: overloading::StandardItemModelNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QStandardItemModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::qt_core::model_index::ModelIndex) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_parent_to_output(self as *const ::standard_item_model::StandardItemModel,
                                                            child as *const ::qt_core::model_index::ModelIndex,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QStandardItemModel::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QStandardItemModel_qt_metacall(self as *mut ::standard_item_model::StandardItemModel,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QStandardItemModel::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QStandardItemModel_qt_metacast(self as *mut ::standard_item_model::StandardItemModel, arg1)
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::removeColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::removeColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::removeColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelRemoveColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::removeRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::removeRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::removeRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelRemoveRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStandardItemModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QStandardItemModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StandardItemModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setColumnCount(int columns)```</span>
  ///
  ///
  pub fn set_column_count(&mut self, columns: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setColumnCount(self as *mut ::standard_item_model::StandardItemModel,
                                                        columns)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItemModel::setHeaderData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt_core::qt::Orientation, &::qt_core::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt_core::qt::Orientation, &::qt_core::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_header_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::StandardItemModelSetHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setHorizontalHeaderItem(int column, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn set_horizontal_header_item(&mut self,
                                           column: ::libc::c_int,
                                           item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItemModel_setHorizontalHeaderItem(self as *mut ::standard_item_model::StandardItemModel,
                                                               column,
                                                               item)
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setHorizontalHeaderLabels(const QStringList& labels)```</span>
  ///
  ///
  pub fn set_horizontal_header_labels(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe { ::ffi::qt_gui_c_QStandardItemModel_setHorizontalHeaderLabels(self as *mut ::standard_item_model::StandardItemModel, labels as *const ::qt_core::string_list::StringList) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::setItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_item(&mut self, (::libc::c_int, *mut ::standard_item::StandardItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setItem(int row, QStandardItem* item)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_item(&mut self, (::libc::c_int, ::libc::c_int, *mut ::standard_item::StandardItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setItem(int row, int column, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemModelSetItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QStandardItemModel::setItemData(const QModelIndex& index, const QMap<int, QVariant>& roles)```</span>
  ///
  ///
  pub fn set_item_data(&mut self,
                       index: &::qt_core::model_index::ModelIndex,
                       roles: &::qt_core::map::MapCIntVariant)
                       -> bool {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setItemData(self as *mut ::standard_item_model::StandardItemModel,
                                                     index as *const ::qt_core::model_index::ModelIndex,
                                                     roles as *const ::qt_core::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setItemPrototype(const QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn set_item_prototype(&mut self, item: *const ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItemModel_setItemPrototype(self as *mut ::standard_item_model::StandardItemModel, item)
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setItemRoleNames(const QHash<int, QByteArray>& roleNames)```</span>
  ///
  ///
  pub fn set_item_role_names(&mut self, role_names: &::qt_core::hash::HashCIntByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setItemRoleNames(self as *mut ::standard_item_model::StandardItemModel,
                                                          role_names as *const ::qt_core::hash::HashCIntByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setRowCount(int rows)```</span>
  ///
  ///
  pub fn set_row_count(&mut self, rows: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setRowCount(self as *mut ::standard_item_model::StandardItemModel, rows)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setSortRole(int role)```</span>
  ///
  ///
  pub fn set_sort_role(&mut self, role: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setSortRole(self as *mut ::standard_item_model::StandardItemModel, role)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setVerticalHeaderItem(int row, QStandardItem* item)```</span>
  ///
  ///
  pub unsafe fn set_vertical_header_item(&mut self, row: ::libc::c_int, item: *mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QStandardItemModel_setVerticalHeaderItem(self as *mut ::standard_item_model::StandardItemModel,
                                                             row,
                                                             item)
  }

  /// C++ method: <span style='color: green;'>```void QStandardItemModel::setVerticalHeaderLabels(const QStringList& labels)```</span>
  ///
  ///
  pub fn set_vertical_header_labels(&mut self, labels: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_setVerticalHeaderLabels(self as *mut ::standard_item_model::StandardItemModel, labels as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QStandardItemModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
  ///
  ///
  pub fn sibling(&self,
                 row: ::libc::c_int,
                 column: ::libc::c_int,
                 idx: &::qt_core::model_index::ModelIndex)
                 -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_sibling_to_output(self as *const ::standard_item_model::StandardItemModel,
                                                             row,
                                                             column,
                                                             idx as *const ::qt_core::model_index::ModelIndex,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStandardItemModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QStandardItemModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::StandardItemModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QStandardItemModel::sortRole() const```</span>
  ///
  ///
  pub fn sort_role(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QStandardItemModel_sortRole(self as *const ::standard_item_model::StandardItemModel) }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QStandardItemModel::takeColumn(int column)```</span>
  ///
  ///
  pub fn take_column(&mut self, column: ::libc::c_int) -> ::list::ListStandardItemMutPtr {
    {
      let mut object: ::list::ListStandardItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_takeColumn_to_output(self as *mut ::standard_item_model::StandardItemModel, column, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::takeHorizontalHeaderItem(int column)```</span>
  ///
  ///
  pub fn take_horizontal_header_item(&mut self, column: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_takeHorizontalHeaderItem(self as *mut ::standard_item_model::StandardItemModel, column)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItemModel::takeItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn take_item(&mut self, ::libc::c_int) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::takeItem(int row)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn take_item(&mut self, (::libc::c_int, ::libc::c_int)) -> *mut ::standard_item::StandardItem```<br>
  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::takeItem(int row, int column = ?)```</span>
  ///
  ///
  pub fn take_item<'largs, Args>(&'largs mut self, args: Args) -> *mut ::standard_item::StandardItem
    where Args: overloading::StandardItemModelTakeItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QStandardItemModel::takeRow(int row)```</span>
  ///
  ///
  pub fn take_row(&mut self, row: ::libc::c_int) -> ::list::ListStandardItemMutPtr {
    {
      let mut object: ::list::ListStandardItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_takeRow_to_output(self as *mut ::standard_item_model::StandardItemModel,
                                                             row,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::takeVerticalHeaderItem(int row)```</span>
  ///
  ///
  pub fn take_vertical_header_item(&mut self, row: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_takeVerticalHeaderItem(self as *mut ::standard_item_model::StandardItemModel,
                                                                row)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStandardItemModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QStandardItemModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStandardItemModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QStandardItemModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QStandardItemModel::verticalHeaderItem(int row) const```</span>
  ///
  ///
  pub fn vertical_header_item(&self, row: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe {
      ::ffi::qt_gui_c_QStandardItemModel_verticalHeaderItem(self as *const ::standard_item_model::StandardItemModel,
                                                            row)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::standard_item_model::StandardItemModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QStandardItemModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `StandardItemModel`.
  pub struct Signals<'a>(&'a ::standard_item_model::StandardItemModel);
  /// Represents a built-in Qt signal `QStandardItemModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsAboutToBeMoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::columnsMoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsMoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsMoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsMoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::itemChanged`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().item_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ItemChanged<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ItemChanged<'a> {
    type Arguments = (*mut ::standard_item::StandardItem,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2itemChanged(QStandardItem*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ItemChanged<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeMoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::modelReset`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ModelReset<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ModelReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelReset()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModelReset<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsMoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsMoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsMoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::qt_core::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsMoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::columnsInserted`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsInserted<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsInserted<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsRemoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsRemoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ModelAboutToBeReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelAboutToBeReset()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ModelAboutToBeReset<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::layoutChanged`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct LayoutChanged<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for LayoutChanged<'a> {
    type Arguments = (&'static ::qt_core::list::ListPersistentModelIndex,
     &'static ::qt_core::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsAboutToBeInserted<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for LayoutAboutToBeChanged<'a> {
    type Arguments = (&'static ::qt_core::list::ListPersistentModelIndex,
     &'static ::qt_core::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutAboutToBeChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayoutAboutToBeChanged<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::dataChanged`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct DataChanged<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for DataChanged<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,
     &'static ::qt_core::model_index::ModelIndex,
     &'static ::qt_core::vector::VectorCInt);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataChanged(const QModelIndex&,const QModelIndex&,const QVector< int >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DataChanged<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for HeaderDataChanged<'a> {
    type Arguments = (&'static ::qt_core::qt::Orientation, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2headerDataChanged(Qt::Orientation,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeaderDataChanged<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsAboutToBeInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsAboutToBeInserted<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ColumnsRemoved<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ColumnsRemoved<'a> {}
  /// Represents a built-in Qt signal `QStandardItemModel::rowsInserted`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct RowsInserted<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for RowsInserted<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RowsInserted<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::itemChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn item_changed(&self) -> ItemChanged {
      ItemChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QStandardItemModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `StandardItemModel`.
  pub struct Slots<'a>(&'a ::standard_item_model::StandardItemModel);
  /// Represents a built-in Qt slot `QStandardItemModel::resetInternalData`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct ResetInternalData<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  /// Represents a built-in Qt slot `QStandardItemModel::revert`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct Revert<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  /// Represents a built-in Qt slot `QStandardItemModel::submit`.
  ///
  /// An object of this type can be created from `StandardItemModel` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `StandardItemModel` object.
  pub struct Submit<'a>(&'a ::standard_item_model::StandardItemModel);
  impl<'a> ::qt_core::connection::Receiver for Submit<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1submit()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QStandardItemModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStandardItemModel::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QStandardItemModel::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
  }
  impl ::standard_item_model::StandardItemModel {
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

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QStandardItem& item)```</span>
///
///
pub fn op_shl<'l0, 'l1>(out: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 ::standard_item::StandardItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QStandardItemModel_G_operator_shl(out as *mut ::qt_core::data_stream::DataStream,
                                                      item as *const ::standard_item::StandardItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QStandardItem& item)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::qt_core::data_stream::DataStream,
                        item: &'l1 mut ::standard_item::StandardItem)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QStandardItemModel_G_operator_shr(in_ as *mut ::qt_core::data_stream::DataStream,
                                                      item as *mut ::standard_item::StandardItem)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

impl ::cpp_utils::StaticCast<::qt_core::abstract_item_model::AbstractItemModel> for ::standard_item_model::StandardItemModel {
fn static_cast_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::standard_item_model::StandardItemModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::standard_item_model::StandardItemModel as *mut ::standard_item_model::StandardItemModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::standard_item_model::StandardItemModel {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QObject_ptr(self as *mut ::standard_item_model::StandardItemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QObject_ptr(self as *const ::standard_item_model::StandardItemModel as *mut ::standard_item_model::StandardItemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::standard_item_model::StandardItemModel> for ::qt_core::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::standard_item_model::StandardItemModel {
let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QAbstractItemModel(self as *mut ::qt_core::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::standard_item_model::StandardItemModel {
let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QAbstractItemModel(self as *const ::qt_core::abstract_item_model::AbstractItemModel as *mut ::qt_core::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::standard_item_model::StandardItemModel> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::standard_item_model::StandardItemModel {
    let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::standard_item_model::StandardItemModel {
    let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QStandardItemModel_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::standard_item_model::StandardItemModel {
  type Target = ::qt_core::abstract_item_model::AbstractItemModel;
  fn deref(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::standard_item_model::StandardItemModel as *mut ::standard_item_model::StandardItemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::standard_item_model::StandardItemModel {
  fn deref_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::standard_item_model::StandardItemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StandardItemModel::column_count](../struct.StandardItemModel.html#method.column_count) method.
  pub trait StandardItemModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int;
  }
  impl<'largs> StandardItemModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QStandardItemModel_columnCount_no_args(original_self as *const ::standard_item_model::StandardItemModel) }
    }
  }
  impl<'largs> StandardItemModelColumnCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_columnCount_parent(original_self as *const ::standard_item_model::StandardItemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::data](../struct.StandardItemModel.html#method.data) method.
  pub trait StandardItemModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> StandardItemModelDataArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant {
      let index = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_data_to_output_index(original_self as *const ::standard_item_model::StandardItemModel, index as *const ::qt_core::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StandardItemModelDataArgs<'largs> for (&'largs ::qt_core::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_data_to_output_index_role(original_self as *const ::standard_item_model::StandardItemModel, index as *const ::qt_core::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::has_children](../struct.StandardItemModel.html#method.has_children) method.
  pub trait StandardItemModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> bool {

      unsafe { ::ffi::qt_gui_c_QStandardItemModel_hasChildren_no_args(original_self as *const ::standard_item_model::StandardItemModel) }
    }
  }
  impl<'largs> StandardItemModelHasChildrenArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> bool {
      let parent = self;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_hasChildren_parent(original_self as *const ::standard_item_model::StandardItemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::header_data](../struct.StandardItemModel.html#method.header_data) method.
  pub trait StandardItemModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> StandardItemModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::Orientation) {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation(original_self as *const ::standard_item_model::StandardItemModel, section, orientation as *const ::qt_core::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StandardItemModelHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_headerData_to_output_section_orientation_role(original_self as *const ::standard_item_model::StandardItemModel, section, orientation as *const ::qt_core::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::index](../struct.StandardItemModel.html#method.index) method.
  pub trait StandardItemModelIndexArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> ::qt_core::model_index::ModelIndex;
  }
  impl<'largs> StandardItemModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_index_to_output_row_column(original_self as *const ::standard_item_model::StandardItemModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> StandardItemModelIndexArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QStandardItemModel_index_to_output_row_column_parent(original_self as *const ::standard_item_model::StandardItemModel, row, column, parent as *const ::qt_core::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::insert_column](../struct.StandardItemModel.html#method.insert_column) method.
  pub trait StandardItemModelInsertColumnArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> Self::ReturnType;
  }
  impl<'largs> StandardItemModelInsertColumnArgs<'largs> for ::libc::c_int {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertColumn_column(original_self as *mut ::standard_item_model::StandardItemModel, column) }
    }
  }
  impl<'largs> StandardItemModelInsertColumnArgs<'largs> for (::libc::c_int, &'largs ::list::ListStandardItemMutPtr) {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let column = self.0;
      let items = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertColumn_column_items(original_self as *mut ::standard_item_model::StandardItemModel, column, items as *const ::list::ListStandardItemMutPtr) }
    }
  }
  impl<'largs> StandardItemModelInsertColumnArgs<'largs> for (::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertColumn_column_parent(original_self as *mut ::standard_item_model::StandardItemModel, column, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::insert_columns](../struct.StandardItemModel.html#method.insert_columns) method.
  pub trait StandardItemModelInsertColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelInsertColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertColumns_column_count(original_self as *mut ::standard_item_model::StandardItemModel, column, count) }
    }
  }
  impl<'largs> StandardItemModelInsertColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertColumns_column_count_parent(original_self as *mut ::standard_item_model::StandardItemModel, column, count, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::insert_row](../struct.StandardItemModel.html#method.insert_row) method.
  pub trait StandardItemModelInsertRowArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> Self::ReturnType;
  }
  impl<'largs> StandardItemModelInsertRowArgs<'largs> for ::libc::c_int {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_insertRow_row(original_self as *mut ::standard_item_model::StandardItemModel, row)
      }
    }
  }
  impl<'largs> StandardItemModelInsertRowArgs<'largs> for (::libc::c_int, &'largs ::list::ListStandardItemMutPtr) {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let row = self.0;
      let items = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertRow_row_items(original_self as *mut ::standard_item_model::StandardItemModel, row, items as *const ::list::ListStandardItemMutPtr) }
    }
  }
  impl<'largs> StandardItemModelInsertRowArgs<'largs> for (::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertRow_row_parent(original_self as *mut ::standard_item_model::StandardItemModel, row, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::insert_rows](../struct.StandardItemModel.html#method.insert_rows) method.
  pub trait StandardItemModelInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertRows_row_count(original_self as *mut ::standard_item_model::StandardItemModel, row, count) }
    }
  }
  impl<'largs> StandardItemModelInsertRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_insertRows_row_count_parent(original_self as *mut ::standard_item_model::StandardItemModel, row, count, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::item](../struct.StandardItemModel.html#method.item) method.
  pub trait StandardItemModelItemArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem;
  }
  impl<'largs> StandardItemModelItemArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem {
      let row = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_item_row(original_self as *const ::standard_item_model::StandardItemModel,
                                                    row)
      }
    }
  }
  impl<'largs> StandardItemModelItemArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem {
      let row = self.0;
      let column = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_item_row_column(original_self as *const ::standard_item_model::StandardItemModel, row, column) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::new](../struct.StandardItemModel.html#method.new) method.
  pub trait StandardItemModelNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>;
  }
  impl StandardItemModelNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl StandardItemModelNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel> {
      let rows = self.0;
      let columns = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QStandardItemModel_new_rows_columns(rows, columns) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::new_unsafe](../struct.StandardItemModel.html#method.new_unsafe) method.
  pub trait StandardItemModelNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel>;
  }
  impl StandardItemModelNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl StandardItemModelNewUnsafeArgs for (::libc::c_int, ::libc::c_int, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::standard_item_model::StandardItemModel> {
      let rows = self.0;
      let columns = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_gui_c_QStandardItemModel_new_rows_columns_parent(rows, columns, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::remove_columns](../struct.StandardItemModel.html#method.remove_columns) method.
  pub trait StandardItemModelRemoveColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelRemoveColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_removeColumns_column_count(original_self as *mut ::standard_item_model::StandardItemModel, column, count) }
    }
  }
  impl<'largs> StandardItemModelRemoveColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_removeColumns_column_count_parent(original_self as *mut ::standard_item_model::StandardItemModel, column, count, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::remove_rows](../struct.StandardItemModel.html#method.remove_rows) method.
  pub trait StandardItemModelRemoveRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelRemoveRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_removeRows_row_count(original_self as *mut ::standard_item_model::StandardItemModel, row, count) }
    }
  }
  impl<'largs> StandardItemModelRemoveRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_removeRows_row_count_parent(original_self as *mut ::standard_item_model::StandardItemModel, row, count, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::row_count](../struct.StandardItemModel.html#method.row_count) method.
  pub trait StandardItemModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int;
  }
  impl<'largs> StandardItemModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QStandardItemModel_rowCount_no_args(original_self as *const ::standard_item_model::StandardItemModel) }
    }
  }
  impl<'largs> StandardItemModelRowCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::standard_item_model::StandardItemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_rowCount_parent(original_self as *const ::standard_item_model::StandardItemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::set_data](../struct.StandardItemModel.html#method.set_data) method.
  pub trait StandardItemModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_setData_index_value(original_self as *mut ::standard_item_model::StandardItemModel, index as *const ::qt_core::model_index::ModelIndex, value as *const ::qt_core::variant::Variant) }
    }
  }
  impl<'largs> StandardItemModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_setData_index_value_role(original_self as *mut ::standard_item_model::StandardItemModel, index as *const ::qt_core::model_index::ModelIndex, value as *const ::qt_core::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::set_header_data](../struct.StandardItemModel.html#method.set_header_data) method.
  pub trait StandardItemModelSetHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool;
  }
  impl<'largs> StandardItemModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::qt::Orientation, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value(original_self as *mut ::standard_item_model::StandardItemModel, section, orientation as *const ::qt_core::qt::Orientation, value as *const ::qt_core::variant::Variant) }
    }
  }
  impl<'largs> StandardItemModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::qt::Orientation, &'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      let role = self.3;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_setHeaderData_section_orientation_value_role(original_self as *mut ::standard_item_model::StandardItemModel, section, orientation as *const ::qt_core::qt::Orientation, value as *const ::qt_core::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::set_item](../struct.StandardItemModel.html#method.set_item) method.
  pub trait StandardItemModelSetItemArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> ();
  }
  impl<'largs> StandardItemModelSetItemArgs<'largs>
    for (::libc::c_int, ::libc::c_int, *mut ::standard_item::StandardItem) {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let row = self.0;
      let column = self.1;
      let item = self.2;
      ::ffi::qt_gui_c_QStandardItemModel_setItem_row_column_item(original_self as *mut ::standard_item_model::StandardItemModel, row, column, item)
    }
  }
  impl<'largs> StandardItemModelSetItemArgs<'largs> for (::libc::c_int, *mut ::standard_item::StandardItem) {
    unsafe fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let row = self.0;
      let item = self.1;
      ::ffi::qt_gui_c_QStandardItemModel_setItem_row_item(original_self as *mut ::standard_item_model::StandardItemModel, row, item)
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::sort](../struct.StandardItemModel.html#method.sort) method.
  pub trait StandardItemModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> ();
  }
  impl<'largs> StandardItemModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_sort_column(original_self as *mut ::standard_item_model::StandardItemModel,
                                                       column)
      }
    }
  }
  impl<'largs> StandardItemModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::standard_item_model::StandardItemModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_sort_column_order(original_self as *mut ::standard_item_model::StandardItemModel, column, order as *const ::qt_core::qt::SortOrder) }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardItemModel::take_item](../struct.StandardItemModel.html#method.take_item) method.
  pub trait StandardItemModelTakeItemArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem;
  }
  impl<'largs> StandardItemModelTakeItemArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs mut ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem {
      let row = self;
      unsafe {
        ::ffi::qt_gui_c_QStandardItemModel_takeItem_row(original_self as *mut ::standard_item_model::StandardItemModel, row)
      }
    }
  }
  impl<'largs> StandardItemModelTakeItemArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs mut ::standard_item_model::StandardItemModel)
            -> *mut ::standard_item::StandardItem {
      let row = self.0;
      let column = self.1;
      unsafe { ::ffi::qt_gui_c_QStandardItemModel_takeItem_row_column(original_self as *mut ::standard_item_model::StandardItemModel, row, column) }
    }
  }
}
