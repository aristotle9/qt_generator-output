/// C++ type: <span style='color: green;'>```QStandardPaths::LocateOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LocateOption {
  /// C++ enum variant: <span style='color: green;'>```LocateFile = 0```</span>
  File = 0,
  /// C++ enum variant: <span style='color: green;'>```LocateDirectory = 1```</span>
  Directory = 1,
}

impl ::flags::FlaggableEnum for LocateOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "LocateOption"
  }
}

/// C++ type: <span style='color: green;'>```QStandardPaths::StandardLocation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum StandardLocation {
  /// C++ enum variant: <span style='color: green;'>```DesktopLocation = 0```</span>
  Desktop = 0,
  /// C++ enum variant: <span style='color: green;'>```DocumentsLocation = 1```</span>
  Documents = 1,
  /// C++ enum variant: <span style='color: green;'>```FontsLocation = 2```</span>
  Fonts = 2,
  /// C++ enum variant: <span style='color: green;'>```ApplicationsLocation = 3```</span>
  Applications = 3,
  /// C++ enum variant: <span style='color: green;'>```MusicLocation = 4```</span>
  Music = 4,
  /// C++ enum variant: <span style='color: green;'>```MoviesLocation = 5```</span>
  Movies = 5,
  /// C++ enum variant: <span style='color: green;'>```PicturesLocation = 6```</span>
  Pictures = 6,
  /// C++ enum variant: <span style='color: green;'>```TempLocation = 7```</span>
  Temp = 7,
  /// C++ enum variant: <span style='color: green;'>```HomeLocation = 8```</span>
  Home = 8,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```DataLocation = 9```</span>
  /// - <span style='color: green;'>```AppLocalDataLocation = 9```</span>
  ///
  Data = 9,
  /// C++ enum variant: <span style='color: green;'>```CacheLocation = 10```</span>
  Cache = 10,
  /// C++ enum variant: <span style='color: green;'>```GenericDataLocation = 11```</span>
  GenericData = 11,
  /// C++ enum variant: <span style='color: green;'>```RuntimeLocation = 12```</span>
  Runtime = 12,
  /// C++ enum variant: <span style='color: green;'>```ConfigLocation = 13```</span>
  Config = 13,
  /// C++ enum variant: <span style='color: green;'>```DownloadLocation = 14```</span>
  Download = 14,
  /// C++ enum variant: <span style='color: green;'>```GenericCacheLocation = 15```</span>
  GenericCache = 15,
  /// C++ enum variant: <span style='color: green;'>```GenericConfigLocation = 16```</span>
  GenericConfig = 16,
  /// C++ enum variant: <span style='color: green;'>```AppDataLocation = 17```</span>
  AppData = 17,
  /// C++ enum variant: <span style='color: green;'>```AppConfigLocation = 18```</span>
  AppConfig = 18,
}

/// C++ type: <span style='color: green;'>```QStandardPaths```</span>
#[repr(C)]
pub struct StandardPaths(u8);

impl StandardPaths {
  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::displayName(QStandardPaths::StandardLocation type)```</span>
  ///
  ///
  pub fn display_name(type_: ::standard_paths::StandardLocation) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStandardPaths_displayName_to_output(type_, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QStandardPaths::enableTestMode(bool testMode)```</span>
  ///
  ///
  pub fn enable_test_mode(test_mode: bool) {
    unsafe { ::ffi::qt_core_c_QStandardPaths_enableTestMode(test_mode) }
  }

