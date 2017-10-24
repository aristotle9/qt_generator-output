/// C++ type: <span style='color: green;'>```QSaveFile```</span>
#[repr(C)]
pub struct SaveFile(u8);

impl SaveFile {
  /// C++ method: <span style='color: green;'>```void QSaveFile::cancelWriting()```</span>
  ///
  ///
  pub fn cancel_writing(&mut self) {
    unsafe { ::ffi::qt_core_c_QSaveFile_cancelWriting(self as *mut ::save_file::SaveFile) }
  }

  /// C++ method: <span style='color: green;'>```bool QSaveFile::commit()```</span>
  ///
  ///
  pub fn commit(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QSaveFile_commit(self as *mut ::save_file::SaveFile) }
  }

  /// C++ method: <span style='color: green;'>```bool QSaveFile::directWriteFallback() const```</span>
  ///
  ///
  pub fn direct_write_fallback(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSaveFile_directWriteFallback(self as *const ::save_file::SaveFile) }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QSaveFile::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSaveFile_fileName_to_output(self as *const ::save_file::SaveFile, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSaveFile::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSaveFile_metaObject(self as *const ::save_file::SaveFile) }
  }

  /// C++ method: <span style='color: green;'>```QSaveFile::QSaveFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::save_file::SaveFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSaveFile::QSaveFile()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::cpp_utils::CppBox<::save_file::SaveFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSaveFile::QSaveFile(const QString& name)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::save_file::SaveFile>
    where Args: overloading::SaveFileNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSaveFile::QSaveFile```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::save_file::SaveFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSaveFile::QSaveFile(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::string::String, *mut ::object::Object)) -> ::cpp_utils::CppBox<::save_file::SaveFile>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSaveFile::QSaveFile(const QString& name, QObject* parent)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::save_file::SaveFile>
    where Args: overloading::SaveFileNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QSaveFile::open(QFlags<QIODevice::OpenModeFlag> flags)```</span>
  ///
  ///
  pub fn open(&mut self, flags: ::flags::Flags<::io_device::OpenModeFlag>) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSaveFile_open(self as *mut ::save_file::SaveFile,
                                      flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSaveFile::setDirectWriteFallback(bool enabled)```</span>
  ///
  ///
  pub fn set_direct_write_fallback(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_core_c_QSaveFile_setDirectWriteFallback(self as *mut ::save_file::SaveFile, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QSaveFile::setFileName(const QString& name)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, name: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QSaveFile_setFileName(self as *mut ::save_file::SaveFile,
                                             name as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSaveFile::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSaveFile_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSaveFile::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSaveFile_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::save_file::SaveFile {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSaveFile_delete
  }
}

impl ::cpp_utils::DynamicCast<::save_file::SaveFile> for ::file_device::FileDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::save_file::SaveFile> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::save_file::SaveFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::save_file::SaveFile> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::save_file::SaveFile> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QIODevice(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::save_file::SaveFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::save_file::SaveFile> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::save_file::SaveFile> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::save_file::SaveFile> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_dynamic_cast_QSaveFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::file_device::FileDevice> for ::save_file::SaveFile {
  fn static_cast_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(self as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::file_device::FileDevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(self as *const ::save_file::SaveFile as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::save_file::SaveFile {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QIODevice_ptr(self as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QIODevice_ptr(self as *const ::save_file::SaveFile as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::save_file::SaveFile {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QObject_ptr(self as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QObject_ptr(self as *const ::save_file::SaveFile as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::save_file::SaveFile> for ::file_device::FileDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::save_file::SaveFile {
    let ffi_result =
      ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QFileDevice(self as *mut ::file_device::FileDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::save_file::SaveFile {
    let ffi_result = ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QFileDevice(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::save_file::SaveFile> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::save_file::SaveFile {
    let ffi_result =
      ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::save_file::SaveFile {
    let ffi_result = ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::save_file::SaveFile> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::save_file::SaveFile {
    let ffi_result = ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::save_file::SaveFile {
    let ffi_result = ::ffi::qt_core_c_QSaveFile_G_static_cast_QSaveFile_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::save_file::SaveFile {
  type Target = ::file_device::FileDevice;
  fn deref(&self) -> &::file_device::FileDevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(self as *const ::save_file::SaveFile as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::save_file::SaveFile {
  fn deref_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSaveFile_G_static_cast_QFileDevice_ptr(self as *mut ::save_file::SaveFile) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SaveFile::new](../struct.SaveFile.html#method.new) method.
  pub trait SaveFileNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile>;
  }
  impl<'a> SaveFileNewArgs for &'a ::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile> {
      let name = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_new_name(name as *const ::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl SaveFileNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QSaveFile_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [SaveFile::new_unsafe](../struct.SaveFile.html#method.new_unsafe) method.
  pub trait SaveFileNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile>;
  }
  impl<'a> SaveFileNewUnsafeArgs for (&'a ::string::String, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile> {
      let name = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QSaveFile_new_name_parent(name as *const ::string::String, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl SaveFileNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::save_file::SaveFile> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QSaveFile_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
