/// C++ type: <span style='color: green;'>```QTemporaryFile```</span>
#[repr(C)]
pub struct TemporaryFile(u8);

impl TemporaryFile {
  /// C++ method: <span style='color: green;'>```bool QTemporaryFile::autoRemove() const```</span>
  ///
  ///
  pub fn auto_remove(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTemporaryFile_autoRemove(self as *const ::temporary_file::TemporaryFile) }
  }

  /// C++ method: <span style='color: green;'>```QTemporaryFile::createLocalFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_local_file(&mut ::file::File) -> *mut ::temporary_file::TemporaryFile```<br>
  /// C++ method: <span style='color: green;'>```static QTemporaryFile* QTemporaryFile::createLocalFile(QFile& file)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_local_file(&::string::String) -> *mut ::temporary_file::TemporaryFile```<br>
  /// C++ method: <span style='color: green;'>```static QTemporaryFile* QTemporaryFile::createLocalFile(const QString& fileName)```</span>
  ///
  ///
  pub fn create_local_file<Args>(args: Args) -> *mut ::temporary_file::TemporaryFile
    where Args: overloading::TemporaryFileCreateLocalFileArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTemporaryFile::createNativeFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_native_file(&mut ::file::File) -> *mut ::temporary_file::TemporaryFile```<br>
  /// C++ method: <span style='color: green;'>```static QTemporaryFile* QTemporaryFile::createNativeFile(QFile& file)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_native_file(&::string::String) -> *mut ::temporary_file::TemporaryFile```<br>
  /// C++ method: <span style='color: green;'>```static QTemporaryFile* QTemporaryFile::createNativeFile(const QString& fileName)```</span>
  ///
  ///
  pub fn create_native_file<Args>(args: Args) -> *mut ::temporary_file::TemporaryFile
    where Args: overloading::TemporaryFileCreateNativeFileArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual QString QTemporaryFile::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_fileName_to_output(self as *const ::temporary_file::TemporaryFile, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTemporaryFile::fileTemplate() const```</span>
  ///
  ///
  pub fn file_template(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_fileTemplate_to_output(self as *const ::temporary_file::TemporaryFile,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTemporaryFile::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QTemporaryFile_metaObject(self as *const ::temporary_file::TemporaryFile) }
  }

  /// C++ method: <span style='color: green;'>```QTemporaryFile::QTemporaryFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryFile::QTemporaryFile()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryFile::QTemporaryFile(const QString& templateName)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>
    where Args: overloading::TemporaryFileNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTemporaryFile::QTemporaryFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryFile::QTemporaryFile(QObject* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTemporaryFile::QTemporaryFile(const QString& templateName, QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>
    where Args: overloading::TemporaryFileNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QTemporaryFile::open()```</span>
  ///
  ///
  pub fn open(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QTemporaryFile_open(self as *mut ::temporary_file::TemporaryFile) }
  }

  /// C++ method: <span style='color: green;'>```void QTemporaryFile::setAutoRemove(bool b)```</span>
  ///
  ///
  pub fn set_auto_remove(&mut self, b: bool) {
    unsafe { ::ffi::qt_core_c_QTemporaryFile_setAutoRemove(self as *mut ::temporary_file::TemporaryFile, b) }
  }

  /// C++ method: <span style='color: green;'>```void QTemporaryFile::setFileTemplate(const QString& name)```</span>
  ///
  ///
  pub fn set_file_template(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QTemporaryFile_setFileTemplate(self as *mut ::temporary_file::TemporaryFile,
                                                      name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTemporaryFile::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTemporaryFile_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTemporaryFile::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTemporaryFile_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::temporary_file::TemporaryFile {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTemporaryFile_delete
  }
}

impl ::cpp_utils::DynamicCast<::temporary_file::TemporaryFile> for ::file::File {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::temporary_file::TemporaryFile> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFile(self as *mut ::file::File) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::temporary_file::TemporaryFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFile(self as *const ::file::File as *mut ::file::File) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::temporary_file::TemporaryFile> for ::file_device::FileDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::temporary_file::TemporaryFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::temporary_file::TemporaryFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::temporary_file::TemporaryFile> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::temporary_file::TemporaryFile> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QIODevice(self as *mut ::io_device::IODevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::temporary_file::TemporaryFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::temporary_file::TemporaryFile> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::temporary_file::TemporaryFile> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QObject(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::temporary_file::TemporaryFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_dynamic_cast_QTemporaryFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::file::File> for ::temporary_file::TemporaryFile {
  fn static_cast_mut(&mut self) -> &mut ::file::File {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(self as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::file::File {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(self as *const ::temporary_file::TemporaryFile as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::file_device::FileDevice> for ::temporary_file::TemporaryFile {
  fn static_cast_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFileDevice_ptr(self as *mut ::temporary_file::TemporaryFile)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::file_device::FileDevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFileDevice_ptr(self as *const ::temporary_file::TemporaryFile as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::temporary_file::TemporaryFile {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QIODevice_ptr(self as *mut ::temporary_file::TemporaryFile)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QIODevice_ptr(self as *const ::temporary_file::TemporaryFile as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::temporary_file::TemporaryFile {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QObject_ptr(self as *mut ::temporary_file::TemporaryFile)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QObject_ptr(self as *const ::temporary_file::TemporaryFile as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::temporary_file::TemporaryFile> for ::file::File {
  unsafe fn static_cast_mut(&mut self) -> &mut ::temporary_file::TemporaryFile {
    let ffi_result =
      ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFile(self as *mut ::file::File);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::temporary_file::TemporaryFile {
    let ffi_result = ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFile(self as *const ::file::File as *mut ::file::File);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::temporary_file::TemporaryFile> for ::file_device::FileDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::temporary_file::TemporaryFile {
    let ffi_result = ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::temporary_file::TemporaryFile {
    let ffi_result = ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::temporary_file::TemporaryFile> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::temporary_file::TemporaryFile {
    let ffi_result =
      ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::temporary_file::TemporaryFile {
    let ffi_result = ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::temporary_file::TemporaryFile> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::temporary_file::TemporaryFile {
    let ffi_result =
      ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::temporary_file::TemporaryFile {
    let ffi_result = ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QTemporaryFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::temporary_file::TemporaryFile {
  type Target = ::file::File;
  fn deref(&self) -> &::file::File {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(self as *const ::temporary_file::TemporaryFile as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::temporary_file::TemporaryFile {
  fn deref_mut(&mut self) -> &mut ::file::File {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTemporaryFile_G_static_cast_QFile_ptr(self as *mut ::temporary_file::TemporaryFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TemporaryFile::create_local_file](../struct.TemporaryFile.html#method.create_local_file) method.
  pub trait TemporaryFileCreateLocalFileArgs {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile;
  }
  impl<'a> TemporaryFileCreateLocalFileArgs for &'a mut ::file::File {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile {
      let file = self;
      unsafe { ::ffi::qt_core_c_QTemporaryFile_createLocalFile_file(file as *mut ::file::File) }
    }
  }
  impl<'a> TemporaryFileCreateLocalFileArgs for &'a ::string::String {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile {
      let file_name = self;
      unsafe { ::ffi::qt_core_c_QTemporaryFile_createLocalFile_fileName(file_name as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [TemporaryFile::create_native_file](../struct.TemporaryFile.html#method.create_native_file) method.
  pub trait TemporaryFileCreateNativeFileArgs {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile;
  }
  impl<'a> TemporaryFileCreateNativeFileArgs for &'a mut ::file::File {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile {
      let file = self;
      unsafe { ::ffi::qt_core_c_QTemporaryFile_createNativeFile_file(file as *mut ::file::File) }
    }
  }
  impl<'a> TemporaryFileCreateNativeFileArgs for &'a ::string::String {
    fn exec(self) -> *mut ::temporary_file::TemporaryFile {
      let file_name = self;
      unsafe { ::ffi::qt_core_c_QTemporaryFile_createNativeFile_fileName(file_name as *const ::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [TemporaryFile::new](../struct.TemporaryFile.html#method.new) method.
  pub trait TemporaryFileNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>;
  }
  impl TemporaryFileNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QTemporaryFile_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TemporaryFileNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile> {
      let template_name = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QTemporaryFile_new_templateName(template_name as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TemporaryFile::new_unsafe](../struct.TemporaryFile.html#method.new_unsafe) method.
  pub trait TemporaryFileNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile>;
  }
  impl TemporaryFileNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QTemporaryFile_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> TemporaryFileNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::temporary_file::TemporaryFile> {
      let template_name = self.0;
      let parent = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QTemporaryFile_new_templateName_parent(template_name as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
