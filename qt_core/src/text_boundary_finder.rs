/// C++ type: <span style='color: green;'>```QTextBoundaryFinder::BoundaryReason```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BoundaryReason {
  /// C++ enum variant: <span style='color: green;'>```NotAtBoundary = 0```</span>
  NotAtBoundary = 0,
  /// C++ enum variant: <span style='color: green;'>```BreakOpportunity = 31```</span>
  BreakOpportunity = 31,
  /// C++ enum variant: <span style='color: green;'>```StartOfItem = 32```</span>
  StartOfItem = 32,
  /// C++ enum variant: <span style='color: green;'>```EndOfItem = 64```</span>
  EndOfItem = 64,
  /// C++ enum variant: <span style='color: green;'>```MandatoryBreak = 128```</span>
  MandatoryBreak = 128,
  /// C++ enum variant: <span style='color: green;'>```SoftHyphen = 256```</span>
  SoftHyphen = 256,
}

impl ::flags::FlaggableEnum for BoundaryReason {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "BoundaryReason"
  }
}

/// C++ type: <span style='color: green;'>```QTextBoundaryFinder::BoundaryType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BoundaryType {
  /// C++ enum variant: <span style='color: green;'>```Grapheme = 0```</span>
  Grapheme = 0,
  /// C++ enum variant: <span style='color: green;'>```Word = 1```</span>
  Word = 1,
  /// C++ enum variant: <span style='color: green;'>```Sentence = 2```</span>
  Sentence = 2,
  /// C++ enum variant: <span style='color: green;'>```Line = 3```</span>
  Line = 3,
}

/// C++ type: <span style='color: green;'>```QTextBoundaryFinder```</span>
#[repr(C)]
pub struct TextBoundaryFinder([u8; ::type_sizes::QT_CORE_TEXT_BOUNDARY_FINDER_TEXT_BOUNDARY_FINDER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextBoundaryFinder {
  unsafe fn new_uninitialized() -> TextBoundaryFinder {
    TextBoundaryFinder(::std::mem::uninitialized())
  }
}

impl TextBoundaryFinder {
  /// C++ method: <span style='color: green;'>```QFlags<QTextBoundaryFinder::BoundaryReason> QTextBoundaryFinder::boundaryReasons() const```</span>
  ///
  ///
  pub fn boundary_reasons(&self) -> ::flags::Flags<::text_boundary_finder::BoundaryReason> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTextBoundaryFinder_boundaryReasons(self as *const ::text_boundary_finder::TextBoundaryFinder)
      };
    ::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```bool QTextBoundaryFinder::isAtBoundary() const```</span>
  ///
  ///
  pub fn is_at_boundary(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QTextBoundaryFinder_isAtBoundary(self as *const ::text_boundary_finder::TextBoundaryFinder)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextBoundaryFinder::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_isValid(self as *const ::text_boundary_finder::TextBoundaryFinder) }
  }

  /// C++ method: <span style='color: green;'>```QTextBoundaryFinder::QTextBoundaryFinder```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::text_boundary_finder::BoundaryType, &::string::String)) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType type, const QString& string)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::text_boundary_finder::TextBoundaryFinder) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder(const QTextBoundaryFinder& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_boundary_finder::TextBoundaryFinder
    where Args: overloading::TextBoundaryFinderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextBoundaryFinder::QTextBoundaryFinder```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((::text_boundary_finder::BoundaryType, *const ::char::Char, ::libc::c_int)) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::text_boundary_finder::BoundaryType, *const ::char::Char, ::libc::c_int, *mut ::libc::c_uchar)) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((::text_boundary_finder::BoundaryType, *const ::char::Char, ::libc::c_int, *mut ::libc::c_uchar, ::libc::c_int)) -> ::text_boundary_finder::TextBoundaryFinder```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextBoundaryFinder::QTextBoundaryFinder(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer = ?, int bufferSize = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::text_boundary_finder::TextBoundaryFinder
    where Args: overloading::TextBoundaryFinderNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QTextBoundaryFinder& QTextBoundaryFinder::operator=(const QTextBoundaryFinder& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::text_boundary_finder::TextBoundaryFinder)
                             -> &'l0 mut ::text_boundary_finder::TextBoundaryFinder {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QTextBoundaryFinder_operator_assign(self as *mut ::text_boundary_finder::TextBoundaryFinder, other as *const ::text_boundary_finder::TextBoundaryFinder)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QTextBoundaryFinder::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_position(self as *const ::text_boundary_finder::TextBoundaryFinder) }
  }

  /// C++ method: <span style='color: green;'>```void QTextBoundaryFinder::setPosition(int position)```</span>
  ///
  ///
  pub fn set_position(&mut self, position: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QTextBoundaryFinder_setPosition(self as *mut ::text_boundary_finder::TextBoundaryFinder,
                                                       position)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextBoundaryFinder::string() const```</span>
  ///
  ///
  pub fn string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTextBoundaryFinder_string_to_output(self as *const ::text_boundary_finder::TextBoundaryFinder, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBoundaryFinder::toEnd()```</span>
  ///
  ///
  pub fn to_end(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_toEnd(self as *mut ::text_boundary_finder::TextBoundaryFinder) }
  }

  /// C++ method: <span style='color: green;'>```int QTextBoundaryFinder::toNextBoundary()```</span>
  ///
  ///
  pub fn to_next_boundary(&mut self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QTextBoundaryFinder_toNextBoundary(self as *mut ::text_boundary_finder::TextBoundaryFinder)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextBoundaryFinder::toPreviousBoundary()```</span>
  ///
  ///
  pub fn to_previous_boundary(&mut self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QTextBoundaryFinder_toPreviousBoundary(self as *mut ::text_boundary_finder::TextBoundaryFinder)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextBoundaryFinder::toStart()```</span>
  ///
  ///
  pub fn to_start(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_toStart(self as *mut ::text_boundary_finder::TextBoundaryFinder) }
  }

  /// C++ method: <span style='color: green;'>```QTextBoundaryFinder::BoundaryType QTextBoundaryFinder::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::text_boundary_finder::BoundaryType {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_type(self as *const ::text_boundary_finder::TextBoundaryFinder) }
  }
}

