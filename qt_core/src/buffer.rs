/// C++ type: <span style='color: green;'>```QBuffer```</span>
#[repr(C)]
pub struct Buffer(u8);

impl Buffer {
  /// C++ method: <span style='color: green;'>```virtual bool QBuffer::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBuffer_atEnd(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QBuffer::buffer() const```</span>
  ///
  ///
  pub fn buffer<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_buffer_const(self as *const ::buffer::Buffer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QBuffer::buffer()```</span>
  ///
  ///
  pub fn buffer_mut<'l0>(&'l0 mut self) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_buffer(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```virtual bool QBuffer::canReadLine() const```</span>
  ///
  ///
  pub fn can_read_line(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBuffer_canReadLine(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QBuffer::close()```</span>
  ///
  ///
  pub fn close(&mut self) {
    unsafe { ::ffi::qt_core_c_QBuffer_close(self as *mut ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QBuffer::data() const```</span>
  ///
  ///
  pub fn data<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_data(self as *const ::buffer::Buffer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QBuffer::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QBuffer_metaObject(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QBuffer::QBuffer()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::buffer::Buffer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QBuffer::QBuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::byte_array::ByteArray) -> ::cpp_utils::CppBox<::buffer::Buffer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBuffer::QBuffer(QByteArray* buf)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::byte_array::ByteArray, *mut ::object::Object)) -> ::cpp_utils::CppBox<::buffer::Buffer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBuffer::QBuffer(QByteArray* buf, QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::object::Object) -> ::cpp_utils::CppBox<::buffer::Buffer>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QBuffer::QBuffer(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::buffer::Buffer>
    where Args: overloading::BufferNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual bool QBuffer::open(QFlags<QIODevice::OpenModeFlag> openMode)```</span>
  ///
  ///
  pub fn open(&mut self, open_mode: ::flags::Flags<::io_device::OpenModeFlag>) -> bool {
    unsafe {
      ::ffi::qt_core_c_QBuffer_open(self as *mut ::buffer::Buffer,
                                    open_mode.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QBuffer::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QBuffer_pos(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QBuffer::seek(qint64 off)```</span>
  ///
  ///
  pub fn seek(&mut self, off: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QBuffer_seek(self as *mut ::buffer::Buffer, off) }
  }

  /// C++ method: <span style='color: green;'>```void QBuffer::setBuffer(QByteArray* a)```</span>
  ///
  ///
  pub unsafe fn set_buffer(&mut self, a: *mut ::byte_array::ByteArray) {
    ::ffi::qt_core_c_QBuffer_setBuffer(self as *mut ::buffer::Buffer, a)
  }

  /// C++ method: <span style='color: green;'>```void QBuffer::setData(const QByteArray& data)```</span>
  ///
  ///
  pub fn set_data(&mut self, data: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QBuffer_setData_data(self as *mut ::buffer::Buffer,
                                            data as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QBuffer::setData(const char* data, int len)```</span>
  ///
  ///
  pub unsafe fn set_data_unsafe(&mut self, data: *const ::libc::c_char, len: ::libc::c_int) {
    ::ffi::qt_core_c_QBuffer_setData_data_len(self as *mut ::buffer::Buffer, data, len)
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QBuffer::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QBuffer_size(self as *const ::buffer::Buffer) }
  }

  /// C++ method: <span style='color: green;'>```static QString QBuffer::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QBuffer_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QBuffer::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QBuffer_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::buffer::Buffer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QBuffer_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Buffer`.
  pub struct Signals<'a>(&'a ::buffer::Buffer);
  /// Represents a built-in Qt signal `QBuffer::bytesWritten`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct BytesWritten<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `QBuffer::aboutToClose`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().about_to_close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct AboutToClose<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `QBuffer::channelBytesWritten`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().channel_bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct ChannelBytesWritten<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `QBuffer::readyRead`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct ReadyRead<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `QBuffer::channelReadyRead`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().channel_ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct ChannelReadyRead<'a>(&'a ::buffer::Buffer);
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
  /// Represents a built-in Qt signal `QBuffer::readChannelFinished`.
  ///
  /// An object of this type can be created from `Buffer` with `object.signals().read_channel_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Buffer` object.
  pub struct ReadChannelFinished<'a>(&'a ::buffer::Buffer);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QBuffer::bytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bytes_written(&self) -> BytesWritten {
      BytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QBuffer::aboutToClose`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_close(&self) -> AboutToClose {
      AboutToClose(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QBuffer::channelBytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_bytes_written(&self) -> ChannelBytesWritten {
      ChannelBytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QBuffer::readyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read(&self) -> ReadyRead {
      ReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QBuffer::channelReadyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_ready_read(&self) -> ChannelReadyRead {
      ChannelReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QBuffer::readChannelFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn read_channel_finished(&self) -> ReadChannelFinished {
      ReadChannelFinished(self.0)
    }
  }
  impl ::buffer::Buffer {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::buffer::Buffer> for ::io_device::IODevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::buffer::Buffer> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QIODevice(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::buffer::Buffer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::buffer::Buffer> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::buffer::Buffer> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QObject(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::buffer::Buffer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_G_dynamic_cast_QBuffer_ptr_QObject(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::io_device::IODevice> for ::buffer::Buffer {
  fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::buffer::Buffer {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_G_static_cast_QObject_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QBuffer_G_static_cast_QObject_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer::Buffer> for ::io_device::IODevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer::Buffer {
    let ffi_result = ::ffi::qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QIODevice(self as *mut ::io_device::IODevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer::Buffer {
    let ffi_result = ::ffi::qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QIODevice(self as *const ::io_device::IODevice as *mut ::io_device::IODevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::buffer::Buffer> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::buffer::Buffer {
    let ffi_result = ::ffi::qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QObject(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::buffer::Buffer {
    let ffi_result = ::ffi::qt_core_c_QBuffer_G_static_cast_QBuffer_ptr_QObject(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::buffer::Buffer {
  type Target = ::io_device::IODevice;
  fn deref(&self) -> &::io_device::IODevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(self as *const ::buffer::Buffer as *mut ::buffer::Buffer)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::buffer::Buffer {
  fn deref_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBuffer_G_static_cast_QIODevice_ptr(self as *mut ::buffer::Buffer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Buffer::new_unsafe](../struct.Buffer.html#method.new_unsafe) method.
  pub trait BufferNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer>;
  }
  impl BufferNewUnsafeArgs for *mut ::byte_array::ByteArray {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer> {
      let buf = self;
      let ffi_result = ::ffi::qt_core_c_QBuffer_new_buf(buf);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl BufferNewUnsafeArgs for (*mut ::byte_array::ByteArray, *mut ::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer> {
      let buf = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_core_c_QBuffer_new_buf_parent(buf, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl BufferNewUnsafeArgs for *mut ::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::buffer::Buffer> {
      let parent = self;
      let ffi_result = ::ffi::qt_core_c_QBuffer_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
