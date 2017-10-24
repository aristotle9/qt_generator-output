/// C++ type: <span style='color: green;'>```QFile```</span>
#[repr(C)]
pub struct File(u8);

impl File {
  /// C++ method: <span style='color: green;'>```bool QFile::copy(const QString& newName)```</span>
  ///
  ///
  pub fn copy(&mut self, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_copy_newName(self as *mut ::file::File,
                                          new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::copy(const QString& fileName, const QString& newName)```</span>
  ///
  ///
  pub fn copy_static(file_name: &::string::String, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_copy_fileName_newName(file_name as *const ::string::String,
                                                   new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::decodeName(const QByteArray& localFileName)```</span>
  ///
  ///
  pub fn decode_name(local_file_name: &::byte_array::ByteArray) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_decodeName_to_output_QByteArray(local_file_name as *const ::byte_array::ByteArray,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::decodeName(const char* localFileName)```</span>
  ///
  ///
  pub unsafe fn decode_name_unsafe(local_file_name: *const ::libc::c_char) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFile_decodeName_to_output_char(local_file_name, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QFile::encodeName(const QString& fileName)```</span>
  ///
  ///
  pub fn encode_name(file_name: &::string::String) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_encodeName_to_output(file_name as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFile::exists() const```</span>
  ///
  ///
  pub fn exists(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_exists_no_args(self as *const ::file::File) }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::exists(const QString& fileName)```</span>
  ///
  ///
  pub fn exists_static(file_name: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_exists_fileName(file_name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QFile::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_fileName_to_output(self as *const ::file::File, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFile::link(const QString& newName)```</span>
  ///
  ///
  pub fn link(&mut self, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_link_newName(self as *mut ::file::File,
                                          new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::link(const QString& oldname, const QString& newName)```</span>
  ///
  ///
  pub fn link_static(oldname: &::string::String, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_link_oldname_newName(oldname as *const ::string::String,
                                                  new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFile::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QFile_metaObject(self as *const ::file::File) }
  }

  /// C++ method: <span style='color: green;'>```QFile::QFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::file::File>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFile::QFile()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::file::File>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFile::QFile(const QString& name)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::file::File>
    where Args: overloading::FileNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFile::QFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::file::File>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFile::QFile(QObject* parent)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::file::File>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFile::QFile(const QString& name, QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::file::File>
    where Args: overloading::FileNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFile::open```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn open(&mut self, (::libc::c_int, ::flags::Flags<::io_device::OpenModeFlag>)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFile::open(int fd, QFlags<QIODevice::OpenModeFlag> ioFlags)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn open(&mut self, (::libc::c_int, ::flags::Flags<::io_device::OpenModeFlag>, ::flags::Flags<::file_device::FileHandleFlag>)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QFile::open(int fd, QFlags<QIODevice::OpenModeFlag> ioFlags, QFlags<QFileDevice::FileHandleFlag> handleFlags = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn open(&mut self, ::flags::Flags<::io_device::OpenModeFlag>) -> bool```<br>
  /// C++ method: <span style='color: green;'>```virtual bool QFile::open(QFlags<QIODevice::OpenModeFlag> flags)```</span>
  ///
  ///
  pub fn open<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::FileOpenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual QFlags<QFileDevice::Permission> QFile::permissions() const```</span>
  ///
  ///
  pub fn permissions(&self) -> ::flags::Flags<::file_device::Permission> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_permissions_no_args(self as *const ::file::File) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```static QFlags<QFileDevice::Permission> QFile::permissions(const QString& filename)```</span>
  ///
  ///
  pub fn permissions_static(filename: &::string::String) -> ::flags::Flags<::file_device::Permission> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_permissions_filename(filename as *const ::string::String) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QString QFile::readLink() const```</span>
  ///
  ///
  pub fn read_link(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_readLink_to_output_no_args(self as *const ::file::File, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::readLink(const QString& fileName)```</span>
  ///
  ///
  pub fn read_link_static(file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_readLink_to_output_fileName(file_name as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFile::remove()```</span>
  ///
  ///
  pub fn remove(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_remove_no_args(self as *mut ::file::File) }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::remove(const QString& fileName)```</span>
  ///
  ///
  pub fn remove_static(file_name: &::string::String) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_remove_fileName(file_name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```bool QFile::rename(const QString& newName)```</span>
  ///
  ///
  pub fn rename(&mut self, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_rename_newName(self as *mut ::file::File,
                                            new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::rename(const QString& oldName, const QString& newName)```</span>
  ///
  ///
  pub fn rename_static(old_name: &::string::String, new_name: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_rename_oldName_newName(old_name as *const ::string::String,
                                                    new_name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFile::resize(qint64 sz)```</span>
  ///
  ///
  pub fn resize(&mut self, sz: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_resize_sz(self as *mut ::file::File, sz) }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::resize(const QString& filename, qint64 sz)```</span>
  ///
  ///
  pub fn resize_static(filename: &::string::String, sz: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QFile_resize_filename_sz(filename as *const ::string::String, sz) }
  }

  /// C++ method: <span style='color: green;'>```void QFile::setFileName(const QString& name)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, name: &::string::String) {
    unsafe { ::ffi::qt_core_c_QFile_setFileName(self as *mut ::file::File, name as *const ::string::String) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFile::setPermissions(QFlags<QFileDevice::Permission> permissionSpec)```</span>
  ///
  ///
  pub fn set_permissions(&mut self, permission_spec: ::flags::Flags<::file_device::Permission>) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_setPermissions_permissionSpec(self as *mut ::file::File,
                                                           permission_spec.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QFile::setPermissions(const QString& filename, QFlags<QFileDevice::Permission> permissionSpec)```</span>
  ///
  ///
  pub fn set_permissions_static(filename: &::string::String,
                                permission_spec: ::flags::Flags<::file_device::Permission>)
                                -> bool {
    unsafe {
      ::ffi::qt_core_c_QFile_setPermissions_filename_permissionSpec(filename as *const ::string::String,
                                                                    permission_spec.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QFile::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QFile_size(self as *const ::file::File) }
  }

  /// C++ method: <span style='color: green;'>```QString QFile::symLinkTarget() const```</span>
  ///
  ///
  pub fn sym_link_target(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_symLinkTarget_to_output_no_args(self as *const ::file::File, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::symLinkTarget(const QString& fileName)```</span>
  ///
  ///
  pub fn sym_link_target_static(file_name: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFile_symLinkTarget_to_output_fileName(file_name as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFile_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFile::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFile_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::file::File {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFile_delete
  }
}

impl ::cpp_utils::DynamicCast<::file::File> for ::file_device::FileDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file::File> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file::File> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::file::File> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file::File> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QIODevice(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file::File> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::file::File> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file::File> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file::File> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_dynamic_cast_QFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::file_device::FileDevice> for ::file::File {
  fn static_cast_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QFileDevice_ptr(self as *mut ::file::File) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::file_device::FileDevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QFileDevice_ptr(self as *const ::file::File as *mut ::file::File) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::file::File {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QIODevice_ptr(self as *mut ::file::File) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QIODevice_ptr(self as *const ::file::File as *mut ::file::File) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::file::File {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QObject_ptr(self as *mut ::file::File) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QObject_ptr(self as *const ::file::File as *mut ::file::File) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file::File> for ::file_device::FileDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file::File {
    let ffi_result =
      ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file::File {
    let ffi_result = ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file::File> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file::File {
    let ffi_result = ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file::File {
    let ffi_result = ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file::File> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file::File {
    let ffi_result = ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file::File {
    let ffi_result = ::ffi::qt_core_c_QFile_G_static_cast_QFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file::File {
  type Target = ::file_device::FileDevice;
  fn deref(&self) -> &::file_device::FileDevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QFileDevice_ptr(self as *const ::file::File as *mut ::file::File) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file::File {
  fn deref_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFile_G_static_cast_QFileDevice_ptr(self as *mut ::file::File) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [File::new](../struct.File.html#method.new) method.
  pub trait FileNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::file::File>;
  }
  impl<'a> FileNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::file::File> {
      let name = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QFile_new_name(name as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl FileNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::file::File> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QFile_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [File::new_unsafe](../struct.File.html#method.new_unsafe) method.
  pub trait FileNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file::File>;
  }
  impl<'a> FileNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file::File> {
      let name = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QFile_new_name_parent(name as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl FileNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::file::File> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QFile_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [File::open](../struct.File.html#method.open) method.
  pub trait FileOpenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file::File) -> bool;
  }
  impl<'largs> FileOpenArgs<'largs> for (::libc::c_int, ::flags::Flags<::io_device::OpenModeFlag>) {
    fn exec(self, original_self: &'largs mut ::file::File) -> bool {
      let fd = self.0;
      let io_flags = self.1;
      unsafe {
        ::ffi::qt_core_c_QFile_open_fd_ioFlags(original_self as *mut ::file::File,
                                               fd,
                                               io_flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> FileOpenArgs<'largs>
    for (::libc::c_int, ::flags::Flags<::io_device::OpenModeFlag>, ::flags::Flags<::file_device::FileHandleFlag>) {
    fn exec(self, original_self: &'largs mut ::file::File) -> bool {
      let fd = self.0;
      let io_flags = self.1;
      let handle_flags = self.2;
      unsafe {
        ::ffi::qt_core_c_QFile_open_fd_ioFlags_handleFlags(original_self as *mut ::file::File,
                                                           fd,
                                                           io_flags.to_int() as ::libc::c_uint,
                                                           handle_flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> FileOpenArgs<'largs> for ::flags::Flags<::io_device::OpenModeFlag> {
    fn exec(self, original_self: &'largs mut ::file::File) -> bool {
      let flags = self;
      unsafe {
        ::ffi::qt_core_c_QFile_open_flags(original_self as *mut ::file::File,
                                          flags.to_int() as ::libc::c_uint)
      }
    }
  }
}
