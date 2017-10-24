/// C++ type: <span style='color: green;'>```QAbstractItemModel```</span>
#[repr(C)]
pub struct AbstractItemModel(u8);

impl AbstractItemModel {
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractItemModel::buddy(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn buddy(&self, index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_buddy_to_output(self as *const ::abstract_item_model::AbstractItemModel,
                                                            index as *const ::model_index::ModelIndex,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::canDropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent) const```</span>
  ///
  ///
  pub unsafe fn can_drop_mime_data(&self,
                                   data: *const ::mime_data::MimeData,
                                   action: &::qt::DropAction,
                                   row: ::libc::c_int,
                                   column: ::libc::c_int,
                                   parent: &::model_index::ModelIndex)
                                   -> bool {
    ::ffi::qt_core_c_QAbstractItemModel_canDropMimeData(self as *const ::abstract_item_model::AbstractItemModel,
                                                        data,
                                                        action as *const ::qt::DropAction,
                                                        row,
                                                        column,
                                                        parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::canFetchMore(const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn can_fetch_more(&self, parent: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_canFetchMore(self as *const ::abstract_item_model::AbstractItemModel,
                                                       parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractItemModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractItemModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::AbstractItemModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::model_index::ModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAbstractItemModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QVariant QAbstractItemModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::AbstractItemModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::mime_data::MimeData,
                               action: &::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_core_c_QAbstractItemModel_dropMimeData(self as *mut ::abstract_item_model::AbstractItemModel,
                                                     data,
                                                     action as *const ::qt::DropAction,
                                                     row,
                                                     column,
                                                     parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemModel::fetchMore(const QModelIndex& parent)```</span>
  ///
  ///
  pub fn fetch_more(&mut self, parent: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_fetchMore(self as *mut ::abstract_item_model::AbstractItemModel,
                                                    parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::hasChildren(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::AbstractItemModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::hasIndex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_index(&self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::hasIndex(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_index(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::hasIndex(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_index<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::AbstractItemModelHasIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractItemModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractItemModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::AbstractItemModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractItemModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractItemModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::model_index::ModelIndex
    where Args: overloading::AbstractItemModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::insertColumn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_column(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::insertColumn(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_column(&mut self, (::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::insertColumn(int column, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_column<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelInsertColumnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::insertColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::insertColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::insertColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelInsertColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::insertRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_row(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::insertRow(int row)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_row(&mut self, (::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::insertRow(int row, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_row<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelInsertRowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::insertRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::insertRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QMap<int, QVariant> QAbstractItemModel::itemData(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn item_data(&self, index: &::model_index::ModelIndex) -> ::map::MapCIntVariant {
    {
      let mut object: ::map::MapCIntVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_itemData_to_output(self as *const ::abstract_item_model::AbstractItemModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractItemModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAbstractItemModel_metaObject(self as *const ::abstract_item_model::AbstractItemModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QAbstractItemModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::list::ListModelIndex) -> *mut ::mime_data::MimeData {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_mimeData(self as *const ::abstract_item_model::AbstractItemModel,
                                                   indexes as *const ::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QAbstractItemModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_mimeTypes_to_output(self as *const ::abstract_item_model::AbstractItemModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::moveColumn(const QModelIndex& sourceParent, int sourceColumn, const QModelIndex& destinationParent, int destinationChild)```</span>
  ///
  ///
  pub fn move_column(&mut self,
                     source_parent: &::model_index::ModelIndex,
                     source_column: ::libc::c_int,
                     destination_parent: &::model_index::ModelIndex,
                     destination_child: ::libc::c_int)
                     -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_moveColumn(self as *mut ::abstract_item_model::AbstractItemModel,
                                                     source_parent as *const ::model_index::ModelIndex,
                                                     source_column,
                                                     destination_parent as *const ::model_index::ModelIndex,
                                                     destination_child)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::moveColumns(const QModelIndex& sourceParent, int sourceColumn, int count, const QModelIndex& destinationParent, int destinationChild)```</span>
  ///
  ///
  pub fn move_columns(&mut self,
                      source_parent: &::model_index::ModelIndex,
                      source_column: ::libc::c_int,
                      count: ::libc::c_int,
                      destination_parent: &::model_index::ModelIndex,
                      destination_child: ::libc::c_int)
                      -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_moveColumns(self as *mut ::abstract_item_model::AbstractItemModel,
                                                      source_parent as *const ::model_index::ModelIndex,
                                                      source_column,
                                                      count,
                                                      destination_parent as *const ::model_index::ModelIndex,
                                                      destination_child)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::moveRow(const QModelIndex& sourceParent, int sourceRow, const QModelIndex& destinationParent, int destinationChild)```</span>
  ///
  ///
  pub fn move_row(&mut self,
                  source_parent: &::model_index::ModelIndex,
                  source_row: ::libc::c_int,
                  destination_parent: &::model_index::ModelIndex,
                  destination_child: ::libc::c_int)
                  -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_moveRow(self as *mut ::abstract_item_model::AbstractItemModel,
                                                  source_parent as *const ::model_index::ModelIndex,
                                                  source_row,
                                                  destination_parent as *const ::model_index::ModelIndex,
                                                  destination_child)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::moveRows(const QModelIndex& sourceParent, int sourceRow, int count, const QModelIndex& destinationParent, int destinationChild)```</span>
  ///
  ///
  pub fn move_rows(&mut self,
                   source_parent: &::model_index::ModelIndex,
                   source_row: ::libc::c_int,
                   count: ::libc::c_int,
                   destination_parent: &::model_index::ModelIndex,
                   destination_child: ::libc::c_int)
                   -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_moveRows(self as *mut ::abstract_item_model::AbstractItemModel,
                                                   source_parent as *const ::model_index::ModelIndex,
                                                   source_row,
                                                   count,
                                                   destination_parent as *const ::model_index::ModelIndex,
                                                   destination_child)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractItemModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_parent_to_output(self as *const ::abstract_item_model::AbstractItemModel,
                                                             child as *const ::model_index::ModelIndex,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel::removeColumn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_column(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::removeColumn(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_column(&mut self, (::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::removeColumn(int column, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_column<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelRemoveColumnArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::removeColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::removeColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::removeColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelRemoveColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::removeRow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_row(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::removeRow(int row)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_row(&mut self, (::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QAbstractItemModel::removeRow(int row, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_row<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelRemoveRowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::removeRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::removeRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::removeRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelRemoveRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual [slot] void QAbstractItemModel::revert()```</span>
  ///
  ///
  pub fn revert(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractItemModel_revert(self as *mut ::abstract_item_model::AbstractItemModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QHash<int, QByteArray> QAbstractItemModel::roleNames() const```</span>
  ///
  ///
  pub fn role_names(&self) -> ::hash::HashCIntByteArray {
    {
      let mut object: ::hash::HashCIntByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_roleNames_to_output(self as *const ::abstract_item_model::AbstractItemModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractItemModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```pure virtual int QAbstractItemModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::AbstractItemModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel::setHeaderData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_header_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractItemModelSetHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractItemModel::setItemData(const QModelIndex& index, const QMap<int, QVariant>& roles)```</span>
  ///
  ///
  pub fn set_item_data(&mut self, index: &::model_index::ModelIndex, roles: &::map::MapCIntVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_setItemData(self as *mut ::abstract_item_model::AbstractItemModel,
                                                      index as *const ::model_index::ModelIndex,
                                                      roles as *const ::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractItemModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
  ///
  ///
  pub fn sibling(&self,
                 row: ::libc::c_int,
                 column: ::libc::c_int,
                 idx: &::model_index::ModelIndex)
                 -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_sibling_to_output(self as *const ::abstract_item_model::AbstractItemModel, row, column, idx as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractItemModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QAbstractItemModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractItemModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractItemModel::span(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn span(&self, index: &::model_index::ModelIndex) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_span_to_output(self as *const ::abstract_item_model::AbstractItemModel,
                                                           index as *const ::model_index::ModelIndex,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual [slot] bool QAbstractItemModel::submit()```</span>
  ///
  ///
  pub fn submit(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QAbstractItemModel_submit(self as *mut ::abstract_item_model::AbstractItemModel) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractItemModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractItemModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractItemModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_item_model::AbstractItemModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractItemModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractItemModel`.
  pub struct Signals<'a>(&'a ::abstract_item_model::AbstractItemModel);
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsInserted`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsInserted<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ModelAboutToBeReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelAboutToBeReset()\0"
    }
  }
  impl<'a> ::connection::Signal for ModelAboutToBeReset<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsInserted`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsInserted<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsInserted<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsMoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsMoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for LayoutAboutToBeChanged<'a> {
    type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutAboutToBeChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::connection::Signal for LayoutAboutToBeChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsMoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsMoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsMoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsMoved(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsMoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::objectNameChanged`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ObjectNameChanged<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsRemoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ColumnsAboutToBeRemoved<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2columnsAboutToBeRemoved(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for ColumnsAboutToBeRemoved<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::dataChanged`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct DataChanged<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for DataChanged<'a> {
    type Arguments = (&'static ::model_index::ModelIndex,
     &'static ::model_index::ModelIndex,
     &'static ::vector::VectorCInt);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2dataChanged(const QModelIndex&,const QModelIndex&,const QVector< int >&)\0"
    }
  }
  impl<'a> ::connection::Signal for DataChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::modelReset`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ModelReset<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ModelReset<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2modelReset()\0"
    }
  }
  impl<'a> ::connection::Signal for ModelReset<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for HeaderDataChanged<'a> {
    type Arguments = (&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2headerDataChanged(Qt::Orientation,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for HeaderDataChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::layoutChanged`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct LayoutChanged<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for LayoutChanged<'a> {
    type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layoutChanged(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
    }
  }
  impl<'a> ::connection::Signal for LayoutChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractItemModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for RowsAboutToBeInserted<'a> {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rowsAboutToBeInserted(const QModelIndex&,int,int)\0"
    }
  }
  impl<'a> ::connection::Signal for RowsAboutToBeInserted<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractItemModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractItemModel`.
  pub struct Slots<'a>(&'a ::abstract_item_model::AbstractItemModel);
  /// Represents a built-in Qt slot `QAbstractItemModel::resetInternalData`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct ResetInternalData<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemModel::submit`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct Submit<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for Submit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1submit()\0"
    }
  }
  /// Represents a built-in Qt slot `QAbstractItemModel::revert`.
  ///
  /// An object of this type can be created from `AbstractItemModel` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractItemModel` object.
  pub struct Revert<'a>(&'a ::abstract_item_model::AbstractItemModel);
  impl<'a> ::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractItemModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemModel::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QAbstractItemModel::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
  }
  impl ::abstract_item_model::AbstractItemModel {
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

/// C++ type: <span style='color: green;'>```QAbstractItemModel::LayoutChangeHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LayoutChangeHint {
  /// C++ enum variant: <span style='color: green;'>```NoLayoutChangeHint = 0```</span>
  NoLayoutChange = 0,
  /// C++ enum variant: <span style='color: green;'>```VerticalSortHint = 1```</span>
  VerticalSort = 1,
  /// C++ enum variant: <span style='color: green;'>```HorizontalSortHint = 2```</span>
  HorizontalSort = 2,
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::model_index::ModelIndex) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QModelIndex& index)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash(&::persistent_model_index::PersistentModelIndex) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QPersistentModelIndex& index)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash((&::persistent_model_index::PersistentModelIndex, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QPersistentModelIndex& index, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QModelIndex& arg2)```</span>
///
///
pub fn op_shl(arg1: &::debug::Debug, arg2: &::model_index::ModelIndex) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QAbstractItemModel_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                                   arg2 as *const ::model_index::ModelIndex,
                                                                   &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPersistentModelIndex& value1, QPersistentModelIndex& value2)```</span>
///
///
pub fn swap(value1: &mut ::persistent_model_index::PersistentModelIndex,
            value2: &mut ::persistent_model_index::PersistentModelIndex) {
  unsafe {
    ::ffi::qt_core_c_QAbstractItemModel_G_swap(value1 as *mut ::persistent_model_index::PersistentModelIndex,
                                               value2 as *mut ::persistent_model_index::PersistentModelIndex)
  }
}

impl ::cpp_utils::DynamicCast<::abstract_item_model::AbstractItemModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_item_model::AbstractItemModel> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_G_dynamic_cast_QAbstractItemModel_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_item_model::AbstractItemModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_dynamic_cast_QAbstractItemModel_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_item_model::AbstractItemModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(self as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_item_model::AbstractItemModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
    let ffi_result =
      ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
    let ffi_result = ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_item_model::AbstractItemModel {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_item_model::AbstractItemModel {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_static_cast_QObject_ptr(self as *mut ::abstract_item_model::AbstractItemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractItemModel::column_count](../struct.AbstractItemModel.html#method.column_count) method.
  pub trait AbstractItemModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int;
  }
  impl<'largs> AbstractItemModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QAbstractItemModel_columnCount_no_args(original_self as *const ::abstract_item_model::AbstractItemModel) }
    }
  }
  impl<'largs> AbstractItemModelColumnCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_columnCount_parent(original_self as *const ::abstract_item_model::AbstractItemModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::data](../struct.AbstractItemModel.html#method.data) method.
  pub trait AbstractItemModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant;
  }
  impl<'largs> AbstractItemModelDataArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant {
      let index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_data_to_output_index(original_self as *const ::abstract_item_model::AbstractItemModel, index as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractItemModelDataArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_data_to_output_index_role(original_self as *const ::abstract_item_model::AbstractItemModel, index as *const ::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::has_children](../struct.AbstractItemModel.html#method.has_children) method.
  pub trait AbstractItemModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool {

      unsafe { ::ffi::qt_core_c_QAbstractItemModel_hasChildren_no_args(original_self as *const ::abstract_item_model::AbstractItemModel) }
    }
  }
  impl<'largs> AbstractItemModelHasChildrenArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_hasChildren_parent(original_self as *const ::abstract_item_model::AbstractItemModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::has_index](../struct.AbstractItemModel.html#method.has_index) method.
  pub trait AbstractItemModelHasIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelHasIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let column = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_hasIndex_row_column(original_self as *const ::abstract_item_model::AbstractItemModel, row, column) }
    }
  }
  impl<'largs> AbstractItemModelHasIndexArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_hasIndex_row_column_parent(original_self as *const ::abstract_item_model::AbstractItemModel, row, column, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::header_data](../struct.AbstractItemModel.html#method.header_data) method.
  pub trait AbstractItemModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant;
  }
  impl<'largs> AbstractItemModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation(original_self as *const ::abstract_item_model::AbstractItemModel, section, orientation as *const ::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractItemModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_headerData_to_output_section_orientation_role(original_self as *const ::abstract_item_model::AbstractItemModel, section, orientation as *const ::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::index](../struct.AbstractItemModel.html#method.index) method.
  pub trait AbstractItemModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::model_index::ModelIndex;
  }
  impl<'largs> AbstractItemModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_index_to_output_row_column(original_self as *const ::abstract_item_model::AbstractItemModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractItemModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractItemModel_index_to_output_row_column_parent(original_self as *const ::abstract_item_model::AbstractItemModel, row, column, parent as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::insert_column](../struct.AbstractItemModel.html#method.insert_column) method.
  pub trait AbstractItemModelInsertColumnArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelInsertColumnArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertColumn_column(original_self as *mut ::abstract_item_model::AbstractItemModel, column) }
    }
  }
  impl<'largs> AbstractItemModelInsertColumnArgs<'largs> for (::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertColumn_column_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, column, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::insert_columns](../struct.AbstractItemModel.html#method.insert_columns) method.
  pub trait AbstractItemModelInsertColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelInsertColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertColumns_column_count(original_self as *mut ::abstract_item_model::AbstractItemModel, column, count) }
    }
  }
  impl<'largs> AbstractItemModelInsertColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertColumns_column_count_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::insert_row](../struct.AbstractItemModel.html#method.insert_row) method.
  pub trait AbstractItemModelInsertRowArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelInsertRowArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertRow_row(original_self as *mut ::abstract_item_model::AbstractItemModel, row) }
    }
  }
  impl<'largs> AbstractItemModelInsertRowArgs<'largs> for (::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertRow_row_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, row, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::insert_rows](../struct.AbstractItemModel.html#method.insert_rows) method.
  pub trait AbstractItemModelInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertRows_row_count(original_self as *mut ::abstract_item_model::AbstractItemModel, row, count) }
    }
  }
  impl<'largs> AbstractItemModelInsertRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_insertRows_row_count_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::remove_column](../struct.AbstractItemModel.html#method.remove_column) method.
  pub trait AbstractItemModelRemoveColumnArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelRemoveColumnArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeColumn_column(original_self as *mut ::abstract_item_model::AbstractItemModel, column) }
    }
  }
  impl<'largs> AbstractItemModelRemoveColumnArgs<'largs> for (::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeColumn_column_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, column, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::remove_columns](../struct.AbstractItemModel.html#method.remove_columns) method.
  pub trait AbstractItemModelRemoveColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelRemoveColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeColumns_column_count(original_self as *mut ::abstract_item_model::AbstractItemModel, column, count) }
    }
  }
  impl<'largs> AbstractItemModelRemoveColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeColumns_column_count_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::remove_row](../struct.AbstractItemModel.html#method.remove_row) method.
  pub trait AbstractItemModelRemoveRowArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelRemoveRowArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeRow_row(original_self as *mut ::abstract_item_model::AbstractItemModel, row) }
    }
  }
  impl<'largs> AbstractItemModelRemoveRowArgs<'largs> for (::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let parent = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeRow_row_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, row, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::remove_rows](../struct.AbstractItemModel.html#method.remove_rows) method.
  pub trait AbstractItemModelRemoveRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelRemoveRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeRows_row_count(original_self as *mut ::abstract_item_model::AbstractItemModel, row, count) }
    }
  }
  impl<'largs> AbstractItemModelRemoveRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_removeRows_row_count_parent(original_self as *mut ::abstract_item_model::AbstractItemModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::row_count](../struct.AbstractItemModel.html#method.row_count) method.
  pub trait AbstractItemModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int;
  }
  impl<'largs> AbstractItemModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QAbstractItemModel_rowCount_no_args(original_self as *const ::abstract_item_model::AbstractItemModel) }
    }
  }
  impl<'largs> AbstractItemModelRowCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_item_model::AbstractItemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_rowCount_parent(original_self as *const ::abstract_item_model::AbstractItemModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::set_data](../struct.AbstractItemModel.html#method.set_data) method.
  pub trait AbstractItemModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelSetDataArgs<'largs> for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_setData_index_value(original_self as *mut ::abstract_item_model::AbstractItemModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> AbstractItemModelSetDataArgs<'largs>
    for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_setData_index_value_role(original_self as *mut ::abstract_item_model::AbstractItemModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::set_header_data](../struct.AbstractItemModel.html#method.set_header_data) method.
  pub trait AbstractItemModelSetHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool;
  }
  impl<'largs> AbstractItemModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value(original_self as *mut ::abstract_item_model::AbstractItemModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> AbstractItemModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      let role = self.3;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_setHeaderData_section_orientation_value_role(original_self as *mut ::abstract_item_model::AbstractItemModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractItemModel::sort](../struct.AbstractItemModel.html#method.sort) method.
  pub trait AbstractItemModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> ();
  }
  impl<'largs> AbstractItemModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_core_c_QAbstractItemModel_sort_column(original_self as *mut ::abstract_item_model::AbstractItemModel, column)
      }
    }
  }
  impl<'largs> AbstractItemModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::abstract_item_model::AbstractItemModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_sort_column_order(original_self as *mut ::abstract_item_model::AbstractItemModel, column, order as *const ::qt::SortOrder) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::model_index::ModelIndex {
    fn exec(self) -> ::libc::c_uint {
      let index = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_qHash_QModelIndex(index as *const ::model_index::ModelIndex) }
    }
  }
  impl<'a> HashArgs for &'a ::persistent_model_index::PersistentModelIndex {
    fn exec(self) -> ::libc::c_uint {
      let index = self;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex(index as *const ::persistent_model_index::PersistentModelIndex) }
    }
  }
  impl<'a> HashArgs for (&'a ::persistent_model_index::PersistentModelIndex, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let index = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractItemModel_G_qHash_QPersistentModelIndex_unsigned_int(index as *const ::persistent_model_index::PersistentModelIndex, seed) }
    }
  }
}
