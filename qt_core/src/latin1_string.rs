/// C++ type: <span style='color: green;'>```QLatin1String```</span>
#[repr(C)]
pub struct Latin1String([u8; ::type_sizes::QT_CORE_LATIN_1_STRING_LATIN_1_STRING]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Latin1String {
  unsafe fn new_uninitialized() -> Latin1String {
    Latin1String(::std::mem::uninitialized())
  }
}

impl Latin1String {
  /// C++ method: <span style='color: green;'>```QLatin1Char QLatin1String::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> ::latin1_char::Latin1Char {
    {
      let mut object: ::latin1_char::Latin1Char =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLatin1String_at_to_output(self as *const ::latin1_string::Latin1String, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QLatin1String::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QLatin1String_data(self as *const ::latin1_string::Latin1String) }
  }

  /// C++ method: <span style='color: green;'>```const char* QLatin1String::latin1() const```</span>
  ///
  ///
  pub fn latin1(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QLatin1String_latin1(self as *const ::latin1_string::Latin1String) }
  }

  /// C++ method: <span style='color: green;'>```QLatin1String QLatin1String::left(int n) const```</span>
  ///
  ///
  pub fn left(&self, n: ::libc::c_int) -> ::latin1_string::Latin1String {
    {
      let mut object: ::latin1_string::Latin1String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLatin1String_left_to_output(self as *const ::latin1_string::Latin1String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```QLatin1String QLatin1String::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```QLatin1String QLatin1String::mid(int pos, int n) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::latin1_string::Latin1String
    where Args: overloading::Latin1StringMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLatin1String::QLatin1String```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLatin1String::QLatin1String()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLatin1String::QLatin1String(const QByteArray& s)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::latin1_string::Latin1String
    where Args: overloading::Latin1StringNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLatin1String::QLatin1String```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_char) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLatin1String::QLatin1String(const char* s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, ::libc::c_int)) -> ::latin1_string::Latin1String```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLatin1String::QLatin1String(const char* s, int sz)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::latin1_string::Latin1String
    where Args: overloading::Latin1StringNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLatin1String::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator==(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator==(const QString& s) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator==(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_eq_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_eq_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::operator>=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_ge(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>=(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_ge(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>=(const QString& s) const```</span>
  ///
  ///
  pub fn op_ge<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpGeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_ge_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_ge_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::operator>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_gt(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_gt(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>(const QString& s) const```</span>
  ///
  ///
  pub fn op_gt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpGtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator>(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_gt_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_gt_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1Char QLatin1String::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_int) -> ::latin1_char::Latin1Char {
    {
      let mut object: ::latin1_char::Latin1Char =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_index_to_output(self as *const ::latin1_string::Latin1String,
                                                                i,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::operator<=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_le(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<=(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_le(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<=(const QString& s) const```</span>
  ///
  ///
  pub fn op_le<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpLeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_le_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_le_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::operator<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_lt(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_lt(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<(const QString& s) const```</span>
  ///
  ///
  pub fn op_lt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpLtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator<(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_lt_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_lt_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1String::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator!=(const QByteArray& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator!=(const QString& s) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::Latin1StringOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QLatin1String::operator!=(const char* s) const```</span>
  ///
  ///
  pub unsafe fn op_neq_unsafe(&self, s: *const ::libc::c_char) -> bool {
    ::ffi::qt_core_c_QLatin1String_operator_neq_char(self as *const ::latin1_string::Latin1String, s)
  }

  /// C++ method: <span style='color: green;'>```QLatin1String QLatin1String::right(int n) const```</span>
  ///
  ///
  pub fn right(&self, n: ::libc::c_int) -> ::latin1_string::Latin1String {
    {
      let mut object: ::latin1_string::Latin1String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLatin1String_right_to_output(self as *const ::latin1_string::Latin1String, n, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QLatin1String::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLatin1String_size(self as *const ::latin1_string::Latin1String) }
  }
}

impl Drop for ::latin1_string::Latin1String {
  /// C++ method: <span style='color: green;'>```[destructor] void QLatin1String::~QLatin1String()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLatin1String_destructor(self as *mut ::latin1_string::Latin1String) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Latin1String::mid](../struct.Latin1String.html#method.mid) method.
  pub trait Latin1StringMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> ::latin1_string::Latin1String;
  }
  impl<'largs> Latin1StringMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> ::latin1_string::Latin1String {
      let pos = self;
      {
        let mut object: ::latin1_string::Latin1String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLatin1String_mid_to_output_pos(original_self as *const ::latin1_string::Latin1String,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> Latin1StringMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> ::latin1_string::Latin1String {
      let pos = self.0;
      let n = self.1;
      {
        let mut object: ::latin1_string::Latin1String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLatin1String_mid_to_output_pos_n(original_self as *const ::latin1_string::Latin1String,
                                                             pos,
                                                             n,
                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::new](../struct.Latin1String.html#method.new) method.
  pub trait Latin1StringNewArgs {
    fn exec(self) -> ::latin1_string::Latin1String;
  }
  impl<'a> Latin1StringNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::latin1_string::Latin1String {
      let s = self;
      {
        let mut object: ::latin1_string::Latin1String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLatin1String_constructor_QByteArray(s as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl Latin1StringNewArgs for () {
    fn exec(self) -> ::latin1_string::Latin1String {

      {
        let mut object: ::latin1_string::Latin1String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLatin1String_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::new_unsafe](../struct.Latin1String.html#method.new_unsafe) method.
  pub trait Latin1StringNewUnsafeArgs {
    unsafe fn exec(self) -> ::latin1_string::Latin1String;
  }
  impl Latin1StringNewUnsafeArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::latin1_string::Latin1String {
      let s = self;
      {
        let mut object: ::latin1_string::Latin1String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QLatin1String_constructor_char(s, &mut object);
        object
      }
    }
  }
  impl Latin1StringNewUnsafeArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::latin1_string::Latin1String {
      let s = self.0;
      let sz = self.1;
      {
        let mut object: ::latin1_string::Latin1String =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QLatin1String_constructor_char_int(s, sz, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_eq](../struct.Latin1String.html#method.op_eq) method.
  pub trait Latin1StringOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpEqArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_eq_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                              s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpEqArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_eq_QString(original_self as *const ::latin1_string::Latin1String,
                                                           s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_ge](../struct.Latin1String.html#method.op_ge) method.
  pub trait Latin1StringOpGeArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpGeArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_ge_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                              s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpGeArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_ge_QString(original_self as *const ::latin1_string::Latin1String,
                                                           s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_gt](../struct.Latin1String.html#method.op_gt) method.
  pub trait Latin1StringOpGtArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpGtArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_gt_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                              s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpGtArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_gt_QString(original_self as *const ::latin1_string::Latin1String,
                                                           s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_le](../struct.Latin1String.html#method.op_le) method.
  pub trait Latin1StringOpLeArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpLeArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_le_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                              s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpLeArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_le_QString(original_self as *const ::latin1_string::Latin1String,
                                                           s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_lt](../struct.Latin1String.html#method.op_lt) method.
  pub trait Latin1StringOpLtArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpLtArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_lt_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                              s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpLtArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_lt_QString(original_self as *const ::latin1_string::Latin1String,
                                                           s as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Latin1String::op_neq](../struct.Latin1String.html#method.op_neq) method.
  pub trait Latin1StringOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool;
  }
  impl<'largs> Latin1StringOpNeqArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_neq_QByteArray(original_self as *const ::latin1_string::Latin1String,
                                                               s as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> Latin1StringOpNeqArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::latin1_string::Latin1String) -> bool {
      let s = self;
      unsafe {
        ::ffi::qt_core_c_QLatin1String_operator_neq_QString(original_self as *const ::latin1_string::Latin1String,
                                                            s as *const ::string::String)
      }
    }
  }
}
