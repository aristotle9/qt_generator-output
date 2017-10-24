/// C++ type: <span style='color: green;'>```QRegExp::CaretMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CaretMode {
  /// C++ enum variant: <span style='color: green;'>```CaretAtZero = 0```</span>
  AtZero = 0,
  /// C++ enum variant: <span style='color: green;'>```CaretAtOffset = 1```</span>
  AtOffset = 1,
  /// C++ enum variant: <span style='color: green;'>```CaretWontMatch = 2```</span>
  WontMatch = 2,
}

/// C++ type: <span style='color: green;'>```QRegExp::PatternSyntax```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PatternSyntax {
  /// C++ enum variant: <span style='color: green;'>```RegExp = 0```</span>
  RegExp = 0,
  /// C++ enum variant: <span style='color: green;'>```Wildcard = 1```</span>
  Wildcard = 1,
  /// C++ enum variant: <span style='color: green;'>```FixedString = 2```</span>
  FixedString = 2,
  /// C++ enum variant: <span style='color: green;'>```RegExp2 = 3```</span>
  RegExp2 = 3,
  /// C++ enum variant: <span style='color: green;'>```WildcardUnix = 4```</span>
  WildcardUnix = 4,
  /// C++ enum variant: <span style='color: green;'>```W3CXmlSchema11 = 5```</span>
  W3CXmlSchema11 = 5,
}