  /// C++ method: <span style='color: green;'>```QStandardPaths::findExecutable```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find_executable(&::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::findExecutable(const QString& executableName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find_executable((&::string::String, &::string_list::StringList)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::findExecutable(const QString& executableName, const QStringList& paths = ?)```</span>
  ///
  ///
  pub fn find_executable<Args>(args: Args) -> ::string::String
    where Args: overloading::StandardPathsFindExecutableArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static bool QStandardPaths::isTestModeEnabled()```</span>
  ///
  ///
  pub fn is_test_mode_enabled() -> bool {
    unsafe { ::ffi::qt_core_c_QStandardPaths_isTestModeEnabled() }
  }

  /// C++ method: <span style='color: green;'>```QStandardPaths::locate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locate((::standard_paths::StandardLocation, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::locate(QStandardPaths::StandardLocation type, const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locate((::standard_paths::StandardLocation, &::string::String, ::flags::Flags<::standard_paths::LocateOption>)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::locate(QStandardPaths::StandardLocation type, const QString& fileName, QFlags<QStandardPaths::LocateOption> options = ?)```</span>
  ///
  ///
  pub fn locate<Args>(args: Args) -> ::string::String
    where Args: overloading::StandardPathsLocateArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStandardPaths::locateAll```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn locate_all((::standard_paths::StandardLocation, &::string::String)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QStandardPaths::locateAll(QStandardPaths::StandardLocation type, const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn locate_all((::standard_paths::StandardLocation, &::string::String, ::flags::Flags<::standard_paths::LocateOption>)) -> ::string_list::StringList```<br>
  /// C++ method: <span style='color: green;'>```static QStringList QStandardPaths::locateAll(QStandardPaths::StandardLocation type, const QString& fileName, QFlags<QStandardPaths::LocateOption> options = ?)```</span>
  ///
  ///
  pub fn locate_all<Args>(args: Args) -> ::string_list::StringList
    where Args: overloading::StandardPathsLocateAllArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static void QStandardPaths::setTestModeEnabled(bool testMode)```</span>
  ///
  ///
  pub fn set_test_mode_enabled(test_mode: bool) {
    unsafe { ::ffi::qt_core_c_QStandardPaths_setTestModeEnabled(test_mode) }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QStandardPaths::standardLocations(QStandardPaths::StandardLocation type)```</span>
  ///
  ///
  pub fn standard_locations(type_: ::standard_paths::StandardLocation) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStandardPaths_standardLocations_to_output(type_, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QStandardPaths::writableLocation(QStandardPaths::StandardLocation type)```</span>
  ///
  ///
  pub fn writable_location(type_: ::standard_paths::StandardLocation) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStandardPaths_writableLocation_to_output(type_, &mut object);
      }
      object
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StandardPaths::find_executable](../struct.StandardPaths.html#method.find_executable) method.
  pub trait StandardPathsFindExecutableArgs {
    fn exec(self) -> ::string::String;
  }
  impl<'a> StandardPathsFindExecutableArgs for &'a ::string::String {
    fn exec(self) -> ::string::String {
      let executable_name = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_findExecutable_to_output_executableName(executable_name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StandardPathsFindExecutableArgs for (&'a ::string::String, &'a ::string_list::StringList) {
    fn exec(self) -> ::string::String {
      let executable_name = self.0;
      let paths = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_findExecutable_to_output_executableName_paths(executable_name as *const ::string::String, paths as *const ::string_list::StringList, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardPaths::locate_all](../struct.StandardPaths.html#method.locate_all) method.
  pub trait StandardPathsLocateAllArgs {
    fn exec(self) -> ::string_list::StringList;
  }
  impl<'a> StandardPathsLocateAllArgs for (::standard_paths::StandardLocation, &'a ::string::String) {
    fn exec(self) -> ::string_list::StringList {
      let type_ = self.0;
      let file_name = self.1;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_locateAll_to_output_type_fileName(type_,
                                                                            file_name as *const ::string::String,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> StandardPathsLocateAllArgs
    for (::standard_paths::StandardLocation, &'a ::string::String, ::flags::Flags<::standard_paths::LocateOption>) {
    fn exec(self) -> ::string_list::StringList {
      let type_ = self.0;
      let file_name = self.1;
      let options = self.2;
      {
        let mut object: ::string_list::StringList =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_locateAll_to_output_type_fileName_options(type_, file_name as *const ::string::String, options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StandardPaths::locate](../struct.StandardPaths.html#method.locate) method.
  pub trait StandardPathsLocateArgs {
    fn exec(self) -> ::string::String;
  }
  impl<'a> StandardPathsLocateArgs for (::standard_paths::StandardLocation, &'a ::string::String) {
    fn exec(self) -> ::string::String {
      let type_ = self.0;
      let file_name = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_locate_to_output_type_fileName(type_,
                                                                         file_name as *const ::string::String,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> StandardPathsLocateArgs
    for (::standard_paths::StandardLocation, &'a ::string::String, ::flags::Flags<::standard_paths::LocateOption>) {
    fn exec(self) -> ::string::String {
      let type_ = self.0;
      let file_name = self.1;
      let options = self.2;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStandardPaths_locate_to_output_type_fileName_options(type_, file_name as *const ::string::String, options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
}
