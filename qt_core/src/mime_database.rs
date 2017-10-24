/// C++ type: <span style='color: green;'>```QMimeDatabase::MatchMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MatchMode {
  /// C++ enum variant: <span style='color: green;'>```MatchDefault = 0```</span>
  Default = 0,
  /// C++ enum variant: <span style='color: green;'>```MatchExtension = 1```</span>
  Extension = 1,
  /// C++ enum variant: <span style='color: green;'>```MatchContent = 2```</span>
  Content = 2,
}

/// C++ type: <span style='color: green;'>```QMimeDatabase```</span>
#[repr(C)]
pub struct MimeDatabase([u8; ::type_sizes::QT_CORE_MIME_DATABASE_MIME_DATABASE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MimeDatabase {
  unsafe fn new_uninitialized() -> MimeDatabase {
    MimeDatabase(::std::mem::uninitialized())
  }
}

impl MimeDatabase {
  /// C++ method: <span style='color: green;'>```QList<QMimeType> QMimeDatabase::allMimeTypes() const```</span>
  ///
  ///
  pub fn all_mime_types(&self) -> ::list::ListMimeType {
    {
      let mut object: ::list::ListMimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_allMimeTypes_to_output(self as *const ::mime_database::MimeDatabase,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForData(const QByteArray& data) const```</span>
  ///
  ///
  pub fn mime_type_for_data(&self, data: &::byte_array::ByteArray) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_mimeTypeForData_to_output_data(self as *const ::mime_database::MimeDatabase,
                                                                      data as *const ::byte_array::ByteArray,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForData(QIODevice* device) const```</span>
  ///
  ///
  pub unsafe fn mime_type_for_data_unsafe(&self, device: *mut ::io_device::IODevice) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMimeDatabase_mimeTypeForData_to_output_device(self as *const ::mime_database::MimeDatabase,
                                                                      device,
                                                                      &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeDatabase::mimeTypeForFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mime_type_for_file(&self, &::file_info::FileInfo) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFile(const QFileInfo& fileInfo) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mime_type_for_file(&self, (&::file_info::FileInfo, ::mime_database::MatchMode)) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFile(const QFileInfo& fileInfo, QMimeDatabase::MatchMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn mime_type_for_file(&self, &::string::String) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFile(const QString& fileName) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn mime_type_for_file(&self, (&::string::String, ::mime_database::MatchMode)) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFile(const QString& fileName, QMimeDatabase::MatchMode mode = ?) const```</span>
  ///
  ///
  pub fn mime_type_for_file<'largs, Args>(&'largs self, args: Args) -> ::mime_type::MimeType
    where Args: overloading::MimeDatabaseMimeTypeForFileArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString& fileName, const QByteArray& data) const```</span>
  ///
  ///
  pub fn mime_type_for_file_name_and_data(&self,
                                          file_name: &::string::String,
                                          data: &::byte_array::ByteArray)
                                          -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_data(self as *const ::mime_database::MimeDatabase, file_name as *const ::string::String, data as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForFileNameAndData(const QString& fileName, QIODevice* device) const```</span>
  ///
  ///
  pub unsafe fn mime_type_for_file_name_and_data_unsafe(&self,
                                                        file_name: &::string::String,
                                                        device: *mut ::io_device::IODevice)
                                                        -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFileNameAndData_to_output_fileName_device(self as *const ::mime_database::MimeDatabase, file_name as *const ::string::String, device, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForName(const QString& nameOrAlias) const```</span>
  ///
  ///
  pub fn mime_type_for_name(&self, name_or_alias: &::string::String) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_mimeTypeForName_to_output(self as *const ::mime_database::MimeDatabase,
                                                                 name_or_alias as *const ::string::String,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QMimeDatabase::mimeTypeForUrl(const QUrl& url) const```</span>
  ///
  ///
  pub fn mime_type_for_url(&self, url: &::url::Url) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_mimeTypeForUrl_to_output(self as *const ::mime_database::MimeDatabase,
                                                                url as *const ::url::Url,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType> QMimeDatabase::mimeTypesForFileName(const QString& fileName) const```</span>
  ///
  ///
  pub fn mime_types_for_file_name(&self, file_name: &::string::String) -> ::list::ListMimeType {
    {
      let mut object: ::list::ListMimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_mimeTypesForFileName_to_output(self as *const ::mime_database::MimeDatabase,
                                                                      file_name as *const ::string::String,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMimeDatabase::QMimeDatabase()```</span>
  ///
  ///
  pub fn new() -> ::mime_database::MimeDatabase {
    {
      let mut object: ::mime_database::MimeDatabase =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QMimeDatabase::suffixForFileName(const QString& fileName) const```</span>
  ///
  ///
  pub fn suffix_for_file_name(&self, file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMimeDatabase_suffixForFileName_to_output(self as *const ::mime_database::MimeDatabase,
                                                                   file_name as *const ::string::String,
                                                                   &mut object);
      }
      object
    }
  }
}

impl Drop for ::mime_database::MimeDatabase {
  /// C++ method: <span style='color: green;'>```[destructor] void QMimeDatabase::~QMimeDatabase()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMimeDatabase_destructor(self as *mut ::mime_database::MimeDatabase) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MimeDatabase::mime_type_for_file](../struct.MimeDatabase.html#method.mime_type_for_file) method.
  pub trait MimeDatabaseMimeTypeForFileArgs<'largs> {
    fn exec(self, original_self: &'largs ::mime_database::MimeDatabase) -> ::mime_type::MimeType;
  }
  impl<'largs> MimeDatabaseMimeTypeForFileArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs ::mime_database::MimeDatabase) -> ::mime_type::MimeType {
      let file_info = self;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo(original_self as *const ::mime_database::MimeDatabase, file_info as *const ::file_info::FileInfo, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MimeDatabaseMimeTypeForFileArgs<'largs> for (&'largs ::file_info::FileInfo, ::mime_database::MatchMode) {
    fn exec(self, original_self: &'largs ::mime_database::MimeDatabase) -> ::mime_type::MimeType {
      let file_info = self.0;
      let mode = self.1;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileInfo_mode(original_self as *const ::mime_database::MimeDatabase, file_info as *const ::file_info::FileInfo, mode, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MimeDatabaseMimeTypeForFileArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::mime_database::MimeDatabase) -> ::mime_type::MimeType {
      let file_name = self;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName(original_self as *const ::mime_database::MimeDatabase, file_name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MimeDatabaseMimeTypeForFileArgs<'largs> for (&'largs ::string::String, ::mime_database::MatchMode) {
    fn exec(self, original_self: &'largs ::mime_database::MimeDatabase) -> ::mime_type::MimeType {
      let file_name = self.0;
      let mode = self.1;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMimeDatabase_mimeTypeForFile_to_output_fileName_mode(original_self as *const ::mime_database::MimeDatabase, file_name as *const ::string::String, mode, &mut object);
        }
        object
      }
    }
  }
}
