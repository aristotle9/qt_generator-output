/// C++ type: <span style='color: green;'>```QFileSystemWatcher```</span>
#[repr(C)]
pub struct FileSystemWatcher(u8);

impl FileSystemWatcher {
  /// C++ method: <span style='color: green;'>```bool QFileSystemWatcher::addPath(const QString& file)```</span>
  ///
  ///
  pub fn add_path(&mut self, file: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFileSystemWatcher_addPath(self as *mut ::file_system_watcher::FileSystemWatcher,
                                                  file as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSystemWatcher::addPaths(const QStringList& files)```</span>
  ///
  ///
  pub fn add_paths(&mut self, files: &::string_list::StringList) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSystemWatcher_addPaths_to_output(self as *mut ::file_system_watcher::FileSystemWatcher,
                                                               files as *const ::string_list::StringList,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSystemWatcher::directories() const```</span>
  ///
  ///
  pub fn directories(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSystemWatcher_directories_to_output(self as *const ::file_system_watcher::FileSystemWatcher, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSystemWatcher::files() const```</span>
  ///
  ///
  pub fn files(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSystemWatcher_files_to_output(self as *const ::file_system_watcher::FileSystemWatcher,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFileSystemWatcher::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QFileSystemWatcher_metaObject(self as *const ::file_system_watcher::FileSystemWatcher) }
  }

  /// C++ method: <span style='color: green;'>```QFileSystemWatcher::QFileSystemWatcher```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemWatcher::QFileSystemWatcher()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string_list::StringList) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemWatcher::QFileSystemWatcher(const QStringList& paths)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>
    where Args: overloading::FileSystemWatcherNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFileSystemWatcher::QFileSystemWatcher```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemWatcher::QFileSystemWatcher(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string_list::StringList, *mut ::object::Object)) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFileSystemWatcher::QFileSystemWatcher(const QStringList& paths, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>
    where Args: overloading::FileSystemWatcherNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QFileSystemWatcher::removePath(const QString& file)```</span>
  ///
  ///
  pub fn remove_path(&mut self, file: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFileSystemWatcher_removePath(self as *mut ::file_system_watcher::FileSystemWatcher,
                                                     file as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QFileSystemWatcher::removePaths(const QStringList& files)```</span>
  ///
  ///
  pub fn remove_paths(&mut self, files: &::string_list::StringList) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileSystemWatcher_removePaths_to_output(self as *mut ::file_system_watcher::FileSystemWatcher, files as *const ::string_list::StringList, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileSystemWatcher::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileSystemWatcher_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileSystemWatcher::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileSystemWatcher_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file_system_watcher::FileSystemWatcher {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFileSystemWatcher_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FileSystemWatcher`.
  pub struct Signals<'a>(&'a ::file_system_watcher::FileSystemWatcher);
  /// Represents a built-in Qt signal `QFileSystemWatcher::objectNameChanged`.
  ///
  /// An object of this type can be created from `FileSystemWatcher` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemWatcher` object.
  pub struct ObjectNameChanged<'a>(&'a ::file_system_watcher::FileSystemWatcher);
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
  /// Represents a built-in Qt signal `QFileSystemWatcher::directoryChanged`.
  ///
  /// An object of this type can be created from `FileSystemWatcher` with `object.signals().directory_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemWatcher` object.
  pub struct DirectoryChanged<'a>(&'a ::file_system_watcher::FileSystemWatcher);
  impl<'a> ::connection::Receiver for DirectoryChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2directoryChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for DirectoryChanged<'a> {}
  /// Represents a built-in Qt signal `QFileSystemWatcher::fileChanged`.
  ///
  /// An object of this type can be created from `FileSystemWatcher` with `object.signals().file_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileSystemWatcher` object.
  pub struct FileChanged<'a>(&'a ::file_system_watcher::FileSystemWatcher);
  impl<'a> ::connection::Receiver for FileChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fileChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for FileChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFileSystemWatcher::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemWatcher::directoryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn directory_changed(&self) -> DirectoryChanged {
      DirectoryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileSystemWatcher::fileChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn file_changed(&self) -> FileChanged {
      FileChanged(self.0)
    }
  }
  impl ::file_system_watcher::FileSystemWatcher {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::file_system_watcher::FileSystemWatcher> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_system_watcher::FileSystemWatcher> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QFileSystemWatcher_G_dynamic_cast_QFileSystemWatcher_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_system_watcher::FileSystemWatcher> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_G_dynamic_cast_QFileSystemWatcher_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::file_system_watcher::FileSystemWatcher {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(self as *mut ::file_system_watcher::FileSystemWatcher) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(self as *const ::file_system_watcher::FileSystemWatcher as *mut ::file_system_watcher::FileSystemWatcher) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_system_watcher::FileSystemWatcher> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_system_watcher::FileSystemWatcher {
    let ffi_result =
      ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QFileSystemWatcher_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_system_watcher::FileSystemWatcher {
    let ffi_result = ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QFileSystemWatcher_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_system_watcher::FileSystemWatcher {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(self as *const ::file_system_watcher::FileSystemWatcher as *mut ::file_system_watcher::FileSystemWatcher) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_system_watcher::FileSystemWatcher {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_G_static_cast_QObject_ptr(self as *mut ::file_system_watcher::FileSystemWatcher) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileSystemWatcher::new](../struct.FileSystemWatcher.html#method.new) method.
  pub trait FileSystemWatcherNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>;
  }
  impl FileSystemWatcherNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QFileSystemWatcher_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> FileSystemWatcherNewArgs for &'a ::string_list::StringList {
    fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher> {
      let paths = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QFileSystemWatcher_new_paths(paths as *const ::string_list::StringList) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [FileSystemWatcher::new_unsafe](../struct.FileSystemWatcher.html#method.new_unsafe) method.
  pub trait FileSystemWatcherNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher>;
  }
  impl FileSystemWatcherNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QFileSystemWatcher_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> FileSystemWatcherNewUnsafeArgs for (&'a ::string_list::StringList, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file_system_watcher::FileSystemWatcher> {
      let paths = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QFileSystemWatcher_new_paths_parent(paths as *const ::string_list::StringList, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
