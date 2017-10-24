/// C++ type: <span style='color: green;'>```QIODevice```</span>
#[repr(C)]
pub struct IODevice(u8);

impl IODevice {
  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_atEnd(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QIODevice::bytesAvailable() const```</span>
  ///
  ///
  pub fn bytes_available(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QIODevice_bytesAvailable(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QIODevice::bytesToWrite() const```</span>
  ///
  ///
  pub fn bytes_to_write(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QIODevice_bytesToWrite(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::canReadLine() const```</span>
  ///
  ///
  pub fn can_read_line(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_canReadLine(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIODevice::close()```</span>
  ///
  ///
  pub fn close(&mut self) {
    unsafe { ::ffi::qt_core_c_QIODevice_close(self as *mut ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::commitTransaction()```</span>
  ///
  ///
  pub fn commit_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QIODevice_commitTransaction(self as *mut ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```int QIODevice::currentReadChannel() const```</span>
  ///
  ///
  pub fn current_read_channel(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QIODevice_currentReadChannel(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```int QIODevice::currentWriteChannel() const```</span>
  ///
  ///
  pub fn current_write_channel(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QIODevice_currentWriteChannel(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```QString QIODevice::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIODevice_errorString_to_output(self as *const ::io_device::IODevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::getChar(char* c)```</span>
  ///
  ///
  pub unsafe fn get_char(&mut self, c: *mut ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QIODevice_getChar(self as *mut ::io_device::IODevice, c)
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::isOpen() const```</span>
  ///
  ///
  pub fn is_open(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isOpen(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::isReadable() const```</span>
  ///
  ///
  pub fn is_readable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isReadable(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::isSequential() const```</span>
  ///
  ///
  pub fn is_sequential(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isSequential(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::isTextModeEnabled() const```</span>
  ///
  ///
  pub fn is_text_mode_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isTextModeEnabled(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::isTransactionStarted() const```</span>
  ///
  ///
  pub fn is_transaction_started(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isTransactionStarted(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::isWritable() const```</span>
  ///
  ///
  pub fn is_writable(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_isWritable(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QIODevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QIODevice_metaObject(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::open(QFlags<QIODevice::OpenModeFlag> mode)```</span>
  ///
  ///
  pub fn open(&mut self, mode: ::flags::Flags<::io_device::OpenModeFlag>) -> bool {
    unsafe {
      ::ffi::qt_core_c_QIODevice_open(self as *mut ::io_device::IODevice,
                                      mode.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QIODevice::OpenModeFlag> QIODevice::openMode() const```</span>
  ///
  ///
  pub fn open_mode(&self) -> ::flags::Flags<::io_device::OpenModeFlag> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIODevice_openMode(self as *const ::io_device::IODevice) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QByteArray QIODevice::peek(qint64 maxlen)```</span>
  ///
  ///
  pub fn peek(&mut self, maxlen: i64) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIODevice_peek_to_output(self as *mut ::io_device::IODevice, maxlen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QIODevice::peek(char* data, qint64 maxlen)```</span>
  ///
  ///
  pub unsafe fn peek_unsafe(&mut self, data: *mut ::libc::c_char, maxlen: i64) -> i64 {
    ::ffi::qt_core_c_QIODevice_peek(self as *mut ::io_device::IODevice, data, maxlen)
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QIODevice::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QIODevice_pos(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```bool QIODevice::putChar(char c)```</span>
  ///
  ///
  pub fn put_char(&mut self, c: ::libc::c_char) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_putChar(self as *mut ::io_device::IODevice, c) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QIODevice::read(qint64 maxlen)```</span>
  ///
  ///
  pub fn read(&mut self, maxlen: i64) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIODevice_read_to_output(self as *mut ::io_device::IODevice, maxlen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QIODevice::readAll()```</span>
  ///
  ///
  pub fn read_all(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QIODevice_readAll_to_output(self as *mut ::io_device::IODevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QIODevice::readChannelCount() const```</span>
  ///
  ///
  pub fn read_channel_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QIODevice_readChannelCount(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice::readLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn read_line(&mut self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QIODevice::readLine()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn read_line(&mut self, i64) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QIODevice::readLine(qint64 maxlen = ?)```</span>
  ///
  ///
  pub fn read_line<'largs, Args>(&'largs mut self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::IODeviceReadLineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```qint64 QIODevice::readLine(char* data, qint64 maxlen)```</span>
  ///
  ///
  pub unsafe fn read_line_unsafe(&mut self, data: *mut ::libc::c_char, maxlen: i64) -> i64 {
    ::ffi::qt_core_c_QIODevice_readLine(self as *mut ::io_device::IODevice, data, maxlen)
  }

  /// C++ method: <span style='color: green;'>```qint64 QIODevice::read(char* data, qint64 maxlen)```</span>
  ///
  ///
  pub unsafe fn read_unsafe(&mut self, data: *mut ::libc::c_char, maxlen: i64) -> i64 {
    ::ffi::qt_core_c_QIODevice_read(self as *mut ::io_device::IODevice, data, maxlen)
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_reset(self as *mut ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::rollbackTransaction()```</span>
  ///
  ///
  pub fn rollback_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QIODevice_rollbackTransaction(self as *mut ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::seek(qint64 pos)```</span>
  ///
  ///
  pub fn seek(&mut self, pos: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_seek(self as *mut ::io_device::IODevice, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::setCurrentReadChannel(int channel)```</span>
  ///
  ///
  pub fn set_current_read_channel(&mut self, channel: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QIODevice_setCurrentReadChannel(self as *mut ::io_device::IODevice, channel) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::setCurrentWriteChannel(int channel)```</span>
  ///
  ///
  pub fn set_current_write_channel(&mut self, channel: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QIODevice_setCurrentWriteChannel(self as *mut ::io_device::IODevice, channel) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::setTextModeEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_text_mode_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_core_c_QIODevice_setTextModeEnabled(self as *mut ::io_device::IODevice, enabled) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QIODevice::size() const```</span>
  ///
  ///
  pub fn size(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QIODevice_size(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::startTransaction()```</span>
  ///
  ///
  pub fn start_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QIODevice_startTransaction(self as *mut ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```static QString QIODevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QIODevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QIODevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QIODevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QIODevice::ungetChar(char c)```</span>
  ///
  ///
  pub fn unget_char(&mut self, c: ::libc::c_char) {
    unsafe { ::ffi::qt_core_c_QIODevice_ungetChar(self as *mut ::io_device::IODevice, c) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::waitForBytesWritten(int msecs)```</span>
  ///
  ///
  pub fn wait_for_bytes_written(&mut self, msecs: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_waitForBytesWritten(self as *mut ::io_device::IODevice, msecs) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QIODevice::waitForReadyRead(int msecs)```</span>
  ///
  ///
  pub fn wait_for_ready_read(&mut self, msecs: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QIODevice_waitForReadyRead(self as *mut ::io_device::IODevice, msecs) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QIODevice::write(const QByteArray& data)```</span>
  ///
  ///
  pub fn write(&mut self, data: &::byte_array::ByteArray) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QIODevice_write_QByteArray(self as *mut ::io_device::IODevice,
                                                  data as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```int QIODevice::writeChannelCount() const```</span>
  ///
  ///
  pub fn write_channel_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QIODevice_writeChannelCount(self as *const ::io_device::IODevice) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice::write```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn write_unsafe(&mut self, *const ::libc::c_char) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qint64 QIODevice::write(const char* data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn write_unsafe(&mut self, (*const ::libc::c_char, i64)) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qint64 QIODevice::write(const char* data, qint64 len)```</span>
  ///
  ///
  pub unsafe fn write_unsafe<'largs, Args>(&'largs mut self, args: Args) -> i64
    where Args: overloading::IODeviceWriteUnsafeArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::io_device::IODevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QIODevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `IODevice`.
  pub struct Signals<'a>(&'a ::io_device::IODevice);
  /// Represents a built-in Qt signal `QIODevice::channelBytesWritten`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().channel_bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct ChannelBytesWritten<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::bytesWritten`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().bytes_written()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct BytesWritten<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::objectNameChanged`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct ObjectNameChanged<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::readyRead`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct ReadyRead<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::aboutToClose`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().about_to_close()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct AboutToClose<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::readChannelFinished`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().read_channel_finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct ReadChannelFinished<'a>(&'a ::io_device::IODevice);
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
  /// Represents a built-in Qt signal `QIODevice::channelReadyRead`.
  ///
  /// An object of this type can be created from `IODevice` with `object.signals().channel_ready_read()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IODevice` object.
  pub struct ChannelReadyRead<'a>(&'a ::io_device::IODevice);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QIODevice::channelBytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_bytes_written(&self) -> ChannelBytesWritten {
      ChannelBytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::bytesWritten`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bytes_written(&self) -> BytesWritten {
      BytesWritten(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::readyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn ready_read(&self) -> ReadyRead {
      ReadyRead(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::aboutToClose`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn about_to_close(&self) -> AboutToClose {
      AboutToClose(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::readChannelFinished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn read_channel_finished(&self) -> ReadChannelFinished {
      ReadChannelFinished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIODevice::channelReadyRead`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn channel_ready_read(&self) -> ChannelReadyRead {
      ChannelReadyRead(self.0)
    }
  }
  impl ::io_device::IODevice {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QIODevice::OpenModeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OpenModeFlag {
  /// C++ enum variant: <span style='color: green;'>```NotOpen = 0```</span>
  NotOpen = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadOnly = 1```</span>
  ReadOnly = 1,
  /// C++ enum variant: <span style='color: green;'>```WriteOnly = 2```</span>
  WriteOnly = 2,
  /// C++ enum variant: <span style='color: green;'>```ReadWrite = 3```</span>
  ReadWrite = 3,
  /// C++ enum variant: <span style='color: green;'>```Append = 4```</span>
  Append = 4,
  /// C++ enum variant: <span style='color: green;'>```Truncate = 8```</span>
  Truncate = 8,
  /// C++ enum variant: <span style='color: green;'>```Text = 16```</span>
  Text = 16,
  /// C++ enum variant: <span style='color: green;'>```Unbuffered = 32```</span>
  Unbuffered = 32,
}

impl ::flags::FlaggableEnum for OpenModeFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "OpenModeFlag"
  }
}

impl ::cpp_utils::DynamicCast<::io_device::IODevice> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::io_device::IODevice> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIODevice_G_dynamic_cast_QIODevice_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::io_device::IODevice> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIODevice_G_dynamic_cast_QIODevice_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::io_device::IODevice {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QIODevice_G_static_cast_QObject_ptr(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIODevice_G_static_cast_QObject_ptr(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::io_device::IODevice> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::io_device::IODevice {
    let ffi_result = ::ffi::qt_core_c_QIODevice_G_static_cast_QIODevice_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::io_device::IODevice {
    let ffi_result = ::ffi::qt_core_c_QIODevice_G_static_cast_QIODevice_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::io_device::IODevice {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QIODevice_G_static_cast_QObject_ptr(self as *const ::io_device::IODevice as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::io_device::IODevice {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QIODevice_G_static_cast_QObject_ptr(self as *mut ::io_device::IODevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [IODevice::read_line](../struct.IODevice.html#method.read_line) method.
  pub trait IODeviceReadLineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> ::byte_array::ByteArray;
  }
  impl<'largs> IODeviceReadLineArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> ::byte_array::ByteArray {
      let maxlen = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIODevice_readLine_to_output_maxlen(original_self as *mut ::io_device::IODevice,
                                                               maxlen,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> IODeviceReadLineArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QIODevice_readLine_to_output_no_args(original_self as *mut ::io_device::IODevice,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [IODevice::write_unsafe](../struct.IODevice.html#method.write_unsafe) method.
  pub trait IODeviceWriteUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> i64;
  }
  impl<'largs> IODeviceWriteUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> i64 {
      let data = self;
      ::ffi::qt_core_c_QIODevice_write_char(original_self as *mut ::io_device::IODevice, data)
    }
  }
  impl<'largs> IODeviceWriteUnsafeArgs<'largs> for (*const ::libc::c_char, i64) {
    unsafe fn exec(self, original_self: &'largs mut ::io_device::IODevice) -> i64 {
      let data = self.0;
      let len = self.1;
      ::ffi::qt_core_c_QIODevice_write_char_qint64(original_self as *mut ::io_device::IODevice, data, len)
    }
  }
}