impl Drop for ::text_boundary_finder::TextBoundaryFinder {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextBoundaryFinder::~QTextBoundaryFinder()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTextBoundaryFinder_destructor(self as *mut ::text_boundary_finder::TextBoundaryFinder) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextBoundaryFinder::new](../struct.TextBoundaryFinder.html#method.new) method.
  pub trait TextBoundaryFinderNewArgs {
    fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder;
  }
  impl TextBoundaryFinderNewArgs for () {
    fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {

      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTextBoundaryFinder_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TextBoundaryFinderNewArgs for &'a ::text_boundary_finder::TextBoundaryFinder {
    fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {
      let other = self;
      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTextBoundaryFinder_constructor_other(other as *const ::text_boundary_finder::TextBoundaryFinder, &mut object);
        }
        object
      }
    }
  }
  impl<'a> TextBoundaryFinderNewArgs for (::text_boundary_finder::BoundaryType, &'a ::string::String) {
    fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {
      let type_ = self.0;
      let string = self.1;
      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QTextBoundaryFinder_constructor_type_string(type_,
                                                                       string as *const ::string::String,
                                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextBoundaryFinder::new_unsafe](../struct.TextBoundaryFinder.html#method.new_unsafe) method.
  pub trait TextBoundaryFinderNewUnsafeArgs {
    unsafe fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder;
  }
  impl TextBoundaryFinderNewUnsafeArgs for (::text_boundary_finder::BoundaryType, *const ::char::Char, ::libc::c_int) {
    unsafe fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {
      let type_ = self.0;
      let chars = self.1;
      let length = self.2;
      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextBoundaryFinder_constructor_type_chars_length(type_, chars, length, &mut object);
        object
      }
    }
  }
  impl TextBoundaryFinderNewUnsafeArgs
    for (::text_boundary_finder::BoundaryType, *const ::char::Char, ::libc::c_int, *mut ::libc::c_uchar) {
    unsafe fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {
      let type_ = self.0;
      let chars = self.1;
      let length = self.2;
      let buffer = self.3;
      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer(type_,
                                                                                  chars,
                                                                                  length,
                                                                                  buffer,
                                                                                  &mut object);
        object
      }
    }
  }
  impl TextBoundaryFinderNewUnsafeArgs
    for (::text_boundary_finder::BoundaryType,
                                                *const ::char::Char,
                                                ::libc::c_int,
                                                *mut ::libc::c_uchar,
                                                ::libc::c_int) {
    unsafe fn exec(self) -> ::text_boundary_finder::TextBoundaryFinder {
      let type_ = self.0;
      let chars = self.1;
      let length = self.2;
      let buffer = self.3;
      let buffer_size = self.4;
      {
        let mut object: ::text_boundary_finder::TextBoundaryFinder =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer_bufferSize(type_,
                                                                                             chars,
                                                                                             length,
                                                                                             buffer,
                                                                                             buffer_size,
                                                                                             &mut object);
        object
      }
    }
  }
}
