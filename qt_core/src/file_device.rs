/// C++ type: <span style='color: green;'>```QFileDevice```</span>
#[repr(C)]
pub struct FileDevice(u8);

impl FileDevice {
  /// C++ method: <span style='color: green;'>```virtual bool QFileDevice::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileDevice_atEnd(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QFileDevice::close()```</span>
  ///
  ///
  pub fn close(&mut self) {
    unsafe { ::ffi::qt_core_c_QFileDevice_close(self as *mut ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```QFileDevice::FileError QFileDevice::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::file_device::FileError {
    unsafe { ::ffi::qt_core_c_QFileDevice_error(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual QString QFileDevice::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFileDevice_fileName_to_output(self as *const ::file_device::FileDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDevice::flush()```</span>
  ///
  ///
  pub fn flush(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileDevice_flush(self as *mut ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```int QFileDevice::handle() const```</span>
  ///
  ///
  pub fn handle(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QFileDevice_handle(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFileDevice::isSequential() const```</span>
  ///
  ///
  pub fn is_sequential(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFileDevice_isSequential(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```QFileDevice::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map(&mut self, (i64, i64)) -> *mut ::libc::c_uchar```<br>
  /// C++ method: <span style='color: green;'>```unsigned char* QFileDevice::map(qint64 offset, qint64 size)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map(&mut self, (i64, i64, ::file_device::MemoryMapFlags)) -> *mut ::libc::c_uchar```<br>
  /// C++ method: <span style='color: green;'>```unsigned char* QFileDevice::map(qint64 offset, qint64 size, QFileDevice::MemoryMapFlags flags = ?)```</span>
  ///
  ///
  pub fn map<'largs, Args>(&'largs mut self, args: Args) -> *mut ::libc::c_uchar
    where Args: overloading::FileDeviceMapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QFileDevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QFileDevice_metaObject(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual QFlags<QFileDevice::Permission> QFileDevice::permissions() const```</span>
  ///
  ///
  pub fn permissions(&self) -> ::flags::Flags<::file_device::Permission> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_permissions(self as *const ::file_device::FileDevice) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QFileDevice::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QFileDevice_pos(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFileDevice::resize(qint64 sz)```</span>
  ///
  ///
  pub fn resize(&mut self, sz: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QFileDevice_resize(self as *mut ::file_device::FileDevice, sz) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFileDevice::seek(qint64 offset)```</span>
  ///
  ///
  pub fn seek(&mut self, offset: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QFileDevice_seek(self as *mut ::file_device::FileDevice, offset) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QFileDevice::setPermissions(QFlags<QFileDevice::Permission> permissionSpec)```</span>
  ///
  ///
  pub fn set_permissions(&mut self, permission_spec: ::flags::Flags<::file_device::Permission>) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFileDevice_setPermissions(self as *mut ::file_device::FileDevice,
                                                  permission_spec.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QFileDevice::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QFileDevice_size(self as *const ::file_device::FileDevice) }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileDevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileDevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QFileDevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QFileDevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFileDevice::unmap(unsigned char* address)```</span>
  ///
  ///
  pub unsafe fn unmap(&mut self, address: *mut ::libc::c_uchar) -> bool {
    ::ffi::qt_core_c_QFileDevice_unmap(self as *mut ::file_device::FileDevice, address)
  }

  /// C++ method: <span style='color: green;'>```void QFileDevice::unsetError()```</span>
  ///
  ///
  pub fn unset_error(&mut self) {
    unsafe { ::ffi::qt_core_c_QFileDevice_unsetError(self as *mut ::file_device::FileDevice) }
  }
}

impl ::cpp_utils::CppDeletable for ::file_device::FileDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QFileDevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FileDevice`.
  pub struct Signals<'a>(&'a ::file_device::FileDevice);
  /// Represents a built-in Qt signal `QFileDevice::channelBytesWritten`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().channel_bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct ChannelBytesWritten<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for ChannelBytesWritten<'a> {
    type Arguments = (::libc::c_int, i64);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2channelBytesWritten(int,qint64)\0"
    }
  }
  impl<'a> ::connection::Signal for ChannelBytesWritten<'a> {}
  /// Represents a built-in Qt signal `QFileDevice::aboutToClose`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().about_to_close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct AboutToClose<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for AboutToClose<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2aboutToClose()\0"
    }
  }
  impl<'a> ::connection::Signal for AboutToClose<'a> {}
  /// Represents a built-in Qt signal `QFileDevice::readChannelFinished`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().read_channel_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct ReadChannelFinished<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for ReadChannelFinished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readChannelFinished()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadChannelFinished<'a> {}
  /// Represents a built-in Qt signal `QFileDevice::channelReadyRead`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().channel_ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct ChannelReadyRead<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for ChannelReadyRead<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2channelReadyRead(int)\0"
    }
  }
  impl<'a> ::connection::Signal for ChannelReadyRead<'a> {}
  /// Represents a built-in Qt signal `QFileDevice::readyRead`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct ReadyRead<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for ReadyRead<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2readyRead()\0"
    }
  }
  impl<'a> ::connection::Signal for ReadyRead<'a> {}
  /// Represents a built-in Qt signal `QFileDevice::bytesWritten`.
  ///
  /// An object of this type can be created from `FileDevice` with `object.signals().bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FileDevice` object.
  pub struct BytesWritten<'a>(&'a ::file_device::FileDevice);
  impl<'a> ::connection::Receiver for BytesWritten<'a> {
    type Arguments = (i64,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bytesWritten(qint64)\0"
    }
  }
  impl<'a> ::connection::Signal for BytesWritten<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QFileDevice::channelBytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_bytes_written(&self) -> ChannelBytesWritten {
      ChannelBytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDevice::aboutToClose`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_close(&self) -> AboutToClose {
      AboutToClose(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDevice::readChannelFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn read_channel_finished(&self) -> ReadChannelFinished {
      ReadChannelFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDevice::channelReadyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_ready_read(&self) -> ChannelReadyRead {
      ChannelReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDevice::readyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read(&self) -> ReadyRead {
      ReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QFileDevice::bytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bytes_written(&self) -> BytesWritten {
      BytesWritten(self.0)
    }
  }
  impl ::file_device::FileDevice {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QFileDevice::FileError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FileError {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadError = 1```</span>
  Read = 1,
  /// C++ enum variant: <span style='color: green;'>```WriteError = 2```</span>
  Write = 2,
  /// C++ enum variant: <span style='color: green;'>```FatalError = 3```</span>
  Fatal = 3,
  /// C++ enum variant: <span style='color: green;'>```ResourceError = 4```</span>
  Resource = 4,
  /// C++ enum variant: <span style='color: green;'>```OpenError = 5```</span>
  Open = 5,
  /// C++ enum variant: <span style='color: green;'>```AbortError = 6```</span>
  Abort = 6,
  /// C++ enum variant: <span style='color: green;'>```TimeOutError = 7```</span>
  TimeOut = 7,
  /// C++ enum variant: <span style='color: green;'>```UnspecifiedError = 8```</span>
  Unspecified = 8,
  /// C++ enum variant: <span style='color: green;'>```RemoveError = 9```</span>
  Remove = 9,
  /// C++ enum variant: <span style='color: green;'>```RenameError = 10```</span>
  Rename = 10,
  /// C++ enum variant: <span style='color: green;'>```PositionError = 11```</span>
  Position = 11,
  /// C++ enum variant: <span style='color: green;'>```ResizeError = 12```</span>
  Resize = 12,
  /// C++ enum variant: <span style='color: green;'>```PermissionsError = 13```</span>
  Permissions = 13,
  /// C++ enum variant: <span style='color: green;'>```CopyError = 14```</span>
  Copy = 14,
}

/// C++ type: <span style='color: green;'>```QFileDevice::FileHandleFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FileHandleFlag {
  /// C++ enum variant: <span style='color: green;'>```DontCloseHandle = 0```</span>
  Dont = 0,
  /// C++ enum variant: <span style='color: green;'>```AutoCloseHandle = 1```</span>
  Auto = 1,
}

impl ::flags::FlaggableEnum for FileHandleFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FileHandleFlag"
  }
}

/// C++ type: <span style='color: green;'>```QFileDevice::MemoryMapFlags```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MemoryMapFlags {
  /// C++ enum variant: <span style='color: green;'>```NoOptions = 0```</span>
  NoOptions = 0,
  /// C++ enum variant: <span style='color: green;'>```MapPrivateOption = 1```</span>
  MapPrivateOption = 1,
}

/// C++ type: <span style='color: green;'>```QFileDevice::Permission```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Permission {
  /// C++ enum variant: <span style='color: green;'>```ExeOther = 1```</span>
  ExeOther = 1,
  /// C++ enum variant: <span style='color: green;'>```WriteOther = 2```</span>
  WriteOther = 2,
  /// C++ enum variant: <span style='color: green;'>```ReadOther = 4```</span>
  ReadOther = 4,
  /// C++ enum variant: <span style='color: green;'>```ExeGroup = 16```</span>
  ExeGroup = 16,
  /// C++ enum variant: <span style='color: green;'>```WriteGroup = 32```</span>
  WriteGroup = 32,
  /// C++ enum variant: <span style='color: green;'>```ReadGroup = 64```</span>
  ReadGroup = 64,
  /// C++ enum variant: <span style='color: green;'>```ExeUser = 256```</span>
  ExeUser = 256,
  /// C++ enum variant: <span style='color: green;'>```WriteUser = 512```</span>
  WriteUser = 512,
  /// C++ enum variant: <span style='color: green;'>```ReadUser = 1024```</span>
  ReadUser = 1024,
  /// C++ enum variant: <span style='color: green;'>```ExeOwner = 4096```</span>
  ExeOwner = 4096,
  /// C++ enum variant: <span style='color: green;'>```WriteOwner = 8192```</span>
  WriteOwner = 8192,
  /// C++ enum variant: <span style='color: green;'>```ReadOwner = 16384```</span>
  ReadOwner = 16384,
}

impl ::flags::FlaggableEnum for Permission {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Permission"
  }
}

impl ::cpp_utils::DynamicCast<::file_device::FileDevice> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_device::FileDevice> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QIODevice(self as *mut ::io_device::IODevice)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_device::FileDevice> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::file_device::FileDevice> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::file_device::FileDevice> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::file_device::FileDevice> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_G_dynamic_cast_QFileDevice_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::file_device::FileDevice {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(self as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::file_device::FileDevice {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QObject_ptr(self as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QObject_ptr(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_device::FileDevice> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result =
      ::ffi::qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_device::FileDevice {
    let ffi_result = ::ffi::qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::file_device::FileDevice> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::file_device::FileDevice {
    let ffi_result =
      ::ffi::qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::file_device::FileDevice {
    let ffi_result = ::ffi::qt_core_c_QFileDevice_G_static_cast_QFileDevice_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::file_device::FileDevice {
  type Target = ::io_device::IODevice;
  fn deref(&self) -> &::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(self as *const ::file_device::FileDevice as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::file_device::FileDevice {
  fn deref_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QFileDevice_G_static_cast_QIODevice_ptr(self as *mut ::file_device::FileDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FileDevice::map](../struct.FileDevice.html#method.map) method.
  pub trait FileDeviceMapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::file_device::FileDevice) -> *mut ::libc::c_uchar;
  }
  impl<'largs> FileDeviceMapArgs<'largs> for (i64, i64) {
    fn exec(self, original_self: &'largs mut ::file_device::FileDevice) -> *mut ::libc::c_uchar {
      let offset = self.0;
      let size = self.1;
      unsafe {
        ::ffi::qt_core_c_QFileDevice_map_offset_size(original_self as *mut ::file_device::FileDevice,
                                                     offset,
                                                     size)
      }
    }
  }
  impl<'largs> FileDeviceMapArgs<'largs> for (i64, i64, ::file_device::MemoryMapFlags) {
    fn exec(self, original_self: &'largs mut ::file_device::FileDevice) -> *mut ::libc::c_uchar {
      let offset = self.0;
      let size = self.1;
      let flags = self.2;
      unsafe {
        ::ffi::qt_core_c_QFileDevice_map_offset_size_flags(original_self as *mut ::file_device::FileDevice,
                                                           offset,
                                                           size,
                                                           flags)
      }
    }
  }
}
