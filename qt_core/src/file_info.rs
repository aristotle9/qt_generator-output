/// C++ type: <span style='color: green;'>```QFileInfo```</span>
#[repr(C)]
pub struct FileInfo([u8; ::type_sizes::QT_CORE_FILE_INFO_FILE_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for FileInfo {
  unsafe fn new_uninitialized() -> FileInfo {
    FileInfo(::std::mem::uninitialized())
  }
}

impl FileInfo {
  /// C++ method: <span style='color: green;'>```QDir QFileInfo::absoluteDir() const```</span>
  ///
  ///
  pub fn absolute_dir(&self) -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_absoluteDir_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::absoluteFilePath() const```</span>
  ///
  ///
  pub fn absolute_file_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_absoluteFilePath_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::absolutePath() const```</span>
  ///
  ///
  pub fn absolute_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_absolutePath_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::baseName() const```</span>
  ///
  ///
  pub fn base_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_baseName_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::bundleName() const```</span>
  ///
  ///
  pub fn bundle_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_bundleName_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::caching() const```</span>
  ///
  ///
  pub fn caching(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_caching(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::canonicalFilePath() const```</span>
  ///
  ///
  pub fn canonical_file_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_canonicalFilePath_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::canonicalPath() const```</span>
  ///
  ///
  pub fn canonical_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_canonicalPath_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::completeBaseName() const```</span>
  ///
  ///
  pub fn complete_base_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_completeBaseName_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::completeSuffix() const```</span>
  ///
  ///
  pub fn complete_suffix(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_completeSuffix_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QFileInfo::created() const```</span>
  ///
  ///
  pub fn created(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_created_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDir QFileInfo::dir() const```</span>
  ///
  ///
  pub fn dir(&self) -> ::dir::Dir {
    {
      let mut object: ::dir::Dir = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_dir_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::exists() const```</span>
  ///
  ///
  pub fn exists(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_exists_no_args(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```static bool QFileInfo::exists(const QString& file)```</span>
  ///
  ///
  pub fn exists_static(file: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_exists_file(file as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_fileName_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::filePath() const```</span>
  ///
  ///
  pub fn file_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_filePath_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::group() const```</span>
  ///
  ///
  pub fn group(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_group_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QFileInfo::groupId() const```</span>
  ///
  ///
  pub fn group_id(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QFileInfo_groupId(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isAbsolute() const```</span>
  ///
  ///
  pub fn is_absolute(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isAbsolute(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isBundle() const```</span>
  ///
  ///
  pub fn is_bundle(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isBundle(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isDir() const```</span>
  ///
  ///
  pub fn is_dir(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isDir(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isExecutable() const```</span>
  ///
  ///
  pub fn is_executable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isExecutable(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isFile() const```</span>
  ///
  ///
  pub fn is_file(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isFile(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isHidden() const```</span>
  ///
  ///
  pub fn is_hidden(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isHidden(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isNativePath() const```</span>
  ///
  ///
  pub fn is_native_path(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isNativePath(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isReadable() const```</span>
  ///
  ///
  pub fn is_readable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isReadable(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isRelative() const```</span>
  ///
  ///
  pub fn is_relative(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isRelative(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isRoot() const```</span>
  ///
  ///
  pub fn is_root(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isRoot(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isSymLink() const```</span>
  ///
  ///
  pub fn is_sym_link(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isSymLink(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::isWritable() const```</span>
  ///
  ///
  pub fn is_writable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_isWritable(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QFileInfo::lastModified() const```</span>
  ///
  ///
  pub fn last_modified(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_lastModified_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QFileInfo::lastRead() const```</span>
  ///
  ///
  pub fn last_read(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_lastRead_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::makeAbsolute()```</span>
  ///
  ///
  pub fn make_absolute(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileInfo_makeAbsolute(self as *mut ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo::QFileInfo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileInfo::QFileInfo()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::dir::Dir, &::string::String)) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileInfo::QFileInfo(const QDir& dir, const QString& file)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::file::File) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileInfo::QFileInfo(const QFile& file)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::file_info::FileInfo) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileInfo::QFileInfo(const QFileInfo& fileinfo)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileInfo::QFileInfo(const QString& file)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::file_info::FileInfo
    where Args: overloading::FileInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFileInfo& QFileInfo::operator=(const QFileInfo& fileinfo)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, fileinfo: &'l1 ::file_info::FileInfo) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QFileInfo_operator_assign(self as *mut ::file_info::FileInfo,
                                                 fileinfo as *const ::file_info::FileInfo)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::operator==(const QFileInfo& fileinfo) const```</span>
  ///
  ///
  pub fn op_eq(&self, fileinfo: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFileInfo_operator_eq(self as *const ::file_info::FileInfo,
                                             fileinfo as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileInfo::operator!=(const QFileInfo& fileinfo) const```</span>
  ///
  ///
  pub fn op_neq(&self, fileinfo: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFileInfo_operator_neq(self as *const ::file_info::FileInfo,
                                              fileinfo as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::owner() const```</span>
  ///
  ///
  pub fn owner(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_owner_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QFileInfo::ownerId() const```</span>
  ///
  ///
  pub fn owner_id(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QFileInfo_ownerId(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::path() const```</span>
  ///
  ///
  pub fn path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_path_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::readLink() const```</span>
  ///
  ///
  pub fn read_link(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_readLink_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileInfo::refresh()```</span>
  ///
  ///
  pub fn refresh(&mut self) {
    unsafe { ::ffi::qt_core_c_QFileInfo_refresh(self as *mut ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QFileInfo::setCaching(bool on)```</span>
  ///
  ///
  pub fn set_caching(&mut self, on: bool) {
    unsafe { ::ffi::qt_core_c_QFileInfo_setCaching(self as *mut ::file_info::FileInfo, on) }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo::setFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_file(&mut self, (&::dir::Dir, &::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileInfo::setFile(const QDir& dir, const QString& file)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_file(&mut self, &::file::File) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileInfo::setFile(const QFile& file)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_file(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QFileInfo::setFile(const QString& file)```</span>
  ///
  ///
  pub fn set_file<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::FileInfoSetFileArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```qint64 QFileInfo::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QFileInfo_size(self as *const ::file_info::FileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::suffix() const```</span>
  ///
  ///
  pub fn suffix(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_suffix_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QFileInfo::swap(QFileInfo& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QFileInfo_swap(self as *mut ::file_info::FileInfo,
                                      other as *mut ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QFileInfo::symLinkTarget() const```</span>
  ///
  ///
  pub fn sym_link_target(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileInfo_symLinkTarget_to_output(self as *const ::file_info::FileInfo, &mut object);
      }
      object
    }
  }
}

impl Drop for ::file_info::FileInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QFileInfo::~QFileInfo()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QFileInfo_destructor(self as *mut ::file_info::FileInfo) }
  }
}

/// C++ method: <span style='color: green;'>```void swap(QFileInfo& value1, QFileInfo& value2)```</span>
///
///
pub fn swap(value1: &mut ::file_info::FileInfo, value2: &mut ::file_info::FileInfo) {
  unsafe {
    ::ffi::qt_core_c_QFileInfo_G_swap(value1 as *mut ::file_info::FileInfo,
                                      value2 as *mut ::file_info::FileInfo)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileInfo::new](../struct.FileInfo.html#method.new) method.
  pub trait FileInfoNewArgs {
    fn exec(self) -> ::file_info::FileInfo;
  }
  impl<'a> FileInfoNewArgs for (&'a ::dir::Dir, &'a ::string::String) {
    fn exec(self) -> ::file_info::FileInfo {
      let dir = self.0;
      let file = self.1;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileInfo_constructor_QDir_QString(dir as *const ::dir::Dir,
                                                              file as *const ::string::String,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'a> FileInfoNewArgs for &'a ::file::File {
    fn exec(self) -> ::file_info::FileInfo {
      let file = self;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileInfo_constructor_QFile(file as *const ::file::File, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FileInfoNewArgs for &'a ::file_info::FileInfo {
    fn exec(self) -> ::file_info::FileInfo {
      let fileinfo = self;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileInfo_constructor_QFileInfo(fileinfo as *const ::file_info::FileInfo, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FileInfoNewArgs for &'a ::string::String {
    fn exec(self) -> ::file_info::FileInfo {
      let file = self;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileInfo_constructor_QString(file as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl FileInfoNewArgs for () {
    fn exec(self) -> ::file_info::FileInfo {

      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QFileInfo_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FileInfo::set_file](../struct.FileInfo.html#method.set_file) method.
  pub trait FileInfoSetFileArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_info::FileInfo) -> ();
  }
  impl<'largs> FileInfoSetFileArgs<'largs> for (&'largs ::dir::Dir, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::file_info::FileInfo) -> () {
      let dir = self.0;
      let file = self.1;
      unsafe {
        ::ffi::qt_core_c_QFileInfo_setFile_QDir_QString(original_self as *mut ::file_info::FileInfo,
                                                        dir as *const ::dir::Dir,
                                                        file as *const ::string::String)
      }
    }
  }
  impl<'largs> FileInfoSetFileArgs<'largs> for &'largs ::file::File {
    fn exec(self, original_self: &'largs mut ::file_info::FileInfo) -> () {
      let file = self;
      unsafe {
        ::ffi::qt_core_c_QFileInfo_setFile_QFile(original_self as *mut ::file_info::FileInfo,
                                                 file as *const ::file::File)
      }
    }
  }
  impl<'largs> FileInfoSetFileArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::file_info::FileInfo) -> () {
      let file = self;
      unsafe {
        ::ffi::qt_core_c_QFileInfo_setFile_QString(original_self as *mut ::file_info::FileInfo,
                                                   file as *const ::string::String)
      }
    }
  }
}
