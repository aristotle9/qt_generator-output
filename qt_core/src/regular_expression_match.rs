/// C++ type: <span style='color: green;'>```QRegularExpressionMatch```</span>
#[repr(C)]
pub struct RegularExpressionMatch([u8; ::type_sizes::QT_CORE_REGULAR_EXPRESSION_MATCH_REGULAR_EXPRESSION_MATCH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for RegularExpressionMatch {
  unsafe fn new_uninitialized() -> RegularExpressionMatch {
    RegularExpressionMatch(::std::mem::uninitialized())
  }
}

impl RegularExpressionMatch {
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::captured```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn captured(&self, ()) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegularExpressionMatch::captured() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn captured(&self, &::string::String) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegularExpressionMatch::captured(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn captured(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QRegularExpressionMatch::captured(int nth = ?) const```</span>
  ///
  ///
  pub fn captured<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::RegularExpressionMatchCapturedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::capturedEnd```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn captured_end(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedEnd() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn captured_end(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedEnd(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn captured_end(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedEnd(int nth = ?) const```</span>
  ///
  ///
  pub fn captured_end<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegularExpressionMatchCapturedEndArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::capturedLength```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn captured_length(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedLength() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn captured_length(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedLength(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn captured_length(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedLength(int nth = ?) const```</span>
  ///
  ///
  pub fn captured_length<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegularExpressionMatchCapturedLengthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::capturedRef```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn captured_ref(&self, ()) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QRegularExpressionMatch::capturedRef() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn captured_ref(&self, &::string::String) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QRegularExpressionMatch::capturedRef(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn captured_ref(&self, ::libc::c_int) -> ::string_ref::StringRef```<br>
  /// C++ method: <span style='color: green;'>```QStringRef QRegularExpressionMatch::capturedRef(int nth = ?) const```</span>
  ///
  ///
  pub fn captured_ref<'largs, Args>(&'largs self, args: Args) -> ::string_ref::StringRef
    where Args: overloading::RegularExpressionMatchCapturedRefArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::capturedStart```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn captured_start(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedStart() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn captured_start(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedStart(const QString& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn captured_start(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::capturedStart(int nth = ?) const```</span>
  ///
  ///
  pub fn captured_start<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::RegularExpressionMatchCapturedStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QRegularExpressionMatch::capturedTexts() const```</span>
  ///
  ///
  pub fn captured_texts(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpressionMatch_capturedTexts_to_output(self as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpressionMatch::hasMatch() const```</span>
  ///
  ///
  pub fn has_match(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_hasMatch(self as *const ::regular_expression_match::RegularExpressionMatch) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpressionMatch::hasPartialMatch() const```</span>
  ///
  ///
  pub fn has_partial_match(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_hasPartialMatch(self as *const ::regular_expression_match::RegularExpressionMatch) }
  }

  /// C++ method: <span style='color: green;'>```bool QRegularExpressionMatch::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_isValid(self as *const ::regular_expression_match::RegularExpressionMatch) }
  }

  /// C++ method: <span style='color: green;'>```int QRegularExpressionMatch::lastCapturedIndex() const```</span>
  ///
  ///
  pub fn last_captured_index(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_lastCapturedIndex(self as *const ::regular_expression_match::RegularExpressionMatch) }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch::QRegularExpressionMatch```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionMatch::QRegularExpressionMatch()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::regular_expression_match::RegularExpressionMatch) -> ::regular_expression_match::RegularExpressionMatch```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionMatch::QRegularExpressionMatch(const QRegularExpressionMatch& match)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::regular_expression_match::RegularExpressionMatch
    where Args: overloading::RegularExpressionMatchNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionMatch& QRegularExpressionMatch::operator=(const QRegularExpressionMatch& match)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             match_: &'l1 ::regular_expression_match::RegularExpressionMatch)
                             -> &'l0 mut ::regular_expression_match::RegularExpressionMatch {
    let ffi_result = unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_operator_assign(self as *mut ::regular_expression_match::RegularExpressionMatch, match_ as *const ::regular_expression_match::RegularExpressionMatch) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression QRegularExpressionMatch::regularExpression() const```</span>
  ///
  ///
  pub fn regular_expression(&self) -> ::regular_expression::RegularExpression {
    {
      let mut object: ::regular_expression::RegularExpression =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QRegularExpressionMatch_regularExpression_to_output(self as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QRegularExpressionMatch::swap(QRegularExpressionMatch& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::regular_expression_match::RegularExpressionMatch) {
    unsafe {
      ::ffi::qt_core_c_QRegularExpressionMatch_swap(self as *mut ::regular_expression_match::RegularExpressionMatch,
                                                    other as *mut ::regular_expression_match::RegularExpressionMatch)
    }
  }
}

impl Drop for ::regular_expression_match::RegularExpressionMatch {
  /// C++ method: <span style='color: green;'>```[destructor] void QRegularExpressionMatch::~QRegularExpressionMatch()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_destructor(self as *mut ::regular_expression_match::RegularExpressionMatch) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::captured](../struct.RegularExpressionMatch.html#method.captured) method.
  pub trait RegularExpressionMatchCapturedArgs<'largs> {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::string::String;
  }
  impl<'largs> RegularExpressionMatchCapturedArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::string::String {
      let name = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_captured_to_output_name(original_self as *const ::regular_expression_match::RegularExpressionMatch, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_captured_to_output_no_args(original_self as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::string::String {
      let nth = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_captured_to_output_nth(original_self as *const ::regular_expression_match::RegularExpressionMatch, nth, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::captured_end](../struct.RegularExpressionMatch.html#method.captured_end) method.
  pub trait RegularExpressionMatchCapturedEndArgs<'largs> {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int;
  }
  impl<'largs> RegularExpressionMatchCapturedEndArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedEnd_name(original_self as *const ::regular_expression_match::RegularExpressionMatch, name as *const ::string::String) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedEndArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedEnd_no_args(original_self as *const ::regular_expression_match::RegularExpressionMatch) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedEndArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let nth = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedEnd_nth(original_self as *const ::regular_expression_match::RegularExpressionMatch, nth) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::captured_length](../struct.RegularExpressionMatch.html#method.captured_length) method.
  pub trait RegularExpressionMatchCapturedLengthArgs<'largs> {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int;
  }
  impl<'largs> RegularExpressionMatchCapturedLengthArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedLength_name(original_self as *const ::regular_expression_match::RegularExpressionMatch, name as *const ::string::String) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedLengthArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedLength_no_args(original_self as *const ::regular_expression_match::RegularExpressionMatch) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedLengthArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let nth = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedLength_nth(original_self as *const ::regular_expression_match::RegularExpressionMatch, nth) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::captured_ref](../struct.RegularExpressionMatch.html#method.captured_ref) method.
  pub trait RegularExpressionMatchCapturedRefArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::regular_expression_match::RegularExpressionMatch)
            -> ::string_ref::StringRef;
  }
  impl<'largs> RegularExpressionMatchCapturedRefArgs<'largs> for &'largs ::string::String {
    fn exec(self,
            original_self: &'largs ::regular_expression_match::RegularExpressionMatch)
            -> ::string_ref::StringRef {
      let name = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_capturedRef_to_output_name(original_self as *const ::regular_expression_match::RegularExpressionMatch, name as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedRefArgs<'largs> for () {
    fn exec(self,
            original_self: &'largs ::regular_expression_match::RegularExpressionMatch)
            -> ::string_ref::StringRef {

      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_capturedRef_to_output_no_args(original_self as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedRefArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::regular_expression_match::RegularExpressionMatch)
            -> ::string_ref::StringRef {
      let nth = self;
      {
        let mut object: ::string_ref::StringRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_capturedRef_to_output_nth(original_self as *const ::regular_expression_match::RegularExpressionMatch, nth, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::captured_start](../struct.RegularExpressionMatch.html#method.captured_start) method.
  pub trait RegularExpressionMatchCapturedStartArgs<'largs> {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int;
  }
  impl<'largs> RegularExpressionMatchCapturedStartArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedStart_name(original_self as *const ::regular_expression_match::RegularExpressionMatch, name as *const ::string::String) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedStartArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedStart_no_args(original_self as *const ::regular_expression_match::RegularExpressionMatch) }
    }
  }
  impl<'largs> RegularExpressionMatchCapturedStartArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::regular_expression_match::RegularExpressionMatch) -> ::libc::c_int {
      let nth = self;
      unsafe { ::ffi::qt_core_c_QRegularExpressionMatch_capturedStart_nth(original_self as *const ::regular_expression_match::RegularExpressionMatch, nth) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionMatch::new](../struct.RegularExpressionMatch.html#method.new) method.
  pub trait RegularExpressionMatchNewArgs {
    fn exec(self) -> ::regular_expression_match::RegularExpressionMatch;
  }
  impl<'a> RegularExpressionMatchNewArgs for &'a ::regular_expression_match::RegularExpressionMatch {
    fn exec(self) -> ::regular_expression_match::RegularExpressionMatch {
      let match_ = self;
      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_constructor_match(match_ as *const ::regular_expression_match::RegularExpressionMatch, &mut object);
        }
        object
      }
    }
  }
  impl RegularExpressionMatchNewArgs for () {
    fn exec(self) -> ::regular_expression_match::RegularExpressionMatch {

      {
        let mut object: ::regular_expression_match::RegularExpressionMatch =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QRegularExpressionMatch_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}