/// C++ type: <span style='color: green;'>```QRegExp```</span>
#[repr(C)]
pub struct RegExp([u8; ::type_sizes::QT_CORE_REG_EXP_REG_EXP]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RegExp {
  unsafe fn new_uninitialized() -> RegExp {
    RegExp(::std::mem::uninitialized())
  }
}

impl RegExp {
  /// C++ method: <span style='color: green;'>```QRegExp::cap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cap(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegExp::cap() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cap(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegExp::cap(int nth = ?) const```</span>
  ///
  ///
  pub fn cap<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::RegExpCapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegExp::cap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cap_mut(&mut self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegExp::cap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cap_mut(&mut self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegExp::cap(int nth = ?)```</span>
  ///
  ///
  pub fn cap_mut<'largs, Args>(&'largs mut self, args: Args) -> ::string::String
    where Args: overloading::RegExpCapMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QRegExp::captureCount() const```</span>
  ///
  ///
  pub fn capture_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRegExp_captureCount(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QRegExp::capturedTexts() const```</span>
  ///
  ///
  pub fn captured_texts(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_capturedTexts_to_output_const(self as *const ::reg_exp::RegExp, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QRegExp::capturedTexts()```</span>
  ///
  ///
  pub fn captured_texts_mut(&mut self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_capturedTexts_to_output(self as *mut ::reg_exp::RegExp, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QRegExp::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_errorString_to_output_const(self as *const ::reg_exp::RegExp, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QRegExp::errorString()```</span>
  ///
  ///
  pub fn error_string_mut(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_errorString_to_output(self as *mut ::reg_exp::RegExp, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegExp::escape(const QString& str)```</span>
  ///
  ///
  pub fn escape(str: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_escape_to_output(str as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRegExp::exactMatch(const QString& str) const```</span>
  ///
  ///
  pub fn exact_match(&self, str: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QRegExp_exactMatch(self as *const ::reg_exp::RegExp,
                                          str as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QRegExp::indexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_in(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::indexIn(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_in(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::indexIn(const QString& str, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn index_in(&self, (&::string::String, ::libc::c_int, ::reg_exp::CaretMode)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::indexIn(const QString& str, int offset = ?, QRegExp::CaretMode caretMode = ?) const```</span>
  ///
  ///
  pub fn index_in<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegExpIndexInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QRegExp::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegExp_isEmpty(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegExp::isMinimal() const```</span>
  ///
  ///
  pub fn is_minimal(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegExp_isMinimal(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegExp::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegExp_isValid(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```QRegExp::lastIndexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_in(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::lastIndexIn(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_in(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::lastIndexIn(const QString& str, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn last_index_in(&self, (&::string::String, ::libc::c_int, ::reg_exp::CaretMode)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::lastIndexIn(const QString& str, int offset = ?, QRegExp::CaretMode caretMode = ?) const```</span>
  ///
  ///
  pub fn last_index_in<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegExpLastIndexInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QRegExp::matchedLength() const```</span>
  ///
  ///
  pub fn matched_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRegExp_matchedLength(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```QRegExp::QRegExp```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::reg_exp::RegExp```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExp::QRegExp()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::reg_exp::RegExp) -> ::reg_exp::RegExp```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExp::QRegExp(const QRegExp& rx)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::reg_exp::RegExp```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExp::QRegExp(const QString& pattern)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, &::qt::CaseSensitivity)) -> ::reg_exp::RegExp```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExp::QRegExp(const QString& pattern, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::string::String, &::qt::CaseSensitivity, ::reg_exp::PatternSyntax)) -> ::reg_exp::RegExp```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExp::QRegExp(const QString& pattern, Qt::CaseSensitivity cs = ?, QRegExp::PatternSyntax syntax = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::reg_exp::RegExp
    where Args: overloading::RegExpNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegExp& QRegExp::operator=(const QRegExp& rx)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, rx: &'l1 ::reg_exp::RegExp) -> &'l0 mut ::reg_exp::RegExp {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QRegExp_operator_assign(self as *mut ::reg_exp::RegExp,
                                               rx as *const ::reg_exp::RegExp)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QRegExp::operator==(const QRegExp& rx) const```</span>
  ///
  ///
  pub fn op_eq(&self, rx: &::reg_exp::RegExp) -> bool {
    unsafe {
      ::ffi::qt_core_c_QRegExp_operator_eq(self as *const ::reg_exp::RegExp,
                                           rx as *const ::reg_exp::RegExp)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRegExp::operator!=(const QRegExp& rx) const```</span>
  ///
  ///
  pub fn op_neq(&self, rx: &::reg_exp::RegExp) -> bool {
    unsafe {
      ::ffi::qt_core_c_QRegExp_operator_neq(self as *const ::reg_exp::RegExp,
                                            rx as *const ::reg_exp::RegExp)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QRegExp::pattern() const```</span>
  ///
  ///
  pub fn pattern(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegExp_pattern_to_output(self as *const ::reg_exp::RegExp, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegExp::PatternSyntax QRegExp::patternSyntax() const```</span>
  ///
  ///
  pub fn pattern_syntax(&self) -> ::reg_exp::PatternSyntax {
    unsafe { ::ffi::qt_core_c_QRegExp_patternSyntax(self as *const ::reg_exp::RegExp) }
  }

  /// C++ method: <span style='color: green;'>```QRegExp::pos```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pos(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::pos() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pos(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::pos(int nth = ?) const```</span>
  ///
  ///
  pub fn pos<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegExpPosArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegExp::pos```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pos_mut(&mut self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::pos()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pos_mut(&mut self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegExp::pos(int nth = ?)```</span>
  ///
  ///
  pub fn pos_mut<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::RegExpPosMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QRegExp::setCaseSensitivity(Qt::CaseSensitivity cs)```</span>
  ///
  ///
  pub fn set_case_sensitivity(&mut self, cs: &::qt::CaseSensitivity) {
    unsafe {
      ::ffi::qt_core_c_QRegExp_setCaseSensitivity(self as *mut ::reg_exp::RegExp,
                                                  cs as *const ::qt::CaseSensitivity)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegExp::setMinimal(bool minimal)```</span>
  ///
  ///
  pub fn set_minimal(&mut self, minimal: bool) {
    unsafe { ::ffi::qt_core_c_QRegExp_setMinimal(self as *mut ::reg_exp::RegExp, minimal) }
  }

  /// C++ method: <span style='color: green;'>```void QRegExp::setPattern(const QString& pattern)```</span>
  ///
  ///
  pub fn set_pattern(&mut self, pattern: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QRegExp_setPattern(self as *mut ::reg_exp::RegExp,
                                          pattern as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegExp::setPatternSyntax(QRegExp::PatternSyntax syntax)```</span>
  ///
  ///
  pub fn set_pattern_syntax(&mut self, syntax: ::reg_exp::PatternSyntax) {
    unsafe { ::ffi::qt_core_c_QRegExp_setPatternSyntax(self as *mut ::reg_exp::RegExp, syntax) }
  }

  /// C++ method: <span style='color: green;'>```void QRegExp::swap(QRegExp& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::reg_exp::RegExp) {
    unsafe {
      ::ffi::qt_core_c_QRegExp_swap(self as *mut ::reg_exp::RegExp,
                                    other as *mut ::reg_exp::RegExp)
    }
  }
}

impl Drop for ::reg_exp::RegExp {
  /// C++ method: <span style='color: green;'>```[destructor] void QRegExp::~QRegExp()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRegExp_destructor(self as *mut ::reg_exp::RegExp) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::reg_exp::RegExp) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRegExp& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::reg_exp::RegExp, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRegExp& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::reg_exp::RegExp)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QRegExp& regExp)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::reg_exp::RegExp)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QRegExp& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QRegExp& regExp)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::data_stream::DataStream,
                        reg_exp: &'l1 mut ::reg_exp::RegExp)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QRegExp_G_operator_shr(in_ as *mut ::data_stream::DataStream,
                                            reg_exp as *mut ::reg_exp::RegExp)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegExp::cap](../struct.RegExp.html#method.cap) method.
  pub trait RegExpCapArgs<'largs> {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::string::String;
  }
  impl<'largs> RegExpCapArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_cap_to_output_const_no_args(original_self as *const ::reg_exp::RegExp, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegExpCapArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::string::String {
      let nth = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_cap_to_output_const_nth(original_self as *const ::reg_exp::RegExp, nth, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::cap_mut](../struct.RegExp.html#method.cap_mut) method.
  pub trait RegExpCapMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::string::String;
  }
  impl<'largs> RegExpCapMutArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_cap_to_output_no_args(original_self as *mut ::reg_exp::RegExp, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegExpCapMutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::string::String {
      let nth = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_cap_to_output_nth(original_self as *mut ::reg_exp::RegExp, nth, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::index_in](../struct.RegExp.html#method.index_in) method.
  pub trait RegExpIndexInArgs<'largs> {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int;
  }
  impl<'largs> RegExpIndexInArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QRegExp_indexIn_str(original_self as *const ::reg_exp::RegExp,
                                             str as *const ::string::String)
      }
    }
  }
  impl<'largs> RegExpIndexInArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self.0;
      let offset = self.1;
      unsafe {
        ::ffi::qt_core_c_QRegExp_indexIn_str_offset(original_self as *const ::reg_exp::RegExp,
                                                    str as *const ::string::String,
                                                    offset)
      }
    }
  }
  impl<'largs> RegExpIndexInArgs<'largs> for (&'largs ::string::String, ::libc::c_int, ::reg_exp::CaretMode) {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self.0;
      let offset = self.1;
      let caret_mode = self.2;
      unsafe {
        ::ffi::qt_core_c_QRegExp_indexIn_str_offset_caretMode(original_self as *const ::reg_exp::RegExp,
                                                              str as *const ::string::String,
                                                              offset,
                                                              caret_mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::last_index_in](../struct.RegExp.html#method.last_index_in) method.
  pub trait RegExpLastIndexInArgs<'largs> {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int;
  }
  impl<'largs> RegExpLastIndexInArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QRegExp_lastIndexIn_str(original_self as *const ::reg_exp::RegExp,
                                                 str as *const ::string::String)
      }
    }
  }
  impl<'largs> RegExpLastIndexInArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self.0;
      let offset = self.1;
      unsafe {
        ::ffi::qt_core_c_QRegExp_lastIndexIn_str_offset(original_self as *const ::reg_exp::RegExp,
                                                        str as *const ::string::String,
                                                        offset)
      }
    }
  }
  impl<'largs> RegExpLastIndexInArgs<'largs> for (&'largs ::string::String, ::libc::c_int, ::reg_exp::CaretMode) {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let str = self.0;
      let offset = self.1;
      let caret_mode = self.2;
      unsafe {
        ::ffi::qt_core_c_QRegExp_lastIndexIn_str_offset_caretMode(original_self as *const ::reg_exp::RegExp,
                                                                  str as *const ::string::String,
                                                                  offset,
                                                                  caret_mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::new](../struct.RegExp.html#method.new) method.
  pub trait RegExpNewArgs {
    fn exec(self) -> ::reg_exp::RegExp;
  }
  impl RegExpNewArgs for () {
    fn exec(self) -> ::reg_exp::RegExp {

      {
        let mut object: ::reg_exp::RegExp =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> RegExpNewArgs for &'a ::string::String {
    fn exec(self) -> ::reg_exp::RegExp {
      let pattern = self;
      {
        let mut object: ::reg_exp::RegExp =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_constructor_pattern(pattern as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RegExpNewArgs for (&'a ::string::String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::reg_exp::RegExp {
      let pattern = self.0;
      let cs = self.1;
      {
        let mut object: ::reg_exp::RegExp =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_constructor_pattern_cs(pattern as *const ::string::String,
                                                          cs as *const ::qt::CaseSensitivity,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> RegExpNewArgs for (&'a ::string::String, &'a ::qt::CaseSensitivity, ::reg_exp::PatternSyntax) {
    fn exec(self) -> ::reg_exp::RegExp {
      let pattern = self.0;
      let cs = self.1;
      let syntax = self.2;
      {
        let mut object: ::reg_exp::RegExp =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_constructor_pattern_cs_syntax(pattern as *const ::string::String,
                                                                 cs as *const ::qt::CaseSensitivity,
                                                                 syntax,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> RegExpNewArgs for &'a ::reg_exp::RegExp {
    fn exec(self) -> ::reg_exp::RegExp {
      let rx = self;
      {
        let mut object: ::reg_exp::RegExp =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_constructor_rx(rx as *const ::reg_exp::RegExp, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::pos](../struct.RegExp.html#method.pos) method.
  pub trait RegExpPosArgs<'largs> {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int;
  }
  impl<'largs> RegExpPosArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QRegExp_pos_const_no_args(original_self as *const ::reg_exp::RegExp) }
    }
  }
  impl<'largs> RegExpPosArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::reg_exp::RegExp) -> ::libc::c_int {
      let nth = self;
      unsafe { ::ffi::qt_core_c_QRegExp_pos_const_nth(original_self as *const ::reg_exp::RegExp, nth) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExp::pos_mut](../struct.RegExp.html#method.pos_mut) method.
  pub trait RegExpPosMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::libc::c_int;
  }
  impl<'largs> RegExpPosMutArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QRegExp_pos_no_args(original_self as *mut ::reg_exp::RegExp) }
    }
  }
  impl<'largs> RegExpPosMutArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::reg_exp::RegExp) -> ::libc::c_int {
      let nth = self;
      unsafe { ::ffi::qt_core_c_QRegExp_pos_nth(original_self as *mut ::reg_exp::RegExp, nth) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::reg_exp::RegExp {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QRegExp_G_qHash_key(key as *const ::reg_exp::RegExp) }
    }
  }
  impl<'a> HashArgs for (&'a ::reg_exp::RegExp, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QRegExp_G_qHash_key_seed(key as *const ::reg_exp::RegExp, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::reg_exp::RegExp) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let out = self.0;
      let reg_exp = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QRegExp_G_operator_shl(out as *mut ::data_stream::DataStream,
                                                reg_exp as *const ::reg_exp::RegExp)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::reg_exp::RegExp) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegExp_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                            arg2 as *const ::reg_exp::RegExp,
                                                            &mut object);
        }
        object
      }
    }
  }
}
