/// C++ type: <span style='color: green;'>```QFileSystemModel```</span>
#[repr(C)]
pub struct FileSystemModel(u8);

impl FileSystemModel {
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::canFetchMore(const QModelIndex& parent) const```</span>
  ///
  ///
  pub fn can_fetch_more(&self, parent: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_canFetchMore(self as *const ::file_system_model::FileSystemModel,
                                                        parent as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::columnCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn column_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QFileSystemModel::columnCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn column_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QFileSystemModel::columnCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn column_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::FileSystemModelColumnCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileSystemModel::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, &::qt_core::model_index::ModelIndex) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QFileSystemModel::data(const QModelIndex& index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, (&::qt_core::model_index::ModelIndex, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QFileSystemModel::data(const QModelIndex& index, int role = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::FileSystemModelDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::dropMimeData(const QMimeData* data, Qt::DropAction action, int row, int column, const QModelIndex& parent)```</span>
  ///
  ///
  pub unsafe fn drop_mime_data(&mut self,
                               data: *const ::qt_core::mime_data::MimeData,
                               action: &::qt_core::qt::DropAction,
                               row: ::libc::c_int,
                               column: ::libc::c_int,
                               parent: &::qt_core::model_index::ModelIndex)
                               -> bool {
    ::ffi::qt_widgets_c_QFileSystemModel_dropMimeData(self as *mut ::file_system_model::FileSystemModel,
                                                      data,
                                                      action as *const ::qt_core::qt::DropAction,
                                                      row,
                                                      column,
                                                      parent as *const ::qt_core::model_index::ModelIndex)
  }

