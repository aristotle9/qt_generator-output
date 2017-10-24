/// C++ type: <span style='color: green;'>```QAbstractProxyModel```</span>
#[repr(C)]
pub struct AbstractProxyModel(u8);

impl AbstractProxyModel {
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractProxyModel::buddy(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn buddy(&self, index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_buddy_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::canDropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent) const```</span>
  ///
  ///
  pub unsafe fn can_drop_mime_data(&self,
                                   data: *const ::mime_data::MimeData,
                                   action: &::qt::DropAction,
                                   row: ::libc::c_int,
                                   column: ::libc::c_int,
                                   parent: &::model_index::ModelIndex)
                                   -> bool {
    ::ffi::qt_core_c_QAbstractProxyModel_canDropMimeData(self as *const ::abstract_proxy_model::AbstractProxyModel,
                                                         data,
                                                         action as *const ::qt::DropAction,
                                                         row,
                                                         column,
                                                         parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::canFetchMore(const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn can_fetch_more(&self, parent: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_canFetchMore(self as *const ::abstract_proxy_model::AbstractProxyModel,
                                                        parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::model_index::ModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractProxyModel::data(const QModelIndex& proxyIndex) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractProxyModel::data(const QModelIndex& proxyIndex, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::AbstractProxyModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::mime_data::MimeData,
                               action: &::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_core_c_QAbstractProxyModel_dropMimeData(self as *mut ::abstract_proxy_model::AbstractProxyModel,
                                                      data,
                                                      action as *const ::qt::DropAction,
                                                      row,
                                                      column,
                                                      parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractProxyModel::fetchMore(const QModelIndex& parent)```</span>
  ///
  ///
  pub fn fetch_more(&mut self, parent: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_fetchMore(self as *mut ::abstract_proxy_model::AbstractProxyModel,
                                                     parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::hasChildren(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::AbstractProxyModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractProxyModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QAbstractProxyModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::AbstractProxyModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QMap<int, QVariant> QAbstractProxyModel::itemData(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn item_data(&self, index: &::model_index::ModelIndex) -> ::map::MapCIntVariant {
    {
      let mut object: ::map::MapCIntVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_itemData_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractProxyModel::mapFromSource(const QModelIndex& sourceIndex) const```</span>
  ///
  ///
  pub fn map_from_source(&self, source_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_mapFromSource_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, source_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QAbstractProxyModel::mapSelectionFromSource(const QItemSelection& selection) const```</span>
  ///
  ///
  pub fn map_selection_from_source(&self,
                                   selection: &::item_selection::ItemSelection)
                                   -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_mapSelectionFromSource_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QAbstractProxyModel::mapSelectionToSource(const QItemSelection& selection) const```</span>
  ///
  ///
  pub fn map_selection_to_source(&self,
                                 selection: &::item_selection::ItemSelection)
                                 -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_mapSelectionToSource_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QModelIndex QAbstractProxyModel::mapToSource(const QModelIndex& proxyIndex) const```</span>
  ///
  ///
  pub fn map_to_source(&self, proxy_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_mapToSource_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, proxy_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAbstractProxyModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_metaObject(self as *const ::abstract_proxy_model::AbstractProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QAbstractProxyModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::list::ListModelIndex) -> *mut ::mime_data::MimeData {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_mimeData(self as *const ::abstract_proxy_model::AbstractProxyModel,
                                                    indexes as *const ::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QAbstractProxyModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_mimeTypes_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractProxyModel::revert()```</span>
  ///
  ///
  pub fn revert(&mut self) {
    unsafe { ::ffi::qt_core_c_QAbstractProxyModel_revert(self as *mut ::abstract_proxy_model::AbstractProxyModel) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractProxyModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::setHeaderData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_header_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::AbstractProxyModelSetHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::setItemData(const QModelIndex& index, const QMap<int, QVariant>& roles)```</span>
  ///
  ///
  pub fn set_item_data(&mut self, index: &::model_index::ModelIndex, roles: &::map::MapCIntVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_setItemData(self as *mut ::abstract_proxy_model::AbstractProxyModel,
                                                       index as *const ::model_index::ModelIndex,
                                                       roles as *const ::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QAbstractProxyModel::setSourceModel(QAbstractItemModel* sourceModel)```</span>
  ///
  ///
  pub unsafe fn set_source_model(&mut self, source_model: *mut ::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_core_c_QAbstractProxyModel_setSourceModel(self as *mut ::abstract_proxy_model::AbstractProxyModel,
                                                        source_model)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QAbstractProxyModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
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
        ::ffi::qt_core_c_QAbstractProxyModel_sibling_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, row, column, idx as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractProxyModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QAbstractProxyModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QAbstractProxyModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::AbstractProxyModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractItemModel* QAbstractProxyModel::sourceModel() const```</span>
  ///
  ///
  pub fn source_model(&self) -> *mut ::abstract_item_model::AbstractItemModel {
    unsafe {
      ::ffi::qt_core_c_QAbstractProxyModel_sourceModel(self as *const ::abstract_proxy_model::AbstractProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QAbstractProxyModel::span(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn span(&self, index: &::model_index::ModelIndex) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAbstractProxyModel_span_to_output(self as *const ::abstract_proxy_model::AbstractProxyModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QAbstractProxyModel::submit()```</span>
  ///
  ///
  pub fn submit(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QAbstractProxyModel_submit(self as *mut ::abstract_proxy_model::AbstractProxyModel) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractProxyModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractProxyModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAbstractProxyModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAbstractProxyModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_proxy_model::AbstractProxyModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAbstractProxyModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractProxyModel`.
  pub struct Signals<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsRemoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::layoutChanged`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct LayoutChanged<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsInserted`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsInserted<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::dataChanged`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct DataChanged<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::modelReset`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ModelReset<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsMoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsMoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::sourceModelChanged`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().source_model_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct SourceModelChanged<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
  impl<'a> ::connection::Receiver for SourceModelChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceModelChanged()\0"
    }
  }
  impl<'a> ::connection::Signal for SourceModelChanged<'a> {}
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsMoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsMoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::columnsInserted`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ColumnsInserted<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
  /// Represents a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
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
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::sourceModelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_model_changed(&self) -> SourceModelChanged {
      SourceModelChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAbstractProxyModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractProxyModel`.
  pub struct Slots<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
  /// Represents a built-in Qt slot `QAbstractProxyModel::resetInternalData`.
  ///
  /// An object of this type can be created from `AbstractProxyModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractProxyModel` object.
  pub struct ResetInternalData<'a>(&'a ::abstract_proxy_model::AbstractProxyModel);
  impl<'a> ::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QAbstractProxyModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
  }
  impl ::abstract_proxy_model::AbstractProxyModel {
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

impl ::cpp_utils::DynamicCast<::abstract_proxy_model::AbstractProxyModel> for ::abstract_item_model::AbstractItemModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_proxy_model::AbstractProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_proxy_model::AbstractProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::abstract_proxy_model::AbstractProxyModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::abstract_proxy_model::AbstractProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::abstract_proxy_model::AbstractProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_dynamic_cast_QAbstractProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_model::AbstractItemModel> for ::abstract_proxy_model::AbstractProxyModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::abstract_proxy_model::AbstractProxyModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QObject_ptr(self as *mut ::abstract_proxy_model::AbstractProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QObject_ptr(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_proxy_model::AbstractProxyModel> for ::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
let ffi_result = ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::abstract_proxy_model::AbstractProxyModel {
let ffi_result = ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::abstract_proxy_model::AbstractProxyModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_proxy_model::AbstractProxyModel {
  type Target = ::abstract_item_model::AbstractItemModel;
  fn deref(&self) -> &::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_proxy_model::AbstractProxyModel {
  fn deref_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAbstractProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::abstract_proxy_model::AbstractProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::data](../struct.AbstractProxyModel.html#method.data) method.
  pub trait AbstractProxyModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant;
  }
  impl<'largs> AbstractProxyModelDataArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant {
      let proxy_index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex(original_self as *const ::abstract_proxy_model::AbstractProxyModel, proxy_index as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractProxyModelDataArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant {
      let proxy_index = self.0;
      let role = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractProxyModel_data_to_output_proxyIndex_role(original_self as *const ::abstract_proxy_model::AbstractProxyModel, proxy_index as *const ::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::has_children](../struct.AbstractProxyModel.html#method.has_children) method.
  pub trait AbstractProxyModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> bool;
  }
  impl<'largs> AbstractProxyModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> bool {

      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_hasChildren_no_args(original_self as *const ::abstract_proxy_model::AbstractProxyModel) }
    }
  }
  impl<'largs> AbstractProxyModelHasChildrenArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> bool {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_hasChildren_parent(original_self as *const ::abstract_proxy_model::AbstractProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::header_data](../struct.AbstractProxyModel.html#method.header_data) method.
  pub trait AbstractProxyModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant;
  }
  impl<'largs> AbstractProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation) {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation(original_self as *const ::abstract_proxy_model::AbstractProxyModel, section, orientation as *const ::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> AbstractProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::abstract_proxy_model::AbstractProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QAbstractProxyModel_headerData_to_output_section_orientation_role(original_self as *const ::abstract_proxy_model::AbstractProxyModel, section, orientation as *const ::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::set_data](../struct.AbstractProxyModel.html#method.set_data) method.
  pub trait AbstractProxyModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool;
  }
  impl<'largs> AbstractProxyModelSetDataArgs<'largs> for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_setData_index_value(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> AbstractProxyModelSetDataArgs<'largs>
    for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_setData_index_value_role(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::set_header_data](../struct.AbstractProxyModel.html#method.set_header_data) method.
  pub trait AbstractProxyModelSetHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool;
  }
  impl<'largs> AbstractProxyModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> AbstractProxyModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      let role = self.3;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_setHeaderData_section_orientation_value_role(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [AbstractProxyModel::sort](../struct.AbstractProxyModel.html#method.sort) method.
  pub trait AbstractProxyModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> ();
  }
  impl<'largs> AbstractProxyModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> () {
      let column = self;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_sort_column(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, column) }
    }
  }
  impl<'largs> AbstractProxyModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::abstract_proxy_model::AbstractProxyModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe { ::ffi::qt_core_c_QAbstractProxyModel_sort_column_order(original_self as *mut ::abstract_proxy_model::AbstractProxyModel, column, order as *const ::qt::SortOrder) }
    }
  }
}
