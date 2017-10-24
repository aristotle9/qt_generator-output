/// C++ type: <span style='color: green;'>```QSettings::Format```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Format {
  /// C++ enum variant: <span style='color: green;'>```NativeFormat = 0```</span>
  NativeFormat = 0,
  /// C++ enum variant: <span style='color: green;'>```IniFormat = 1```</span>
  IniFormat = 1,
  /// C++ enum variant: <span style='color: green;'>```InvalidFormat = 16```</span>
  InvalidFormat = 16,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat1 = 17```</span>
  CustomFormat1 = 17,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat2 = 18```</span>
  CustomFormat2 = 18,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat3 = 19```</span>
  CustomFormat3 = 19,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat4 = 20```</span>
  CustomFormat4 = 20,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat5 = 21```</span>
  CustomFormat5 = 21,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat6 = 22```</span>
  CustomFormat6 = 22,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat7 = 23```</span>
  CustomFormat7 = 23,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat8 = 24```</span>
  CustomFormat8 = 24,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat9 = 25```</span>
  CustomFormat9 = 25,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat10 = 26```</span>
  CustomFormat10 = 26,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat11 = 27```</span>
  CustomFormat11 = 27,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat12 = 28```</span>
  CustomFormat12 = 28,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat13 = 29```</span>
  CustomFormat13 = 29,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat14 = 30```</span>
  CustomFormat14 = 30,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat15 = 31```</span>
  CustomFormat15 = 31,
  /// C++ enum variant: <span style='color: green;'>```CustomFormat16 = 32```</span>
  CustomFormat16 = 32,
}

/// C++ type: <span style='color: green;'>```QSettings::Scope```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Scope {
  /// C++ enum variant: <span style='color: green;'>```UserScope = 0```</span>
  User = 0,
  /// C++ enum variant: <span style='color: green;'>```SystemScope = 1```</span>
  System = 1,
}

/// C++ type: <span style='color: green;'>```QSettings```</span>
#[repr(C)]
pub struct Settings(u8);

impl Settings {
  /// C++ method: <span style='color: green;'>```QStringList QSettings::allKeys() const```</span>
  ///
  ///
  pub fn all_keys(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_allKeys_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QSettings::applicationName() const```</span>
  ///
  ///
  pub fn application_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_applicationName_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::beginGroup(const QString& prefix)```</span>
  ///
  ///
  pub fn begin_group(&mut self, prefix: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QSettings_beginGroup(self as *mut ::settings::Settings,
                                            prefix as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QSettings::beginReadArray(const QString& prefix)```</span>
  ///
  ///
  pub fn begin_read_array(&mut self, prefix: &::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QSettings_beginReadArray(self as *mut ::settings::Settings,
                                                prefix as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSettings::beginWriteArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn begin_write_array(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSettings::beginWriteArray(const QString& prefix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn begin_write_array(&mut self, (&::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSettings::beginWriteArray(const QString& prefix, int size = ?)```</span>
  ///
  ///
  pub fn begin_write_array<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SettingsBeginWriteArrayArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QSettings::childGroups() const```</span>
  ///
  ///
  pub fn child_groups(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_childGroups_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QSettings::childKeys() const```</span>
  ///
  ///
  pub fn child_keys(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_childKeys_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QSettings_clear(self as *mut ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```bool QSettings::contains(const QString& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSettings_contains(self as *const ::settings::Settings,
                                          key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QSettings::Format QSettings::defaultFormat()```</span>
  ///
  ///
  pub fn default_format() -> ::settings::Format {
    unsafe { ::ffi::qt_core_c_QSettings_defaultFormat() }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::endArray()```</span>
  ///
  ///
  pub fn end_array(&mut self) {
    unsafe { ::ffi::qt_core_c_QSettings_endArray(self as *mut ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::endGroup()```</span>
  ///
  ///
  pub fn end_group(&mut self) {
    unsafe { ::ffi::qt_core_c_QSettings_endGroup(self as *mut ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```bool QSettings::fallbacksEnabled() const```</span>
  ///
  ///
  pub fn fallbacks_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSettings_fallbacksEnabled(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```QString QSettings::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_fileName_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSettings::Format QSettings::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::settings::Format {
    unsafe { ::ffi::qt_core_c_QSettings_format(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```QString QSettings::group() const```</span>
  ///
  ///
  pub fn group(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_group_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec* QSettings::iniCodec() const```</span>
  ///
  ///
  pub fn ini_codec(&self) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QSettings_iniCodec(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```bool QSettings::isWritable() const```</span>
  ///
  ///
  pub fn is_writable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSettings_isWritable(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSettings::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSettings_metaObject(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```QSettings::QSettings```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::settings::Format, ::settings::Scope, &::string::String)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Format format, QSettings::Scope scope, const QString& organization)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::settings::Format, ::settings::Scope, &::string::String, &::string::String)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Format format, QSettings::Scope scope, const QString& organization, const QString& application = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::settings::Scope, &::string::String)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Scope scope, const QString& organization)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::settings::Scope, &::string::String, &::string::String)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Scope scope, const QString& organization, const QString& application = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((&::string::String, ::settings::Format)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(const QString& fileName, QSettings::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(const QString& organization)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((&::string::String, &::string::String)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(const QString& organization, const QString& application = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::settings::Settings>
    where Args: overloading::SettingsNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSettings::QSettings```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::settings::Format, ::settings::Scope, &::string::String, &::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Format format, QSettings::Scope scope, const QString& organization, const QString& application = ?, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::settings::Scope, &::string::String, &::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(QSettings::Scope scope, const QString& organization, const QString& application = ?, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, ::settings::Format, *mut ::object::Object)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(const QString& fileName, QSettings::Format format, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, &::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::settings::Settings>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSettings::QSettings(const QString& organization, const QString& application = ?, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::settings::Settings>
    where Args: overloading::SettingsNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QSettings::organizationName() const```</span>
  ///
  ///
  pub fn organization_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSettings_organizationName_to_output(self as *const ::settings::Settings, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::remove(const QString& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QSettings_remove(self as *mut ::settings::Settings,
                                        key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSettings::Scope QSettings::scope() const```</span>
  ///
  ///
  pub fn scope(&self) -> ::settings::Scope {
    unsafe { ::ffi::qt_core_c_QSettings_scope(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::setArrayIndex(int i)```</span>
  ///
  ///
  pub fn set_array_index(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QSettings_setArrayIndex(self as *mut ::settings::Settings, i) }
  }

  /// C++ method: <span style='color: green;'>```static void QSettings::setDefaultFormat(QSettings::Format format)```</span>
  ///
  ///
  pub fn set_default_format(format: ::settings::Format) {
    unsafe { ::ffi::qt_core_c_QSettings_setDefaultFormat(format) }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::setFallbacksEnabled(bool b)```</span>
  ///
  ///
  pub fn set_fallbacks_enabled(&mut self, b: bool) {
    unsafe { ::ffi::qt_core_c_QSettings_setFallbacksEnabled(self as *mut ::settings::Settings, b) }
  }

  /// C++ method: <span style='color: green;'>```QSettings::setIniCodec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_ini_codec(&mut self, *mut ::text_codec::TextCodec) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSettings::setIniCodec(QTextCodec* codec)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_ini_codec(&mut self, *const ::libc::c_char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSettings::setIniCodec(const char* codecName)```</span>
  ///
  ///
  pub unsafe fn set_ini_codec<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SettingsSetIniCodecArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static void QSettings::setPath(QSettings::Format format, QSettings::Scope scope, const QString& path)```</span>
  ///
  ///
  pub fn set_path(format: ::settings::Format, scope: ::settings::Scope, path: &::string::String) {
    unsafe { ::ffi::qt_core_c_QSettings_setPath(format, scope, path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QSettings::setSystemIniPath(const QString& dir)```</span>
  ///
  ///
  pub fn set_system_ini_path(dir: &::string::String) {
    unsafe { ::ffi::qt_core_c_QSettings_setSystemIniPath(dir as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```static void QSettings::setUserIniPath(const QString& dir)```</span>
  ///
  ///
  pub fn set_user_ini_path(dir: &::string::String) {
    unsafe { ::ffi::qt_core_c_QSettings_setUserIniPath(dir as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::setValue(const QString& key, const QVariant& value)```</span>
  ///
  ///
  pub fn set_value(&mut self, key: &::string::String, value: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QSettings_setValue(self as *mut ::settings::Settings,
                                          key as *const ::string::String,
                                          value as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QSettings::Status QSettings::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::settings::Status {
    unsafe { ::ffi::qt_core_c_QSettings_status(self as *const ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```void QSettings::sync()```</span>
  ///
  ///
  pub fn sync(&mut self) {
    unsafe { ::ffi::qt_core_c_QSettings_sync(self as *mut ::settings::Settings) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSettings::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSettings_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSettings::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSettings_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSettings::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QSettings::value(const QString& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::variant::Variant)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QSettings::value(const QString& key, const QVariant& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::SettingsValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::settings::Settings {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSettings_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Settings`.
  pub struct Signals<'a>(&'a ::settings::Settings);
  /// Represents a built-in Qt signal `QSettings::objectNameChanged`.
  ///
  /// An object of this type can be created from `Settings` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Settings` object.
  pub struct ObjectNameChanged<'a>(&'a ::settings::Settings);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSettings::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::settings::Settings {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QSettings::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```AccessError = 1```</span>
  Access = 1,
  /// C++ enum variant: <span style='color: green;'>```FormatError = 2```</span>
  Format = 2,
}

impl ::cpp_utils::DynamicCast<::settings::Settings> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::settings::Settings> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_dynamic_cast_QSettings_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::settings::Settings> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_dynamic_cast_QSettings_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::settings::Settings {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_static_cast_QObject_ptr(self as *mut ::settings::Settings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_static_cast_QObject_ptr(self as *const ::settings::Settings as *mut ::settings::Settings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::settings::Settings> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::settings::Settings {
    let ffi_result = ::ffi::qt_core_c_QSettings_G_static_cast_QSettings_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::settings::Settings {
    let ffi_result = ::ffi::qt_core_c_QSettings_G_static_cast_QSettings_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::settings::Settings {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_static_cast_QObject_ptr(self as *const ::settings::Settings as *mut ::settings::Settings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::settings::Settings {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_G_static_cast_QObject_ptr(self as *mut ::settings::Settings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Settings::begin_write_array](../struct.Settings.html#method.begin_write_array) method.
  pub trait SettingsBeginWriteArrayArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::settings::Settings) -> ();
  }
  impl<'largs> SettingsBeginWriteArrayArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::settings::Settings) -> () {
      let prefix = self;
      unsafe {
        ::ffi::qt_core_c_QSettings_beginWriteArray_prefix(original_self as *mut ::settings::Settings,
                                                          prefix as *const ::string::String)
      }
    }
  }
  impl<'largs> SettingsBeginWriteArrayArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::settings::Settings) -> () {
      let prefix = self.0;
      let size = self.1;
      unsafe {
        ::ffi::qt_core_c_QSettings_beginWriteArray_prefix_size(original_self as *mut ::settings::Settings,
                                                               prefix as *const ::string::String,
                                                               size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Settings::new](../struct.Settings.html#method.new) method.
  pub trait SettingsNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings>;
  }
  impl<'a> SettingsNewArgs for (&'a ::string::String, ::settings::Format) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let file_name = self.0;
      let format = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QSettings_new_fileName_format(file_name as *const ::string::String, format) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for (::settings::Format, ::settings::Scope, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let format = self.0;
      let scope = self.1;
      let organization = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSettings_new_format_scope_organization(format,
                                                                   scope,
                                                                   organization as *const ::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for (::settings::Format, ::settings::Scope, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let format = self.0;
      let scope = self.1;
      let organization = self.2;
      let application = self.3;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSettings_new_format_scope_organization_application(format, scope, organization as *const ::string::String, application as *const ::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl SettingsNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let organization = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QSettings_new_organization(organization as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let organization = self.0;
      let application = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSettings_new_organization_application(organization as *const ::string::String,
                                                                  application as *const ::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for (::settings::Scope, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let scope = self.0;
      let organization = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QSettings_new_scope_organization(scope, organization as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> SettingsNewArgs for (::settings::Scope, &'a ::string::String, &'a ::string::String) {
    fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let scope = self.0;
      let organization = self.1;
      let application = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QSettings_new_scope_organization_application(scope,
                                                                        organization as *const ::string::String,
                                                                        application as *const ::string::String)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Settings::new_unsafe](../struct.Settings.html#method.new_unsafe) method.
  pub trait SettingsNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings>;
  }
  impl<'a> SettingsNewUnsafeArgs for (&'a ::string::String, ::settings::Format, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let file_name = self.0;
      let format = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QSettings_new_fileName_format_parent(file_name as *const ::string::String, format, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> SettingsNewUnsafeArgs
    for (::settings::Format, ::settings::Scope, &'a ::string::String, &'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let format = self.0;
      let scope = self.1;
      let organization = self.2;
      let application = self.3;
      let parent = self.4;
      let ffi_result = ::ffi::qt_core_c_QSettings_new_format_scope_organization_application_parent(format, scope, organization as *const ::string::String, application as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> SettingsNewUnsafeArgs for (&'a ::string::String, &'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let organization = self.0;
      let application = self.1;
      let parent = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QSettings_new_organization_application_parent(organization as *const ::string::String,
                                                                       application as *const ::string::String,
                                                                       parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SettingsNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QSettings_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> SettingsNewUnsafeArgs
    for (::settings::Scope, &'a ::string::String, &'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::settings::Settings> {
      let scope = self.0;
      let organization = self.1;
      let application = self.2;
      let parent = self.3;
      let ffi_result =
        ::ffi::qt_core_c_QSettings_new_scope_organization_application_parent(scope,
                                                                             organization as *const ::string::String,
                                                                             application as *const ::string::String,
                                                                             parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Settings::set_ini_codec](../struct.Settings.html#method.set_ini_codec) method.
  pub trait SettingsSetIniCodecArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::settings::Settings) -> ();
  }
  impl<'largs> SettingsSetIniCodecArgs<'largs> for *mut ::text_codec::TextCodec {
    unsafe fn exec(self, original_self: &'largs mut ::settings::Settings) -> () {
      let codec = self;
      ::ffi::qt_core_c_QSettings_setIniCodec_codec(original_self as *mut ::settings::Settings, codec)
    }
  }
  impl<'largs> SettingsSetIniCodecArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::settings::Settings) -> () {
      let codec_name = self;
      ::ffi::qt_core_c_QSettings_setIniCodec_codecName(original_self as *mut ::settings::Settings, codec_name)
    }
  }
  /// This trait represents a set of arguments accepted by [Settings::value](../struct.Settings.html#method.value) method.
  pub trait SettingsValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::settings::Settings) -> ::variant::Variant;
  }
  impl<'largs> SettingsValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::settings::Settings) -> ::variant::Variant {
      let key = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSettings_value_to_output_key(original_self as *const ::settings::Settings,
                                                         key as *const ::string::String,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> SettingsValueArgs<'largs> for (&'largs ::string::String, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs ::settings::Settings) -> ::variant::Variant {
      let key = self.0;
      let default_value = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSettings_value_to_output_key_defaultValue(original_self as *const ::settings::Settings,
                                                                      key as *const ::string::String,
                                                                      default_value as *const ::variant::Variant,
                                                                      &mut object);
        }
        object
      }
    }
  }
}