  /// C++ method: <span style='color: green;'>```virtual void QFileSystemModel::fetchMore(const QModelIndex& parent)```</span>
  ///
  ///
  pub fn fetch_more(&mut self, parent: &::qt_core::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_fetchMore(self as *mut ::file_system_model::FileSystemModel,
                                                     parent as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QIcon QFileSystemModel::fileIcon(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_icon(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_gui::icon::Icon {
    {
      let mut object: ::qt_gui::icon::Icon =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_fileIcon_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                index as *const ::qt_core::model_index::ModelIndex,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo QFileSystemModel::fileInfo(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_info(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::file_info::FileInfo {
    {
      let mut object: ::qt_core::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_fileInfo_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                index as *const ::qt_core::model_index::ModelIndex,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileSystemModel::fileName(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_name(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_fileName_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                index as *const ::qt_core::model_index::ModelIndex,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileSystemModel::filePath(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn file_path(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_filePath_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                index as *const ::qt_core::model_index::ModelIndex,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::hasChildren```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn has_children(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::hasChildren() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn has_children(&self, &::qt_core::model_index::ModelIndex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::hasChildren(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn has_children<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::FileSystemModelHasChildrenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileSystemModel::headerData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QFileSystemModel::headerData(int section, Qt::Orientation orientation) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn header_data(&self, (::libc::c_int, &::qt_core::qt::Orientation, ::libc::c_int)) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```virtual QVariant QFileSystemModel::headerData(int section, Qt::Orientation orientation, int role = ?) const```</span>
  ///
  ///
  pub fn header_data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::FileSystemModelHeaderDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileIconProvider* QFileSystemModel::iconProvider() const```</span>
  ///
  ///
  pub fn icon_provider(&self) -> *mut ::file_icon_provider::FileIconProvider {
    unsafe { ::ffi::qt_widgets_c_QFileSystemModel_iconProvider(self as *const ::file_system_model::FileSystemModel) }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::index```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index(&self, &::qt_core::string::String) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QFileSystemModel::index(const QString& path) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index(&self, (&::qt_core::string::String, ::libc::c_int)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QFileSystemModel::index(const QString& path, int column = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QFileSystemModel::index(int row, int column) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index(&self, (::libc::c_int, ::libc::c_int, &::qt_core::model_index::ModelIndex)) -> ::qt_core::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```virtual QModelIndex QFileSystemModel::index(int row, int column, const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn index<'largs, Args>(&'largs self, args: Args) -> ::qt_core::model_index::ModelIndex
    where Args: overloading::FileSystemModelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::isDir(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn is_dir(&self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_isDir(self as *const ::file_system_model::FileSystemModel,
                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileSystemModel_isReadOnly(self as *const ::file_system_model::FileSystemModel) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QFileSystemModel::lastModified(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn last_modified(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::date_time::DateTime {
    {
      let mut object: ::qt_core::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_lastModified_to_output(self as *const ::file_system_model::FileSystemModel, index as *const ::qt_core::model_index::ModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFileSystemModel::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_widgets_c_QFileSystemModel_metaObject(self as *const ::file_system_model::FileSystemModel) }
  }

  /// C++ method: <span style='color: green;'>```virtual QMimeData* QFileSystemModel::mimeData(const QList<QModelIndex>& indexes) const```</span>
  ///
  ///
  pub fn mime_data(&self, indexes: &::qt_core::list::ListModelIndex) -> *mut ::qt_core::mime_data::MimeData {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_mimeData(self as *const ::file_system_model::FileSystemModel,
                                                    indexes as *const ::qt_core::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList QFileSystemModel::mimeTypes() const```</span>
  ///
  ///
  pub fn mime_types(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_mimeTypes_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QFileSystemModel::mkdir(const QModelIndex& parent, const QString& name)```</span>
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
        ::ffi::qt_widgets_c_QFileSystemModel_mkdir_to_output(self as *mut ::file_system_model::FileSystemModel,
                                                             parent as *const ::qt_core::model_index::ModelIndex,
                                                             name as *const ::qt_core::string::String,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::myComputer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn my_computer(&self, ()) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QFileSystemModel::myComputer() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn my_computer(&self, ::libc::c_int) -> ::qt_core::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QFileSystemModel::myComputer(int role = ?) const```</span>
  ///
  ///
  pub fn my_computer<'largs, Args>(&'largs self, args: Args) -> ::qt_core::variant::Variant
    where Args: overloading::FileSystemModelMyComputerArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::nameFilterDisables() const```</span>
  ///
  ///
  pub fn name_filter_disables(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_nameFilterDisables(self as *const ::file_system_model::FileSystemModel)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSystemModel::nameFilters() const```</span>
  ///
  ///
  pub fn name_filters(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_nameFilters_to_output(self as *const ::file_system_model::FileSystemModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemModel::QFileSystemModel()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::file_system_model::FileSystemModel> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemModel::QFileSystemModel(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::file_system_model::FileSystemModel> {
    let ffi_result = ::ffi::qt_widgets_c_QFileSystemModel_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QFileSystemModel::parent(const QModelIndex& child) const```</span>
  ///
  ///
  pub fn parent(&self, child: &::qt_core::model_index::ModelIndex) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_parent_to_output(self as *const ::file_system_model::FileSystemModel,
                                                              child as *const ::qt_core::model_index::ModelIndex,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QFileSystemModel::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_widgets_c_QFileSystemModel_qt_metacall(self as *mut ::file_system_model::FileSystemModel,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QFileSystemModel::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QFileSystemModel_qt_metacast(self as *mut ::file_system_model::FileSystemModel, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::remove(const QModelIndex& index)```</span>
  ///
  ///
  pub fn remove(&mut self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_remove(self as *mut ::file_system_model::FileSystemModel,
                                                  index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::resolveSymlinks() const```</span>
  ///
  ///
  pub fn resolve_symlinks(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QFileSystemModel_resolveSymlinks(self as *const ::file_system_model::FileSystemModel) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileSystemModel::rmdir(const QModelIndex& index)```</span>
  ///
  ///
  pub fn rmdir(&mut self, index: &::qt_core::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_rmdir(self as *mut ::file_system_model::FileSystemModel,
                                                 index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QDir QFileSystemModel::rootDirectory() const```</span>
  ///
  ///
  pub fn root_directory(&self) -> ::qt_core::dir::Dir {
    {
      let mut object: ::qt_core::dir::Dir =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_rootDirectory_to_output(self as *const ::file_system_model::FileSystemModel, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileSystemModel::rootPath() const```</span>
  ///
  ///
  pub fn root_path(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_rootPath_to_output(self as *const ::file_system_model::FileSystemModel,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::rowCount```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn row_count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QFileSystemModel::rowCount() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn row_count(&self, &::qt_core::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```virtual int QFileSystemModel::rowCount(const QModelIndex& parent = ?) const```</span>
  ///
  ///
  pub fn row_count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::FileSystemModelRowCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileSystemModel::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::setData(const QModelIndex& index, const QVariant& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::model_index::ModelIndex, &::qt_core::variant::Variant, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QFileSystemModel::setData(const QModelIndex& index, const QVariant& value, int role = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::FileSystemModelSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QFileSystemModel::setIconProvider(QFileIconProvider* provider)```</span>
  ///
  ///
  pub unsafe fn set_icon_provider(&mut self, provider: *mut ::file_icon_provider::FileIconProvider) {
    ::ffi::qt_widgets_c_QFileSystemModel_setIconProvider(self as *mut ::file_system_model::FileSystemModel, provider)
  }

  /// C++ method: <span style='color: green;'>```void QFileSystemModel::setNameFilterDisables(bool enable)```</span>
  ///
  ///
  pub fn set_name_filter_disables(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_setNameFilterDisables(self as *mut ::file_system_model::FileSystemModel,
                                                                 enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileSystemModel::setNameFilters(const QStringList& filters)```</span>
  ///
  ///
  pub fn set_name_filters(&mut self, filters: &::qt_core::string_list::StringList) {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_setNameFilters(self as *mut ::file_system_model::FileSystemModel,
                                                          filters as *const ::qt_core::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileSystemModel::setReadOnly(bool enable)```</span>
  ///
  ///
  pub fn set_read_only(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_setReadOnly(self as *mut ::file_system_model::FileSystemModel, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileSystemModel::setResolveSymlinks(bool enable)```</span>
  ///
  ///
  pub fn set_resolve_symlinks(&mut self, enable: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_setResolveSymlinks(self as *mut ::file_system_model::FileSystemModel, enable)
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QFileSystemModel::setRootPath(const QString& path)```</span>
  ///
  ///
  pub fn set_root_path(&mut self, path: &::qt_core::string::String) -> ::qt_core::model_index::ModelIndex {
    {
      let mut object: ::qt_core::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_setRootPath_to_output(self as *mut ::file_system_model::FileSystemModel,
                                                                   path as *const ::qt_core::string::String,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QModelIndex QFileSystemModel::sibling(int row, int column, const QModelIndex& idx) const```</span>
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
        ::ffi::qt_widgets_c_QFileSystemModel_sibling_to_output(self as *const ::file_system_model::FileSystemModel,
                                                               row,
                                                               column,
                                                               idx as *const ::qt_core::model_index::ModelIndex,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QFileSystemModel::size(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn size(&self, index: &::qt_core::model_index::ModelIndex) -> i64 {
    unsafe {
      ::ffi::qt_widgets_c_QFileSystemModel_size(self as *const ::file_system_model::FileSystemModel,
                                                index as *const ::qt_core::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemModel::sort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn sort(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QFileSystemModel::sort(int column)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn sort(&mut self, (::libc::c_int, &::qt_core::qt::SortOrder)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QFileSystemModel::sort(int column, Qt::SortOrder order = ?)```</span>
  ///
  ///
  pub fn sort<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FileSystemModelSortArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QFileSystemModel::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFileSystemModel_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileSystemModel::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QFileSystemModel_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileSystemModel::type(const QModelIndex& index) const```</span>
  ///
  ///
  pub fn type_(&self, index: &::qt_core::model_index::ModelIndex) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_type_to_output(self as *const ::file_system_model::FileSystemModel,
                                                            index as *const ::qt_core::model_index::ModelIndex,
                                                            &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file_system_model::FileSystemModel {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QFileSystemModel_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FileSystemModel`.
  pub struct Signals<'a>(&'a ::file_system_model::FileSystemModel);
  /// Represents a built-in Qt signal `QFileSystemModel::columnsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsAboutToBeRemoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::layoutAboutToBeChanged`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().layout_about_to_be_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct LayoutAboutToBeChanged<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::directoryLoaded`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().directory_loaded()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct DirectoryLoaded<'a>(&'a ::file_system_model::FileSystemModel);
  impl<'a> ::qt_core::connection::Receiver for DirectoryLoaded<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directoryLoaded(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DirectoryLoaded<'a> {}
  /// Represents a built-in Qt signal `QFileSystemModel::columnsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsAboutToBeMoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rowsAboutToBeRemoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_about_to_be_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsAboutToBeRemoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::fileRenamed`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().file_renamed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct FileRenamed<'a>(&'a ::file_system_model::FileSystemModel);
  impl<'a> ::qt_core::connection::Receiver for FileRenamed<'a> {
    type Arguments = (&'static ::qt_core::string::String,
     &'static ::qt_core::string::String,
     &'static ::qt_core::string::String);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fileRenamed(const QString&,const QString&,const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FileRenamed<'a> {}
  /// Represents a built-in Qt signal `QFileSystemModel::rowsInserted`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsInserted<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::columnsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsAboutToBeInserted<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::modelAboutToBeReset`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().model_about_to_be_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ModelAboutToBeReset<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::headerDataChanged`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().header_data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct HeaderDataChanged<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::dataChanged`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().data_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct DataChanged<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::columnsMoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsMoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rowsAboutToBeInserted`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_about_to_be_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsAboutToBeInserted<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rowsAboutToBeMoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_about_to_be_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsAboutToBeMoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rootPathChanged`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().root_path_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RootPathChanged<'a>(&'a ::file_system_model::FileSystemModel);
  impl<'a> ::qt_core::connection::Receiver for RootPathChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rootPathChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RootPathChanged<'a> {}
  /// Represents a built-in Qt signal `QFileSystemModel::layoutChanged`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().layout_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct LayoutChanged<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::modelReset`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().model_reset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ModelReset<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::columnsInserted`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_inserted()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsInserted<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rowsRemoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsRemoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::rowsMoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().rows_moved()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct RowsMoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  /// Represents a built-in Qt signal `QFileSystemModel::columnsRemoved`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.signals().columns_removed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ColumnsRemoved<'a>(&'a ::file_system_model::FileSystemModel);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_removed(&self) -> ColumnsAboutToBeRemoved {
      ColumnsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::layoutAboutToBeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_about_to_be_changed(&self) -> LayoutAboutToBeChanged {
      LayoutAboutToBeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::directoryLoaded`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn directory_loaded(&self) -> DirectoryLoaded {
      DirectoryLoaded(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_moved(&self) -> ColumnsAboutToBeMoved {
      ColumnsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsAboutToBeRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_removed(&self) -> RowsAboutToBeRemoved {
      RowsAboutToBeRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::fileRenamed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn file_renamed(&self) -> FileRenamed {
      FileRenamed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_inserted(&self) -> RowsInserted {
      RowsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_about_to_be_inserted(&self) -> ColumnsAboutToBeInserted {
      ColumnsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::modelAboutToBeReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_about_to_be_reset(&self) -> ModelAboutToBeReset {
      ModelAboutToBeReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::headerDataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn header_data_changed(&self) -> HeaderDataChanged {
      HeaderDataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::dataChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn data_changed(&self) -> DataChanged {
      DataChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_moved(&self) -> ColumnsMoved {
      ColumnsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsAboutToBeInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_inserted(&self) -> RowsAboutToBeInserted {
      RowsAboutToBeInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsAboutToBeMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_about_to_be_moved(&self) -> RowsAboutToBeMoved {
      RowsAboutToBeMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rootPathChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn root_path_changed(&self) -> RootPathChanged {
      RootPathChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::layoutChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layout_changed(&self) -> LayoutChanged {
      LayoutChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::modelReset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn model_reset(&self) -> ModelReset {
      ModelReset(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsInserted`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_inserted(&self) -> ColumnsInserted {
      ColumnsInserted(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_removed(&self) -> RowsRemoved {
      RowsRemoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::rowsMoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rows_moved(&self) -> RowsMoved {
      RowsMoved(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemModel::columnsRemoved`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn columns_removed(&self) -> ColumnsRemoved {
      ColumnsRemoved(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `FileSystemModel`.
  pub struct Slots<'a>(&'a ::file_system_model::FileSystemModel);
  /// Represents a built-in Qt slot `QFileSystemModel::resetInternalData`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.slots().reset_internal_data()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct ResetInternalData<'a>(&'a ::file_system_model::FileSystemModel);
  impl<'a> ::qt_core::connection::Receiver for ResetInternalData<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resetInternalData()\0"
    }
  }
  /// Represents a built-in Qt slot `QFileSystemModel::revert`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.slots().revert()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct Revert<'a>(&'a ::file_system_model::FileSystemModel);
  impl<'a> ::qt_core::connection::Receiver for Revert<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1revert()\0"
    }
  }
  /// Represents a built-in Qt slot `QFileSystemModel::submit`.
  ///
  /// An object of this type can be created from `FileSystemModel` with `object.slots().submit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemModel` object.
  pub struct Submit<'a>(&'a ::file_system_model::FileSystemModel);
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
    /// Returns an object representing a built-in Qt slot `QFileSystemModel::resetInternalData`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reset_internal_data(&self) -> ResetInternalData {
      ResetInternalData(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFileSystemModel::revert`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn revert(&self) -> Revert {
      Revert(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QFileSystemModel::submit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn submit(&self) -> Submit {
      Submit(self.0)
    }
  }
  impl ::file_system_model::FileSystemModel {
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

/// C++ type: <span style='color: green;'>```QFileSystemModel::Roles```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Roles {
  /// C++ enum variant: <span style='color: green;'>```FileIconRole = 1```</span>
  IconRole = 1,
  /// C++ enum variant: <span style='color: green;'>```FilePathRole = 257```</span>
  PathRole = 257,
  /// C++ enum variant: <span style='color: green;'>```FileNameRole = 258```</span>
  NameRole = 258,
  /// C++ enum variant: <span style='color: green;'>```FilePermissions = 259```</span>
  Permissions = 259,
}

impl ::cpp_utils::StaticCast<::qt_core::abstract_item_model::AbstractItemModel> for ::file_system_model::FileSystemModel {
fn static_cast_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::file_system_model::FileSystemModel) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::file_system_model::FileSystemModel as *mut ::file_system_model::FileSystemModel) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::file_system_model::FileSystemModel {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QObject_ptr(self as *mut ::file_system_model::FileSystemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QObject_ptr(self as *const ::file_system_model::FileSystemModel as *mut ::file_system_model::FileSystemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_system_model::FileSystemModel> for ::qt_core::abstract_item_model::AbstractItemModel {
unsafe fn static_cast_mut(&mut self) -> &mut ::file_system_model::FileSystemModel {
let ffi_result = ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QAbstractItemModel(self as *mut ::qt_core::abstract_item_model::AbstractItemModel);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::file_system_model::FileSystemModel {
let ffi_result = ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QAbstractItemModel(self as *const ::qt_core::abstract_item_model::AbstractItemModel as *mut ::qt_core::abstract_item_model::AbstractItemModel);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::file_system_model::FileSystemModel> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_system_model::FileSystemModel {
    let ffi_result = ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_system_model::FileSystemModel {
    let ffi_result = ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QFileSystemModel_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_system_model::FileSystemModel {
  type Target = ::qt_core::abstract_item_model::AbstractItemModel;
  fn deref(&self) -> &::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(self as *const ::file_system_model::FileSystemModel as *mut ::file_system_model::FileSystemModel) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_system_model::FileSystemModel {
  fn deref_mut(&mut self) -> &mut ::qt_core::abstract_item_model::AbstractItemModel {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QFileSystemModel_G_static_cast_QAbstractItemModel_ptr(self as *mut ::file_system_model::FileSystemModel) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileSystemModel::column_count](../struct.FileSystemModel.html#method.column_count) method.
  pub trait FileSystemModelColumnCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int;
  }
  impl<'largs> FileSystemModelColumnCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_columnCount_no_args(original_self as *const ::file_system_model::FileSystemModel) }
    }
  }
  impl<'largs> FileSystemModelColumnCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_columnCount_parent(original_self as *const ::file_system_model::FileSystemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::data](../struct.FileSystemModel.html#method.data) method.
  pub trait FileSystemModelDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> FileSystemModelDataArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {
      let index = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_data_to_output_index(original_self as *const ::file_system_model::FileSystemModel, index as *const ::qt_core::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelDataArgs<'largs> for (&'largs ::qt_core::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {
      let index = self.0;
      let role = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_data_to_output_index_role(original_self as *const ::file_system_model::FileSystemModel, index as *const ::qt_core::model_index::ModelIndex, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::has_children](../struct.FileSystemModel.html#method.has_children) method.
  pub trait FileSystemModelHasChildrenArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> bool;
  }
  impl<'largs> FileSystemModelHasChildrenArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> bool {

      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_hasChildren_no_args(original_self as *const ::file_system_model::FileSystemModel) }
    }
  }
  impl<'largs> FileSystemModelHasChildrenArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> bool {
      let parent = self;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_hasChildren_parent(original_self as *const ::file_system_model::FileSystemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::header_data](../struct.FileSystemModel.html#method.header_data) method.
  pub trait FileSystemModelHeaderDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> FileSystemModelHeaderDataArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::Orientation) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation(original_self as *const ::file_system_model::FileSystemModel, section, orientation as *const ::qt_core::qt::Orientation, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelHeaderDataArgs<'largs>
    for (::libc::c_int, &'largs ::qt_core::qt::Orientation, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {
      let section = self.0;
      let orientation = self.1;
      let role = self.2;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_headerData_to_output_section_orientation_role(original_self as *const ::file_system_model::FileSystemModel, section, orientation as *const ::qt_core::qt::Orientation, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::index](../struct.FileSystemModel.html#method.index) method.
  pub trait FileSystemModelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::model_index::ModelIndex;
  }
  impl<'largs> FileSystemModelIndexArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::model_index::ModelIndex {
      let path = self;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_index_to_output_path(original_self as *const ::file_system_model::FileSystemModel, path as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelIndexArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::model_index::ModelIndex {
      let path = self.0;
      let column = self.1;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_index_to_output_path_column(original_self as *const ::file_system_model::FileSystemModel, path as *const ::qt_core::string::String, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_index_to_output_row_column(original_self as *const ::file_system_model::FileSystemModel, row, column, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelIndexArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::model_index::ModelIndex {
      let row = self.0;
      let column = self.1;
      let parent = self.2;
      {
        let mut object: ::qt_core::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_index_to_output_row_column_parent(original_self as *const ::file_system_model::FileSystemModel, row, column, parent as *const ::qt_core::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::my_computer](../struct.FileSystemModel.html#method.my_computer) method.
  pub trait FileSystemModelMyComputerArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant;
  }
  impl<'largs> FileSystemModelMyComputerArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {

      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_myComputer_to_output_no_args(original_self as *const ::file_system_model::FileSystemModel, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FileSystemModelMyComputerArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::qt_core::variant::Variant {
      let role = self;
      {
        let mut object: ::qt_core::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QFileSystemModel_myComputer_to_output_role(original_self as *const ::file_system_model::FileSystemModel, role, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::row_count](../struct.FileSystemModel.html#method.row_count) method.
  pub trait FileSystemModelRowCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int;
  }
  impl<'largs> FileSystemModelRowCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_rowCount_no_args(original_self as *const ::file_system_model::FileSystemModel) }
    }
  }
  impl<'largs> FileSystemModelRowCountArgs<'largs> for &'largs ::qt_core::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::file_system_model::FileSystemModel) -> ::libc::c_int {
      let parent = self;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_rowCount_parent(original_self as *const ::file_system_model::FileSystemModel, parent as *const ::qt_core::model_index::ModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::set_data](../struct.FileSystemModel.html#method.set_data) method.
  pub trait FileSystemModelSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> bool;
  }
  impl<'largs> FileSystemModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant) {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> bool {
      let index = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_setData_index_value(original_self as *mut ::file_system_model::FileSystemModel, index as *const ::qt_core::model_index::ModelIndex, value as *const ::qt_core::variant::Variant) }
    }
  }
  impl<'largs> FileSystemModelSetDataArgs<'largs>
    for (&'largs ::qt_core::model_index::ModelIndex, &'largs ::qt_core::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> bool {
      let index = self.0;
      let value = self.1;
      let role = self.2;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_setData_index_value_role(original_self as *mut ::file_system_model::FileSystemModel, index as *const ::qt_core::model_index::ModelIndex, value as *const ::qt_core::variant::Variant, role) }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemModel::sort](../struct.FileSystemModel.html#method.sort) method.
  pub trait FileSystemModelSortArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> ();
  }
  impl<'largs> FileSystemModelSortArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> () {
      let column = self;
      unsafe {
        ::ffi::qt_widgets_c_QFileSystemModel_sort_column(original_self as *mut ::file_system_model::FileSystemModel,
                                                         column)
      }
    }
  }
  impl<'largs> FileSystemModelSortArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::SortOrder) {
    fn exec(self, original_self: &'largs mut ::file_system_model::FileSystemModel) -> () {
      let column = self.0;
      let order = self.1;
      unsafe { ::ffi::qt_widgets_c_QFileSystemModel_sort_column_order(original_self as *mut ::file_system_model::FileSystemModel, column, order as *const ::qt_core::qt::SortOrder) }
    }
  }
}
