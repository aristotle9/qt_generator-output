/// C++ type: <span style='color: green;'>```QStorageInfo```</span>
#[repr(C)]
pub struct StorageInfo([u8; ::type_sizes::QT_CORE_STORAGE_INFO_STORAGE_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StorageInfo {
  unsafe fn new_uninitialized() -> StorageInfo {
    StorageInfo(::std::mem::uninitialized())
  }
}

impl StorageInfo {
  /// C++ method: <span style='color: green;'>```int QStorageInfo::blockSize() const```</span>
  ///
  ///
  pub fn block_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QStorageInfo_blockSize(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QStorageInfo::bytesAvailable() const```</span>
  ///
  ///
  pub fn bytes_available(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QStorageInfo_bytesAvailable(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QStorageInfo::bytesFree() const```</span>
  ///
  ///
  pub fn bytes_free(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QStorageInfo_bytesFree(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QStorageInfo::bytesTotal() const```</span>
  ///
  ///
  pub fn bytes_total(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QStorageInfo_bytesTotal(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QStorageInfo::device() const```</span>
  ///
  ///
  pub fn device(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_device_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStorageInfo::displayName() const```</span>
  ///
  ///
  pub fn display_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_displayName_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QStorageInfo::fileSystemType() const```</span>
  ///
  ///
  pub fn file_system_type(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_fileSystemType_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QStorageInfo::isReadOnly() const```</span>
  ///
  ///
  pub fn is_read_only(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStorageInfo_isReadOnly(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QStorageInfo::isReady() const```</span>
  ///
  ///
  pub fn is_ready(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStorageInfo_isReady(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QStorageInfo::isRoot() const```</span>
  ///
  ///
  pub fn is_root(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStorageInfo_isRoot(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QStorageInfo::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QStorageInfo_isValid(self as *const ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```static QList<QStorageInfo> QStorageInfo::mountedVolumes()```</span>
  ///
  ///
  pub fn mounted_volumes() -> ::list::ListStorageInfo {
    {
      let mut object: ::list::ListStorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_mountedVolumes_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStorageInfo::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_name_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo::QStorageInfo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStorageInfo::QStorageInfo()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::dir::Dir) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStorageInfo::QStorageInfo(const QDir& dir)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::storage_info::StorageInfo) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStorageInfo::QStorageInfo(const QStorageInfo& other)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStorageInfo::QStorageInfo(const QString& path)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::storage_info::StorageInfo
    where Args: overloading::StorageInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStorageInfo& QStorageInfo::operator=(const QStorageInfo& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::storage_info::StorageInfo)
                             -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QStorageInfo_operator_assign(self as *mut ::storage_info::StorageInfo,
                                                    other as *const ::storage_info::StorageInfo)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QStorageInfo::refresh()```</span>
  ///
  ///
  pub fn refresh(&mut self) {
    unsafe { ::ffi::qt_core_c_QStorageInfo_refresh(self as *mut ::storage_info::StorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```static QStorageInfo QStorageInfo::root()```</span>
  ///
  ///
  pub fn root() -> ::storage_info::StorageInfo {
    {
      let mut object: ::storage_info::StorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_root_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QStorageInfo::rootPath() const```</span>
  ///
  ///
  pub fn root_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_rootPath_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStorageInfo::setPath(const QString& path)```</span>
  ///
  ///
  pub fn set_path(&mut self, path: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QStorageInfo_setPath(self as *mut ::storage_info::StorageInfo,
                                            path as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QStorageInfo::subvolume() const```</span>
  ///
  ///
  pub fn subvolume(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStorageInfo_subvolume_to_output(self as *const ::storage_info::StorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStorageInfo::swap(QStorageInfo& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QStorageInfo_swap(self as *mut ::storage_info::StorageInfo,
                                         other as *mut ::storage_info::StorageInfo)
    }
  }
}

impl Drop for ::storage_info::StorageInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QStorageInfo::~QStorageInfo()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStorageInfo_destructor(self as *mut ::storage_info::StorageInfo) }
  }
}

/// C++ method: <span style='color: green;'>```bool operator!=(const QStorageInfo& first, const QStorageInfo& second)```</span>
///
///
pub fn op_neq(first: &::storage_info::StorageInfo, second: &::storage_info::StorageInfo) -> bool {
  unsafe {
    ::ffi::qt_core_c_QStorageInfo_G_operator_neq(first as *const ::storage_info::StorageInfo,
                                                 second as *const ::storage_info::StorageInfo)
  }
}

/// C++ method: <span style='color: green;'>```void swap(QStorageInfo& value1, QStorageInfo& value2)```</span>
///
///
pub fn swap(value1: &mut ::storage_info::StorageInfo, value2: &mut ::storage_info::StorageInfo) {
  unsafe {
    ::ffi::qt_core_c_QStorageInfo_G_swap(value1 as *mut ::storage_info::StorageInfo,
                                         value2 as *mut ::storage_info::StorageInfo)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StorageInfo::new](../struct.StorageInfo.html#method.new) method.
  pub trait StorageInfoNewArgs {
    fn exec(self) -> ::storage_info::StorageInfo;
  }
  impl<'a> StorageInfoNewArgs for &'a ::dir::Dir {
    fn exec(self) -> ::storage_info::StorageInfo {
      let dir = self;
      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStorageInfo_constructor_dir(dir as *const ::dir::Dir, &mut object);
        }
        object
      }
    }
  }
  impl StorageInfoNewArgs for () {
    fn exec(self) -> ::storage_info::StorageInfo {

      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStorageInfo_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StorageInfoNewArgs for &'a ::storage_info::StorageInfo {
    fn exec(self) -> ::storage_info::StorageInfo {
      let other = self;
      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStorageInfo_constructor_other(other as *const ::storage_info::StorageInfo, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StorageInfoNewArgs for &'a ::string::String {
    fn exec(self) -> ::storage_info::StorageInfo {
      let path = self;
      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStorageInfo_constructor_path(path as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
}
