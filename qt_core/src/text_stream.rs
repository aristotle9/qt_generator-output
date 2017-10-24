/// C++ type: <span style='color: green;'>```QTextStream::FieldAlignment```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FieldAlignment {
  /// C++ enum variant: <span style='color: green;'>```AlignLeft = 0```</span>
  Left = 0,
  /// C++ enum variant: <span style='color: green;'>```AlignRight = 1```</span>
  Right = 1,
  /// C++ enum variant: <span style='color: green;'>```AlignCenter = 2```</span>
  Center = 2,
  /// C++ enum variant: <span style='color: green;'>```AlignAccountingStyle = 3```</span>
  AccountingStyle = 3,
}

/// C++ type: <span style='color: green;'>```QTextStream::NumberFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NumberFlag {
  /// C++ enum variant: <span style='color: green;'>```ShowBase = 1```</span>
  ShowBase = 1,
  /// C++ enum variant: <span style='color: green;'>```ForcePoint = 2```</span>
  ForcePoint = 2,
  /// C++ enum variant: <span style='color: green;'>```ForceSign = 4```</span>
  ForceSign = 4,
  /// C++ enum variant: <span style='color: green;'>```UppercaseBase = 8```</span>
  UppercaseBase = 8,
  /// C++ enum variant: <span style='color: green;'>```UppercaseDigits = 16```</span>
  UppercaseDigits = 16,
}

impl ::flags::FlaggableEnum for NumberFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "NumberFlag"
  }
}

/// C++ type: <span style='color: green;'>```QTextStream::RealNumberNotation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RealNumberNotation {
  /// C++ enum variant: <span style='color: green;'>```SmartNotation = 0```</span>
  Smart = 0,
  /// C++ enum variant: <span style='color: green;'>```FixedNotation = 1```</span>
  Fixed = 1,
  /// C++ enum variant: <span style='color: green;'>```ScientificNotation = 2```</span>
  Scientific = 2,
}

/// C++ type: <span style='color: green;'>```QTextStream::Status```</span>
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

/// C++ type: <span style='color: green;'>```QTextStream```</span>
#[repr(C)]
pub struct TextStream(u8);

