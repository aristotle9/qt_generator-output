/// C++ type: <span style='color: green;'>```QPluginLoader```</span>
#[repr(C)]
pub struct PluginLoader(u8);

impl PluginLoader {
  /// C++ method: <span style='color: green;'>```QString QPluginLoader::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPluginLoader_errorString_to_output(self as *const ::plugin_loader::PluginLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QPluginLoader::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPluginLoader_fileName_to_output(self as *const ::plugin_loader::PluginLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QObject* QPluginLoader::instance()```</span>
  ///
  ///
  pub fn instance(&mut self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QPluginLoader_instance(self as *mut ::plugin_loader::PluginLoader) }
  }

  /// C++ method: <span style='color: green;'>```bool QPluginLoader::isLoaded() const```</span>
  ///
  ///
  pub fn is_loaded(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QPluginLoader_isLoaded(self as *const ::plugin_loader::PluginLoader) }
  }

  /// C++ method: <span style='color: green;'>```bool QPluginLoader::load()```</span>
  ///
  ///
  pub fn load(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QPluginLoader_load(self as *mut ::plugin_loader::PluginLoader) }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject QPluginLoader::metaData() const```</span>
  ///
  ///
  pub fn meta_data(&self) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPluginLoader_metaData_to_output(self as *const ::plugin_loader::PluginLoader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QPluginLoader::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QPluginLoader_metaObject(self as *const ::plugin_loader::PluginLoader) }
  }

  /// C++ method: <span style='color: green;'>```QPluginLoader::QPluginLoader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPluginLoader::QPluginLoader()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPluginLoader::QPluginLoader(const QString& fileName)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>
    where Args: overloading::PluginLoaderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPluginLoader::QPluginLoader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPluginLoader::QPluginLoader(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPluginLoader::QPluginLoader(const QString& fileName, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>
    where Args: overloading::PluginLoaderNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QPluginLoader::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QPluginLoader_setFileName(self as *mut ::plugin_loader::PluginLoader,
                                                 file_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QObject*> QPluginLoader::staticInstances()```</span>
  ///
  ///
  pub fn static_instances() -> ::list::ListObjectMutPtr {
    {
      let mut object: ::list::ListObjectMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPluginLoader_staticInstances_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QVector<QStaticPlugin> QPluginLoader::staticPlugins()```</span>
  ///
  ///
  pub fn static_plugins() -> ::vector::VectorStaticPlugin {
    {
      let mut object: ::vector::VectorStaticPlugin =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPluginLoader_staticPlugins_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPluginLoader::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPluginLoader_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QPluginLoader::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QPluginLoader_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPluginLoader::unload()```</span>
  ///
  ///
  pub fn unload(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QPluginLoader_unload(self as *mut ::plugin_loader::PluginLoader) }
  }
}

impl ::cpp_utils::CppDeletable for ::plugin_loader::PluginLoader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QPluginLoader_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PluginLoader`.
  pub struct Signals<'a>(&'a ::plugin_loader::PluginLoader);
  /// Represents a built-in Qt signal `QPluginLoader::objectNameChanged`.
  ///
  /// An object of this type can be created from `PluginLoader` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PluginLoader` object.
  pub struct ObjectNameChanged<'a>(&'a ::plugin_loader::PluginLoader);
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
    /// Returns an object representing a built-in Qt signal `QPluginLoader::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::plugin_loader::PluginLoader {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::plugin_loader::PluginLoader> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::plugin_loader::PluginLoader> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QPluginLoader_G_dynamic_cast_QPluginLoader_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::plugin_loader::PluginLoader> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPluginLoader_G_dynamic_cast_QPluginLoader_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::plugin_loader::PluginLoader {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(self as *mut ::plugin_loader::PluginLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(self as *const ::plugin_loader::PluginLoader as *mut ::plugin_loader::PluginLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plugin_loader::PluginLoader> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plugin_loader::PluginLoader {
    let ffi_result = ::ffi::qt_core_c_QPluginLoader_G_static_cast_QPluginLoader_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plugin_loader::PluginLoader {
    let ffi_result = ::ffi::qt_core_c_QPluginLoader_G_static_cast_QPluginLoader_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::plugin_loader::PluginLoader {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(self as *const ::plugin_loader::PluginLoader as *mut ::plugin_loader::PluginLoader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::plugin_loader::PluginLoader {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QPluginLoader_G_static_cast_QObject_ptr(self as *mut ::plugin_loader::PluginLoader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PluginLoader::new](../struct.PluginLoader.html#method.new) method.
  pub trait PluginLoaderNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>;
  }
  impl<'a> PluginLoaderNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader> {
      let file_name = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QPluginLoader_new_fileName(file_name as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl PluginLoaderNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QPluginLoader_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [PluginLoader::new_unsafe](../struct.PluginLoader.html#method.new_unsafe) method.
  pub trait PluginLoaderNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader>;
  }
  impl<'a> PluginLoaderNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader> {
      let file_name = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QPluginLoader_new_fileName_parent(file_name as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl PluginLoaderNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::plugin_loader::PluginLoader> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QPluginLoader_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
