/// C++ type: <span style='color: green;'>```QDataStream::ByteOrder```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ByteOrder {
  /// C++ enum variant: <span style='color: green;'>```BigEndian = 0```</span>
  Big = 0,
  /// C++ enum variant: <span style='color: green;'>```LittleEndian = 1```</span>
  Little = 1,
}

/// C++ type: <span style='color: green;'>```QDataStream```</span>
#[repr(C)]
pub struct DataStream([u8; ::type_sizes::QT_CORE_DATA_STREAM_DATA_STREAM]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for DataStream {
  unsafe fn new_uninitialized() -> DataStream {
    DataStream(::std::mem::uninitialized())
  }
}

impl DataStream {
  /// C++ method: <span style='color: green;'>```void QDataStream::abortTransaction()```</span>
  ///
  ///
  pub fn abort_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_abortTransaction(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```bool QDataStream::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDataStream_atEnd(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QDataStream::ByteOrder QDataStream::byteOrder() const```</span>
  ///
  ///
  pub fn byte_order(&self) -> ::data_stream::ByteOrder {
    unsafe { ::ffi::qt_core_c_QDataStream_byteOrder(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```bool QDataStream::commitTransaction()```</span>
  ///
  ///
  pub fn commit_transaction(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QDataStream_commitTransaction(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QDataStream::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::io_device::IODevice {
    unsafe { ::ffi::qt_core_c_QDataStream_device(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QDataStream::FloatingPointPrecision QDataStream::floatingPointPrecision() const```</span>
  ///
  ///
  pub fn floating_point_precision(&self) -> ::data_stream::FloatingPointPrecision {
    unsafe { ::ffi::qt_core_c_QDataStream_floatingPointPrecision(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QDataStream::QDataStream```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDataStream::QDataStream()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDataStream::QDataStream(const QByteArray& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::data_stream::DataStream
    where Args: overloading::DataStreamNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QDataStream::QDataStream(QIODevice* arg1)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: *mut ::io_device::IODevice) -> ::data_stream::DataStream {
    {
      let mut object: ::data_stream::DataStream =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QDataStream_constructor_QIODevice(arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDataStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, bool) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(bool i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, ::libc::c_double) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(double f)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, i16) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(qint16 i)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, i32) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(qint32 i)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, i64) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(qint64 i)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, i8) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(qint8 i)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, u16) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(quint16 i)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, u32) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(quint32 i)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, u64) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(quint64 i)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, u8) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(quint8 i)```</span>
  ///
  ///
  pub fn op_shl0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::data_stream::DataStream
    where Args: overloading::DataStreamOpShl0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(const char* str)```</span>
  ///
  ///
  pub unsafe fn op_shl1<'l0>(&'l0 mut self, str: *const ::libc::c_char) -> &'l0 mut ::data_stream::DataStream {
    let ffi_result = ::ffi::qt_core_c_QDataStream_operator_shl_char(self as *mut ::data_stream::DataStream, str);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator<<(float f)```</span>
  ///
  ///
  pub fn op_shl2<'l0>(&'l0 mut self, f: ::libc::c_float) -> &'l0 mut ::data_stream::DataStream {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_float(self as *mut ::data_stream::DataStream, f) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDataStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut bool) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(bool& i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::libc::c_double) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(double& f)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut i16) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(qint16& i)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut i32) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(qint32& i)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut i64) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(qint64& i)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut i8) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(qint8& i)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut u16) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(quint16& i)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut u32) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(quint32& i)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut u64) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(quint64& i)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut u8) -> &'l0 mut ::data_stream::DataStream```<br>
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(quint8& i)```</span>
  ///
  ///
  pub fn op_shr0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::data_stream::DataStream
    where Args: overloading::DataStreamOpShr0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(char*& str)```</span>
  ///
  ///
  pub unsafe fn op_shr1<'l0, 'l1>(&'l0 mut self,
                                  str: &'l1 mut *mut ::libc::c_char)
                                  -> &'l0 mut ::data_stream::DataStream {
    let ffi_result = ::ffi::qt_core_c_QDataStream_operator_shr_char(self as *mut ::data_stream::DataStream,
                                                                    str as *mut *mut ::libc::c_char);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::operator>>(float& f)```</span>
  ///
  ///
  pub fn op_shr2<'l0, 'l1>(&'l0 mut self, f: &'l1 mut ::libc::c_float) -> &'l0 mut ::data_stream::DataStream {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QDataStream_operator_shr_float(self as *mut ::data_stream::DataStream,
                                                      f as *mut ::libc::c_float)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::readBytes(char*& arg1, unsigned int& len)```</span>
  ///
  ///
  pub unsafe fn read_bytes<'l0, 'l1, 'l2>(&'l0 mut self,
                                          arg1: &'l1 mut *mut ::libc::c_char,
                                          len: &'l2 mut ::libc::c_uint)
                                          -> &'l0 mut ::data_stream::DataStream {
    let ffi_result = ::ffi::qt_core_c_QDataStream_readBytes(self as *mut ::data_stream::DataStream,
                                                            arg1 as *mut *mut ::libc::c_char,
                                                            len as *mut ::libc::c_uint);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QDataStream::readRawData(char* arg1, int len)```</span>
  ///
  ///
  pub unsafe fn read_raw_data(&mut self, arg1: *mut ::libc::c_char, len: ::libc::c_int) -> ::libc::c_int {
    ::ffi::qt_core_c_QDataStream_readRawData(self as *mut ::data_stream::DataStream, arg1, len)
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::resetStatus()```</span>
  ///
  ///
  pub fn reset_status(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_resetStatus(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::rollbackTransaction()```</span>
  ///
  ///
  pub fn rollback_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_rollbackTransaction(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::setByteOrder(QDataStream::ByteOrder arg1)```</span>
  ///
  ///
  pub fn set_byte_order(&mut self, arg1: ::data_stream::ByteOrder) {
    unsafe { ::ffi::qt_core_c_QDataStream_setByteOrder(self as *mut ::data_stream::DataStream, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::setDevice(QIODevice* arg1)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, arg1: *mut ::io_device::IODevice) {
    ::ffi::qt_core_c_QDataStream_setDevice(self as *mut ::data_stream::DataStream, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::setFloatingPointPrecision(QDataStream::FloatingPointPrecision precision)```</span>
  ///
  ///
  pub fn set_floating_point_precision(&mut self, precision: ::data_stream::FloatingPointPrecision) {
    unsafe { ::ffi::qt_core_c_QDataStream_setFloatingPointPrecision(self as *mut ::data_stream::DataStream, precision) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::setStatus(QDataStream::Status status)```</span>
  ///
  ///
  pub fn set_status(&mut self, status: ::data_stream::Status) {
    unsafe { ::ffi::qt_core_c_QDataStream_setStatus(self as *mut ::data_stream::DataStream, status) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::setVersion(int arg1)```</span>
  ///
  ///
  pub fn set_version(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QDataStream_setVersion(self as *mut ::data_stream::DataStream, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QDataStream::skipRawData(int len)```</span>
  ///
  ///
  pub fn skip_raw_data(&mut self, len: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDataStream_skipRawData(self as *mut ::data_stream::DataStream, len) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::startTransaction()```</span>
  ///
  ///
  pub fn start_transaction(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_startTransaction(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QDataStream::Status QDataStream::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::data_stream::Status {
    unsafe { ::ffi::qt_core_c_QDataStream_status(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```void QDataStream::unsetDevice()```</span>
  ///
  ///
  pub fn unset_device(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_unsetDevice(self as *mut ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```int QDataStream::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDataStream_version(self as *const ::data_stream::DataStream) }
  }

  /// C++ method: <span style='color: green;'>```QDataStream& QDataStream::writeBytes(const char* arg1, unsigned int len)```</span>
  ///
  ///
  pub unsafe fn write_bytes<'l0>(&'l0 mut self,
                                 arg1: *const ::libc::c_char,
                                 len: ::libc::c_uint)
                                 -> &'l0 mut ::data_stream::DataStream {
    let ffi_result = ::ffi::qt_core_c_QDataStream_writeBytes(self as *mut ::data_stream::DataStream, arg1, len);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QDataStream::writeRawData(const char* arg1, int len)```</span>
  ///
  ///
  pub unsafe fn write_raw_data(&mut self, arg1: *const ::libc::c_char, len: ::libc::c_int) -> ::libc::c_int {
    ::ffi::qt_core_c_QDataStream_writeRawData(self as *mut ::data_stream::DataStream, arg1, len)
  }
}

impl Drop for ::data_stream::DataStream {
  /// C++ method: <span style='color: green;'>```[destructor] void QDataStream::~QDataStream()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDataStream_destructor(self as *mut ::data_stream::DataStream) }
  }
}

/// C++ type: <span style='color: green;'>```QDataStream::FloatingPointPrecision```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FloatingPointPrecision {
  /// C++ enum variant: <span style='color: green;'>```SinglePrecision = 0```</span>
  Single = 0,
  /// C++ enum variant: <span style='color: green;'>```DoublePrecision = 1```</span>
  Double = 1,
}

/// C++ type: <span style='color: green;'>```QDataStream::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```Ok = 0```</span>
  Ok = 0,
  /// C++ enum variant: <span style='color: green;'>```ReadPastEnd = 1```</span>
  ReadPastEnd = 1,
  /// C++ enum variant: <span style='color: green;'>```ReadCorruptData = 2```</span>
  ReadCorruptData = 2,
  /// C++ enum variant: <span style='color: green;'>```WriteFailed = 3```</span>
  WriteFailed = 3,
}

/// C++ type: <span style='color: green;'>```QDataStream::Version```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Version {
  /// C++ enum variant: <span style='color: green;'>```Qt_1_0 = 1```</span>
  Qt10 = 1,
  /// C++ enum variant: <span style='color: green;'>```Qt_2_0 = 2```</span>
  Qt20 = 2,
  /// C++ enum variant: <span style='color: green;'>```Qt_2_1 = 3```</span>
  Qt21 = 3,
  /// C++ enum variant: <span style='color: green;'>```Qt_3_0 = 4```</span>
  Qt30 = 4,
  /// C++ enum variant: <span style='color: green;'>```Qt_3_1 = 5```</span>
  Qt31 = 5,
  /// C++ enum variant: <span style='color: green;'>```Qt_3_3 = 6```</span>
  Qt33 = 6,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Qt_4_0 = 7```</span>
  /// - <span style='color: green;'>```Qt_4_1 = 7```</span>
  ///
  Qt40 = 7,
  /// C++ enum variant: <span style='color: green;'>```Qt_4_2 = 8```</span>
  Qt42 = 8,
  /// C++ enum variant: <span style='color: green;'>```Qt_4_3 = 9```</span>
  Qt43 = 9,
  /// C++ enum variant: <span style='color: green;'>```Qt_4_4 = 10```</span>
  Qt44 = 10,
  /// C++ enum variant: <span style='color: green;'>```Qt_4_5 = 11```</span>
  Qt45 = 11,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Qt_4_6 = 12```</span>
  /// - <span style='color: green;'>```Qt_4_7 = 12```</span>
  /// - <span style='color: green;'>```Qt_4_8 = 12```</span>
  /// - <span style='color: green;'>```Qt_4_9 = 12```</span>
  ///
  Qt46 = 12,
  /// C++ enum variant: <span style='color: green;'>```Qt_5_0 = 13```</span>
  Qt50 = 13,
  /// C++ enum variant: <span style='color: green;'>```Qt_5_1 = 14```</span>
  Qt51 = 14,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Qt_5_2 = 15```</span>
  /// - <span style='color: green;'>```Qt_5_3 = 15```</span>
  ///
  Qt52 = 15,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Qt_5_4 = 16```</span>
  /// - <span style='color: green;'>```Qt_5_5 = 16```</span>
  ///
  Qt54 = 16,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Qt_5_6 = 17```</span>
  /// - <span style='color: green;'>```Qt_5_7 = 17```</span>
  /// - <span style='color: green;'>```Qt_5_8 = 17```</span>
  /// - <span style='color: green;'>```Qt_5_9 = 17```</span>
  /// - <span style='color: green;'>```Qt_DefaultCompiledVersion = 17```</span>
  ///
  Qt56 = 17,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DataStream::new](../struct.DataStream.html#method.new) method.
  pub trait DataStreamNewArgs {
    fn exec(self) -> ::data_stream::DataStream;
  }
  impl<'a> DataStreamNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::data_stream::DataStream {
      let arg1 = self;
      {
        let mut object: ::data_stream::DataStream =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDataStream_constructor_QByteArray(arg1 as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl DataStreamNewArgs for () {
    fn exec(self) -> ::data_stream::DataStream {

      {
        let mut object: ::data_stream::DataStream =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDataStream_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DataStream::op_shl0](../struct.DataStream.html#method.op_shl0) method.
  pub trait DataStreamOpShl0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_bool(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let f = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_double(original_self as *mut ::data_stream::DataStream, f) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for i16 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_qint16(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for i32 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_qint32(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_qint64(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for i8 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_qint8(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for u16 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shl_quint16(original_self as *mut ::data_stream::DataStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for u32 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shl_quint32(original_self as *mut ::data_stream::DataStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for u64 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shl_quint64(original_self as *mut ::data_stream::DataStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShl0Args<'largs> for u8 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDataStream_operator_shl_quint8(original_self as *mut ::data_stream::DataStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [DataStream::op_shr0](../struct.DataStream.html#method.op_shr0) method.
  pub trait DataStreamOpShr0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut bool {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDataStream_operator_shr_bool(original_self as *mut ::data_stream::DataStream,
                                                       i as *mut bool)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let f = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_double(original_self as *mut ::data_stream::DataStream,
                                                           f as *mut ::libc::c_double)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut i16 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_qint16(original_self as *mut ::data_stream::DataStream,
                                                           i as *mut i16)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut i32 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_qint32(original_self as *mut ::data_stream::DataStream,
                                                           i as *mut i32)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut i64 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_qint64(original_self as *mut ::data_stream::DataStream,
                                                           i as *mut i64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut i8 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_qint8(original_self as *mut ::data_stream::DataStream,
                                                          i as *mut i8)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut u16 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_quint16(original_self as *mut ::data_stream::DataStream,
                                                            i as *mut u16)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut u32 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_quint32(original_self as *mut ::data_stream::DataStream,
                                                            i as *mut u32)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut u64 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_quint64(original_self as *mut ::data_stream::DataStream,
                                                            i as *mut u64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DataStreamOpShr0Args<'largs> for &'largs mut u8 {
    fn exec(self, original_self: &'largs mut ::data_stream::DataStream) -> &'largs mut ::data_stream::DataStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QDataStream_operator_shr_quint8(original_self as *mut ::data_stream::DataStream,
                                                           i as *mut u8)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
