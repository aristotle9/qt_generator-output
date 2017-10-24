/// C++ type: <span style='color: green;'>```QByteArrayMatcher```</span>
#[repr(C)]
pub struct ByteArrayMatcher([u8; ::type_sizes::QT_CORE_BYTE_ARRAY_MATCHER_BYTE_ARRAY_MATCHER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ByteArrayMatcher {
  unsafe fn new_uninitialized() -> ByteArrayMatcher {
    ByteArrayMatcher(::std::mem::uninitialized())
  }
}

impl ByteArrayMatcher {
  /// C++ method: <span style='color: green;'>```QByteArrayMatcher::indexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_in(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArrayMatcher::indexIn(const QByteArray& ba) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_in(&self, (&::byte_array::ByteArray, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArrayMatcher::indexIn(const QByteArray& ba, int from = ?) const```</span>
  ///
  ///
  pub fn index_in<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayMatcherIndexInArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArrayMatcher::indexIn```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_in_unsafe(&self, (*const ::libc::c_char, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArrayMatcher::indexIn(const char* str, int len) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_in_unsafe(&self, (*const ::libc::c_char, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QByteArrayMatcher::indexIn(const char* str, int len, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_in_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ByteArrayMatcherIndexInUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArrayMatcher::QByteArrayMatcher```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::byte_array_matcher::ByteArrayMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArrayMatcher::QByteArrayMatcher()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::byte_array_matcher::ByteArrayMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArrayMatcher::QByteArrayMatcher(const QByteArray& pattern)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::byte_array_matcher::ByteArrayMatcher) -> ::byte_array_matcher::ByteArrayMatcher```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArrayMatcher::QByteArrayMatcher(const QByteArrayMatcher& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::byte_array_matcher::ByteArrayMatcher
    where Args: overloading::ByteArrayMatcherNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QByteArrayMatcher::QByteArrayMatcher(const char* pattern, int length)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(pattern: *const ::libc::c_char,
                           length: ::libc::c_int)
                           -> ::byte_array_matcher::ByteArrayMatcher {
    {
      let mut object: ::byte_array_matcher::ByteArrayMatcher =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QByteArrayMatcher_constructor_pattern_length(pattern, length, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArrayMatcher& QByteArrayMatcher::operator=(const QByteArrayMatcher& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::byte_array_matcher::ByteArrayMatcher)
                             -> &'l0 mut ::byte_array_matcher::ByteArrayMatcher {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QByteArrayMatcher_operator_assign(self as *mut ::byte_array_matcher::ByteArrayMatcher,
                                                           other as *const ::byte_array_matcher::ByteArrayMatcher)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray QByteArrayMatcher::pattern() const```</span>
  ///
  ///
  pub fn pattern(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QByteArrayMatcher_pattern_to_output(self as *const ::byte_array_matcher::ByteArrayMatcher,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QByteArrayMatcher::setPattern(const QByteArray& pattern)```</span>
  ///
  ///
  pub fn set_pattern(&mut self, pattern: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QByteArrayMatcher_setPattern(self as *mut ::byte_array_matcher::ByteArrayMatcher,
                                                    pattern as *const ::byte_array::ByteArray)
    }
  }
}

impl Drop for ::byte_array_matcher::ByteArrayMatcher {
  /// C++ method: <span style='color: green;'>```[destructor] void QByteArrayMatcher::~QByteArrayMatcher()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QByteArrayMatcher_destructor(self as *mut ::byte_array_matcher::ByteArrayMatcher) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ByteArrayMatcher::index_in](../struct.ByteArrayMatcher.html#method.index_in) method.
  pub trait ByteArrayMatcherIndexInArgs<'largs> {
    fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayMatcherIndexInArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int {
      let ba = self;
      unsafe {
        ::ffi::qt_core_c_QByteArrayMatcher_indexIn_ba(original_self as *const ::byte_array_matcher::ByteArrayMatcher,
                                                      ba as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ByteArrayMatcherIndexInArgs<'largs> for (&'largs ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int {
      let ba = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QByteArrayMatcher_indexIn_ba_from(original_self as *const ::byte_array_matcher::ByteArrayMatcher, ba as *const ::byte_array::ByteArray, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArrayMatcher::index_in_unsafe](../struct.ByteArrayMatcher.html#method.index_in_unsafe) method.
  pub trait ByteArrayMatcherIndexInUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int;
  }
  impl<'largs> ByteArrayMatcherIndexInUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int {
      let str = self.0;
      let len = self.1;
      ::ffi::qt_core_c_QByteArrayMatcher_indexIn_str_len(original_self as *const ::byte_array_matcher::ByteArrayMatcher, str, len)
    }
  }
  impl<'largs> ByteArrayMatcherIndexInUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::byte_array_matcher::ByteArrayMatcher) -> ::libc::c_int {
      let str = self.0;
      let len = self.1;
      let from = self.2;
      ::ffi::qt_core_c_QByteArrayMatcher_indexIn_str_len_from(original_self as *const ::byte_array_matcher::ByteArrayMatcher, str, len, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ByteArrayMatcher::new](../struct.ByteArrayMatcher.html#method.new) method.
  pub trait ByteArrayMatcherNewArgs {
    fn exec(self) -> ::byte_array_matcher::ByteArrayMatcher;
  }
  impl ByteArrayMatcherNewArgs for () {
    fn exec(self) -> ::byte_array_matcher::ByteArrayMatcher {

      {
        let mut object: ::byte_array_matcher::ByteArrayMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArrayMatcher_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ByteArrayMatcherNewArgs for &'a ::byte_array_matcher::ByteArrayMatcher {
    fn exec(self) -> ::byte_array_matcher::ByteArrayMatcher {
      let other = self;
      {
        let mut object: ::byte_array_matcher::ByteArrayMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArrayMatcher_constructor_other(other as *const ::byte_array_matcher::ByteArrayMatcher, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ByteArrayMatcherNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::byte_array_matcher::ByteArrayMatcher {
      let pattern = self;
      {
        let mut object: ::byte_array_matcher::ByteArrayMatcher =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QByteArrayMatcher_constructor_pattern(pattern as *const ::byte_array::ByteArray,
                                                                 &mut object);
        }
        object
      }
    }
  }
}
