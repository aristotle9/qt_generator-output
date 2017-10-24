/// C++ type: <span style='color: green;'>```QSortFilterProxyModel```</span>
#[repr(C)]
pub struct SortFilterProxyModel(u8);

impl SortFilterProxyModel {
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::buddy(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn buddy(&self, index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_buddy_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::canFetchMore(const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn can_fetch_more(&self, parent: &::model_index::ModelIndex) -> bool {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_canFetchMore(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, parent as *const ::model_index::ModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSortFilterProxyModel::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_clear(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QSortFilterProxyModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QSortFilterProxyModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::SortFilterProxyModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::model_index::ModelIndex) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QSortFilterProxyModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QSortFilterProxyModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::SortFilterProxyModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::mime_data::MimeData,
                               action: &::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_core_c_QSortFilterProxyModel_dropMimeData(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel,
                                                        data,
                                                        action as *const ::qt::DropAction,
                                                        row,
                                                        column,
                                                        parent as *const ::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```bool QSortFilterProxyModel::dynamicSortFilter() const```</span>
  ///
  ///
  pub fn dynamic_sort_filter(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_dynamicSortFilter(self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QSortFilterProxyModel::fetchMore(const QModelIndex& parent)```</span>
  ///
  ///
  pub fn fetch_more(&mut self, parent: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_fetchMore(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel,
                                                       parent as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```int QSortFilterProxyModel::filterKeyColumn() const```</span>
  ///
  ///
  pub fn filter_key_column(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_filterKeyColumn(self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
  }

  /// C++ method: <span style='color: green;'>```QRegExp QSortFilterProxyModel::filterRegExp() const```</span>
  ///
  ///
  pub fn filter_reg_exp(&self) -> ::reg_exp::RegExp {
    {
      let mut object: ::reg_exp::RegExp =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_filterRegExp_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSortFilterProxyModel::filterRole() const```</span>
  ///
  ///
  pub fn filter_role(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_filterRole(self as *const ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::hasChildren(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QSortFilterProxyModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt::Orientation, ::libc::c_int)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QSortFilterProxyModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::SortFilterProxyModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::model_index::ModelIndex
    where Args: overloading::SortFilterProxyModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::insertColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::insertColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::insertColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelInsertColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::insertRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::insertRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::insertRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn insert_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelInsertRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QSortFilterProxyModel::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_invalidate(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSortFilterProxyModel::isSortLocaleAware() const```</span>
  ///
  ///
  pub fn is_sort_locale_aware(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_isSortLocaleAware(self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::mapFromSource(const QModelIndex& sourceIndex) const```</span>
  ///
  ///
  pub fn map_from_source(&self, source_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_mapFromSource_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, source_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QSortFilterProxyModel::mapSelectionFromSource(const QItemSelection& sourceSelection) const```</span>
  ///
  ///
  pub fn map_selection_from_source(&self,
                                   source_selection: &::item_selection::ItemSelection)
                                   -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_mapSelectionFromSource_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, source_selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QItemSelection QSortFilterProxyModel::mapSelectionToSource(const QItemSelection& proxySelection) const```</span>
  ///
  ///
  pub fn map_selection_to_source(&self,
                                 proxy_selection: &::item_selection::ItemSelection)
                                 -> ::item_selection::ItemSelection {
    {
      let mut object: ::item_selection::ItemSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_mapSelectionToSource_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, proxy_selection as *const ::item_selection::ItemSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::mapToSource(const QModelIndex& proxyIndex) const```</span>
  ///
  ///
  pub fn map_to_source(&self, proxy_index: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_mapToSource_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, proxy_index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSortFilterProxyModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_metaObject(self as *const ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QSortFilterProxyModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::list::ListModelIndex) -> *mut ::mime_data::MimeData {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_mimeData(self as *const ::sort_filter_proxy_model::SortFilterProxyModel,
                                                      indexes as *const ::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QSortFilterProxyModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_mimeTypes_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSortFilterProxyModel::QSortFilterProxyModel()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::sort_filter_proxy_model::SortFilterProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSortFilterProxyModel::QSortFilterProxyModel(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::sort_filter_proxy_model::SortFilterProxyModel> {
    let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::model_index::ModelIndex) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_parent_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, child as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::removeColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::removeColumns(int column, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_columns(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::removeColumns(int column, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_columns<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelRemoveColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::removeRows```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::removeRows(int row, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove_rows(&mut self, (::libc::c_int, ::libc::c_int, &::model_index::ModelIndex)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::removeRows(int row, int count, const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn remove_rows<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelRemoveRowsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QSortFilterProxyModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QSortFilterProxyModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::SortFilterProxyModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::model_index::ModelIndex, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setDynamicSortFilter(bool enable)```</span>
  ///
  ///
  pub fn set_dynamic_sort_filter(&mut self, enable: bool) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setDynamicSortFilter(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setFilterCaseSensitivity(Qt::CaseSensitivity cs)```</span>
  ///
  ///
  pub fn set_filter_case_sensitivity(&mut self, cs: &::qt::CaseSensitivity) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterCaseSensitivity(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, cs as *const ::qt::CaseSensitivity) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSortFilterProxyModel::setFilterFixedString(const QString& pattern)```</span>
  ///
  ///
  pub fn set_filter_fixed_string(&mut self, pattern: &::string::String) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterFixedString(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, pattern as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setFilterKeyColumn(int column)```</span>
  ///
  ///
  pub fn set_filter_key_column(&mut self, column: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterKeyColumn(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column) }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::setFilterRegExp```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_filter_reg_exp(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QSortFilterProxyModel::setFilterRegExp(const QString& pattern)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_filter_reg_exp(&mut self, &::reg_exp::RegExp) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setFilterRegExp(const QRegExp& regExp)```</span>
  ///
  ///
  pub fn set_filter_reg_exp<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SortFilterProxyModelSetFilterRegExpArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setFilterRole(int role)```</span>
  ///
  ///
  pub fn set_filter_role(&mut self, role: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_setFilterRole(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, role)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSortFilterProxyModel::setFilterWildcard(const QString& pattern)```</span>
  ///
  ///
  pub fn set_filter_wildcard(&mut self, pattern: &::string::String) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterWildcard(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, pattern as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::setHeaderData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_header_data(&mut self, (::libc::c_int, &::qt::Orientation, &::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QSortFilterProxyModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_header_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SortFilterProxyModelSetHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setSortCaseSensitivity(Qt::CaseSensitivity cs)```</span>
  ///
  ///
  pub fn set_sort_case_sensitivity(&mut self, cs: &::qt::CaseSensitivity) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setSortCaseSensitivity(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, cs as *const ::qt::CaseSensitivity) }
  }

  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setSortLocaleAware(bool on)```</span>
  ///
  ///
  pub fn set_sort_locale_aware(&mut self, on: bool) {
    unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setSortLocaleAware(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, on) }
  }

  /// C++ method: <span style='color: green;'>```void QSortFilterProxyModel::setSortRole(int role)```</span>
  ///
  ///
  pub fn set_sort_role(&mut self, role: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_setSortRole(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel,
                                                         role)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QSortFilterProxyModel::setSourceModel(QAbstractItemModel* sourceModel)```</span>
  ///
  ///
  pub unsafe fn set_source_model(&mut self, source_model: *mut ::abstract_item_model::AbstractItemModel) {
    ::ffi::qt_core_c_QSortFilterProxyModel_setSourceModel(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, source_model)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QSortFilterProxyModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
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
        ::ffi::qt_core_c_QSortFilterProxyModel_sibling_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, row, column, idx as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSortFilterProxyModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QSortFilterProxyModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QSortFilterProxyModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SortFilterProxyModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QSortFilterProxyModel::sortColumn() const```</span>
  ///
  ///
  pub fn sort_column(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_sortColumn(self as *const ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```int QSortFilterProxyModel::sortRole() const```</span>
  ///
  ///
  pub fn sort_role(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QSortFilterProxyModel_sortRole(self as *const ::sort_filter_proxy_model::SortFilterProxyModel)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QSize QSortFilterProxyModel::span(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn span(&self, index: &::model_index::ModelIndex) -> ::size::Size {
    {
      let mut object: ::size::Size = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSortFilterProxyModel_span_to_output(self as *const ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSortFilterProxyModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSortFilterProxyModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSortFilterProxyModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSortFilterProxyModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::sort_filter_proxy_model::SortFilterProxyModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSortFilterProxyModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SortFilterProxyModel`.
  pub struct Signals<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  /// Represents a built-in Qt signal `QSortFilterProxyModel::sourceModelChanged`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.signals().source_model_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct SourceModelChanged<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSortFilterProxyModel::sourceModelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_model_changed(&self) -> SourceModelChanged {
      SourceModelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SortFilterProxyModel`.
  pub struct Slots<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  /// Represents a built-in Qt slot `QSortFilterProxyModel::setFilterFixedString`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().set_filter_fixed_string()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct SetFilterFixedString<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  impl<'a> ::connection::Receiver for SetFilterFixedString<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFilterFixedString(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSortFilterProxyModel::setFilterRegExp`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().set_filter_reg_exp()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct SetFilterRegExp<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  impl<'a> ::connection::Receiver for SetFilterRegExp<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFilterRegExp(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSortFilterProxyModel::setFilterWildcard`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().set_filter_wildcard()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct SetFilterWildcard<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  impl<'a> ::connection::Receiver for SetFilterWildcard<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFilterWildcard(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `QSortFilterProxyModel::clear`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().clear()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct Clear<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  impl<'a> ::connection::Receiver for Clear<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1clear()\0"
    }
  }
  /// Represents a built-in Qt slot `QSortFilterProxyModel::invalidate`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().invalidate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct Invalidate<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
  impl<'a> ::connection::Receiver for Invalidate<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1invalidate()\0"
    }
  }
  /// Represents a built-in Qt slot `QSortFilterProxyModel::resetInternalData`.
  ///
  /// An object of this type can be created from `SortFilterProxyModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SortFilterProxyModel` object.
  pub struct ResetInternalData<'a>(&'a ::sort_filter_proxy_model::SortFilterProxyModel);
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
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::setFilterFixedString`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_filter_fixed_string(&self) -> SetFilterFixedString {
      SetFilterFixedString(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::setFilterRegExp`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_filter_reg_exp(&self) -> SetFilterRegExp {
      SetFilterRegExp(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::setFilterWildcard`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_filter_wildcard(&self) -> SetFilterWildcard {
      SetFilterWildcard(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::clear`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clear(&self) -> Clear {
      Clear(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::invalidate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn invalidate(&self) -> Invalidate {
      Invalidate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QSortFilterProxyModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
  }
  impl ::sort_filter_proxy_model::SortFilterProxyModel {
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

impl ::cpp_utils::DynamicCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::abstract_item_model::AbstractItemModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sort_filter_proxy_model::SortFilterProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::sort_filter_proxy_model::SortFilterProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::abstract_proxy_model::AbstractProxyModel {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sort_filter_proxy_model::SortFilterProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(self as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::sort_filter_proxy_model::SortFilterProxyModel> {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::sort_filter_proxy_model::SortFilterProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::sort_filter_proxy_model::SortFilterProxyModel> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_dynamic_cast_QSortFilterProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::abstract_item_model::AbstractItemModel> for ::sort_filter_proxy_model::SortFilterProxyModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::sort_filter_proxy_model::SortFilterProxyModel as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::abstract_proxy_model::AbstractProxyModel> for ::sort_filter_proxy_model::SortFilterProxyModel {
fn static_cast_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_proxy_model::AbstractProxyModel {
let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *const ::sort_filter_proxy_model::SortFilterProxyModel as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::object::Object> for ::sort_filter_proxy_model::SortFilterProxyModel {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QObject_ptr(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QObject_ptr(self as *const ::sort_filter_proxy_model::SortFilterProxyModel as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::sort_filter_proxy_model::SortFilterProxyModel {
let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(self as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::sort_filter_proxy_model::SortFilterProxyModel {
let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractItemModel(self as *const ::abstract_item_model::AbstractItemModel as *mut ::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::abstract_proxy_model::AbstractProxyModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::sort_filter_proxy_model::SortFilterProxyModel {
let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(self as *mut ::abstract_proxy_model::AbstractProxyModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::sort_filter_proxy_model::SortFilterProxyModel {
let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QAbstractProxyModel(self as *const ::abstract_proxy_model::AbstractProxyModel as *mut ::abstract_proxy_model::AbstractProxyModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::sort_filter_proxy_model::SortFilterProxyModel> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sort_filter_proxy_model::SortFilterProxyModel {
    let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sort_filter_proxy_model::SortFilterProxyModel {
    let ffi_result = ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QSortFilterProxyModel_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::sort_filter_proxy_model::SortFilterProxyModel {
  type Target = ::abstract_proxy_model::AbstractProxyModel;
  fn deref(&self) -> &::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *const ::sort_filter_proxy_model::SortFilterProxyModel as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::sort_filter_proxy_model::SortFilterProxyModel {
  fn deref_mut(&mut self) -> &mut ::abstract_proxy_model::AbstractProxyModel {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_G_static_cast_QAbstractProxyModel_ptr(self as *mut ::sort_filter_proxy_model::SortFilterProxyModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::column_count](../struct.SortFilterProxyModel.html#method.column_count) method.
  pub trait SortFilterProxyModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int;
  }
  impl<'largs> SortFilterProxyModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_columnCount_no_args(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
    }
  }
  impl<'largs> SortFilterProxyModelColumnCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_columnCount_parent(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::data](../struct.SortFilterProxyModel.html#method.data) method.
  pub trait SortFilterProxyModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant;
  }
  impl<'largs> SortFilterProxyModelDataArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant {
      let index = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_data_to_output_index(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SortFilterProxyModelDataArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_data_to_output_index_role(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::has_children](../struct.SortFilterProxyModel.html#method.has_children) method.
  pub trait SortFilterProxyModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {

      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_hasChildren_no_args(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
    }
  }
  impl<'largs> SortFilterProxyModelHasChildrenArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_hasChildren_parent(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::header_data](../struct.SortFilterProxyModel.html#method.header_data) method.
  pub trait SortFilterProxyModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant;
  }
  impl<'largs> SortFilterProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation) {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, section, orientation as *const ::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SortFilterProxyModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_headerData_to_output_section_orientation_role(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, section, orientation as *const ::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::index](../struct.SortFilterProxyModel.html#method.index) method.
  pub trait SortFilterProxyModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::model_index::ModelIndex;
  }
  impl<'largs> SortFilterProxyModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_index_to_output_row_column(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SortFilterProxyModelIndexArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSortFilterProxyModel_index_to_output_row_column_parent(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, row, column, parent as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::insert_columns](../struct.SortFilterProxyModel.html#method.insert_columns) method.
  pub trait SortFilterProxyModelInsertColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelInsertColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_insertColumns_column_count(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column, count) }
    }
  }
  impl<'largs> SortFilterProxyModelInsertColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_insertColumns_column_count_parent(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::insert_rows](../struct.SortFilterProxyModel.html#method.insert_rows) method.
  pub trait SortFilterProxyModelInsertRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelInsertRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_insertRows_row_count(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, row, count) }
    }
  }
  impl<'largs> SortFilterProxyModelInsertRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_insertRows_row_count_parent(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::remove_columns](../struct.SortFilterProxyModel.html#method.remove_columns) method.
  pub trait SortFilterProxyModelRemoveColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelRemoveColumnsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_removeColumns_column_count(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column, count) }
    }
  }
  impl<'largs> SortFilterProxyModelRemoveColumnsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let column = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_removeColumns_column_count_parent(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::remove_rows](../struct.SortFilterProxyModel.html#method.remove_rows) method.
  pub trait SortFilterProxyModelRemoveRowsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelRemoveRowsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_removeRows_row_count(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, row, count) }
    }
  }
  impl<'largs> SortFilterProxyModelRemoveRowsArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let row = self.0;
      let count = self.1;
      let parent = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_removeRows_row_count_parent(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, row, count, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::row_count](../struct.SortFilterProxyModel.html#method.row_count) method.
  pub trait SortFilterProxyModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int;
  }
  impl<'largs> SortFilterProxyModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_rowCount_no_args(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel) }
    }
  }
  impl<'largs> SortFilterProxyModelRowCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::sort_filter_proxy_model::SortFilterProxyModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_rowCount_parent(original_self as *const ::sort_filter_proxy_model::SortFilterProxyModel, parent as *const ::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::set_data](../struct.SortFilterProxyModel.html#method.set_data) method.
  pub trait SortFilterProxyModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelSetDataArgs<'largs>
    for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setData_index_value(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> SortFilterProxyModelSetDataArgs<'largs>
    for (&'largs ::model_index::ModelIndex, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setData_index_value_role(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, index as *const ::model_index::ModelIndex, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::set_filter_reg_exp](../struct.SortFilterProxyModel.html#method.set_filter_reg_exp) method.
  pub trait SortFilterProxyModelSetFilterRegExpArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> ();
  }
  impl<'largs> SortFilterProxyModelSetFilterRegExpArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> () {
      let pattern = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterRegExp_pattern(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, pattern as *const ::string::String) }
    }
  }
  impl<'largs> SortFilterProxyModelSetFilterRegExpArgs<'largs> for &'largs ::reg_exp::RegExp {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> () {
      let reg_exp = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setFilterRegExp_regExp(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, reg_exp as *const ::reg_exp::RegExp) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::set_header_data](../struct.SortFilterProxyModel.html#method.set_header_data) method.
  pub trait SortFilterProxyModelSetHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool;
  }
  impl<'largs> SortFilterProxyModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant) }
    }
  }
  impl<'largs> SortFilterProxyModelSetHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt::Orientation, &'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> bool {
      let section = self.0;
      let orientation = self.1;
      let value = self.2;
      let role = self.3;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_setHeaderData_section_orientation_value_role(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, section, orientation as *const ::qt::Orientation, value as *const ::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [SortFilterProxyModel::sort](../struct.SortFilterProxyModel.html#method.sort) method.
  pub trait SortFilterProxyModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> ();
  }
  impl<'largs> SortFilterProxyModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> () {
      let column = self;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_sort_column(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column) }
    }
  }
  impl<'largs> SortFilterProxyModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::sort_filter_proxy_model::SortFilterProxyModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe { ::ffi::qt_core_c_QSortFilterProxyModel_sort_column_order(original_self as *mut ::sort_filter_proxy_model::SortFilterProxyModel, column, order as *const ::qt::SortOrder) }
    }
  }
}
