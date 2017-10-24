/// C++ type: <span style='color: green;'>```QRegularExpression::MatchOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MatchOption {
  /// C++ enum variant: <span style='color: green;'>```NoMatchOption = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```AnchoredMatchOption = 1```</span>
  Anchored = 1,
  /// C++ enum variant: <span style='color: green;'>```DontCheckSubjectStringMatchOption = 2```</span>
  DontCheckSubjectString = 2,
}

impl ::flags::FlaggableEnum for MatchOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "MatchOption"
  }
}

/// C++ type: <span style='color: green;'>```QRegularExpression::MatchType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MatchType {
  /// C++ enum variant: <span style='color: green;'>```NormalMatch = 0```</span>
  Normal = 0,
  /// C++ enum variant: <span style='color: green;'>```PartialPreferCompleteMatch = 1```</span>
  PartialPreferComplete = 1,
  /// C++ enum variant: <span style='color: green;'>```PartialPreferFirstMatch = 2```</span>
  PartialPreferFirst = 2,
  /// C++ enum variant: <span style='color: green;'>```NoMatch = 3```</span>
  No = 3,
}

/// C++ type: <span style='color: green;'>```QRegularExpression::PatternOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PatternOption {
  /// C++ enum variant: <span style='color: green;'>```NoPatternOption = 0```</span>
  NoPattern = 0,
  /// C++ enum variant: <span style='color: green;'>```CaseInsensitiveOption = 1```</span>
  CaseInsensitive = 1,
  /// C++ enum variant: <span style='color: green;'>```DotMatchesEverythingOption = 2```</span>
  DotMatchesEverything = 2,
  /// C++ enum variant: <span style='color: green;'>```MultilineOption = 4```</span>
  Multiline = 4,
  /// C++ enum variant: <span style='color: green;'>```ExtendedPatternSyntaxOption = 8```</span>
  ExtendedPatternSyntax = 8,
  /// C++ enum variant: <span style='color: green;'>```InvertedGreedinessOption = 16```</span>
  InvertedGreediness = 16,
  /// C++ enum variant: <span style='color: green;'>```DontCaptureOption = 32```</span>
  DontCapture = 32,
  /// C++ enum variant: <span style='color: green;'>```UseUnicodePropertiesOption = 64```</span>
  UseUnicodeProperties = 64,
  /// C++ enum variant: <span style='color: green;'>```OptimizeOnFirstUsageOption = 128```</span>
  OptimizeOnFirstUsage = 128,
  /// C++ enum variant: <span style='color: green;'>```DontAutomaticallyOptimizeOption = 256```</span>
  DontAutomaticallyOptimize = 256,
}

impl ::flags::FlaggableEnum for PatternOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "PatternOption"
  }
}

