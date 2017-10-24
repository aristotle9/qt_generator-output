/// C++ type: <span style='color: green;'>```QDirModel```</span>
#[repr(C)]
pub struct DirModel(u8);

impl DirModel {
  /// C++ method: <span style='color: green;'>```QDirModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QDirModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QDirModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::DirModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDirModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::qt_core::model_index::ModelIndex) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QDirModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::qt_core::model_index::ModelIndex, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QDirModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::DirModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QDirModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::qt_core::mime_data::MimeData,
                               action: &::qt_core::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::qt_core::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_widgets_c_QDirModel_dropMimeData(self as *mut ::dir_model::DirModel,
                                               data,
                                               action as *const ::qt_core::qt::DropAction,
                                               row,
                                               column,
                                               parent as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```QIcon QDirModel::fileIcon(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_icon(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_fileIcon_to_output(self as *const ::dir_model::DirModel,
                                                         index as *const ::qt_core::model_index::ModelIndex,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo QDirModel::fileInfo(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_info(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::file_info::FileInfo {
    {
      let mut object: ::qt_core::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_fileInfo_to_output(self as *const ::dir_model::DirModel,
                                                         index as *const ::qt_core::model_index::ModelIndex,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDirModel::fileName(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_name(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_fileName_to_output(self as *const ::dir_model::DirModel,
                                                         index as *const ::qt_core::model_index::ModelIndex,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDirModel::filePath(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_path(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_filePath_to_output(self as *const ::dir_model::DirModel,
                                                         index as *const ::qt_core::model_index::ModelIndex,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDirModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QDirModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::qt_core::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QDirModel::hasChildren(const QModelIndex& index = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::DirModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDirModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QDirModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QDirModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::DirModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileIconProvider* QDirModel::iconProvider() const```</span>
  ///
  ///
  pub fn icon_provider(&self) -> *mut ::file_icon_provider::FileIconProvider {
    unsafe { ::ffi::qt_widgets_c_QDirModel_iconProvider(self as *const ::dir_model::DirModel) }
  }

  /// C++ method: <span style='color: green;'>```QDirModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, &::qt_core::string::String) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QDirModel::index(const QString& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (&::qt_core::string::String, ::libc::c_int)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QDirModel::index(const QString& path, int column = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QDirModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QDirModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::qt_core::model_index::ModelIndex
    where Args: overloading::DirModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QDirModel::isDir(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn is_dir(&self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QDirModel_isDir(self as *const ::dir_model::DirModel,
                                          index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDirModel::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDirModel_isReadOnly(self as *const ::dir_model::DirModel) }
  }

  /// C++ method: <span style='color: green;'>```bool QDirModel::lazyChildCount() const```</span>
  ///
  ///
  pub fn lazy_child_count(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDirModel_lazyChildCount(self as *const ::dir_model::DirModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDirModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QDirModel_metaObject(self as *const ::dir_model::DirModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QDirModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::qt_core::list::ListModelIndex) -> *mut ::qt_core::mime_data::MimeData {
    unsafe {
      ::ffi::qt_widgets_c_QDirModel_mimeData(self as *const ::dir_model::DirModel,
                                             indexes as *const ::qt_core::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QDirModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_mimeTypes_to_output(self as *const ::dir_model::DirModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QDirModel::mkdir(const QModelIndex& parent, const QString& name)```</span>
  ///
  ///
  pub fn mkdir(&mut self,
               parent: &::qt_core::model_index::ModelIndex,
               name: &::qt_core::string::String)
               -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_mkdir_to_output(self as *mut ::dir_model::DirModel,
                                                      parent as *const ::qt_core::model_index::ModelIndex,
                                                      name as *const ::qt_core::string::String,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QDirModel::nameFilters() const```</span>
  ///
  ///
  pub fn name_filters(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_nameFilters_to_output(self as *const ::dir_model::DirModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDirModel::QDirModel()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::dir_model::DirModel> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDirModel_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDirModel::QDirModel(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::dir_model::DirModel> {
    let ffi_result = ::ffi::qt_widgets_c_QDirModel_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QDirModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::qt_core::model_index::ModelIndex) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_parent_to_output(self as *const ::dir_model::DirModel,
                                                       child as *const ::qt_core::model_index::ModelIndex,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDirModel::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QDirModel_qt_metacall(self as *mut ::dir_model::DirModel,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDirModel::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QDirModel_qt_metacast(self as *mut ::dir_model::DirModel, arg1)
  }

  /// C++ method: <span style='color: green;'>```QDirModel::refresh```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn refresh(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QDirModel::refresh()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn refresh(&mut self, &::qt_core::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QDirModel::refresh(const QModelIndex& parent = ?)```</span>
  ///
  ///
  pub fn refresh<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DirModelRefreshArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QDirModel::remove(const QModelIndex& index)```</span>
  ///
  ///
  pub fn remove(&mut self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QDirModel_remove(self as *mut ::dir_model::DirModel,
                                           index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDirModel::resolveSymlinks() const```</span>
  ///
  ///
  pub fn resolve_symlinks(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QDirModel_resolveSymlinks(self as *const ::dir_model::DirModel) }
  }

  /// C++ method: <span style='color: green;'>```bool QDirModel::rmdir(const QModelIndex& index)```</span>
  ///
  ///
  pub fn rmdir(&mut self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QDirModel_rmdir(self as *mut ::dir_model::DirModel,
                                          index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QDirModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QDirModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QDirModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::DirModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDirModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QDirModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QDirModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::DirModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QDirModel::setIconProvider(QFileIconProvider* provider)```</span>
  ///
  ///
  pub unsafe fn set_icon_provider(&mut self, provider: *mut ::file_icon_provider::FileIconProvider) {
    ::ffi::qt_widgets_c_QDirModel_setIconProvider(self as *mut ::dir_model::DirModel, provider)
  }

  /// C++ method: <span style='color: green;'>```void QDirModel::setLazyChildCount(bool enable)```</span>
  ///
  ///
  pub fn set_lazy_child_count(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QDirModel_setLazyChildCount(self as *mut ::dir_model::DirModel, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QDirModel::setNameFilters(const QStringList& filters)```</span>
  ///
  ///
  pub fn set_name_filters(&mut self, filters: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QDirModel_setNameFilters(self as *mut ::dir_model::DirModel,
                                                   filters as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDirModel::setReadOnly(bool enable)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QDirModel_setReadOnly(self as *mut ::dir_model::DirModel, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QDirModel::setResolveSymlinks(bool enable)```</span>
  ///
  ///
  pub fn set_resolve_symlinks(&mut self, enable: bool) {
    unsafe { ::ffi::qt_widgets_c_QDirModel_setResolveSymlinks(self as *mut ::dir_model::DirModel, enable) }
  }

  /// C++ method: <span style='color: green;'>```QDirModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QDirModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QDirModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DirModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QDirModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDirModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDirModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QDirModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dir_model::DirModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QDirModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DirModel`.
  pub struct Signals<'a>(&'a ::dir_model::DirModel);
  /// Represents a built-in Qt signal `QDirModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsRemoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsMoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsMoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsInserted`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsInserted<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::dataChanged`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct DataChanged<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::rowsMoved`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsMoved<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::modelReset`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ModelReset<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::layoutChanged`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct LayoutChanged<'a>(&'a ::dir_model::DirModel);
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
  /// Represents a built-in Qt signal `QDirModel::rowsInserted`.
  ///
  /// An object of this type can be created from `DirModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct RowsInserted<'a>(&'a ::dir_model::DirModel);
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
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDirModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `DirModel`.
  pub struct Slots<'a>(&'a ::dir_model::DirModel);
  /// Represents a built-in Qt slot `QDirModel::refresh`.
  ///
  /// An object of this type can be created from `DirModel` with `object.slots().refresh()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct Refresh<'a>(&'a ::dir_model::DirModel);
  impl<'a> ::qt_core::connection::Receiver for Refresh<'a> {
    type Arguments = (&'static ::qt_core::model_index::ModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1refresh(const QModelIndex&)\0"
    }
  }
  /// Represents a built-in Qt slot `QDirModel::revert`.
  ///
  /// An object of this type can be created from `DirModel` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct Revert<'a>(&'a ::dir_model::DirModel);
  impl<'a> ::qt_core::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  /// Represents a built-in Qt slot `QDirModel::resetInternalData`.
  ///
  /// An object of this type can be created from `DirModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct ResetInternalData<'a>(&'a ::dir_model::DirModel);
  impl<'a> ::qt_core::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  /// Represents a built-in Qt slot `QDirModel::submit`.
  ///
  /// An object of this type can be created from `DirModel` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DirModel` object.
  pub struct Submit<'a>(&'a ::dir_model::DirModel);
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
    /// Returns an object representing a built-in Qt slot `QDirModel::refresh`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn refresh(&self) -> Refresh {
      Refresh(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDirModel::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDirModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QDirModel::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
  }
  impl ::dir_model::DirModel {
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

/// C++ type: <span style='color: green;'>```QDirModel::Roles```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Roles {
  /// C++ enum variant: <span style='color: green;'>```FileIconRole = 1```</span>
  Icon = 1,
  /// C++ enum variant: <span style='color: green;'>```FilePathRole = 257```</span>
  Path = 257,
  /// C++ enum variant: <span style='color: green;'>```FileNameRole = 258```</span>
  Name = 258,
}

impl ::cpp_utils::StaticCast<::qt_core::abstract_item_model::AbstractItemModel> for ::dir_model::DirModel {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::dir_model::DirModel as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::dir_model::DirModel {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QObject_ptr(self as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QObject_ptr(self as *const ::dir_model::DirModel as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dir_model::DirModel> for ::qt_core::abstract_item_model::AbstractItemModel {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dir_model::DirModel {
    let ffi_result = ::ffi::qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QAbstractItemModel(self as *mut ::qt_core::abstract_item_model::AbstractItemModel);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dir_model::DirModel {
    let ffi_result = ::ffi::qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QAbstractItemModel(self as *const ::qt_core::abstract_item_model::AbstractItemModel as *mut ::qt_core::abstract_item_model::AbstractItemModel);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dir_model::DirModel> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dir_model::DirModel {
    let ffi_result =
      ::ffi::qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dir_model::DirModel {
    let ffi_result = ::ffi::qt_widgets_c_QDirModel_G_static_cast_QDirModel_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dir_model::DirModel {
  type Target = ::qt_core::abstract_item_model::AbstractItemModel;
  fn deref(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::dir_model::DirModel as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dir_model::DirModel {
  fn deref_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QDirModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::dir_model::DirModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DirModel::column_count](../struct.DirModel.html#method.column_count) method.
  pub trait DirModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int;
  }
  impl<'largs> DirModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QDirModel_columnCount_no_args(original_self as *const ::dir_model::DirModel) }
    }
  }
  impl<'largs> DirModelColumnCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int {
      let parent = self;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_columnCount_parent(original_self as *const ::dir_model::DirModel,
                                                         parent as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::data](../struct.DirModel.html#method.data) method.
  pub trait DirModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> DirModelDataArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant {
      let index = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_data_to_output_index(original_self as *const ::dir_model::DirModel,
                                                             index as *const ::qt_core::model_index::ModelIndex,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirModelDataArgs<'largs> for (&'largs ::qt_core::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_data_to_output_index_role(original_self as *const ::dir_model::DirModel,
                                                                  index as *const ::qt_core::model_index::ModelIndex,
                                                                  role,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::has_children](../struct.DirModel.html#method.has_children) method.
  pub trait DirModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> bool;
  }
  impl<'largs> DirModelHasChildrenArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> bool {
      let index = self;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_hasChildren_index(original_self as *const ::dir_model::DirModel,
                                                        index as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> DirModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> bool {

      unsafe { ::ffi::qt_widgets_c_QDirModel_hasChildren_no_args(original_self as *const ::dir_model::DirModel) }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::header_data](../struct.DirModel.html#method.header_data) method.
  pub trait DirModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> DirModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::Orientation) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_headerData_to_output_section_orientation(original_self as *const ::dir_model::DirModel, section, orientation as *const ::qt_core::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_headerData_to_output_section_orientation_role(original_self as *const ::dir_model::DirModel, section, orientation as *const ::qt_core::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::index](../struct.DirModel.html#method.index) method.
  pub trait DirModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::model_index::ModelIndex;
  }
  impl<'largs> DirModelIndexArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::model_index::ModelIndex {
      let path = self;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_index_to_output_path(original_self as *const ::dir_model::DirModel,
                                                             path as *const ::qt_core::string::String,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirModelIndexArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::model_index::ModelIndex {
      let path = self.0;
      let column = self.1;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_index_to_output_path_column(original_self as *const ::dir_model::DirModel,
                                                                    path as *const ::qt_core::string::String,
                                                                    column,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_index_to_output_row_column(original_self as *const ::dir_model::DirModel,
                                                                   row,
                                                                   column,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QDirModel_index_to_output_row_column_parent(original_self as *const ::dir_model::DirModel, row, column, parent as *const ::qt_core::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::refresh](../struct.DirModel.html#method.refresh) method.
  pub trait DirModelRefreshArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> ();
  }
  impl<'largs> DirModelRefreshArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> () {

      unsafe { ::ffi::qt_widgets_c_QDirModel_refresh_no_args(original_self as *mut ::dir_model::DirModel) }
    }
  }
  impl<'largs> DirModelRefreshArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> () {
      let parent = self;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_refresh_parent(original_self as *mut ::dir_model::DirModel,
                                                     parent as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::row_count](../struct.DirModel.html#method.row_count) method.
  pub trait DirModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int;
  }
  impl<'largs> DirModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QDirModel_rowCount_no_args(original_self as *const ::dir_model::DirModel) }
    }
  }
  impl<'largs> DirModelRowCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::dir_model::DirModel) -> ::libc::c_int {
      let parent = self;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_rowCount_parent(original_self as *const ::dir_model::DirModel,
                                                      parent as *const ::qt_core::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::set_data](../struct.DirModel.html#method.set_data) method.
  pub trait DirModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> bool;
  }
  impl<'largs> DirModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_setData_index_value(original_self as *mut ::dir_model::DirModel,
                                                          index as *const ::qt_core::model_index::ModelIndex,
                                                          value as *const ::qt_core::variant::Variant)
      }
    }
  }
  impl<'largs> DirModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_setData_index_value_role(original_self as *mut ::dir_model::DirModel,
                                                               index as *const ::qt_core::model_index::ModelIndex,
                                                               value as *const ::qt_core::variant::Variant,
                                                               role)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DirModel::sort](../struct.DirModel.html#method.sort) method.
  pub trait DirModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> ();
  }
  impl<'largs> DirModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> () {
      let column = self;
      unsafe { ::ffi::qt_widgets_c_QDirModel_sort_column(original_self as *mut ::dir_model::DirModel, column) }
    }
  }
  impl<'largs> DirModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::dir_model::DirModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QDirModel_sort_column_order(original_self as *mut ::dir_model::DirModel,
                                                        column,
                                                        order as *const ::qt_core::qt::SortOrder)
      }
    }
  }
}
