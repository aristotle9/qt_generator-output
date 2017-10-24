/// C++ type: <span style='color: green;'>```QResource```</span>
#[repr(C)]
pub struct Resource([u8; ::type_sizes::QT_CORE_RESOURCE_RESOURCE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Resource {
  unsafe fn new_uninitialized() -> Resource {
    Resource(::std::mem::uninitialized())
  }
}

impl Resource {
  /// C++ method: <span style='color: green;'>```QString QResource::absoluteFilePath() const```</span>
  ///
  ///
  pub fn absolute_file_path(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QResource_absoluteFilePath_to_output(self as *const ::resource::Resource, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QResource::addSearchPath(const QString& path)```</span>
  ///
  ///
  pub fn add_search_path(path: &::string::String) {
    unsafe { ::ffi::qt_core_c_QResource_addSearchPath(path as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned char* QResource::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_uchar {
    unsafe { ::ffi::qt_core_c_QResource_data(self as *const ::resource::Resource) }
  }

  /// C++ method: <span style='color: green;'>```QString QResource::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QResource_fileName_to_output(self as *const ::resource::Resource, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QResource::isCompressed() const```</span>
  ///
  ///
  pub fn is_compressed(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QResource_isCompressed(self as *const ::resource::Resource) }
  }

  /// C++ method: <span style='color: green;'>```bool QResource::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QResource_isValid(self as *const ::resource::Resource) }
  }

  /// C++ method: <span style='color: green;'>```QDateTime QResource::lastModified() const```</span>
  ///
  ///
  pub fn last_modified(&self) -> ::date_time::DateTime {
    {
      let mut object: ::date_time::DateTime =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QResource_lastModified_to_output(self as *const ::resource::Resource, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QResource::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QResource_locale_to_output(self as *const ::resource::Resource, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QResource::QResource```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::resource::Resource```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QResource::QResource()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::resource::Resource```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QResource::QResource(const QString& file = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, &::locale::Locale)) -> ::resource::Resource```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QResource::QResource(const QString& file = ?, const QLocale& locale = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::resource::Resource
    where Args: overloading::ResourceNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QResource::registerResource```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn register_resource(&::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::registerResource(const QString& rccFilename)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn register_resource((&::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::registerResource(const QString& rccFilename, const QString& resourceRoot = ?)```</span>
  ///
  ///
  pub fn register_resource<Args>(args: Args) -> bool
    where Args: overloading::ResourceRegisterResourceArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QResource::registerResource```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn register_resource_unsafe(*const ::libc::c_uchar) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::registerResource(const unsigned char* rccData)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn register_resource_unsafe((*const ::libc::c_uchar, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::registerResource(const unsigned char* rccData, const QString& resourceRoot = ?)```</span>
  ///
  ///
  pub unsafe fn register_resource_unsafe<Args>(args: Args) -> bool
    where Args: overloading::ResourceRegisterResourceUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QStringList QResource::searchPaths()```</span>
  ///
  ///
  pub fn search_paths() -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QResource_searchPaths_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QResource::setFileName(const QString& file)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QResource_setFileName(self as *mut ::resource::Resource,
                                             file as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QResource::setLocale(const QLocale& locale)```</span>
  ///
  ///
  pub fn set_locale(&mut self, locale: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QResource_setLocale(self as *mut ::resource::Resource,
                                           locale as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QResource::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QResource_size(self as *const ::resource::Resource) }
  }

  /// C++ method: <span style='color: green;'>```QResource::unregisterResource```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn unregister_resource(&::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::unregisterResource(const QString& rccFilename)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn unregister_resource((&::string::String, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::unregisterResource(const QString& rccFilename, const QString& resourceRoot = ?)```</span>
  ///
  ///
  pub fn unregister_resource<Args>(args: Args) -> bool
    where Args: overloading::ResourceUnregisterResourceArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QResource::unregisterResource```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn unregister_resource_unsafe(*const ::libc::c_uchar) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::unregisterResource(const unsigned char* rccData)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn unregister_resource_unsafe((*const ::libc::c_uchar, &::string::String)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QResource::unregisterResource(const unsigned char* rccData, const QString& resourceRoot = ?)```</span>
  ///
  ///
  pub unsafe fn unregister_resource_unsafe<Args>(args: Args) -> bool
    where Args: overloading::ResourceUnregisterResourceUnsafeArgs
  {
    args.exec()
  }
}

impl Drop for ::resource::Resource {
  /// C++ method: <span style='color: green;'>```[destructor] void QResource::~QResource()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QResource_destructor(self as *mut ::resource::Resource) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Resource::new](../struct.Resource.html#method.new) method.
  pub trait ResourceNewArgs {
    fn exec(self) -> ::resource::Resource;
  }
  impl<'a> ResourceNewArgs for &'a ::string::String {
    fn exec(self) -> ::resource::Resource {
      let file = self;
      {
        let mut object: ::resource::Resource =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QResource_constructor_file(file as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ResourceNewArgs for (&'a ::string::String, &'a ::locale::Locale) {
    fn exec(self) -> ::resource::Resource {
      let file = self.0;
      let locale = self.1;
      {
        let mut object: ::resource::Resource =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QResource_constructor_file_locale(file as *const ::string::String,
                                                             locale as *const ::locale::Locale,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl ResourceNewArgs for () {
    fn exec(self) -> ::resource::Resource {

      {
        let mut object: ::resource::Resource =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QResource_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Resource::register_resource](../struct.Resource.html#method.register_resource) method.
  pub trait ResourceRegisterResourceArgs {
    fn exec(self) -> bool;
  }
  impl<'a> ResourceRegisterResourceArgs for &'a ::string::String {
    fn exec(self) -> bool {
      let rcc_filename = self;
      unsafe { ::ffi::qt_core_c_QResource_registerResource_rccFilename(rcc_filename as *const ::string::String) }
    }
  }
  impl<'a> ResourceRegisterResourceArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> bool {
      let rcc_filename = self.0;
      let resource_root = self.1;
      unsafe {
        ::ffi::qt_core_c_QResource_registerResource_rccFilename_resourceRoot(rcc_filename as *const ::string::String,
                                                                             resource_root as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Resource::register_resource_unsafe](../struct.Resource.html#method.register_resource_unsafe) method.
  pub trait ResourceRegisterResourceUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl ResourceRegisterResourceUnsafeArgs for *const ::libc::c_uchar {
    unsafe fn exec(self) -> bool {
      let rcc_data = self;
      ::ffi::qt_core_c_QResource_registerResource_rccData(rcc_data)
    }
  }
  impl<'a> ResourceRegisterResourceUnsafeArgs for (*const ::libc::c_uchar, &'a ::string::String) {
    unsafe fn exec(self) -> bool {
      let rcc_data = self.0;
      let resource_root = self.1;
      ::ffi::qt_core_c_QResource_registerResource_rccData_resourceRoot(rcc_data,
                                                                       resource_root as *const ::string::String)
    }
  }
  /// This trait represents a set of arguments accepted by [Resource::unregister_resource](../struct.Resource.html#method.unregister_resource) method.
  pub trait ResourceUnregisterResourceArgs {
    fn exec(self) -> bool;
  }
  impl<'a> ResourceUnregisterResourceArgs for &'a ::string::String {
    fn exec(self) -> bool {
      let rcc_filename = self;
      unsafe { ::ffi::qt_core_c_QResource_unregisterResource_rccFilename(rcc_filename as *const ::string::String) }
    }
  }
  impl<'a> ResourceUnregisterResourceArgs for (&'a ::string::String, &'a ::string::String) {
    fn exec(self) -> bool {
      let rcc_filename = self.0;
      let resource_root = self.1;
      unsafe {
        ::ffi::qt_core_c_QResource_unregisterResource_rccFilename_resourceRoot(rcc_filename as *const ::string::String, resource_root as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Resource::unregister_resource_unsafe](../struct.Resource.html#method.unregister_resource_unsafe) method.
  pub trait ResourceUnregisterResourceUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl ResourceUnregisterResourceUnsafeArgs for *const ::libc::c_uchar {
    unsafe fn exec(self) -> bool {
      let rcc_data = self;
      ::ffi::qt_core_c_QResource_unregisterResource_rccData(rcc_data)
    }
  }
  impl<'a> ResourceUnregisterResourceUnsafeArgs for (*const ::libc::c_uchar, &'a ::string::String) {
    unsafe fn exec(self) -> bool {
      let rcc_data = self.0;
      let resource_root = self.1;
      ::ffi::qt_core_c_QResource_unregisterResource_rccData_resourceRoot(rcc_data,
                                                                         resource_root as *const ::string::String)
    }
  }
}
