/// C++ type: <span style='color: green;'>```QStringMatcher```</span>
#[repr(C)]
pub struct StringMatcher([u8; ::type_sizes::QT_CORE_STRING_MATCHER_STRING_MATCHER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for StringMatcher {
  unsafe fn new_uninitialized() -> StringMatcher {
    StringMatcher(::std::mem::uninitialized())
  }
}

impl StringMatcher {
  /// C++ method: <span style='color: green;'>```QStringMatcher::indexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_in(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringMatcher::indexIn(const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_in(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringMatcher::indexIn(const QString& str, int from = ?) const```</span>
  ///
  ///
  pub fn index_in<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringMatcherIndexInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringMatcher::indexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_in_unsafe(&self, (*const ::char::Char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringMatcher::indexIn(const QChar* str, int length) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_in_unsafe(&self, (*const ::char::Char, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QStringMatcher::indexIn(const QChar* str, int length, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_in_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::StringMatcherIndexInUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringMatcher::QStringMatcher```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher(const QString& pattern)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, &::qt::CaseSensitivity)) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher(const QString& pattern, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::string_matcher::StringMatcher) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher(const QStringMatcher& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::string_matcher::StringMatcher
    where Args: overloading::StringMatcherNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringMatcher::QStringMatcher```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::char::Char, ::libc::c_int)) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher(const QChar* uc, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::char::Char, ::libc::c_int, &::qt::CaseSensitivity)) -> ::string_matcher::StringMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QStringMatcher::QStringMatcher(const QChar* uc, int len, Qt::CaseSensitivity cs = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::string_matcher::StringMatcher
    where Args: overloading::StringMatcherNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QStringMatcher& QStringMatcher::operator=(const QStringMatcher& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::string_matcher::StringMatcher)
                             -> &'l0 mut ::string_matcher::StringMatcher {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QStringMatcher_operator_assign(self as *mut ::string_matcher::StringMatcher,
                                                      other as *const ::string_matcher::StringMatcher)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString QStringMatcher::pattern() const```</span>
  ///
  ///
  pub fn pattern(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QStringMatcher_pattern_to_output(self as *const ::string_matcher::StringMatcher, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QStringMatcher::setCaseSensitivity(Qt::CaseSensitivity cs)```</span>
  ///
  ///
  pub fn set_case_sensitivity(&mut self, cs: &::qt::CaseSensitivity) {
    unsafe {
      ::ffi::qt_core_c_QStringMatcher_setCaseSensitivity(self as *mut ::string_matcher::StringMatcher,
                                                         cs as *const ::qt::CaseSensitivity)
    }
  }

  /// C++ method: <span style='color: green;'>```void QStringMatcher::setPattern(const QString& pattern)```</span>
  ///
  ///
  pub fn set_pattern(&mut self, pattern: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QStringMatcher_setPattern(self as *mut ::string_matcher::StringMatcher,
                                                 pattern as *const ::string::String)
    }
  }
}

impl Drop for ::string_matcher::StringMatcher {
  /// C++ method: <span style='color: green;'>```[destructor] void QStringMatcher::~QStringMatcher()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QStringMatcher_destructor(self as *mut ::string_matcher::StringMatcher) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [StringMatcher::index_in](../struct.StringMatcher.html#method.index_in) method.
  pub trait StringMatcherIndexInArgs<'largs> {
    fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int;
  }
  impl<'largs> StringMatcherIndexInArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int {
      let str = self;
      unsafe {
        ::ffi::qt_core_c_QStringMatcher_indexIn_str(original_self as *const ::string_matcher::StringMatcher,
                                                    str as *const ::string::String)
      }
    }
  }
  impl<'largs> StringMatcherIndexInArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int {
      let str = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QStringMatcher_indexIn_str_from(original_self as *const ::string_matcher::StringMatcher,
                                                         str as *const ::string::String,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringMatcher::index_in_unsafe](../struct.StringMatcher.html#method.index_in_unsafe) method.
  pub trait StringMatcherIndexInUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int;
  }
  impl<'largs> StringMatcherIndexInUnsafeArgs<'largs> for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int {
      let str = self.0;
      let length = self.1;
      ::ffi::qt_core_c_QStringMatcher_indexIn_str_length(original_self as *const ::string_matcher::StringMatcher,
                                                         str,
                                                         length)
    }
  }
  impl<'largs> StringMatcherIndexInUnsafeArgs<'largs> for (*const ::char::Char, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::string_matcher::StringMatcher) -> ::libc::c_int {
      let str = self.0;
      let length = self.1;
      let from = self.2;
      ::ffi::qt_core_c_QStringMatcher_indexIn_str_length_from(original_self as *const ::string_matcher::StringMatcher,
                                                              str,
                                                              length,
                                                              from)
    }
  }
  /// This trait represents a set of arguments accepted by [StringMatcher::new](../struct.StringMatcher.html#method.new) method.
  pub trait StringMatcherNewArgs {
    fn exec(self) -> ::string_matcher::StringMatcher;
  }
  impl StringMatcherNewArgs for () {
    fn exec(self) -> ::string_matcher::StringMatcher {

      {
        let mut object: ::string_matcher::StringMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringMatcher_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> StringMatcherNewArgs for &'a ::string_matcher::StringMatcher {
    fn exec(self) -> ::string_matcher::StringMatcher {
      let other = self;
      {
        let mut object: ::string_matcher::StringMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringMatcher_constructor_other(other as *const ::string_matcher::StringMatcher,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringMatcherNewArgs for &'a ::string::String {
    fn exec(self) -> ::string_matcher::StringMatcher {
      let pattern = self;
      {
        let mut object: ::string_matcher::StringMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringMatcher_constructor_pattern(pattern as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> StringMatcherNewArgs for (&'a ::string::String, &'a ::qt::CaseSensitivity) {
    fn exec(self) -> ::string_matcher::StringMatcher {
      let pattern = self.0;
      let cs = self.1;
      {
        let mut object: ::string_matcher::StringMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QStringMatcher_constructor_pattern_cs(pattern as *const ::string::String,
                                                                 cs as *const ::qt::CaseSensitivity,
                                                                 &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [StringMatcher::new_unsafe](../struct.StringMatcher.html#method.new_unsafe) method.
  pub trait StringMatcherNewUnsafeArgs {
    unsafe fn exec(self) -> ::string_matcher::StringMatcher;
  }
  impl StringMatcherNewUnsafeArgs for (*const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string_matcher::StringMatcher {
      let uc = self.0;
      let len = self.1;
      {
        let mut object: ::string_matcher::StringMatcher =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QStringMatcher_constructor_uc_len(uc, len, &mut object);
        object
      }
    }
  }
  impl<'a> StringMatcherNewUnsafeArgs for (*const ::char::Char, ::libc::c_int, &'a ::qt::CaseSensitivity) {
    unsafe fn exec(self) -> ::string_matcher::StringMatcher {
      let uc = self.0;
      let len = self.1;
      let cs = self.2;
      {
        let mut object: ::string_matcher::StringMatcher =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QStringMatcher_constructor_uc_len_cs(uc, len, cs as *const ::qt::CaseSensitivity, &mut object);
        object
      }
    }
  }
}