/// C++ type: <span style='color: green;'>```QRegularExpression```</span>
#[repr(C)]
pub struct RegularExpression([u8; ::type_sizes::QT_CORE_REGULAR_EXPRESSION_REGULAR_EXPRESSION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RegularExpression {
  unsafe fn new_uninitialized() -> RegularExpression {
    RegularExpression(::std::mem::uninitialized())
  }
}

impl RegularExpression {
  /// C++ method: <span style='color: green;'>```int QRegularExpression::captureCount() const```</span>
  ///
  ///
  pub fn capture_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRegularExpression_captureCount(self as *const ::regular_expression::RegularExpression) }
  }

  /// C++ method: <span style='color: green;'>```QString QRegularExpression::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_errorString_to_output(self as *const ::regular_expression::RegularExpression, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegularExpression::escape(const QString& str)```</span>
  ///
  ///
  pub fn escape(str: &::string::String) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_escape_to_output(str as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression::globalMatch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn global_match(&self, &::string::String) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QString& subject) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string::String, ::libc::c_int)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QString& subject, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string::String, ::libc::c_int, ::regular_expression::MatchType)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QString& subject, int offset = ?, QRegularExpression::MatchType matchType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string::String, ::libc::c_int, ::regular_expression::MatchType, ::flags::Flags<::regular_expression::MatchOption>)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QString& subject, int offset = ?, QRegularExpression::MatchType matchType = ?, QFlags<QRegularExpression::MatchOption> matchOptions = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn global_match(&self, &::string_ref::StringRef) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QStringRef& subjectRef) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QStringRef& subjectRef, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QStringRef& subjectRef, int offset = ?, QRegularExpression::MatchType matchType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn global_match(&self, (&::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType, ::flags::Flags<::regular_expression::MatchOption>)) -> ::regular_expression_match_iterator::RegularExpressionMatchIterator```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatchIterator QRegularExpression::globalMatch(const QStringRef& subjectRef, int offset = ?, QRegularExpression::MatchType matchType = ?, QFlags<QRegularExpression::MatchOption> matchOptions = ?) const```</span>
  ///
  ///
  pub fn global_match<'largs, Args>(&'largs self,
                                    args: Args)
                                    -> ::regular_expression_match_iterator::RegularExpressionMatchIterator
    where Args: overloading::RegularExpressionGlobalMatchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QRegularExpression::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpression_isValid(self as *const ::regular_expression::RegularExpression) }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression::match```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn match_(&self, &::string::String) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QString& subject) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn match_(&self, (&::string::String, ::libc::c_int)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QString& subject, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn match_(&self, (&::string::String, ::libc::c_int, ::regular_expression::MatchType)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QString& subject, int offset = ?, QRegularExpression::MatchType matchType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn match_(&self, (&::string::String, ::libc::c_int, ::regular_expression::MatchType, ::flags::Flags<::regular_expression::MatchOption>)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QString& subject, int offset = ?, QRegularExpression::MatchType matchType = ?, QFlags<QRegularExpression::MatchOption> matchOptions = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn match_(&self, &::string_ref::StringRef) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QStringRef& subjectRef) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn match_(&self, (&::string_ref::StringRef, ::libc::c_int)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QStringRef& subjectRef, int offset = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn match_(&self, (&::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QStringRef& subjectRef, int offset = ?, QRegularExpression::MatchType matchType = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn match_(&self, (&::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType, ::flags::Flags<::regular_expression::MatchOption>)) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch QRegularExpression::match(const QStringRef& subjectRef, int offset = ?, QRegularExpression::MatchType matchType = ?, QFlags<QRegularExpression::MatchOption> matchOptions = ?) const```</span>
  ///
  ///
  pub fn match_<'largs, Args>(&'largs self, args: Args) -> ::regular_expression_match::RegularExpressionMatch
    where Args: overloading::RegularExpressionMatchArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QRegularExpression::namedCaptureGroups() const```</span>
  ///
  ///
  pub fn named_capture_groups(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_namedCaptureGroups_to_output(self as *const ::regular_expression::RegularExpression, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression::QRegularExpression```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::regular_expression::RegularExpression```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpression::QRegularExpression()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::regular_expression::RegularExpression) -> ::regular_expression::RegularExpression```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpression::QRegularExpression(const QRegularExpression& re)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::regular_expression::RegularExpression```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpression::QRegularExpression(const QString& pattern)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((&::string::String, ::flags::Flags<::regular_expression::PatternOption>)) -> ::regular_expression::RegularExpression```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpression::QRegularExpression(const QString& pattern, QFlags<QRegularExpression::PatternOption> options = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::regular_expression::RegularExpression
    where Args: overloading::RegularExpressionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegularExpression& QRegularExpression::operator=(const QRegularExpression& re)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             re: &'l1 ::regular_expression::RegularExpression)
                             -> &'l0 mut ::regular_expression::RegularExpression {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_operator_assign(self as *mut ::regular_expression::RegularExpression,
                                                            re as *const ::regular_expression::RegularExpression)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpression::operator==(const QRegularExpression& re) const```</span>
  ///
  ///
  pub fn op_eq(&self, re: &::regular_expression::RegularExpression) -> bool {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_operator_eq(self as *const ::regular_expression::RegularExpression,
                                                      re as *const ::regular_expression::RegularExpression)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpression::operator!=(const QRegularExpression& re) const```</span>
  ///
  ///
  pub fn op_neq(&self, re: &::regular_expression::RegularExpression) -> bool {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_operator_neq(self as *const ::regular_expression::RegularExpression,
                                                       re as *const ::regular_expression::RegularExpression)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpression::optimize() const```</span>
  ///
  ///
  pub fn optimize(&self) {
    unsafe { ::ffi::qt_core_c_QRegularExpression_optimize(self as *const ::regular_expression::RegularExpression) }
  }

  /// C++ method: <span style='color: green;'>```QString QRegularExpression::pattern() const```</span>
  ///
  ///
  pub fn pattern(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_pattern_to_output(self as *const ::regular_expression::RegularExpression,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QRegularExpression::patternErrorOffset() const```</span>
  ///
  ///
  pub fn pattern_error_offset(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_patternErrorOffset(self as *const ::regular_expression::RegularExpression)
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QRegularExpression::PatternOption> QRegularExpression::patternOptions() const```</span>
  ///
  ///
  pub fn pattern_options(&self) -> ::flags::Flags<::regular_expression::PatternOption> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_patternOptions(self as *const ::regular_expression::RegularExpression)
      };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpression::setPattern(const QString& pattern)```</span>
  ///
  ///
  pub fn set_pattern(&mut self, pattern: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_setPattern(self as *mut ::regular_expression::RegularExpression,
                                                     pattern as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpression::setPatternOptions(QFlags<QRegularExpression::PatternOption> options)```</span>
  ///
  ///
  pub fn set_pattern_options(&mut self, options: ::flags::Flags<::regular_expression::PatternOption>) {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_setPatternOptions(self as *mut ::regular_expression::RegularExpression,
                                                            options.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpression::swap(QRegularExpression& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::regular_expression::RegularExpression) {
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_swap(self as *mut ::regular_expression::RegularExpression,
                                               other as *mut ::regular_expression::RegularExpression)
    }
  }
}

impl Drop for ::regular_expression::RegularExpression {
  /// C++ method: <span style='color: green;'>```[destructor] void QRegularExpression::~QRegularExpression()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRegularExpression_destructor(self as *mut ::regular_expression::RegularExpression) }
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
/// Rust arguments: ```fn hash(&::regular_expression::RegularExpression) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRegularExpression& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::regular_expression::RegularExpression, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QRegularExpression& key, unsigned int seed = ?)```</span>
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::regular_expression::RegularExpression)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QRegularExpression& re)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::regular_expression::RegularExpression)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QRegularExpression& re)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::regular_expression_match::RegularExpressionMatch)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QRegularExpressionMatch& match)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QRegularExpression& re)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::data_stream::DataStream,
                        re: &'l1 mut ::regular_expression::RegularExpression)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result =
    unsafe {
      ::ffi::qt_core_c_QRegularExpression_G_operator_shr(in_ as *mut ::data_stream::DataStream,
                                                         re as *mut ::regular_expression::RegularExpression)
    };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```swap```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn swap((&mut ::regular_expression::RegularExpression, &mut ::regular_expression::RegularExpression)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QRegularExpression& value1, QRegularExpression& value2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn swap((&mut ::regular_expression_match::RegularExpressionMatch, &mut ::regular_expression_match::RegularExpressionMatch)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QRegularExpressionMatch& value1, QRegularExpressionMatch& value2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn swap((&mut ::regular_expression_match_iterator::RegularExpressionMatchIterator, &mut ::regular_expression_match_iterator::RegularExpressionMatchIterator)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QRegularExpressionMatchIterator& value1, QRegularExpressionMatchIterator& value2)```</span>
///
///
pub fn swap<Args>(args: Args) -> ()
  where Args: overloading::SwapArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegularExpression::global_match](../struct.RegularExpression.html#method.global_match) method.
  pub trait RegularExpressionGlobalMatchArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator;
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs> for &'largs ::string::String {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject = self;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subject(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject_ref = self;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject_ref = self.0;
      let offset = self.1;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject_ref = self.0;
      let offset = self.1;
      let match_type = self.2;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, match_type, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs>
    for (&'largs ::string_ref::StringRef,
                                                                 ::libc::c_int,
                                                                 ::regular_expression::MatchType,
                                                                 ::flags::Flags<::regular_expression::MatchOption>) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject_ref = self.0;
      let offset = self.1;
      let match_type = self.2;
      let match_options = self.3;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subjectRef_offset_matchType_matchOptions(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, match_type, match_options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject = self.0;
      let offset = self.1;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, ::regular_expression::MatchType) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject = self.0;
      let offset = self.1;
      let match_type = self.2;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, match_type, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionGlobalMatchArgs<'largs>
    for (&'largs ::string::String,
                                                                 ::libc::c_int,
                                                                 ::regular_expression::MatchType,
                                                                 ::flags::Flags<::regular_expression::MatchOption>) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match_iterator::RegularExpressionMatchIterator {
      let subject = self.0;
      let offset = self.1;
      let match_type = self.2;
      let match_options = self.3;
      {
        let mut object: ::regular_expression_match_iterator::RegularExpressionMatchIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_globalMatch_to_output_subject_offset_matchType_matchOptions(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, match_type, match_options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpression::match_](../struct.RegularExpression.html#method.match_) method.
  pub trait RegularExpressionMatchArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch;
  }
  impl<'largs> RegularExpressionMatchArgs<'largs> for &'largs ::string::String {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject = self;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subject(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject_ref = self;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subjectRef(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs> for (&'largs ::string_ref::StringRef, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject_ref = self.0;
      let offset = self.1;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subjectRef_offset(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs>
    for (&'largs ::string_ref::StringRef, ::libc::c_int, ::regular_expression::MatchType) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject_ref = self.0;
      let offset = self.1;
      let match_type = self.2;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, match_type, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs>
    for (&'largs ::string_ref::StringRef,
                                                           ::libc::c_int,
                                                           ::regular_expression::MatchType,
                                                           ::flags::Flags<::regular_expression::MatchOption>) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject_ref = self.0;
      let offset = self.1;
      let match_type = self.2;
      let match_options = self.3;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subjectRef_offset_matchType_matchOptions(original_self as *const ::regular_expression::RegularExpression, subject_ref as *const ::string_ref::StringRef, offset, match_type, match_options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject = self.0;
      let offset = self.1;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subject_offset(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, ::regular_expression::MatchType) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject = self.0;
      let offset = self.1;
      let match_type = self.2;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, match_type, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchArgs<'largs>
    for (&'largs ::string::String,
                                                           ::libc::c_int,
                                                           ::regular_expression::MatchType,
                                                           ::flags::Flags<::regular_expression::MatchOption>) {
    fn exec(self,
            original_self: &'largs ::regular_expression::RegularExpression)
            -> ::regular_expression_match::RegularExpressionMatch {
      let subject = self.0;
      let offset = self.1;
      let match_type = self.2;
      let match_options = self.3;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_match_to_output_subject_offset_matchType_matchOptions(original_self as *const ::regular_expression::RegularExpression, subject as *const ::string::String, offset, match_type, match_options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpression::new](../struct.RegularExpression.html#method.new) method.
  pub trait RegularExpressionNewArgs {
    fn exec(self) -> ::regular_expression::RegularExpression;
  }
  impl RegularExpressionNewArgs for () {
    fn exec(self) -> ::regular_expression::RegularExpression {

      {
        let mut object: ::regular_expression::RegularExpression =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> RegularExpressionNewArgs for &'a ::string::String {
    fn exec(self) -> ::regular_expression::RegularExpression {
      let pattern = self;
      {
        let mut object: ::regular_expression::RegularExpression =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_constructor_pattern(pattern as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> RegularExpressionNewArgs for (&'a ::string::String, ::flags::Flags<::regular_expression::PatternOption>) {
    fn exec(self) -> ::regular_expression::RegularExpression {
      let pattern = self.0;
      let options = self.1;
      {
        let mut object: ::regular_expression::RegularExpression =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_constructor_pattern_options(pattern as *const ::string::String,
                                                                          options.to_int() as ::libc::c_uint,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> RegularExpressionNewArgs for &'a ::regular_expression::RegularExpression {
    fn exec(self) -> ::regular_expression::RegularExpression {
      let re = self;
      {
        let mut object: ::regular_expression::RegularExpression =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_constructor_re(re as *const ::regular_expression::RegularExpression,
                                                             &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::regular_expression::RegularExpression {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QRegularExpression_G_qHash_key(key as *const ::regular_expression::RegularExpression) }
    }
  }
  impl<'a> HashArgs for (&'a ::regular_expression::RegularExpression, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_core_c_QRegularExpression_G_qHash_key_seed(key as *const ::regular_expression::RegularExpression,
                                                             seed)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::regular_expression::RegularExpression) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let out = self.0;
      let re = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_G_operator_shl(out as *mut ::data_stream::DataStream,
                                                             re as *const ::regular_expression::RegularExpression)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::regular_expression_match::RegularExpressionMatch) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let debug = self.0;
      let match_ = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_match(debug as *const ::debug::Debug, match_ as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::regular_expression::RegularExpression) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let debug = self.0;
      let re = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpression_G_operator_shl_to_output_debug_re(debug as *const ::debug::Debug, re as *const ::regular_expression::RegularExpression, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [swap](../fn.swap.html) method.
  pub trait SwapArgs {
    fn exec(self) -> ();
  }
  impl<'a> SwapArgs
    for (&'a mut ::regular_expression_match_iterator::RegularExpressionMatchIterator,
                             &'a mut ::regular_expression_match_iterator::RegularExpressionMatchIterator) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatchIterator_QRegularExpressionMatchIterator(value1 as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator, value2 as *mut ::regular_expression_match_iterator::RegularExpressionMatchIterator) }
    }
  }
  impl<'a> SwapArgs
    for (&'a mut ::regular_expression_match::RegularExpressionMatch,
                             &'a mut ::regular_expression_match::RegularExpressionMatch) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_core_c_QRegularExpression_G_swap_QRegularExpressionMatch_QRegularExpressionMatch(value1 as *mut ::regular_expression_match::RegularExpressionMatch, value2 as *mut ::regular_expression_match::RegularExpressionMatch) }
    }
  }
  impl<'a> SwapArgs
    for (&'a mut ::regular_expression::RegularExpression, &'a mut ::regular_expression::RegularExpression) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe { ::ffi::qt_core_c_QRegularExpression_G_swap_QRegularExpression_QRegularExpression(value1 as *mut ::regular_expression::RegularExpression, value2 as *mut ::regular_expression::RegularExpression) }
    }
  }
}
