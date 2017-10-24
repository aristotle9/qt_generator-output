/// C++ type: <span style='color: green;'>```QLibraryInfo```</span>
#[repr(C)]
pub struct LibraryInfo(u8);

impl LibraryInfo {
  /// C++ method: <span style='color: green;'>```static const char* QLibraryInfo::build()```</span>
  ///
  ///
  pub fn build() -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QLibraryInfo_build() }
  }

  /// C++ method: <span style='color: green;'>```static QDate QLibraryInfo::buildDate()```</span>
  ///
  ///
  pub fn build_date() -> ::date::Date {
    {
      let mut object: ::date::Date = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_buildDate_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QLibraryInfo::isDebugBuild()```</span>
  ///
  ///
  pub fn is_debug_build() -> bool {
    unsafe { ::ffi::qt_core_c_QLibraryInfo_isDebugBuild() }
  }

  /// C++ method: <span style='color: green;'>```static QString QLibraryInfo::licensedProducts()```</span>
  ///
  ///
  pub fn licensed_products() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_licensedProducts_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLibraryInfo::licensee()```</span>
  ///
  ///
  pub fn licensee() -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_licensee_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QLibraryInfo::location(QLibraryInfo::LibraryLocation arg1)```</span>
  ///
  ///
  pub fn location(arg1: ::library_info::LibraryLocation) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_location_to_output(arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QLibraryInfo::platformPluginArguments(const QString& platformName)```</span>
  ///
  ///
  pub fn platform_plugin_arguments(platform_name: &::string::String) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_platformPluginArguments_to_output(platform_name as *const ::string::String,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QVersionNumber QLibraryInfo::version()```</span>
  ///
  ///
  pub fn version() -> ::version_number::VersionNumber {
    {
      let mut object: ::version_number::VersionNumber =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLibraryInfo_version_to_output(&mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::library_info::LibraryInfo {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QLibraryInfo_delete
  }
}

/// C++ type: <span style='color: green;'>```QLibraryInfo::LibraryLocation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LibraryLocation {
  /// C++ enum variant: <span style='color: green;'>```PrefixPath = 0```</span>
  Prefix = 0,
  /// C++ enum variant: <span style='color: green;'>```DocumentationPath = 1```</span>
  Documentation = 1,
  /// C++ enum variant: <span style='color: green;'>```HeadersPath = 2```</span>
  Headers = 2,
  /// C++ enum variant: <span style='color: green;'>```LibrariesPath = 3```</span>
  Libraries = 3,
  /// C++ enum variant: <span style='color: green;'>```LibraryExecutablesPath = 4```</span>
  LibraryExecutables = 4,
  /// C++ enum variant: <span style='color: green;'>```BinariesPath = 5```</span>
  Binaries = 5,
  /// C++ enum variant: <span style='color: green;'>```PluginsPath = 6```</span>
  Plugins = 6,
  /// C++ enum variant: <span style='color: green;'>```ImportsPath = 7```</span>
  Imports = 7,
  /// C++ enum variant: <span style='color: green;'>```Qml2ImportsPath = 8```</span>
  Qml2Imports = 8,
  /// C++ enum variant: <span style='color: green;'>```ArchDataPath = 9```</span>
  ArchData = 9,
  /// C++ enum variant: <span style='color: green;'>```DataPath = 10```</span>
  Data = 10,
  /// C++ enum variant: <span style='color: green;'>```TranslationsPath = 11```</span>
  Translations = 11,
  /// C++ enum variant: <span style='color: green;'>```ExamplesPath = 12```</span>
  Examples = 12,
  /// C++ enum variant: <span style='color: green;'>```TestsPath = 13```</span>
  Tests = 13,
  /// C++ enum variant: <span style='color: green;'>```SettingsPath = 100```</span>
  Settings = 100,
}
