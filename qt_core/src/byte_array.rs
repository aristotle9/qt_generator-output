/// C++ type: <span style='color: green;'>```QByteArray::Base64Option```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Base64Option {
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Base64Encoding = 0```</span>
  /// - <span style='color: green;'>```KeepTrailingEquals = 0```</span>
  ///
  Base64Encoding = 0,
  /// C++ enum variant: <span style='color: green;'>```Base64UrlEncoding = 1```</span>
  Base64UrlEncoding = 1,
  /// C++ enum variant: <span style='color: green;'>```OmitTrailingEquals = 2```</span>
  OmitTrailingEquals = 2,
}

impl ::flags::FlaggableEnum for Base64Option {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "Base64Option"
  }
}

/// C++ type: <span style='color: green;'>```QByteArray```</span>
#[repr(C)]
pub struct ByteArray([u8; ::type_sizes::QT_CORE_BYTE_ARRAY_BYTE_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ByteArray {
  unsafe fn new_uninitialized() -> ByteArray {
    ByteArray(::std::mem::uninitialized())
  }
}

impl ByteArray {
  /// C++ method: <span style='color: green;'>```QByteArray::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn append(&mut self, &'l1 ::string::String) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn append(&mut self, (::libc::c_int, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(int count, char c)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append_unsafe(&mut self, *const ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(const char* s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::append(const char* s, int len)```</span>
  ///
  ///
  pub unsafe fn append_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayAppendUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const char* QByteArray::operator const char *() const```</span>
  ///
  ///
  pub fn as_const_char_ptr(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_convert_to_const_char_ptr(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const void* QByteArray::operator const void *() const```</span>
  ///
  ///
  pub fn as_const_void_ptr(&self) -> *const ::libc::c_void {
    unsafe { ::ffi::qt_core_c_QByteArray_convert_to_const_void_ptr(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```char QByteArray::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_at(self as *const ::byte_array::ByteArray, i) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_begin_const(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```char* QByteArray::begin()```</span>
  ///
  ///
  pub fn begin_mut(&mut self) -> *mut ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_begin(self as *mut ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```int QByteArray::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QByteArray_capacity(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::cbegin() const```</span>
  ///
  ///
  pub fn cbegin(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_cbegin(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::cend() const```</span>
  ///
  ///
  pub fn cend(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_cend(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::chop(int n)```</span>
  ///
  ///
  pub fn chop(&mut self, n: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QByteArray_chop(self as *mut ::byte_array::ByteArray, n) }
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QByteArray_clear(self as *mut ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::constBegin() const```</span>
  ///
  ///
  pub fn const_begin(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_constBegin(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_constData(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::constEnd() const```</span>
  ///
  ///
  pub fn const_end(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_constEnd(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, ::libc::c_char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::contains(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::contains(const QByteArray& a) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ByteArrayContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteArray::contains(const char* a) const```</span>
  ///
  ///
  pub unsafe fn contains_unsafe(&self, a: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QByteArray_contains_char_a(self as *const ::byte_array::ByteArray, a)
  }

  /// C++ method: <span style='color: green;'>```QByteArray::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::count(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn count(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::count(const QByteArray& a) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QByteArray::count(const char* a) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, a: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_core_c_QByteArray_count_char_a(self as *const ::byte_array::ByteArray, a)
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_data_const(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```char* QByteArray::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_data(self as *mut ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const char* QByteArray::end() const```</span>
  ///
  ///
  pub fn end(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_end_const(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```char* QByteArray::end()```</span>
  ///
  ///
  pub fn end_mut(&mut self) -> *mut ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QByteArray_end(self as *mut ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::endsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ends_with(&self, ::libc::c_char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::endsWith(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ends_with(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::endsWith(const QByteArray& a) const```</span>
  ///
  ///
  pub fn ends_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ByteArrayEndsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteArray::endsWith(const char* c) const```</span>
  ///
  ///
  pub unsafe fn ends_with_unsafe(&self, c: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QByteArray_endsWith_const_char_ptr(self as *const ::byte_array::ByteArray, c)
  }

  /// C++ method: <span style='color: green;'>```QByteArray::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::fill(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::fill(char c, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::fromBase64```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_base64(&::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromBase64(const QByteArray& base64)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_base64((&::byte_array::ByteArray, ::flags::Flags<::byte_array::Base64Option>)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromBase64(const QByteArray& base64, QFlags<QByteArray::Base64Option> options)```</span>
  ///
  ///
  pub fn from_base64<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayFromBase64Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromHex(const QByteArray& hexEncoded)```</span>
  ///
  ///
  pub fn from_hex(hex_encoded: &::byte_array::ByteArray) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_fromHex_to_output(hex_encoded as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::fromPercentEncoding```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_percent_encoding(&::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromPercentEncoding(const QByteArray& pctEncoded)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_percent_encoding((&::byte_array::ByteArray, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromPercentEncoding(const QByteArray& pctEncoded, char percent = ?)```</span>
  ///
  ///
  pub fn from_percent_encoding<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayFromPercentEncodingArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::fromRawData(const char* arg1, int size)```</span>
  ///
  ///
  pub unsafe fn from_raw_data(arg1: *const ::libc::c_char, size: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QByteArray_fromRawData_to_output(arg1, size, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (::libc::c_char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(char c, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index_of(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const QByteArray& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn index_of(&self, (&::byte_array::ByteArray, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const QByteArray& a, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const QString& s, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of_unsafe(&self, *const ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const char* c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of_unsafe(&self, (*const ::libc::c_char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::indexOf(const char* c, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayIndexOfUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &'l1 ::string::String)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, int count, char c)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert_unsafe(&mut self, (::libc::c_int, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, const char* s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert_unsafe(&mut self, (::libc::c_int, *const ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::insert(int i, const char* s, int len)```</span>
  ///
  ///
  pub unsafe fn insert_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayInsertUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteArray::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QByteArray_isEmpty(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QByteArray_isNull(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (::libc::c_char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(char c, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const QByteArray& a) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::byte_array::ByteArray, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const QByteArray& a, int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const QString& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const QString& s, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of_unsafe(&self, *const ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const char* c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of_unsafe(&self, (*const ::libc::c_char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::lastIndexOf(const char* c, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayLastIndexOfUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::left(int len) const```</span>
  ///
  ///
  pub fn left(&self, len: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_left_to_output(self as *const ::byte_array::ByteArray, len, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::leftJustified```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn left_justified(&self, ::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::leftJustified(int width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn left_justified(&self, (::libc::c_int, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::leftJustified(int width, char fill = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn left_justified(&self, (::libc::c_int, ::libc::c_char, bool)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::leftJustified(int width, char fill = ?, bool truncate = ?) const```</span>
  ///
  ///
  pub fn left_justified<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayLeftJustifiedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QByteArray::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QByteArray_length(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::mid(int index) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::mid(int index, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::QByteArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArray::QByteArray()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArray::QByteArray(const QByteArray& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArray::QByteArray(int size, char c)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QByteArray::QByteArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArray::QByteArray(const char* arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArray::QByteArray(const char* arg1, int size = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QByteArray::number```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn number0(::libc::c_double) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn number0((::libc::c_double, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(double arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn number0((::libc::c_double, ::libc::c_char, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(double arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn number0(::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn number0((::libc::c_int, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(int arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn number0(u64) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(qulonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn number0((u64, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(qulonglong arg1, int base = ?)```</span>
  ///
  ///
  pub fn number0<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayNumber0Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QByteArray::number```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn number1(i64) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(qlonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn number1((i64, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(qlonglong arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn number1(::libc::c_uint) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(unsigned int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn number1((::libc::c_uint, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```static QByteArray QByteArray::number(unsigned int arg1, int base = ?)```</span>
  ///
  ///
  pub fn number1<Args>(args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayNumber1Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QByteArray::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator+=(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator+=(const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator+=(const QString& s)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator+=(const char* s)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0>(&'l0 mut self, s: *const ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = ::ffi::qt_core_c_QByteArray_operator_add_assign_char_s(self as *mut ::byte_array::ByteArray, s);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator=(const QByteArray& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::byte_array::ByteArray) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QByteArray_operator_assign_arg1(self as *mut ::byte_array::ByteArray,
                                                       arg1 as *const ::byte_array::ByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::operator=(const char* str)```</span>
  ///
  ///
  pub unsafe fn op_assign_unsafe<'l0>(&'l0 mut self, str: *const ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = ::ffi::qt_core_c_QByteArray_operator_assign_str(self as *mut ::byte_array::ByteArray, str);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::operator==(const QString& s2) const```</span>
  ///
  ///
  pub fn op_eq(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_eq(self as *const ::byte_array::ByteArray,
                                              s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::operator>=(const QString& s2) const```</span>
  ///
  ///
  pub fn op_ge(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_ge(self as *const ::byte_array::ByteArray,
                                              s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::operator>(const QString& s2) const```</span>
  ///
  ///
  pub fn op_gt(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_gt(self as *const ::byte_array::ByteArray,
                                              s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_int) -> ::libc::c_char```<br>
  /// C++ method: <span style='color: green;'>```char QByteArray::operator[](int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index(&self, ::libc::c_uint) -> ::libc::c_char```<br>
  /// C++ method: <span style='color: green;'>```char QByteArray::operator[](unsigned int i) const```</span>
  ///
  ///
  pub fn op_index<'largs, Args>(&'largs self, args: Args) -> ::libc::c_char
    where Args: overloading::ByteArrayOpIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_int) -> ::byte_ref::ByteRef```<br>
  /// C++ method: <span style='color: green;'>```QByteRef QByteArray::operator[](int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, ::libc::c_uint) -> ::byte_ref::ByteRef```<br>
  /// C++ method: <span style='color: green;'>```QByteRef QByteArray::operator[](unsigned int i)```</span>
  ///
  ///
  pub fn op_index_mut<'largs, Args>(&'largs mut self, args: Args) -> ::byte_ref::ByteRef
    where Args: overloading::ByteArrayOpIndexMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteArray::operator<=(const QString& s2) const```</span>
  ///
  ///
  pub fn op_le(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_le(self as *const ::byte_array::ByteArray,
                                              s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::operator<(const QString& s2) const```</span>
  ///
  ///
  pub fn op_lt(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_lt(self as *const ::byte_array::ByteArray,
                                              s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QByteArray::operator!=(const QString& s2) const```</span>
  ///
  ///
  pub fn op_neq(&self, s2: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QByteArray_operator_neq(self as *const ::byte_array::ByteArray,
                                               s2 as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::prepend```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn prepend(&mut self, ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::prepend(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn prepend(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::prepend(const QByteArray& a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn prepend(&mut self, (::libc::c_int, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::prepend(int count, char c)```</span>
  ///
  ///
  pub fn prepend<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayPrependArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::prepend```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn prepend_unsafe(&mut self, *const ::libc::c_char) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::prepend(const char* s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn prepend_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::prepend(const char* s, int len)```</span>
  ///
  ///
  pub unsafe fn prepend_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayPrependUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::push_back```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn push_back(&mut self, ::libc::c_char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QByteArray::push_back(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn push_back(&mut self, &::byte_array::ByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QByteArray::push_back(const QByteArray& a)```</span>
  ///
  ///
  pub fn push_back<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ByteArrayPushBackArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QByteArray::push_back(const char* c)```</span>
  ///
  ///
  pub unsafe fn push_back_unsafe(&mut self, c: *const ::libc::c_char) {
    ::ffi::qt_core_c_QByteArray_push_back_const_char_ptr(self as *mut ::byte_array::ByteArray, c)
  }

  /// C++ method: <span style='color: green;'>```QByteArray::push_front```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn push_front(&mut self, ::libc::c_char) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QByteArray::push_front(char c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn push_front(&mut self, &::byte_array::ByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QByteArray::push_front(const QByteArray& a)```</span>
  ///
  ///
  pub fn push_front<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ByteArrayPushFrontArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QByteArray::push_front(const char* c)```</span>
  ///
  ///
  pub unsafe fn push_front_unsafe(&mut self, c: *const ::libc::c_char) {
    ::ffi::qt_core_c_QByteArray_push_front_const_char_ptr(self as *mut ::byte_array::ByteArray, c)
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::remove(int index, int len)```</span>
  ///
  ///
  pub fn remove<'l0>(&'l0 mut self, index: ::libc::c_int, len: ::libc::c_int) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QByteArray_remove(self as *mut ::byte_array::ByteArray, index, len) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::repeated(int times) const```</span>
  ///
  ///
  pub fn repeated(&self, times: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_repeated_to_output(self as *const ::byte_array::ByteArray, times, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::replace```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_char, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(char before, char after)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_char, &'l1 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(char before, const QByteArray& after)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_char, &'l1 ::string::String)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(char c, const QString& after)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::byte_array::ByteArray, &'l2 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const QByteArray& before, const QByteArray& after)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn replace(&mut self, (&'l1 ::string::String, &'l2 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const QString& before, const QByteArray& after)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn replace(&mut self, (::libc::c_int, ::libc::c_int, &'l1 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(int index, int len, const QByteArray& s)```</span>
  ///
  ///
  pub fn replace<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayReplaceArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::replace```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (::libc::c_char, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(char before, const char* after)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (&'l1 ::byte_array::ByteArray, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const QByteArray& before, const char* after)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (&'l1 ::string::String, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const QString& before, const char* after)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (*const ::libc::c_char, &'l1 ::byte_array::ByteArray)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const char* before, const QByteArray& after)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (*const ::libc::c_char, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const char* before, const char* after)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(const char* before, int bsize, const char* after, int asize)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (::libc::c_int, ::libc::c_int, *const ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(int index, int len, const char* s)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn replace_unsafe(&mut self, (::libc::c_int, ::libc::c_int, *const ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::replace(int index, int len, const char* s, int alen)```</span>
  ///
  ///
  pub unsafe fn replace_unsafe<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArrayReplaceUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QByteArray::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QByteArray_reserve(self as *mut ::byte_array::ByteArray, size) }
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QByteArray_resize(self as *mut ::byte_array::ByteArray, size) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::right(int len) const```</span>
  ///
  ///
  pub fn right(&self, len: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_right_to_output(self as *const ::byte_array::ByteArray, len, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::rightJustified```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn right_justified(&self, ::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::rightJustified(int width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn right_justified(&self, (::libc::c_int, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::rightJustified(int width, char fill = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn right_justified(&self, (::libc::c_int, ::libc::c_char, bool)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::rightJustified(int width, char fill = ?, bool truncate = ?) const```</span>
  ///
  ///
  pub fn right_justified<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayRightJustifiedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num0(&mut self, ::libc::c_double) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(double arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_double, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(double arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_double, ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(double arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num0(&mut self, ::libc::c_int) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (::libc::c_int, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(int arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_num0(&mut self, u64) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(qulonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_num0(&mut self, (u64, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(qulonglong arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArraySetNum0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num1(&mut self, ::libc::c_float) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(float arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_float, ::libc::c_char)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(float arg1, char f = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_float, ::libc::c_char, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(float arg1, char f = ?, int prec = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num1(&mut self, i64) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(qlonglong arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (i64, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(qlonglong arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_num1(&mut self, ::libc::c_uint) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(unsigned int arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_num1(&mut self, (::libc::c_uint, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(unsigned int arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num1<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArraySetNum1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::setNum```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num2(&mut self, ::libc::c_short) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(short arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num2(&mut self, (::libc::c_short, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(short arg1, int base = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_num2(&mut self, ::libc::c_ushort) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(unsigned short arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_num2(&mut self, (::libc::c_ushort, ::libc::c_int)) -> &'l0 mut ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setNum(unsigned short arg1, int base = ?)```</span>
  ///
  ///
  pub fn set_num2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::byte_array::ByteArray
    where Args: overloading::ByteArraySetNum2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray& QByteArray::setRawData(const char* a, unsigned int n)```</span>
  ///
  ///
  pub unsafe fn set_raw_data<'l0>(&'l0 mut self,
                                  a: *const ::libc::c_char,
                                  n: ::libc::c_uint)
                                  -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = ::ffi::qt_core_c_QByteArray_setRawData(self as *mut ::byte_array::ByteArray, a, n);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::simplified() const```</span>
  ///
  ///
  pub fn simplified(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_simplified_to_output_const(self as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::simplified()```</span>
  ///
  ///
  pub fn simplified_mut(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_simplified_to_output(self as *mut ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QByteArray::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QByteArray_size(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QByteArray::split(char sep) const```</span>
  ///
  ///
  pub fn split(&self, sep: ::libc::c_char) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_split_to_output(self as *const ::byte_array::ByteArray, sep, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QByteArray_squeeze(self as *mut ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::startsWith```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn starts_with(&self, ::libc::c_char) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::startsWith(char c) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn starts_with(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QByteArray::startsWith(const QByteArray& a) const```</span>
  ///
  ///
  pub fn starts_with<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ByteArrayStartsWithArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QByteArray::startsWith(const char* c) const```</span>
  ///
  ///
  pub unsafe fn starts_with_unsafe(&self, c: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QByteArray_startsWith_const_char_ptr(self as *const ::byte_array::ByteArray, c)
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::swap(QByteArray& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QByteArray_swap(self as *mut ::byte_array::ByteArray,
                                       other as *mut ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toBase64```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_base64(&self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toBase64() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_base64(&self, ::flags::Flags<::byte_array::Base64Option>) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toBase64(QFlags<QByteArray::Base64Option> options) const```</span>
  ///
  ///
  pub fn to_base64<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayToBase64Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QByteArray::toDouble() const```</span>
  ///
  ///
  pub fn to_double(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QByteArray_toDouble_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```double QByteArray::toDouble(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_double_unsafe(&self, ok: *mut bool) -> ::libc::c_double {
    ::ffi::qt_core_c_QByteArray_toDouble_ok(self as *const ::byte_array::ByteArray, ok)
  }

  /// C++ method: <span style='color: green;'>```float QByteArray::toFloat() const```</span>
  ///
  ///
  pub fn to_float(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_core_c_QByteArray_toFloat_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```float QByteArray::toFloat(bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn to_float_unsafe(&self, ok: *mut bool) -> ::libc::c_float {
    ::ffi::qt_core_c_QByteArray_toFloat_ok(self as *const ::byte_array::ByteArray, ok)
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toHex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_hex(&self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toHex() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_hex(&self, ::libc::c_char) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toHex(char separator) const```</span>
  ///
  ///
  pub fn to_hex<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayToHexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QByteArray::toInt() const```</span>
  ///
  ///
  pub fn to_int(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QByteArray_toInt_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, *mut bool) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::toInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArray::toInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayToIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```long QByteArray::toLong() const```</span>
  ///
  ///
  pub fn to_long(&self) -> ::libc::c_long {
    unsafe { ::ffi::qt_core_c_QByteArray_toLong_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```qlonglong QByteArray::toLongLong() const```</span>
  ///
  ///
  pub fn to_long_long(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QByteArray_toLongLong_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toLongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, *mut bool) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QByteArray::toLongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> i64```<br>
  /// C++ method: <span style='color: green;'>```qlonglong QByteArray::toLongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> i64
    where Args: overloading::ByteArrayToLongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::toLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, *mut bool) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QByteArray::toLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_long```<br>
  /// C++ method: <span style='color: green;'>```long QByteArray::toLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_long
    where Args: overloading::ByteArrayToLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toLower() const```</span>
  ///
  ///
  pub fn to_lower(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_toLower_to_output_const(self as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toLower()```</span>
  ///
  ///
  pub fn to_lower_mut(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_toLower_to_output(self as *mut ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toPercentEncoding```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_percent_encoding(&self, ()) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toPercentEncoding() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_percent_encoding(&self, &::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toPercentEncoding(const QByteArray& exclude = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_percent_encoding(&self, (&::byte_array::ByteArray, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toPercentEncoding(const QByteArray& exclude = ?, const QByteArray& include = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn to_percent_encoding(&self, (&::byte_array::ByteArray, &::byte_array::ByteArray, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toPercentEncoding(const QByteArray& exclude = ?, const QByteArray& include = ?, char percent = ?) const```</span>
  ///
  ///
  pub fn to_percent_encoding<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ByteArrayToPercentEncodingArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```short QByteArray::toShort() const```</span>
  ///
  ///
  pub fn to_short(&self) -> ::libc::c_short {
    unsafe { ::ffi::qt_core_c_QByteArray_toShort_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, *mut bool) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QByteArray::toShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_short```<br>
  /// C++ method: <span style='color: green;'>```short QByteArray::toShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_short
    where Args: overloading::ByteArrayToShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned int QByteArray::toUInt() const```</span>
  ///
  ///
  pub fn to_u_int(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QByteArray_toUInt_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toUInt```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, *mut bool) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QByteArray::toUInt(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_int_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QByteArray::toUInt(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_int_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_uint
    where Args: overloading::ByteArrayToUIntUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned long QByteArray::toULong() const```</span>
  ///
  ///
  pub fn to_u_long(&self) -> ::libc::c_ulong {
    unsafe { ::ffi::qt_core_c_QByteArray_toULong_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```qulonglong QByteArray::toULongLong() const```</span>
  ///
  ///
  pub fn to_u_long_long(&self) -> u64 {
    unsafe { ::ffi::qt_core_c_QByteArray_toULongLong_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toULongLong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, *mut bool) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QByteArray::toULongLong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> u64```<br>
  /// C++ method: <span style='color: green;'>```qulonglong QByteArray::toULongLong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_long_unsafe<'largs, Args>(&'largs self, args: Args) -> u64
    where Args: overloading::ByteArrayToULongLongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray::toULong```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, *mut bool) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QByteArray::toULong(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_long_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ulong```<br>
  /// C++ method: <span style='color: green;'>```unsigned long QByteArray::toULong(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_long_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ulong
    where Args: overloading::ByteArrayToULongUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```unsigned short QByteArray::toUShort() const```</span>
  ///
  ///
  pub fn to_u_short(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QByteArray_toUShort_no_args(self as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray::toUShort```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, *mut bool) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QByteArray::toUShort(bool* ok = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_u_short_unsafe(&self, (*mut bool, ::libc::c_int)) -> ::libc::c_ushort```<br>
  /// C++ method: <span style='color: green;'>```unsigned short QByteArray::toUShort(bool* ok = ?, int base = ?) const```</span>
  ///
  ///
  pub unsafe fn to_u_short_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_ushort
    where Args: overloading::ByteArrayToUShortUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toUpper() const```</span>
  ///
  ///
  pub fn to_upper(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_toUpper_to_output_const(self as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::toUpper()```</span>
  ///
  ///
  pub fn to_upper_mut(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_toUpper_to_output(self as *mut ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::trimmed() const```</span>
  ///
  ///
  pub fn trimmed(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_trimmed_to_output_const(self as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArray::trimmed()```</span>
  ///
  ///
  pub fn trimmed_mut(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArray_trimmed_to_output(self as *mut ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QByteArray::truncate(int pos)```</span>
  ///
  ///
  pub fn truncate(&mut self, pos: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QByteArray_truncate(self as *mut ::byte_array::ByteArray, pos) }
  }
}

impl Drop for ::byte_array::ByteArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QByteArray::~QByteArray()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QByteArray_destructor(self as *mut ::byte_array::ByteArray) }
  }
}

/// C++ method: <span style='color: green;'>```qChecksum```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn checksum((*const ::libc::c_char, ::libc::c_uint)) -> u16```<br>
/// C++ method: <span style='color: green;'>```quint16 qChecksum(const char* s, unsigned int len)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn checksum((*const ::libc::c_char, ::libc::c_uint, &::qt::ChecksumType)) -> u16```<br>
/// C++ method: <span style='color: green;'>```quint16 qChecksum(const char* s, unsigned int len, Qt::ChecksumType standard)```</span>
///
///
pub unsafe fn checksum<Args>(args: Args) -> u16
  where Args: overloading::ChecksumArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qCompress```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn compress(&::byte_array::ByteArray) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```QByteArray qCompress(const QByteArray& data)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn compress((&::byte_array::ByteArray, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```QByteArray qCompress(const QByteArray& data, int compressionLevel = ?)```</span>
///
///
pub fn compress<Args>(args: Args) -> ::byte_array::ByteArray
  where Args: overloading::CompressArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qCompress```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn compress_unsafe((*const ::libc::c_uchar, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```QByteArray qCompress(const unsigned char* data, int nbytes)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn compress_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```QByteArray qCompress(const unsigned char* data, int nbytes, int compressionLevel = ?)```</span>
///
///
pub unsafe fn compress_unsafe<Args>(args: Args) -> ::byte_array::ByteArray
  where Args: overloading::CompressUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add((::libc::c_char, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```const QByteArray operator+(char a1, const QByteArray& a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add((&::byte_array::ByteArray, ::libc::c_char)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```const QByteArray operator+(const QByteArray& a1, char a2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_add((&::byte_array::ByteArray, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```const QByteArray operator+(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_add<Args>(args: Args) -> ::byte_array::ByteArray
  where Args: overloading::OpAddArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator+```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_add_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```const QByteArray operator+(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_add_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
/// C++ method: <span style='color: green;'>```const QByteArray operator+(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_add_unsafe<Args>(args: Args) -> ::byte_array::ByteArray
  where Args: overloading::OpAddUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_eq(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_eq_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                    a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator==```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_eq_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_eq_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator==(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_eq_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpEqUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator>=(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_ge(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_ge_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                    a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator>=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_ge_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_ge_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>=(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_ge_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpGeUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator>(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_gt(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_gt_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                    a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_gt_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_gt_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator>(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_gt_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpGtUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator<=(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_le(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_le_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                    a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator<=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_le_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_le_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<=(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_le_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpLeUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator<(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_lt(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_lt_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                    a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_lt_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_lt_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator<(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_lt_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpLtUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator!=(const QByteArray& a1, const QByteArray& a2)```</span>
///
///
pub fn op_neq(a1: &::byte_array::ByteArray, a2: &::byte_array::ByteArray) -> bool {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_neq_QByteArray_QByteArray(a1 as *const ::byte_array::ByteArray,
                                                                     a2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```operator!=```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_neq_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const QByteArray& a1, const char* a2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_neq_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> bool```<br>
/// C++ method: <span style='color: green;'>```bool operator!=(const char* a1, const QByteArray& a2)```</span>
///
///
pub unsafe fn op_neq_unsafe<Args>(args: Args) -> bool
  where Args: overloading::OpNeqUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QByteArray& arg2)```</span>
///
///
pub fn op_shl<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 ::byte_array::ByteArray)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_shl(arg1 as *mut ::data_stream::DataStream,
                                               arg2 as *const ::byte_array::ByteArray)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QByteArray& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 mut ::byte_array::ByteArray)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QByteArray_G_operator_shr(arg1 as *mut ::data_stream::DataStream,
                                               arg2 as *mut ::byte_array::ByteArray)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```int qstrcmp(const QByteArray& str1, const QByteArray& str2)```</span>
///
///
pub fn qstrcmp(str1: &::byte_array::ByteArray, str2: &::byte_array::ByteArray) -> ::libc::c_int {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_qstrcmp_QByteArray_QByteArray(str1 as *const ::byte_array::ByteArray,
                                                                str2 as *const ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```qstrcmp```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn qstrcmp_unsafe((&::byte_array::ByteArray, *const ::libc::c_char)) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qstrcmp(const QByteArray& str1, const char* str2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn qstrcmp_unsafe((*const ::libc::c_char, &::byte_array::ByteArray)) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qstrcmp(const char* str1, const QByteArray& str2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn qstrcmp_unsafe((*const ::libc::c_char, *const ::libc::c_char)) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qstrcmp(const char* str1, const char* str2)```</span>
///
///
pub unsafe fn qstrcmp_unsafe<Args>(args: Args) -> ::libc::c_int
  where Args: overloading::QstrcmpUnsafeArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```char* qstrcpy(char* dst, const char* src)```</span>
///
///
pub unsafe fn qstrcpy(dst: *mut ::libc::c_char, src: *const ::libc::c_char) -> *mut ::libc::c_char {
  ::ffi::qt_core_c_QByteArray_G_qstrcpy(dst, src)
}

/// C++ method: <span style='color: green;'>```char* qstrdup(const char* arg1)```</span>
///
///
pub unsafe fn qstrdup(arg1: *const ::libc::c_char) -> *mut ::libc::c_char {
  ::ffi::qt_core_c_QByteArray_G_qstrdup(arg1)
}

/// C++ method: <span style='color: green;'>```int qstricmp(const char* arg1, const char* arg2)```</span>
///
///
pub unsafe fn qstricmp(arg1: *const ::libc::c_char, arg2: *const ::libc::c_char) -> ::libc::c_int {
  ::ffi::qt_core_c_QByteArray_G_qstricmp(arg1, arg2)
}

/// C++ method: <span style='color: green;'>```unsigned int qstrlen(const char* str)```</span>
///
///
pub unsafe fn qstrlen(str: *const ::libc::c_char) -> ::libc::c_uint {
  ::ffi::qt_core_c_QByteArray_G_qstrlen(str)
}

/// C++ method: <span style='color: green;'>```int qstrncmp(const char* str1, const char* str2, unsigned int len)```</span>
///
///
pub unsafe fn qstrncmp(str1: *const ::libc::c_char, str2: *const ::libc::c_char, len: ::libc::c_uint) -> ::libc::c_int {
  ::ffi::qt_core_c_QByteArray_G_qstrncmp(str1, str2, len)
}

/// C++ method: <span style='color: green;'>```char* qstrncpy(char* dst, const char* src, unsigned int len)```</span>
///
///
pub unsafe fn qstrncpy(dst: *mut ::libc::c_char,
                       src: *const ::libc::c_char,
                       len: ::libc::c_uint)
                       -> *mut ::libc::c_char {
  ::ffi::qt_core_c_QByteArray_G_qstrncpy(dst, src, len)
}

/// C++ method: <span style='color: green;'>```int qstrnicmp(const char* arg1, const char* arg2, unsigned int len)```</span>
///
///
pub unsafe fn qstrnicmp(arg1: *const ::libc::c_char,
                        arg2: *const ::libc::c_char,
                        len: ::libc::c_uint)
                        -> ::libc::c_int {
  ::ffi::qt_core_c_QByteArray_G_qstrnicmp(arg1, arg2, len)
}

/// C++ method: <span style='color: green;'>```unsigned int qstrnlen(const char* str, unsigned int maxlen)```</span>
///
///
pub unsafe fn qstrnlen(str: *const ::libc::c_char, maxlen: ::libc::c_uint) -> ::libc::c_uint {
  ::ffi::qt_core_c_QByteArray_G_qstrnlen(str, maxlen)
}

/// C++ method: <span style='color: green;'>```void swap(QByteArray& value1, QByteArray& value2)```</span>
///
///
pub fn swap(value1: &mut ::byte_array::ByteArray, value2: &mut ::byte_array::ByteArray) {
  unsafe {
    ::ffi::qt_core_c_QByteArray_G_swap(value1 as *mut ::byte_array::ByteArray,
                                       value2 as *mut ::byte_array::ByteArray)
  }
}

/// C++ method: <span style='color: green;'>```QByteArray qUncompress(const QByteArray& data)```</span>
///
///
pub fn uncompress(data: &::byte_array::ByteArray) -> ::byte_array::ByteArray {
  {
    let mut object: ::byte_array::ByteArray =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QByteArray_G_qUncompress_to_output(data as *const ::byte_array::ByteArray, &mut object);
    }
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ByteArray::append](../struct.ByteArray.html#method.append) method.
  pub trait ByteArrayAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayAppendArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let a = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_append_QByteArray_a(original_self as *mut ::byte_array::ByteArray,
                                                        a as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayAppendArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_append_QString_s(original_self as *mut ::byte_array::ByteArray,
                                                     s as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayAppendArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_append_char_c(original_self as *mut ::byte_array::ByteArray, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayAppendArgs<'largs> for (::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let count = self.0;
      let c = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_append_int_count_char_c(original_self as *mut ::byte_array::ByteArray, count, c)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::append_unsafe](../struct.ByteArray.html#method.append_unsafe) method.
  pub trait ByteArrayAppendUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayAppendUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self;
      let ffi_result = ::ffi::qt_core_c_QByteArray_append_char_s(original_self as *mut ::byte_array::ByteArray, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayAppendUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self.0;
      let len = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_append_char_s_int_len(original_self as *mut ::byte_array::ByteArray, s, len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::contains](../struct.ByteArray.html#method.contains) method.
  pub trait ByteArrayContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool;
  }
  impl<'largs> ByteArrayContainsArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_contains_QByteArray_a(original_self as *const ::byte_array::ByteArray,
                                                          a as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ByteArrayContainsArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_contains_char_c(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::count](../struct.ByteArray.html#method.count) method.
  pub trait ByteArrayCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayCountArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_count_QByteArray_a(original_self as *const ::byte_array::ByteArray,
                                                       a as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ByteArrayCountArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_count_char_c(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QByteArray_count_no_args(original_self as *const ::byte_array::ByteArray) }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::ends_with](../struct.ByteArray.html#method.ends_with) method.
  pub trait ByteArrayEndsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool;
  }
  impl<'largs> ByteArrayEndsWithArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_endsWith_char(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayEndsWithArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_endsWith_const_QByteArray_ref(original_self as *const ::byte_array::ByteArray,
                                                                  a as *const ::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::fill](../struct.ByteArray.html#method.fill) method.
  pub trait ByteArrayFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayFillArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QByteArray_fill_c(original_self as *mut ::byte_array::ByteArray, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayFillArgs<'largs> for (::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self.0;
      let size = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_fill_c_size(original_self as *mut ::byte_array::ByteArray, c, size) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::from_base64](../struct.ByteArray.html#method.from_base64) method.
  pub trait ByteArrayFromBase64Args {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> ByteArrayFromBase64Args for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array::ByteArray {
      let base64 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_fromBase64_to_output_base64(base64 as *const ::byte_array::ByteArray,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'a> ByteArrayFromBase64Args for (&'a ::byte_array::ByteArray, ::flags::Flags<::byte_array::Base64Option>) {
    fn exec(self) -> ::byte_array::ByteArray {
      let base64 = self.0;
      let options = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_fromBase64_to_output_base64_options(base64 as *const ::byte_array::ByteArray,
                                                                          options.to_int() as ::libc::c_uint,
                                                                          &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::from_percent_encoding](../struct.ByteArray.html#method.from_percent_encoding) method.
  pub trait ByteArrayFromPercentEncodingArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> ByteArrayFromPercentEncodingArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array::ByteArray {
      let pct_encoded = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_fromPercentEncoding_to_output_pctEncoded(pct_encoded as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ByteArrayFromPercentEncodingArgs for (&'a ::byte_array::ByteArray, ::libc::c_char) {
    fn exec(self) -> ::byte_array::ByteArray {
      let pct_encoded = self.0;
      let percent = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_fromPercentEncoding_to_output_pctEncoded_percent(pct_encoded as *const ::byte_array::ByteArray, percent, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::index_of](../struct.ByteArray.html#method.index_of) method.
  pub trait ByteArrayIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_indexOf_char(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for (::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QByteArray_indexOf_char_int(original_self as *const ::byte_array::ByteArray, c, from) }
    }
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_indexOf_const_QByteArray_ref(original_self as *const ::byte_array::ByteArray,
                                                                 a as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for (&'largs ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let a = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QByteArray_indexOf_const_QByteArray_ref_int(original_self as *const ::byte_array::ByteArray,
                                                                     a as *const ::byte_array::ByteArray,
                                                                     from)
      }
    }
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_indexOf_const_QString_ref(original_self as *const ::byte_array::ByteArray,
                                                              s as *const ::string::String)
      }
    }
  }
  impl<'largs> ByteArrayIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QByteArray_indexOf_const_QString_ref_int(original_self as *const ::byte_array::ByteArray,
                                                                  s as *const ::string::String,
                                                                  from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::index_of_unsafe](../struct.ByteArray.html#method.index_of_unsafe) method.
  pub trait ByteArrayIndexOfUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayIndexOfUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self;
      ::ffi::qt_core_c_QByteArray_indexOf_const_char_ptr(original_self as *const ::byte_array::ByteArray, c)
    }
  }
  impl<'largs> ByteArrayIndexOfUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QByteArray_indexOf_const_char_ptr_int(original_self as *const ::byte_array::ByteArray, c, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::insert](../struct.ByteArray.html#method.insert) method.
  pub trait ByteArrayInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayInsertArgs<'largs> for (::libc::c_int, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let a = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_insert_int_i_QByteArray_a(original_self as *mut ::byte_array::ByteArray,
                                                                i,
                                                                a as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayInsertArgs<'largs> for (::libc::c_int, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let s = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_insert_int_i_QString_s(original_self as *mut ::byte_array::ByteArray,
                                                             i,
                                                             s as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayInsertArgs<'largs> for (::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let c = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_insert_int_i_char_c(original_self as *mut ::byte_array::ByteArray, i, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let count = self.1;
      let c = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_insert_int_i_int_count_char_c(original_self as *mut ::byte_array::ByteArray,
                                                                    i,
                                                                    count,
                                                                    c)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::insert_unsafe](../struct.ByteArray.html#method.insert_unsafe) method.
  pub trait ByteArrayInsertUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayInsertUnsafeArgs<'largs> for (::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let s = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_insert_int_i_char_s(original_self as *mut ::byte_array::ByteArray, i, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayInsertUnsafeArgs<'largs> for (::libc::c_int, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let i = self.0;
      let s = self.1;
      let len = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_insert_int_i_char_s_int_len(original_self as *mut ::byte_array::ByteArray,
                                                                i,
                                                                s,
                                                                len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::last_index_of](../struct.ByteArray.html#method.last_index_of) method.
  pub trait ByteArrayLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_lastIndexOf_char(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for (::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QByteArray_lastIndexOf_char_int(original_self as *const ::byte_array::ByteArray, c, from)
      }
    }
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_lastIndexOf_const_QByteArray_ref(original_self as *const ::byte_array::ByteArray,
                                                                     a as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for (&'largs ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let a = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QByteArray_lastIndexOf_const_QByteArray_ref_int(original_self as *const ::byte_array::ByteArray, a as *const ::byte_array::ByteArray, from) }
    }
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_lastIndexOf_const_QString_ref(original_self as *const ::byte_array::ByteArray,
                                                                  s as *const ::string::String)
      }
    }
  }
  impl<'largs> ByteArrayLastIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let s = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QByteArray_lastIndexOf_const_QString_ref_int(original_self as *const ::byte_array::ByteArray,
                                                                      s as *const ::string::String,
                                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::last_index_of_unsafe](../struct.ByteArray.html#method.last_index_of_unsafe) method.
  pub trait ByteArrayLastIndexOfUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayLastIndexOfUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self;
      ::ffi::qt_core_c_QByteArray_lastIndexOf_const_char_ptr(original_self as *const ::byte_array::ByteArray, c)
    }
  }
  impl<'largs> ByteArrayLastIndexOfUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let c = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QByteArray_lastIndexOf_const_char_ptr_int(original_self as *const ::byte_array::ByteArray,
                                                                 c,
                                                                 from)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::left_justified](../struct.ByteArray.html#method.left_justified) method.
  pub trait ByteArrayLeftJustifiedArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayLeftJustifiedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_leftJustified_to_output_width(original_self as *const ::byte_array::ByteArray,
                                                                    width,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayLeftJustifiedArgs<'largs> for (::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self.0;
      let fill = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_leftJustified_to_output_width_fill(original_self as *const ::byte_array::ByteArray, width, fill, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayLeftJustifiedArgs<'largs> for (::libc::c_int, ::libc::c_char, bool) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self.0;
      let fill = self.1;
      let truncate = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_leftJustified_to_output_width_fill_truncate(original_self as *const ::byte_array::ByteArray, width, fill, truncate, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::mid](../struct.ByteArray.html#method.mid) method.
  pub trait ByteArrayMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let index = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_mid_to_output_index(original_self as *const ::byte_array::ByteArray,
                                                          index,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let index = self.0;
      let len = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_mid_to_output_index_len(original_self as *const ::byte_array::ByteArray,
                                                              index,
                                                              len,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::new](../struct.ByteArray.html#method.new) method.
  pub trait ByteArrayNewArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> ByteArrayNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_constructor_QByteArray(arg1 as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNewArgs for (::libc::c_int, ::libc::c_char) {
    fn exec(self) -> ::byte_array::ByteArray {
      let size = self.0;
      let c = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_constructor_int_char(size, c, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNewArgs for () {
    fn exec(self) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::new_unsafe](../struct.ByteArray.html#method.new_unsafe) method.
  pub trait ByteArrayNewUnsafeArgs {
    unsafe fn exec(self) -> ::byte_array::ByteArray;
  }
  impl ByteArrayNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_constructor_char(arg1, &mut object);
        object
      }
    }
  }
  impl ByteArrayNewUnsafeArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let size = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_constructor_char_int(arg1, size, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::number0](../struct.ByteArray.html#method.number0) method.
  pub trait ByteArrayNumber0Args {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl ByteArrayNumber0Args for ::libc::c_double {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_double(arg1, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for (::libc::c_double, ::libc::c_char) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_double_char(arg1, f, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for (::libc::c_double, ::libc::c_char, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_double_char_int(arg1, f, prec, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for ::libc::c_int {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_int(arg1, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_int_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for u64 {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_qulonglong(arg1, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber0Args for (u64, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_qulonglong_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::number1](../struct.ByteArray.html#method.number1) method.
  pub trait ByteArrayNumber1Args {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl ByteArrayNumber1Args for i64 {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_qlonglong(arg1, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber1Args for (i64, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_qlonglong_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber1Args for ::libc::c_uint {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_unsigned_int(arg1, &mut object);
        }
        object
      }
    }
  }
  impl ByteArrayNumber1Args for (::libc::c_uint, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_number_to_output_unsigned_int_int(arg1, base, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::op_add_assign](../struct.ByteArray.html#method.op_add_assign) method.
  pub trait ByteArrayOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayOpAddAssignArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let a = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_operator_add_assign_QByteArray_a(original_self as *mut ::byte_array::ByteArray,
                                                                       a as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayOpAddAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_operator_add_assign_QString_s(original_self as *mut ::byte_array::ByteArray,
                                                                    s as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayOpAddAssignArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_operator_add_assign_char_c(original_self as *mut ::byte_array::ByteArray, c)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::op_index](../struct.ByteArray.html#method.op_index) method.
  pub trait ByteArrayOpIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_char;
  }
  impl<'largs> ByteArrayOpIndexArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_char {
      let i = self;
      unsafe { ::ffi::qt_core_c_QByteArray_operator_index_int(original_self as *const ::byte_array::ByteArray, i) }
    }
  }
  impl<'largs> ByteArrayOpIndexArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_char {
      let i = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_operator_index_unsigned_int(original_self as *const ::byte_array::ByteArray, i)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::op_index_mut](../struct.ByteArray.html#method.op_index_mut) method.
  pub trait ByteArrayOpIndexMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> ::byte_ref::ByteRef;
  }
  impl<'largs> ByteArrayOpIndexMutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> ::byte_ref::ByteRef {
      let i = self;
      {
        let mut object: ::byte_ref::ByteRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_operator_index_to_output_int(original_self as *mut ::byte_array::ByteArray,
                                                                   i,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayOpIndexMutArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> ::byte_ref::ByteRef {
      let i = self;
      {
        let mut object: ::byte_ref::ByteRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_operator_index_to_output_unsigned_int(original_self as *mut ::byte_array::ByteArray, i, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::prepend](../struct.ByteArray.html#method.prepend) method.
  pub trait ByteArrayPrependArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayPrependArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let a = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_prepend_a(original_self as *mut ::byte_array::ByteArray,
                                              a as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayPrependArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_prepend_c(original_self as *mut ::byte_array::ByteArray, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayPrependArgs<'largs> for (::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let count = self.0;
      let c = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_prepend_count_c(original_self as *mut ::byte_array::ByteArray, count, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::prepend_unsafe](../struct.ByteArray.html#method.prepend_unsafe) method.
  pub trait ByteArrayPrependUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayPrependUnsafeArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self;
      let ffi_result = ::ffi::qt_core_c_QByteArray_prepend_s(original_self as *mut ::byte_array::ByteArray, s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayPrependUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let s = self.0;
      let len = self.1;
      let ffi_result = ::ffi::qt_core_c_QByteArray_prepend_s_len(original_self as *mut ::byte_array::ByteArray, s, len);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::push_back](../struct.ByteArray.html#method.push_back) method.
  pub trait ByteArrayPushBackArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> ();
  }
  impl<'largs> ByteArrayPushBackArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> () {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_push_back_char(original_self as *mut ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayPushBackArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> () {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_push_back_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray,
                                                                   a as *const ::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::push_front](../struct.ByteArray.html#method.push_front) method.
  pub trait ByteArrayPushFrontArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> ();
  }
  impl<'largs> ByteArrayPushFrontArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> () {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_push_front_char(original_self as *mut ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayPushFrontArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> () {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_push_front_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray,
                                                                    a as *const ::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::replace](../struct.ByteArray.html#method.replace) method.
  pub trait ByteArrayReplaceArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (::libc::c_char, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_replace_char_char(original_self as *mut ::byte_array::ByteArray, before, after)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (::libc::c_char, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_replace_char_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray,
                                                                        before,
                                                                        after as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (::libc::c_char, &'largs ::string::String) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let c = self.0;
      let after = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_replace_char_const_QString_ref(original_self as *mut ::byte_array::ByteArray,
                                                                     c,
                                                                     after as *const ::string::String)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (&'largs ::byte_array::ByteArray, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QByteArray_replace_const_QByteArray_ref_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray, before as *const ::byte_array::ByteArray, after as *const ::byte_array::ByteArray) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (&'largs ::string::String, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = unsafe { ::ffi::qt_core_c_QByteArray_replace_const_QString_ref_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray, before as *const ::string::String, after as *const ::byte_array::ByteArray) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let index = self.0;
      let len = self.1;
      let s = self.2;
      let ffi_result = unsafe { ::ffi::qt_core_c_QByteArray_replace_int_int_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray, index, len, s as *const ::byte_array::ByteArray) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::replace_unsafe](../struct.ByteArray.html#method.replace_unsafe) method.
  pub trait ByteArrayReplaceUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_replace_char_const_char_ptr(original_self as *mut ::byte_array::ByteArray,
                                                                before,
                                                                after);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (&'largs ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_core_c_QByteArray_replace_const_QByteArray_ref_const_char_ptr(original_self as *mut ::byte_array::ByteArray, before as *const ::byte_array::ByteArray, after);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (&'largs ::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_core_c_QByteArray_replace_const_QString_ref_const_char_ptr(original_self as *mut ::byte_array::ByteArray, before as *const ::string::String, after);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (*const ::libc::c_char, &'largs ::byte_array::ByteArray) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_core_c_QByteArray_replace_const_char_ptr_const_QByteArray_ref(original_self as *mut ::byte_array::ByteArray, before, after as *const ::byte_array::ByteArray);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (*const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let after = self.1;
      let ffi_result = ::ffi::qt_core_c_QByteArray_replace_const_char_ptr_const_char_ptr(original_self as *mut ::byte_array::ByteArray, before, after);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let before = self.0;
      let bsize = self.1;
      let after = self.2;
      let asize = self.3;
      let ffi_result = ::ffi::qt_core_c_QByteArray_replace_const_char_ptr_int_const_char_ptr_int(original_self as *mut ::byte_array::ByteArray, before, bsize, after, asize);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs> for (::libc::c_int, ::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let index = self.0;
      let len = self.1;
      let s = self.2;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_replace_int_int_const_char_ptr(original_self as *mut ::byte_array::ByteArray,
                                                                   index,
                                                                   len,
                                                                   s);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArrayReplaceUnsafeArgs<'largs>
    for (::libc::c_int, ::libc::c_int, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let index = self.0;
      let len = self.1;
      let s = self.2;
      let alen = self.3;
      let ffi_result =
        ::ffi::qt_core_c_QByteArray_replace_int_int_const_char_ptr_int(original_self as *mut ::byte_array::ByteArray,
                                                                       index,
                                                                       len,
                                                                       s,
                                                                       alen);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::right_justified](../struct.ByteArray.html#method.right_justified) method.
  pub trait ByteArrayRightJustifiedArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayRightJustifiedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_rightJustified_to_output_width(original_self as *const ::byte_array::ByteArray,
                                                                     width,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayRightJustifiedArgs<'largs> for (::libc::c_int, ::libc::c_char) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self.0;
      let fill = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_rightJustified_to_output_width_fill(original_self as *const ::byte_array::ByteArray, width, fill, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayRightJustifiedArgs<'largs> for (::libc::c_int, ::libc::c_char, bool) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let width = self.0;
      let fill = self.1;
      let truncate = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_rightJustified_to_output_width_fill_truncate(original_self as *const ::byte_array::ByteArray, width, fill, truncate, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::set_num0](../struct.ByteArray.html#method.set_num0) method.
  pub trait ByteArraySetNum0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_double(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for (::libc::c_double, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_setNum_double_char(original_self as *mut ::byte_array::ByteArray, arg1, f)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for (::libc::c_double, ::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_double_char_int(original_self as *mut ::byte_array::ByteArray,
                                                             arg1,
                                                             f,
                                                             prec)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_int(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_setNum_int_int(original_self as *mut ::byte_array::ByteArray, arg1, base)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for u64 {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_qulonglong(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum0Args<'largs> for (u64, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_qulonglong_int(original_self as *mut ::byte_array::ByteArray, arg1, base)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::set_num1](../struct.ByteArray.html#method.set_num1) method.
  pub trait ByteArraySetNum1Args<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_float(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for (::libc::c_float, ::libc::c_char) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_setNum_float_char(original_self as *mut ::byte_array::ByteArray, arg1, f)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for (::libc::c_float, ::libc::c_char, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let f = self.1;
      let prec = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_float_char_int(original_self as *mut ::byte_array::ByteArray,
                                                            arg1,
                                                            f,
                                                            prec)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_qlonglong(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for (i64, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_setNum_qlonglong_int(original_self as *mut ::byte_array::ByteArray, arg1, base)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_unsigned_int(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum1Args<'largs> for (::libc::c_uint, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_unsigned_int_int(original_self as *mut ::byte_array::ByteArray, arg1, base)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::set_num2](../struct.ByteArray.html#method.set_num2) method.
  pub trait ByteArraySetNum2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray;
  }
  impl<'largs> ByteArraySetNum2Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QByteArray_setNum_short(original_self as *mut ::byte_array::ByteArray, arg1) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum2Args<'largs> for (::libc::c_short, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QByteArray_setNum_short_int(original_self as *mut ::byte_array::ByteArray, arg1, base)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum2Args<'largs> for ::libc::c_ushort {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_unsigned_short(original_self as *mut ::byte_array::ByteArray, arg1)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ByteArraySetNum2Args<'largs> for (::libc::c_ushort, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::byte_array::ByteArray) -> &'largs mut ::byte_array::ByteArray {
      let arg1 = self.0;
      let base = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QByteArray_setNum_unsigned_short_int(original_self as *mut ::byte_array::ByteArray,
                                                                arg1,
                                                                base)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::starts_with](../struct.ByteArray.html#method.starts_with) method.
  pub trait ByteArrayStartsWithArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool;
  }
  impl<'largs> ByteArrayStartsWithArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let c = self;
      unsafe { ::ffi::qt_core_c_QByteArray_startsWith_char(original_self as *const ::byte_array::ByteArray, c) }
    }
  }
  impl<'largs> ByteArrayStartsWithArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> bool {
      let a = self;
      unsafe {
        ::ffi::qt_core_c_QByteArray_startsWith_const_QByteArray_ref(original_self as *const ::byte_array::ByteArray,
                                                                    a as *const ::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_base64](../struct.ByteArray.html#method.to_base64) method.
  pub trait ByteArrayToBase64Args<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayToBase64Args<'largs> for () {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toBase64_to_output_no_args(original_self as *const ::byte_array::ByteArray,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayToBase64Args<'largs> for ::flags::Flags<::byte_array::Base64Option> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let options = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toBase64_to_output_options(original_self as *const ::byte_array::ByteArray,
                                                                 options.to_int() as ::libc::c_uint,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_hex](../struct.ByteArray.html#method.to_hex) method.
  pub trait ByteArrayToHexArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayToHexArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toHex_to_output_no_args(original_self as *const ::byte_array::ByteArray,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayToHexArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let separator = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toHex_to_output_separator(original_self as *const ::byte_array::ByteArray,
                                                                separator,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_int_unsafe](../struct.ByteArray.html#method.to_int_unsafe) method.
  pub trait ByteArrayToIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayToIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toInt_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_int {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toInt_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_long_long_unsafe](../struct.ByteArray.html#method.to_long_long_unsafe) method.
  pub trait ByteArrayToLongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> i64;
  }
  impl<'largs> ByteArrayToLongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> i64 {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toLongLong_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToLongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> i64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toLongLong_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_long_unsafe](../struct.ByteArray.html#method.to_long_unsafe) method.
  pub trait ByteArrayToLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_long;
  }
  impl<'largs> ByteArrayToLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_long {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toLong_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_long {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toLong_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_percent_encoding](../struct.ByteArray.html#method.to_percent_encoding) method.
  pub trait ByteArrayToPercentEncodingArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ByteArrayToPercentEncodingArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let exclude = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toPercentEncoding_to_output_exclude(original_self as *const ::byte_array::ByteArray, exclude as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayToPercentEncodingArgs<'largs>
    for (&'largs ::byte_array::ByteArray, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let exclude = self.0;
      let include = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toPercentEncoding_to_output_exclude_include(original_self as *const ::byte_array::ByteArray, exclude as *const ::byte_array::ByteArray, include as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayToPercentEncodingArgs<'largs>
    for (&'largs ::byte_array::ByteArray, &'largs ::byte_array::ByteArray, ::libc::c_char) {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {
      let exclude = self.0;
      let include = self.1;
      let percent = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toPercentEncoding_to_output_exclude_include_percent(original_self as *const ::byte_array::ByteArray, exclude as *const ::byte_array::ByteArray, include as *const ::byte_array::ByteArray, percent, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ByteArrayToPercentEncodingArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::byte_array::ByteArray {

      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_toPercentEncoding_to_output_no_args(original_self as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_short_unsafe](../struct.ByteArray.html#method.to_short_unsafe) method.
  pub trait ByteArrayToShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_short;
  }
  impl<'largs> ByteArrayToShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_short {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toShort_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_short {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toShort_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_u_int_unsafe](../struct.ByteArray.html#method.to_u_int_unsafe) method.
  pub trait ByteArrayToUIntUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_uint;
  }
  impl<'largs> ByteArrayToUIntUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_uint {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toUInt_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToUIntUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_uint {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toUInt_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_u_long_long_unsafe](../struct.ByteArray.html#method.to_u_long_long_unsafe) method.
  pub trait ByteArrayToULongLongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> u64;
  }
  impl<'largs> ByteArrayToULongLongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> u64 {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toULongLong_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToULongLongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> u64 {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toULongLong_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_u_long_unsafe](../struct.ByteArray.html#method.to_u_long_unsafe) method.
  pub trait ByteArrayToULongUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ulong;
  }
  impl<'largs> ByteArrayToULongUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ulong {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toULong_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToULongUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ulong {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toULong_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArray::to_u_short_unsafe](../struct.ByteArray.html#method.to_u_short_unsafe) method.
  pub trait ByteArrayToUShortUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ushort;
  }
  impl<'largs> ByteArrayToUShortUnsafeArgs<'largs> for *mut bool {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ushort {
      let ok = self;
      ::ffi::qt_core_c_QByteArray_toUShort_ok(original_self as *const ::byte_array::ByteArray, ok)
    }
  }
  impl<'largs> ByteArrayToUShortUnsafeArgs<'largs> for (*mut bool, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array::ByteArray) -> ::libc::c_ushort {
      let ok = self.0;
      let base = self.1;
      ::ffi::qt_core_c_QByteArray_toUShort_ok_base(original_self as *const ::byte_array::ByteArray, ok, base)
    }
  }
  /// This trait represents a set of arguments accepted by [checksum](../fn.checksum.html) method.
  pub trait ChecksumArgs {
    unsafe fn exec(self) -> u16;
  }
  impl ChecksumArgs for (*const ::libc::c_char, ::libc::c_uint) {
    unsafe fn exec(self) -> u16 {
      let s = self.0;
      let len = self.1;
      ::ffi::qt_core_c_QByteArray_G_qChecksum_s_len(s, len)
    }
  }
  impl<'a> ChecksumArgs for (*const ::libc::c_char, ::libc::c_uint, &'a ::qt::ChecksumType) {
    unsafe fn exec(self) -> u16 {
      let s = self.0;
      let len = self.1;
      let standard = self.2;
      ::ffi::qt_core_c_QByteArray_G_qChecksum_s_len_standard(s, len, standard as *const ::qt::ChecksumType)
    }
  }
  /// This trait represents a set of arguments accepted by [compress](../fn.compress.html) method.
  pub trait CompressArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> CompressArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array::ByteArray {
      let data = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_G_qCompress_to_output_data(data as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> CompressArgs for (&'a ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self) -> ::byte_array::ByteArray {
      let data = self.0;
      let compression_level = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_G_qCompress_to_output_data_compressionLevel(data as *const ::byte_array::ByteArray, compression_level, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [compress_unsafe](../fn.compress_unsafe.html) method.
  pub trait CompressUnsafeArgs {
    unsafe fn exec(self) -> ::byte_array::ByteArray;
  }
  impl CompressUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int) {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let data = self.0;
      let nbytes = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_G_qCompress_to_output_data_nbytes(data, nbytes, &mut object);
        object
      }
    }
  }
  impl CompressUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let data = self.0;
      let nbytes = self.1;
      let compression_level = self.2;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_G_qCompress_to_output_data_nbytes_compressionLevel(data,
                                                                                       nbytes,
                                                                                       compression_level,
                                                                                       &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_add](../fn.op_add.html) method.
  pub trait OpAddArgs {
    fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> OpAddArgs for (::libc::c_char, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::byte_array::ByteArray {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_G_operator_add_to_output_char_const_QByteArray_ref(a1, a2 as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::byte_array::ByteArray, ::libc::c_char) {
    fn exec(self) -> ::byte_array::ByteArray {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_char(a1 as *const ::byte_array::ByteArray, a2, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpAddArgs for (&'a ::byte_array::ByteArray, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::byte_array::ByteArray {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_const_QByteArray_ref(a1 as *const ::byte_array::ByteArray, a2 as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_add_unsafe](../fn.op_add_unsafe.html) method.
  pub trait OpAddUnsafeArgs {
    unsafe fn exec(self) -> ::byte_array::ByteArray;
  }
  impl<'a> OpAddUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_const_char_ptr(a1 as *const ::byte_array::ByteArray, a2, &mut object);
        object
      }
    }
  }
  impl<'a> OpAddUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> ::byte_array::ByteArray {
      let a1 = self.0;
      let a2 = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QByteArray_G_operator_add_to_output_const_char_ptr_const_QByteArray_ref(a1, a2 as *const ::byte_array::ByteArray, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_eq_unsafe](../fn.op_eq_unsafe.html) method.
  pub trait OpEqUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpEqUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_eq_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpEqUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_eq_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [op_ge_unsafe](../fn.op_ge_unsafe.html) method.
  pub trait OpGeUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpGeUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_ge_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpGeUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_ge_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [op_gt_unsafe](../fn.op_gt_unsafe.html) method.
  pub trait OpGtUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpGtUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_gt_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpGtUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_gt_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [op_le_unsafe](../fn.op_le_unsafe.html) method.
  pub trait OpLeUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpLeUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_le_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpLeUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_le_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [op_lt_unsafe](../fn.op_lt_unsafe.html) method.
  pub trait OpLtUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpLtUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_lt_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpLtUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_lt_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [op_neq_unsafe](../fn.op_neq_unsafe.html) method.
  pub trait OpNeqUnsafeArgs {
    unsafe fn exec(self) -> bool;
  }
  impl<'a> OpNeqUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_neq_QByteArray_char(a1 as *const ::byte_array::ByteArray, a2)
    }
  }
  impl<'a> OpNeqUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> bool {
      let a1 = self.0;
      let a2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_operator_neq_char_QByteArray(a1, a2 as *const ::byte_array::ByteArray)
    }
  }
  /// This trait represents a set of arguments accepted by [qstrcmp_unsafe](../fn.qstrcmp_unsafe.html) method.
  pub trait QstrcmpUnsafeArgs {
    unsafe fn exec(self) -> ::libc::c_int;
  }
  impl<'a> QstrcmpUnsafeArgs for (&'a ::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::libc::c_int {
      let str1 = self.0;
      let str2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_qstrcmp_QByteArray_char(str1 as *const ::byte_array::ByteArray, str2)
    }
  }
  impl<'a> QstrcmpUnsafeArgs for (*const ::libc::c_char, &'a ::byte_array::ByteArray) {
    unsafe fn exec(self) -> ::libc::c_int {
      let str1 = self.0;
      let str2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_qstrcmp_char_QByteArray(str1, str2 as *const ::byte_array::ByteArray)
    }
  }
  impl QstrcmpUnsafeArgs for (*const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::libc::c_int {
      let str1 = self.0;
      let str2 = self.1;
      ::ffi::qt_core_c_QByteArray_G_qstrcmp_char_char(str1, str2)
    }
  }
}
