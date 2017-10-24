/// C++ type: <span style='color: green;'>```QDir```</span>
#[repr(C)]
pub struct Dir([u8; ::type_sizes::QT_CORE_DIR_DIR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Dir {
  unsafe fn new_uninitialized() -> Dir {
    Dir(::std::mem::uninitialized())
  }
}

impl Dir {
  /// C++ method: <span style='color: green;'>```QString QDir::absoluteFilePath(const QString& fileName) const```</span>
  ///
  ///
  pub fn absolute_file_path(&self, file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_absoluteFilePath_to_output(self as *const ::dir::Dir,
                                                         file_name as *const ::string::String,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::absolutePath() const```</span>
  ///
  ///
  pub fn absolute_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_absolutePath_to_output(self as *const ::dir::Dir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QDir::addResourceSearchPath(const QString& path)```</span>
  ///
  ///
  pub fn add_resource_search_path(path: &::string::String) {
    unsafe { ::ffi::qt_core_c_QDir_addResourceSearchPath(path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QDir::addSearchPath(const QString& prefix, const QString& path)```</span>
  ///
  ///
  pub fn add_search_path(prefix: &::string::String, path: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QDir_addSearchPath(prefix as *const ::string::String,
                                          path as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::canonicalPath() const```</span>
  ///
  ///
  pub fn canonical_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_canonicalPath_to_output(self as *const ::dir::Dir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::cd(const QString& dirName)```</span>
  ///
  ///
  pub fn cd(&mut self, dir_name: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_cd(self as *mut ::dir::Dir, dir_name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::cdUp()```</span>
  ///
  ///
  pub fn cd_up(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_cdUp(self as *mut ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::cleanPath(const QString& path)```</span>
  ///
  ///
  pub fn clean_path(path: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_cleanPath_to_output(path as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QDir::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QDir_count(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static QDir QDir::current()```</span>
  ///
  ///
  pub fn current() -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_current_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::currentPath()```</span>
  ///
  ///
  pub fn current_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_currentPath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::dirName() const```</span>
  ///
  ///
  pub fn dir_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_dirName_to_output(self as *const ::dir::Dir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QFileInfo> QDir::drives()```</span>
  ///
  ///
  pub fn drives() -> ::list::ListFileInfo {
    {
      let mut object: ::list::ListFileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_drives_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDir::entryInfoList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn entry_info_list(&self, ()) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn entry_info_list(&self, ::flags::Flags<::dir::Filter>) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList(QFlags<QDir::Filter> filters = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn entry_info_list(&self, (::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>)) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList(QFlags<QDir::Filter> filters = ?, QFlags<QDir::SortFlag> sort = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn entry_info_list(&self, &::string_list::StringList) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList(const QStringList& nameFilters) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn entry_info_list(&self, (&::string_list::StringList, ::flags::Flags<::dir::Filter>)) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList(const QStringList& nameFilters, QFlags<QDir::Filter> filters = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn entry_info_list(&self, (&::string_list::StringList, ::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>)) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QDir::entryInfoList(const QStringList& nameFilters, QFlags<QDir::Filter> filters = ?, QFlags<QDir::SortFlag> sort = ?) const```</span>
  ///
  ///
  pub fn entry_info_list<'largs, Args>(&'largs self, args: Args) -> ::list::ListFileInfo
    where Args: overloading::DirEntryInfoListArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDir::entryList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn entry_list(&self, ()) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn entry_list(&self, ::flags::Flags<::dir::Filter>) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList(QFlags<QDir::Filter> filters = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn entry_list(&self, (::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList(QFlags<QDir::Filter> filters = ?, QFlags<QDir::SortFlag> sort = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn entry_list(&self, &::string_list::StringList) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList(const QStringList& nameFilters) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn entry_list(&self, (&::string_list::StringList, ::flags::Flags<::dir::Filter>)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList(const QStringList& nameFilters, QFlags<QDir::Filter> filters = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn entry_list(&self, (&::string_list::StringList, ::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```QStringList QDir::entryList(const QStringList& nameFilters, QFlags<QDir::Filter> filters = ?, QFlags<QDir::SortFlag> sort = ?) const```</span>
  ///
  ///
  pub fn entry_list<'largs, Args>(&'largs self, args: Args) -> ::string_list::StringList
    where Args: overloading::DirEntryListArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDir::exists```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exists(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QDir::exists() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exists(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QDir::exists(const QString& name) const```</span>
  ///
  ///
  pub fn exists<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::DirExistsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QDir::filePath(const QString& fileName) const```</span>
  ///
  ///
  pub fn file_path(&self, file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_filePath_to_output(self as *const ::dir::Dir,
                                                 file_name as *const ::string::String,
                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QDir::Filter> QDir::filter() const```</span>
  ///
  ///
  pub fn filter(&self) -> ::flags::Flags<::dir::Filter> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDir_filter(self as *const ::dir::Dir) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::fromNativeSeparators(const QString& pathName)```</span>
  ///
  ///
  pub fn from_native_separators(path_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_fromNativeSeparators_to_output(path_name as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QDir QDir::home()```</span>
  ///
  ///
  pub fn home() -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_home_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::homePath()```</span>
  ///
  ///
  pub fn home_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_homePath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::isAbsolute() const```</span>
  ///
  ///
  pub fn is_absolute(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isAbsolute(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static bool QDir::isAbsolutePath(const QString& path)```</span>
  ///
  ///
  pub fn is_absolute_path(path: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isAbsolutePath(path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QDir::isEmpty```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_empty(&self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QDir::isEmpty() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_empty(&self, ::flags::Flags<::dir::Filter>) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QDir::isEmpty(QFlags<QDir::Filter> filters = ?) const```</span>
  ///
  ///
  pub fn is_empty<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::DirIsEmptyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QDir::isReadable() const```</span>
  ///
  ///
  pub fn is_readable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isReadable(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::isRelative() const```</span>
  ///
  ///
  pub fn is_relative(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isRelative(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static bool QDir::isRelativePath(const QString& path)```</span>
  ///
  ///
  pub fn is_relative_path(path: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isRelativePath(path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::isRoot() const```</span>
  ///
  ///
  pub fn is_root(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_isRoot(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static QChar QDir::listSeparator()```</span>
  ///
  ///
  pub fn list_separator() -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_listSeparator_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::makeAbsolute()```</span>
  ///
  ///
  pub fn make_absolute(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_makeAbsolute(self as *mut ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```QDir::match```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn match_((&::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QDir::match(const QString& filter, const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn match_((&::string_list::StringList, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QDir::match(const QStringList& filters, const QString& fileName)```</span>
  ///
  ///
  pub fn match_<Args>(args: Args) -> bool
    where Args: overloading::DirMatchArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QDir::mkdir(const QString& dirName) const```</span>
  ///
  ///
  pub fn mkdir(&self, dir_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_mkdir(self as *const ::dir::Dir,
                                  dir_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::mkpath(const QString& dirPath) const```</span>
  ///
  ///
  pub fn mkpath(&self, dir_path: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_mkpath(self as *const ::dir::Dir,
                                   dir_path as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QDir::nameFilters() const```</span>
  ///
  ///
  pub fn name_filters(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_nameFilters_to_output(self as *const ::dir::Dir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QDir::nameFiltersFromString(const QString& nameFilter)```</span>
  ///
  ///
  pub fn name_filters_from_string(name_filter: &::string::String) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_nameFiltersFromString_to_output(name_filter as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDir::QDir```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::dir::Dir) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir(const QDir& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir(const QString& path = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir(const QString& path, const QString& nameFilter)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String, ::flags::Flags<::dir::SortFlag>)) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir(const QString& path, const QString& nameFilter, QFlags<QDir::SortFlag> sort = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String, ::flags::Flags<::dir::SortFlag>, ::flags::Flags<::dir::Filter>)) -> ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDir::QDir(const QString& path, const QString& nameFilter, QFlags<QDir::SortFlag> sort = ?, QFlags<QDir::Filter> filter = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::dir::Dir
    where Args: overloading::DirNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDir::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::dir::Dir) -> &'l0 mut ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```QDir& QDir::operator=(const QDir& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::dir::Dir```<br>
  /// C++ method: <span style='color: green;'>```QDir& QDir::operator=(const QString& path)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::dir::Dir
    where Args: overloading::DirOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QDir::operator==(const QDir& dir) const```</span>
  ///
  ///
  pub fn op_eq(&self, dir: &::dir::Dir) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_operator_eq(self as *const ::dir::Dir, dir as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::operator[](int arg1) const```</span>
  ///
  ///
  pub fn op_index(&self, arg1: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_operator_index_to_output(self as *const ::dir::Dir, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::operator!=(const QDir& dir) const```</span>
  ///
  ///
  pub fn op_neq(&self, dir: &::dir::Dir) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_operator_neq(self as *const ::dir::Dir, dir as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::path() const```</span>
  ///
  ///
  pub fn path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_path_to_output(self as *const ::dir::Dir, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QDir::refresh() const```</span>
  ///
  ///
  pub fn refresh(&self) {
    unsafe { ::ffi::qt_core_c_QDir_refresh(self as *const ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```QString QDir::relativeFilePath(const QString& fileName) const```</span>
  ///
  ///
  pub fn relative_file_path(&self, file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_relativeFilePath_to_output(self as *const ::dir::Dir,
                                                         file_name as *const ::string::String,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::remove(const QString& fileName)```</span>
  ///
  ///
  pub fn remove(&mut self, file_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_remove(self as *mut ::dir::Dir,
                                   file_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::removeRecursively()```</span>
  ///
  ///
  pub fn remove_recursively(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_removeRecursively(self as *mut ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::rename(const QString& oldName, const QString& newName)```</span>
  ///
  ///
  pub fn rename(&mut self, old_name: &::string::String, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_rename(self as *mut ::dir::Dir,
                                   old_name as *const ::string::String,
                                   new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::rmdir(const QString& dirName) const```</span>
  ///
  ///
  pub fn rmdir(&self, dir_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_rmdir(self as *const ::dir::Dir,
                                  dir_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QDir::rmpath(const QString& dirPath) const```</span>
  ///
  ///
  pub fn rmpath(&self, dir_path: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QDir_rmpath(self as *const ::dir::Dir,
                                   dir_path as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QDir QDir::root()```</span>
  ///
  ///
  pub fn root() -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_root_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::rootPath()```</span>
  ///
  ///
  pub fn root_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_rootPath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QDir::searchPaths(const QString& prefix)```</span>
  ///
  ///
  pub fn search_paths(prefix: &::string::String) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_searchPaths_to_output(prefix as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QChar QDir::separator()```</span>
  ///
  ///
  pub fn separator() -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_separator_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QDir::setCurrent(const QString& path)```</span>
  ///
  ///
  pub fn set_current(path: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QDir_setCurrent(path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QDir::setFilter(QFlags<QDir::Filter> filter)```</span>
  ///
  ///
  pub fn set_filter(&mut self, filter: ::flags::Flags<::dir::Filter>) {
    unsafe { ::ffi::qt_core_c_QDir_setFilter(self as *mut ::dir::Dir, filter.to_int() as ::libc::c_uint) }
  }

  /// C++ method: <span style='color: green;'>```void QDir::setNameFilters(const QStringList& nameFilters)```</span>
  ///
  ///
  pub fn set_name_filters(&mut self, name_filters: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QDir_setNameFilters(self as *mut ::dir::Dir,
                                           name_filters as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDir::setPath(const QString& path)```</span>
  ///
  ///
  pub fn set_path(&mut self, path: &::string::String) {
    unsafe { ::ffi::qt_core_c_QDir_setPath(self as *mut ::dir::Dir, path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QDir::setSearchPaths(const QString& prefix, const QStringList& searchPaths)```</span>
  ///
  ///
  pub fn set_search_paths(prefix: &::string::String, search_paths: &::string_list::StringList) {
    unsafe {
      ::ffi::qt_core_c_QDir_setSearchPaths(prefix as *const ::string::String,
                                           search_paths as *const ::string_list::StringList)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDir::setSorting(QFlags<QDir::SortFlag> sort)```</span>
  ///
  ///
  pub fn set_sorting(&mut self, sort: ::flags::Flags<::dir::SortFlag>) {
    unsafe { ::ffi::qt_core_c_QDir_setSorting(self as *mut ::dir::Dir, sort.to_int() as ::libc::c_uint) }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QDir::SortFlag> QDir::sorting() const```</span>
  ///
  ///
  pub fn sorting(&self) -> ::flags::Flags<::dir::SortFlag> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDir_sorting(self as *const ::dir::Dir) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```void QDir::swap(QDir& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::dir::Dir) {
    unsafe { ::ffi::qt_core_c_QDir_swap(self as *mut ::dir::Dir, other as *mut ::dir::Dir) }
  }

  /// C++ method: <span style='color: green;'>```static QDir QDir::temp()```</span>
  ///
  ///
  pub fn temp() -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_temp_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::tempPath()```</span>
  ///
  ///
  pub fn temp_path() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_tempPath_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDir::toNativeSeparators(const QString& pathName)```</span>
  ///
  ///
  pub fn to_native_separators(path_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDir_toNativeSeparators_to_output(path_name as *const ::string::String, &mut object);
      }
      object
    }
  }
}

impl Drop for ::dir::Dir {
  /// C++ method: <span style='color: green;'>```[destructor] void QDir::~QDir()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDir_destructor(self as *mut ::dir::Dir) }
  }
}

/// C++ type: <span style='color: green;'>```QDir::Filter```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Filter {
  /// C++ enum variant: <span style='color: green;'>```NoFilter = -1```</span>
  NoFilter = -1,
  /// C++ enum variant: <span style='color: green;'>```Dirs = 1```</span>
  Dirs = 1,
  /// C++ enum variant: <span style='color: green;'>```Files = 2```</span>
  Files = 2,
  /// C++ enum variant: <span style='color: green;'>```Drives = 4```</span>
  Drives = 4,
  /// C++ enum variant: <span style='color: green;'>```AllEntries = 7```</span>
  AllEntries = 7,
  /// C++ enum variant: <span style='color: green;'>```NoSymLinks = 8```</span>
  NoSymLinks = 8,
  /// C++ enum variant: <span style='color: green;'>```TypeMask = 15```</span>
  TypeMask = 15,
  /// C++ enum variant: <span style='color: green;'>```Readable = 16```</span>
  Readable = 16,
  /// C++ enum variant: <span style='color: green;'>```Writable = 32```</span>
  Writable = 32,
  /// C++ enum variant: <span style='color: green;'>```Executable = 64```</span>
  Executable = 64,
  /// C++ enum variant: <span style='color: green;'>```PermissionMask = 112```</span>
  PermissionMask = 112,
  /// C++ enum variant: <span style='color: green;'>```Modified = 128```</span>
  Modified = 128,
  /// C++ enum variant: <span style='color: green;'>```Hidden = 256```</span>
  Hidden = 256,
  /// C++ enum variant: <span style='color: green;'>```System = 512```</span>
  System = 512,
  /// C++ enum variant: <span style='color: green;'>```AccessMask = 1008```</span>
  AccessMask = 1008,
  /// C++ enum variant: <span style='color: green;'>```AllDirs = 1024```</span>
  AllDirs = 1024,
  /// C++ enum variant: <span style='color: green;'>```CaseSensitive = 2048```</span>
  CaseSensitive = 2048,
  /// C++ enum variant: <span style='color: green;'>```NoDot = 8192```</span>
  NoDot = 8192,
  /// C++ enum variant: <span style='color: green;'>```NoDotDot = 16384```</span>
  NoDotDot = 16384,
  /// C++ enum variant: <span style='color: green;'>```NoDotAndDotDot = 24576```</span>
  NoDotAndDotDot = 24576,
}

impl ::flags::FlaggableEnum for Filter {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Filter"
  }
}

/// C++ type: <span style='color: green;'>```QDir::SortFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SortFlag {
  /// C++ enum variant: <span style='color: green;'>```NoSort = -1```</span>
  NoSort = -1,
  /// C++ enum variant: <span style='color: green;'>```Name = 0```</span>
  Name = 0,
  /// C++ enum variant: <span style='color: green;'>```Time = 1```</span>
  Time = 1,
  /// C++ enum variant: <span style='color: green;'>```Size = 2```</span>
  Size = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Unsorted = 3```</span>
  /// - <span style='color: green;'>```SortByMask = 3```</span>
  ///
  Unsorted = 3,
  /// C++ enum variant: <span style='color: green;'>```DirsFirst = 4```</span>
  DirsFirst = 4,
  /// C++ enum variant: <span style='color: green;'>```Reversed = 8```</span>
  Reversed = 8,
  /// C++ enum variant: <span style='color: green;'>```IgnoreCase = 16```</span>
  IgnoreCase = 16,
  /// C++ enum variant: <span style='color: green;'>```DirsLast = 32```</span>
  DirsLast = 32,
  /// C++ enum variant: <span style='color: green;'>```LocaleAware = 64```</span>
  LocaleAware = 64,
  /// C++ enum variant: <span style='color: green;'>```Type = 128```</span>
  Type = 128,
}

impl ::flags::FlaggableEnum for SortFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "SortFlag"
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QDir& dir)```</span>
///
///
pub fn op_shl(debug: &::debug::Debug, dir: &::dir::Dir) -> ::debug::Debug {
  {
    let mut object: ::debug::Debug = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QDir_G_operator_shl_to_output(debug as *const ::debug::Debug,
                                                     dir as *const ::dir::Dir,
                                                     &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QDir& value1, QDir& value2)```</span>
///
///
pub fn swap(value1: &mut ::dir::Dir, value2: &mut ::dir::Dir) {
  unsafe { ::ffi::qt_core_c_QDir_G_swap(value1 as *mut ::dir::Dir, value2 as *mut ::dir::Dir) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Dir::entry_info_list](../struct.Dir.html#method.entry_info_list) method.
  pub trait DirEntryInfoListArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo;
  }
  impl<'largs> DirEntryInfoListArgs<'largs> for ::flags::Flags<::dir::Filter> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {
      let filters = self;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_filters(original_self as *const ::dir::Dir,
                                                                filters.to_int() as ::libc::c_uint,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryInfoListArgs<'largs> for (::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {
      let filters = self.0;
      let sort = self.1;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_filters_sort(original_self as *const ::dir::Dir,
                                                                     filters.to_int() as ::libc::c_uint,
                                                                     sort.to_int() as ::libc::c_uint,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryInfoListArgs<'largs> for &'largs ::string_list::StringList {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {
      let name_filters = self;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_nameFilters(original_self as *const ::dir::Dir,
                                                                    name_filters as *const ::string_list::StringList,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryInfoListArgs<'largs> for (&'largs ::string_list::StringList, ::flags::Flags<::dir::Filter>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {
      let name_filters = self.0;
      let filters = self.1;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters(original_self as *const ::dir::Dir, name_filters as *const ::string_list::StringList, filters.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryInfoListArgs<'largs>
    for (&'largs ::string_list::StringList, ::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {
      let name_filters = self.0;
      let filters = self.1;
      let sort = self.2;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_nameFilters_filters_sort(original_self as *const ::dir::Dir, name_filters as *const ::string_list::StringList, filters.to_int() as ::libc::c_uint, sort.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryInfoListArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::list::ListFileInfo {

      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryInfoList_to_output_no_args(original_self as *const ::dir::Dir, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::entry_list](../struct.Dir.html#method.entry_list) method.
  pub trait DirEntryListArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList;
  }
  impl<'largs> DirEntryListArgs<'largs> for ::flags::Flags<::dir::Filter> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {
      let filters = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_filters(original_self as *const ::dir::Dir,
                                                            filters.to_int() as ::libc::c_uint,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryListArgs<'largs> for (::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {
      let filters = self.0;
      let sort = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_filters_sort(original_self as *const ::dir::Dir,
                                                                 filters.to_int() as ::libc::c_uint,
                                                                 sort.to_int() as ::libc::c_uint,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryListArgs<'largs> for &'largs ::string_list::StringList {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {
      let name_filters = self;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_nameFilters(original_self as *const ::dir::Dir,
                                                                name_filters as *const ::string_list::StringList,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryListArgs<'largs> for (&'largs ::string_list::StringList, ::flags::Flags<::dir::Filter>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {
      let name_filters = self.0;
      let filters = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_nameFilters_filters(original_self as *const ::dir::Dir, name_filters as *const ::string_list::StringList, filters.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryListArgs<'largs>
    for (&'largs ::string_list::StringList, ::flags::Flags<::dir::Filter>, ::flags::Flags<::dir::SortFlag>) {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {
      let name_filters = self.0;
      let filters = self.1;
      let sort = self.2;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_nameFilters_filters_sort(original_self as *const ::dir::Dir, name_filters as *const ::string_list::StringList, filters.to_int() as ::libc::c_uint, sort.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> DirEntryListArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir::Dir) -> ::string_list::StringList {

      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_entryList_to_output_no_args(original_self as *const ::dir::Dir, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::exists](../struct.Dir.html#method.exists) method.
  pub trait DirExistsArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool;
  }
  impl<'largs> DirExistsArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool {
      let name = self;
      unsafe {
        ::ffi::qt_core_c_QDir_exists_name(original_self as *const ::dir::Dir,
                                          name as *const ::string::String)
      }
    }
  }
  impl<'largs> DirExistsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool {

      unsafe { ::ffi::qt_core_c_QDir_exists_no_args(original_self as *const ::dir::Dir) }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::is_empty](../struct.Dir.html#method.is_empty) method.
  pub trait DirIsEmptyArgs<'largs> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool;
  }
  impl<'largs> DirIsEmptyArgs<'largs> for ::flags::Flags<::dir::Filter> {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool {
      let filters = self;
      unsafe {
        ::ffi::qt_core_c_QDir_isEmpty_filters(original_self as *const ::dir::Dir,
                                              filters.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> DirIsEmptyArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::dir::Dir) -> bool {

      unsafe { ::ffi::qt_core_c_QDir_isEmpty_no_args(original_self as *const ::dir::Dir) }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::match_](../struct.Dir.html#method.match_) method.
  pub trait DirMatchArgs {
    fn exec(self) -> bool;
  }
  impl<'a> DirMatchArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> bool {
      let filter = self.0;
      let file_name = self.1;
      unsafe {
        ::ffi::qt_core_c_QDir_match_filter_fileName(filter as *const ::string::String,
                                                    file_name as *const ::string::String)
      }
    }
  }
  impl<'a> DirMatchArgs for (&'a ::string_list::StringList, &'a ::string::String) {
    fn exec(self) -> bool {
      let filters = self.0;
      let file_name = self.1;
      unsafe {
        ::ffi::qt_core_c_QDir_match_filters_fileName(filters as *const ::string_list::StringList,
                                                     file_name as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::new](../struct.Dir.html#method.new) method.
  pub trait DirNewArgs {
    fn exec(self) -> ::dir::Dir;
  }
  impl<'a> DirNewArgs for &'a ::dir::Dir {
    fn exec(self) -> ::dir::Dir {
      let arg1 = self;
      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_arg1(arg1 as *const ::dir::Dir, &mut object);
        }
        object
      }
    }
  }
  impl DirNewArgs for () {
    fn exec(self) -> ::dir::Dir {

      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> DirNewArgs for &'a ::string::String {
    fn exec(self) -> ::dir::Dir {
      let path = self;
      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_path(path as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::dir::Dir {
      let path = self.0;
      let name_filter = self.1;
      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_path_nameFilter(path as *const ::string::String,
                                                            name_filter as *const ::string::String,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirNewArgs for (&'a ::string::String, &'a ::string::String, ::flags::Flags<::dir::SortFlag>) {
    fn exec(self) -> ::dir::Dir {
      let path = self.0;
      let name_filter = self.1;
      let sort = self.2;
      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_path_nameFilter_sort(path as *const ::string::String,
                                                                 name_filter as *const ::string::String,
                                                                 sort.to_int() as ::libc::c_uint,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> DirNewArgs
    for (&'a ::string::String, &'a ::string::String, ::flags::Flags<::dir::SortFlag>, ::flags::Flags<::dir::Filter>) {
    fn exec(self) -> ::dir::Dir {
      let path = self.0;
      let name_filter = self.1;
      let sort = self.2;
      let filter = self.3;
      {
        let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDir_constructor_path_nameFilter_sort_filter(path as *const ::string::String,
                                                                        name_filter as *const ::string::String,
                                                                        sort.to_int() as ::libc::c_uint,
                                                                        filter.to_int() as ::libc::c_uint,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Dir::op_assign](../struct.Dir.html#method.op_assign) method.
  pub trait DirOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::dir::Dir) -> &'largs mut ::dir::Dir;
  }
  impl<'largs> DirOpAssignArgs<'largs> for &'largs ::dir::Dir {
    fn exec(self, original_self: &'largs mut ::dir::Dir) -> &'largs mut ::dir::Dir {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDir_operator_assign_arg1(original_self as *mut ::dir::Dir, arg1 as *const ::dir::Dir)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DirOpAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::dir::Dir) -> &'largs mut ::dir::Dir {
      let path = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDir_operator_assign_path(original_self as *mut ::dir::Dir,
                                                   path as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