impl TextStream {
  /// C++ method: <span style='color: green;'>```bool QTextStream::atEnd() const```</span>
  ///
  ///
  pub fn at_end(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextStream_atEnd(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextStream::autoDetectUnicode() const```</span>
  ///
  ///
  pub fn auto_detect_unicode(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextStream_autoDetectUnicode(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QTextCodec* QTextStream::codec() const```</span>
  ///
  ///
  pub fn codec(&self) -> *mut ::text_codec::TextCodec {
    unsafe { ::ffi::qt_core_c_QTextStream_codec(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QTextStream::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::io_device::IODevice {
    unsafe { ::ffi::qt_core_c_QTextStream_device(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QTextStream::FieldAlignment QTextStream::fieldAlignment() const```</span>
  ///
  ///
  pub fn field_alignment(&self) -> ::text_stream::FieldAlignment {
    unsafe { ::ffi::qt_core_c_QTextStream_fieldAlignment(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```int QTextStream::fieldWidth() const```</span>
  ///
  ///
  pub fn field_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextStream_fieldWidth(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::flush()```</span>
  ///
  ///
  pub fn flush(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextStream_flush(self as *mut ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextStream::generateByteOrderMark() const```</span>
  ///
  ///
  pub fn generate_byte_order_mark(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextStream_generateByteOrderMark(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```int QTextStream::integerBase() const```</span>
  ///
  ///
  pub fn integer_base(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextStream_integerBase(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QLocale QTextStream::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextStream_locale_to_output(self as *const ::text_stream::TextStream, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextStream::QTextStream()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::text_stream::TextStream> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextStream::QTextStream(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(device: *mut ::io_device::IODevice) -> ::cpp_utils::CppBox<::text_stream::TextStream> {
    let ffi_result = ::ffi::qt_core_c_QTextStream_new_device(device);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTextStream::NumberFlag> QTextStream::numberFlags() const```</span>
  ///
  ///
  pub fn number_flags(&self) -> ::flags::Flags<::text_stream::NumberFlag> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_numberFlags(self as *const ::text_stream::TextStream) };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::char::Char) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(QChar ch)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(QLatin1String s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, ::libc::c_char) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(char ch)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(const QByteArray& array)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::string::String) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(const QStringRef& s)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, ::libc::c_double) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(double f)```</span>
  ///
  ///
  pub fn op_shl0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl1(&mut self, *const ::libc::c_char) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(const char* c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl1(&mut self, *const ::libc::c_void) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(const void* ptr)```</span>
  ///
  ///
  pub unsafe fn op_shl1<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, ::libc::c_float) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(float f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, ::libc::c_int) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, u64) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(qulonglong i)```</span>
  ///
  ///
  pub fn op_shl2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl3(&mut self, ::libc::c_long) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(long i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl3(&mut self, ::libc::c_uint) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(unsigned int i)```</span>
  ///
  ///
  pub fn op_shl3<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl3Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl4(&mut self, i64) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(qlonglong i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl4(&mut self, ::libc::c_ulong) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(unsigned long i)```</span>
  ///
  ///
  pub fn op_shl4<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl4Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl5(&mut self, ::libc::c_short) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(short i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl5(&mut self, ::libc::c_ushort) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator<<(unsigned short i)```</span>
  ///
  ///
  pub fn op_shl5<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShl5Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::byte_array::ByteArray) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(QByteArray& array)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::char::Char) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(QChar& ch)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::string::String) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::libc::c_char) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(char& ch)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_shr0(&mut self, &'l1 mut ::libc::c_double) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(double& f)```</span>
  ///
  ///
  pub fn op_shr0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShr0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(char* c)```</span>
  ///
  ///
  pub unsafe fn op_shr1<'l0>(&'l0 mut self, c: *mut ::libc::c_char) -> &'l0 mut ::text_stream::TextStream {
    let ffi_result = ::ffi::qt_core_c_QTextStream_operator_shr_char_c(self as *mut ::text_stream::TextStream, c);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr2(&mut self, &'l1 mut ::libc::c_float) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(float& f)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr2(&mut self, &'l1 mut ::libc::c_int) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(int& i)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shr2(&mut self, &'l1 mut u64) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(qulonglong& i)```</span>
  ///
  ///
  pub fn op_shr2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShr2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr3(&mut self, &'l1 mut ::libc::c_long) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(long& i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr3(&mut self, &'l1 mut ::libc::c_uint) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(unsigned int& i)```</span>
  ///
  ///
  pub fn op_shr3<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShr3Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr4(&mut self, &'l1 mut i64) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(qlonglong& i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr4(&mut self, &'l1 mut ::libc::c_ulong) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(unsigned long& i)```</span>
  ///
  ///
  pub fn op_shr4<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShr4Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::operator>>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shr5(&mut self, &'l1 mut ::libc::c_short) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(short& i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shr5(&mut self, &'l1 mut ::libc::c_ushort) -> &'l0 mut ::text_stream::TextStream```<br>
  /// C++ method: <span style='color: green;'>```QTextStream& QTextStream::operator>>(unsigned short& i)```</span>
  ///
  ///
  pub fn op_shr5<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::text_stream::TextStream
    where Args: overloading::TextStreamOpShr5Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QChar QTextStream::padChar() const```</span>
  ///
  ///
  pub fn pad_char(&self) -> ::char::Char {
    {
      let mut object: ::char::Char = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextStream_padChar_to_output(self as *const ::text_stream::TextStream, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QTextStream::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QTextStream_pos(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextStream::read(qint64 maxlen)```</span>
  ///
  ///
  pub fn read(&mut self, maxlen: i64) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextStream_read_to_output(self as *mut ::text_stream::TextStream, maxlen, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextStream::readAll()```</span>
  ///
  ///
  pub fn read_all(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextStream_readAll_to_output(self as *mut ::text_stream::TextStream, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextStream::readLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn read_line(&mut self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextStream::readLine()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn read_line(&mut self, i64) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QTextStream::readLine(qint64 maxlen = ?)```</span>
  ///
  ///
  pub fn read_line<'largs, Args>(&'largs mut self, args: Args) -> ::string::String
    where Args: overloading::TextStreamReadLineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::readLineInto```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn read_line_into(&mut self, *mut ::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextStream::readLineInto(QString* line)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn read_line_into(&mut self, (*mut ::string::String, i64)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QTextStream::readLineInto(QString* line, qint64 maxlen = ?)```</span>
  ///
  ///
  pub unsafe fn read_line_into<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::TextStreamReadLineIntoArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextStream::RealNumberNotation QTextStream::realNumberNotation() const```</span>
  ///
  ///
  pub fn real_number_notation(&self) -> ::text_stream::RealNumberNotation {
    unsafe { ::ffi::qt_core_c_QTextStream_realNumberNotation(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```int QTextStream::realNumberPrecision() const```</span>
  ///
  ///
  pub fn real_number_precision(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextStream_realNumberPrecision(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextStream_reset(self as *mut ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::resetStatus()```</span>
  ///
  ///
  pub fn reset_status(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextStream_resetStatus(self as *mut ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextStream::seek(qint64 pos)```</span>
  ///
  ///
  pub fn seek(&mut self, pos: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QTextStream_seek(self as *mut ::text_stream::TextStream, pos) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setAutoDetectUnicode(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_detect_unicode(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_core_c_QTextStream_setAutoDetectUnicode(self as *mut ::text_stream::TextStream, enabled) }
  }

  /// C++ method: <span style='color: green;'>```QTextStream::setCodec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_codec(&mut self, *mut ::text_codec::TextCodec) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextStream::setCodec(QTextCodec* codec)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_codec(&mut self, *const ::libc::c_char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextStream::setCodec(const char* codecName)```</span>
  ///
  ///
  pub unsafe fn set_codec<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextStreamSetCodecArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextStream::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::io_device::IODevice) {
    ::ffi::qt_core_c_QTextStream_setDevice(self as *mut ::text_stream::TextStream, device)
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setFieldAlignment(QTextStream::FieldAlignment alignment)```</span>
  ///
  ///
  pub fn set_field_alignment(&mut self, alignment: ::text_stream::FieldAlignment) {
    unsafe { ::ffi::qt_core_c_QTextStream_setFieldAlignment(self as *mut ::text_stream::TextStream, alignment) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setFieldWidth(int width)```</span>
  ///
  ///
  pub fn set_field_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTextStream_setFieldWidth(self as *mut ::text_stream::TextStream, width) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setGenerateByteOrderMark(bool generate)```</span>
  ///
  ///
  pub fn set_generate_byte_order_mark(&mut self, generate: bool) {
    unsafe { ::ffi::qt_core_c_QTextStream_setGenerateByteOrderMark(self as *mut ::text_stream::TextStream, generate) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setIntegerBase(int base)```</span>
  ///
  ///
  pub fn set_integer_base(&mut self, base: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTextStream_setIntegerBase(self as *mut ::text_stream::TextStream, base) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setLocale(const QLocale& locale)```</span>
  ///
  ///
  pub fn set_locale(&mut self, locale: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QTextStream_setLocale(self as *mut ::text_stream::TextStream,
                                             locale as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setNumberFlags(QFlags<QTextStream::NumberFlag> flags)```</span>
  ///
  ///
  pub fn set_number_flags(&mut self, flags: ::flags::Flags<::text_stream::NumberFlag>) {
    unsafe {
      ::ffi::qt_core_c_QTextStream_setNumberFlags(self as *mut ::text_stream::TextStream,
                                                  flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setPadChar(QChar ch)```</span>
  ///
  ///
  pub fn set_pad_char(&mut self, ch: &::char::Char) {
    unsafe {
      ::ffi::qt_core_c_QTextStream_setPadChar(self as *mut ::text_stream::TextStream,
                                              ch as *const ::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setRealNumberNotation(QTextStream::RealNumberNotation notation)```</span>
  ///
  ///
  pub fn set_real_number_notation(&mut self, notation: ::text_stream::RealNumberNotation) {
    unsafe { ::ffi::qt_core_c_QTextStream_setRealNumberNotation(self as *mut ::text_stream::TextStream, notation) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setRealNumberPrecision(int precision)```</span>
  ///
  ///
  pub fn set_real_number_precision(&mut self, precision: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTextStream_setRealNumberPrecision(self as *mut ::text_stream::TextStream, precision) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::setStatus(QTextStream::Status status)```</span>
  ///
  ///
  pub fn set_status(&mut self, status: ::text_stream::Status) {
    unsafe { ::ffi::qt_core_c_QTextStream_setStatus(self as *mut ::text_stream::TextStream, status) }
  }

  /// C++ method: <span style='color: green;'>```void QTextStream::skipWhiteSpace()```</span>
  ///
  ///
  pub fn skip_white_space(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextStream_skipWhiteSpace(self as *mut ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QTextStream::Status QTextStream::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::text_stream::Status {
    unsafe { ::ffi::qt_core_c_QTextStream_status(self as *const ::text_stream::TextStream) }
  }

  /// C++ method: <span style='color: green;'>```QString* QTextStream::string() const```</span>
  ///
  ///
  pub fn string(&self) -> *mut ::string::String {
    unsafe { ::ffi::qt_core_c_QTextStream_string(self as *const ::text_stream::TextStream) }
  }
}

impl ::cpp_utils::CppDeletable for ::text_stream::TextStream {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTextStream_delete
  }
}

/// C++ method: <span style='color: green;'>```QTextStream& bin(QTextStream& s)```</span>
///
///
pub fn bin<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_bin(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& bom(QTextStream& s)```</span>
///
///
pub fn bom<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_bom(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& center(QTextStream& s)```</span>
///
///
pub fn center<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_center(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& dec(QTextStream& s)```</span>
///
///
pub fn dec<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_dec(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& endl(QTextStream& s)```</span>
///
///
pub fn endl<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_endl(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& fixed(QTextStream& s)```</span>
///
///
pub fn fixed<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_fixed(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& flush(QTextStream& s)```</span>
///
///
pub fn flush<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_flush(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& forcepoint(QTextStream& s)```</span>
///
///
pub fn forcepoint<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_forcepoint(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& forcesign(QTextStream& s)```</span>
///
///
pub fn forcesign<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_forcesign(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& hex(QTextStream& s)```</span>
///
///
pub fn hex<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_hex(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& left(QTextStream& s)```</span>
///
///
pub fn left<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_left(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& lowercasebase(QTextStream& s)```</span>
///
///
pub fn lowercasebase<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_lowercasebase(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& lowercasedigits(QTextStream& s)```</span>
///
///
pub fn lowercasedigits<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_lowercasedigits(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& noforcepoint(QTextStream& s)```</span>
///
///
pub fn noforcepoint<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_noforcepoint(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& noforcesign(QTextStream& s)```</span>
///
///
pub fn noforcesign<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_noforcesign(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& noshowbase(QTextStream& s)```</span>
///
///
pub fn noshowbase<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_noshowbase(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& oct(QTextStream& s)```</span>
///
///
pub fn oct<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_oct(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& operator<<(QTextStream& s, QTextStreamManipulator m)```</span>
///
///
pub fn op_shl<'l0, 'l1>(s: &'l0 mut ::text_stream::TextStream,
                        m: &'l1 ::text_stream_manipulator::TextStreamManipulator)
                        -> &'l0 mut ::text_stream::TextStream {
  let ffi_result =
    unsafe {
      ::ffi::qt_core_c_QTextStream_G_operator_shl(s as *mut ::text_stream::TextStream,
                                                  m as *const ::text_stream_manipulator::TextStreamManipulator)
    };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& reset(QTextStream& s)```</span>
///
///
pub fn reset<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_reset(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& right(QTextStream& s)```</span>
///
///
pub fn right<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_right(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& scientific(QTextStream& s)```</span>
///
///
pub fn scientific<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_scientific(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStreamManipulator qSetFieldWidth(int width)```</span>
///
///
pub fn set_field_width(width: ::libc::c_int) -> ::text_stream_manipulator::TextStreamManipulator {
  {
    let mut object: ::text_stream_manipulator::TextStreamManipulator =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QTextStream_G_qSetFieldWidth_to_output(width, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QTextStreamManipulator qSetPadChar(QChar ch)```</span>
///
///
pub fn set_pad_char(ch: &::char::Char) -> ::text_stream_manipulator::TextStreamManipulator {
  {
    let mut object: ::text_stream_manipulator::TextStreamManipulator =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QTextStream_G_qSetPadChar_to_output(ch as *const ::char::Char, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QTextStreamManipulator qSetRealNumberPrecision(int precision)```</span>
///
///
pub fn set_real_number_precision(precision: ::libc::c_int) -> ::text_stream_manipulator::TextStreamManipulator {
  {
    let mut object: ::text_stream_manipulator::TextStreamManipulator =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QTextStream_G_qSetRealNumberPrecision_to_output(precision, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```QTextStream& showbase(QTextStream& s)```</span>
///
///
pub fn showbase<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_showbase(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& uppercasebase(QTextStream& s)```</span>
///
///
pub fn uppercasebase<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_uppercasebase(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& uppercasedigits(QTextStream& s)```</span>
///
///
pub fn uppercasedigits<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_uppercasedigits(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QTextStream& ws(QTextStream& s)```</span>
///
///
pub fn ws<'l0>(s: &'l0 mut ::text_stream::TextStream) -> &'l0 mut ::text_stream::TextStream {
  let ffi_result = unsafe { ::ffi::qt_core_c_QTextStream_G_ws(s as *mut ::text_stream::TextStream) };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextStream::op_shl0](../struct.TextStream.html#method.op_shl0) method.
  pub trait TextStreamOpShl0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let array = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_QByteArray_array(original_self as *mut ::text_stream::TextStream,
                                                                     array as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let ch = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_QChar_ch(original_self as *mut ::text_stream::TextStream,
                                                             ch as *const ::char::Char)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_QLatin1String_s(original_self as *mut ::text_stream::TextStream,
                                                                    s as *const ::latin1_string::Latin1String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_QStringRef_s(original_self as *mut ::text_stream::TextStream,
                                                                 s as *const ::string_ref::StringRef)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_QString_s(original_self as *mut ::text_stream::TextStream,
                                                              s as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let ch = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_char_ch(original_self as *mut ::text_stream::TextStream, ch)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let f = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_double_f(original_self as *mut ::text_stream::TextStream, f)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shl1](../struct.TextStream.html#method.op_shl1) method.
  pub trait TextStreamOpShl1Args<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::text_stream::TextStream)
                   -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl1Args<'largs> for *const ::libc::c_char {
    unsafe fn exec(self,
                   original_self: &'largs mut ::text_stream::TextStream)
                   -> &'largs mut ::text_stream::TextStream {
      let c = self;
      let ffi_result =
        ::ffi::qt_core_c_QTextStream_operator_shl_char_c(original_self as *mut ::text_stream::TextStream, c);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl1Args<'largs> for *const ::libc::c_void {
    unsafe fn exec(self,
                   original_self: &'largs mut ::text_stream::TextStream)
                   -> &'largs mut ::text_stream::TextStream {
      let ptr = self;
      let ffi_result =
        ::ffi::qt_core_c_QTextStream_operator_shl_void_ptr(original_self as *mut ::text_stream::TextStream, ptr);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shl2](../struct.TextStream.html#method.op_shl2) method.
  pub trait TextStreamOpShl2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl2Args<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let f = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_float_f(original_self as *mut ::text_stream::TextStream, f)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl2Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QTextStream_operator_shl_int_i(original_self as *mut ::text_stream::TextStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl2Args<'largs> for u64 {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_qulonglong_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shl3](../struct.TextStream.html#method.op_shl3) method.
  pub trait TextStreamOpShl3Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl3Args<'largs> for ::libc::c_long {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QTextStream_operator_shl_long_i(original_self as *mut ::text_stream::TextStream, i) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl3Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_unsigned_int_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shl4](../struct.TextStream.html#method.op_shl4) method.
  pub trait TextStreamOpShl4Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl4Args<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_qlonglong_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl4Args<'largs> for ::libc::c_ulong {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_unsigned_long_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shl5](../struct.TextStream.html#method.op_shl5) method.
  pub trait TextStreamOpShl5Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShl5Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_short_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShl5Args<'largs> for ::libc::c_ushort {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shl_unsigned_short_i(original_self as *mut ::text_stream::TextStream, i)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shr0](../struct.TextStream.html#method.op_shr0) method.
  pub trait TextStreamOpShr0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShr0Args<'largs> for &'largs mut ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let array = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_QByteArray_array(original_self as *mut ::text_stream::TextStream,
                                                                     array as *mut ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr0Args<'largs> for &'largs mut ::char::Char {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let ch = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_QChar_ch(original_self as *mut ::text_stream::TextStream,
                                                             ch as *mut ::char::Char)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr0Args<'largs> for &'largs mut ::string::String {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_QString_s(original_self as *mut ::text_stream::TextStream,
                                                              s as *mut ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr0Args<'largs> for &'largs mut ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let ch = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_char_ch(original_self as *mut ::text_stream::TextStream,
                                                            ch as *mut ::libc::c_char)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr0Args<'largs> for &'largs mut ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let f = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_double_f(original_self as *mut ::text_stream::TextStream,
                                                             f as *mut ::libc::c_double)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shr2](../struct.TextStream.html#method.op_shr2) method.
  pub trait TextStreamOpShr2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShr2Args<'largs> for &'largs mut ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let f = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_float_f(original_self as *mut ::text_stream::TextStream,
                                                            f as *mut ::libc::c_float)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr2Args<'largs> for &'largs mut ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_int_i(original_self as *mut ::text_stream::TextStream,
                                                          i as *mut ::libc::c_int)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr2Args<'largs> for &'largs mut u64 {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_qulonglong_i(original_self as *mut ::text_stream::TextStream,
                                                                 i as *mut u64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shr3](../struct.TextStream.html#method.op_shr3) method.
  pub trait TextStreamOpShr3Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShr3Args<'largs> for &'largs mut ::libc::c_long {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_long_i(original_self as *mut ::text_stream::TextStream,
                                                           i as *mut ::libc::c_long)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr3Args<'largs> for &'largs mut ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_unsigned_int_i(original_self as *mut ::text_stream::TextStream,
                                                                   i as *mut ::libc::c_uint)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shr4](../struct.TextStream.html#method.op_shr4) method.
  pub trait TextStreamOpShr4Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShr4Args<'largs> for &'largs mut i64 {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_qlonglong_i(original_self as *mut ::text_stream::TextStream,
                                                                i as *mut i64)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr4Args<'largs> for &'largs mut ::libc::c_ulong {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_unsigned_long_i(original_self as *mut ::text_stream::TextStream,
                                                                    i as *mut ::libc::c_ulong)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::op_shr5](../struct.TextStream.html#method.op_shr5) method.
  pub trait TextStreamOpShr5Args<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream;
  }
  impl<'largs> TextStreamOpShr5Args<'largs> for &'largs mut ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_short_i(original_self as *mut ::text_stream::TextStream,
                                                            i as *mut ::libc::c_short)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> TextStreamOpShr5Args<'largs> for &'largs mut ::libc::c_ushort {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> &'largs mut ::text_stream::TextStream {
      let i = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QTextStream_operator_shr_unsigned_short_i(original_self as *mut ::text_stream::TextStream,
                                                                     i as *mut ::libc::c_ushort)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::read_line](../struct.TextStream.html#method.read_line) method.
  pub trait TextStreamReadLineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> ::string::String;
  }
  impl<'largs> TextStreamReadLineArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> ::string::String {
      let maxlen = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTextStream_readLine_to_output_maxlen(original_self as *mut ::text_stream::TextStream,
                                                                 maxlen,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextStreamReadLineArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTextStream_readLine_to_output_no_args(original_self as *mut ::text_stream::TextStream,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::read_line_into](../struct.TextStream.html#method.read_line_into) method.
  pub trait TextStreamReadLineIntoArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> bool;
  }
  impl<'largs> TextStreamReadLineIntoArgs<'largs> for *mut ::string::String {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> bool {
      let line = self;
      ::ffi::qt_core_c_QTextStream_readLineInto_line(original_self as *mut ::text_stream::TextStream, line)
    }
  }
  impl<'largs> TextStreamReadLineIntoArgs<'largs> for (*mut ::string::String, i64) {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> bool {
      let line = self.0;
      let maxlen = self.1;
      ::ffi::qt_core_c_QTextStream_readLineInto_line_maxlen(original_self as *mut ::text_stream::TextStream,
                                                            line,
                                                            maxlen)
    }
  }
  /// This trait represents a set of arguments accepted by [TextStream::set_codec](../struct.TextStream.html#method.set_codec) method.
  pub trait TextStreamSetCodecArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> ();
  }
  impl<'largs> TextStreamSetCodecArgs<'largs> for *mut ::text_codec::TextCodec {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> () {
      let codec = self;
      ::ffi::qt_core_c_QTextStream_setCodec_codec(original_self as *mut ::text_stream::TextStream, codec)
    }
  }
  impl<'largs> TextStreamSetCodecArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::text_stream::TextStream) -> () {
      let codec_name = self;
      ::ffi::qt_core_c_QTextStream_setCodec_codecName(original_self as *mut ::text_stream::TextStream, codec_name)
    }
  }
}
