/// C++ type: <span style='color: green;'>```QDebug```</span>
#[repr(C)]
pub struct Debug([u8; ::type_sizes::QT_CORE_DEBUG_DEBUG]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Debug {
  unsafe fn new_uninitialized() -> Debug {
    Debug(::std::mem::uninitialized())
  }
}

impl Debug {
  /// C++ method: <span style='color: green;'>```bool QDebug::autoInsertSpaces() const```</span>
  ///
  ///
  pub fn auto_insert_spaces(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDebug_autoInsertSpaces(self as *const ::debug::Debug) }
  }

  /// C++ method: <span style='color: green;'>```QDebug::maybeQuote```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn maybe_quote(&mut self, ()) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::maybeQuote()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn maybe_quote(&mut self, ::libc::c_char) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::maybeQuote(char c = ?)```</span>
  ///
  ///
  pub fn maybe_quote<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugMaybeQuoteArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::maybeSpace()```</span>
  ///
  ///
  pub fn maybe_space<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_maybeSpace(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDebug::QDebug```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(::message_log_context::MsgType) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDebug::QDebug(QtMsgType t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::debug::Debug) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDebug::QDebug(const QDebug& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::debug::Debug
    where Args: overloading::DebugNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDebug::QDebug```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::io_device::IODevice) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDebug::QDebug(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::string::String) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDebug::QDebug(QString* string)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::debug::Debug
    where Args: overloading::DebugNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::noquote()```</span>
  ///
  ///
  pub fn noquote<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_noquote(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDebug& QDebug::nospace()```</span>
  ///
  ///
  pub fn nospace<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_nospace(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator=(const QDebug& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::debug::Debug) -> &'l0 mut ::debug::Debug {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QDebug_operator_assign(self as *mut ::debug::Debug, other as *const ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::char::Char) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(QChar t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::latin1_string::Latin1String) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(QLatin1String t)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::text_stream_manipulator::TextStreamManipulator) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(QTextStreamManipulator m)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, bool) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(bool t)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, ::libc::c_char) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(char t)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(const QByteArray& t)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::string::String) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(const QString& t)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, &'l1 ::string_ref::StringRef) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(const QStringRef& t)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn op_shl0(&mut self, ::libc::c_double) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(double t)```</span>
  ///
  ///
  pub fn op_shl0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl1(&mut self, *const ::libc::c_char) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(const char* t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl1(&mut self, *const ::libc::c_void) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(const void* t)```</span>
  ///
  ///
  pub unsafe fn op_shl1<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl1Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, ::libc::c_float) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(float t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, ::libc::c_int) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(int t)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn op_shl2(&mut self, u64) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(quint64 t)```</span>
  ///
  ///
  pub fn op_shl2<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl2Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl3(&mut self, ::libc::c_long) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(long t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl3(&mut self, ::libc::c_uint) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(unsigned int t)```</span>
  ///
  ///
  pub fn op_shl3<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl3Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl4(&mut self, i64) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(qint64 t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl4(&mut self, ::libc::c_ulong) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(unsigned long t)```</span>
  ///
  ///
  pub fn op_shl4<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl4Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl5(&mut self, ::libc::c_short) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(short t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl5(&mut self, ::libc::c_ushort) -> &'l0 mut ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::operator<<(unsigned short t)```</span>
  ///
  ///
  pub fn op_shl5<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::debug::Debug
    where Args: overloading::DebugOpShl5Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDebug& QDebug::quote()```</span>
  ///
  ///
  pub fn quote<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_quote(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDebug& QDebug::resetFormat()```</span>
  ///
  ///
  pub fn reset_format<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_resetFormat(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QDebug::setAutoInsertSpaces(bool b)```</span>
  ///
  ///
  pub fn set_auto_insert_spaces(&mut self, b: bool) {
    unsafe { ::ffi::qt_core_c_QDebug_setAutoInsertSpaces(self as *mut ::debug::Debug, b) }
  }

  /// C++ method: <span style='color: green;'>```void QDebug::setVerbosity(int verbosityLevel)```</span>
  ///
  ///
  pub fn set_verbosity(&mut self, verbosity_level: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QDebug_setVerbosity(self as *mut ::debug::Debug, verbosity_level) }
  }

  /// C++ method: <span style='color: green;'>```QDebug& QDebug::space()```</span>
  ///
  ///
  pub fn space<'l0>(&'l0 mut self) -> &'l0 mut ::debug::Debug {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_space(self as *mut ::debug::Debug) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QDebug::swap(QDebug& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::debug::Debug) {
    unsafe { ::ffi::qt_core_c_QDebug_swap(self as *mut ::debug::Debug, other as *mut ::debug::Debug) }
  }

  /// C++ method: <span style='color: green;'>```int QDebug::verbosity() const```</span>
  ///
  ///
  pub fn verbosity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QDebug_verbosity(self as *const ::debug::Debug) }
  }
}

impl Drop for ::debug::Debug {
  /// C++ method: <span style='color: green;'>```[destructor] void QDebug::~QDebug()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDebug_destructor(self as *mut ::debug::Debug) }
  }
}

/// C++ method: <span style='color: green;'>```void swap(QDebug& value1, QDebug& value2)```</span>
///
///
pub fn swap(value1: &mut ::debug::Debug, value2: &mut ::debug::Debug) {
  unsafe { ::ffi::qt_core_c_QDebug_G_swap(value1 as *mut ::debug::Debug, value2 as *mut ::debug::Debug) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Debug::maybe_quote](../struct.Debug.html#method.maybe_quote) method.
  pub trait DebugMaybeQuoteArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugMaybeQuoteArgs<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let c = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_maybeQuote_c(original_self as *mut ::debug::Debug, c) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugMaybeQuoteArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {

      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_maybeQuote_no_args(original_self as *mut ::debug::Debug) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::new](../struct.Debug.html#method.new) method.
  pub trait DebugNewArgs {
    fn exec(self) -> ::debug::Debug;
  }
  impl<'a> DebugNewArgs for &'a ::debug::Debug {
    fn exec(self) -> ::debug::Debug {
      let o = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDebug_constructor_o(o as *const ::debug::Debug, &mut object);
        }
        object
      }
    }
  }
  impl DebugNewArgs for ::message_log_context::MsgType {
    fn exec(self) -> ::debug::Debug {
      let t = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDebug_constructor_t(t, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::new_unsafe](../struct.Debug.html#method.new_unsafe) method.
  pub trait DebugNewUnsafeArgs {
    unsafe fn exec(self) -> ::debug::Debug;
  }
  impl DebugNewUnsafeArgs for *mut ::io_device::IODevice {
    unsafe fn exec(self) -> ::debug::Debug {
      let device = self;
      {
        let mut object: ::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QDebug_constructor_device(device, &mut object);
        object
      }
    }
  }
  impl DebugNewUnsafeArgs for *mut ::string::String {
    unsafe fn exec(self) -> ::debug::Debug {
      let string = self;
      {
        let mut object: ::debug::Debug = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QDebug_constructor_string(string, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl0](../struct.Debug.html#method.op_shl0) method.
  pub trait DebugOpShl0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::char::Char {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDebug_operator_shl_QChar(original_self as *mut ::debug::Debug,
                                                   t as *const ::char::Char)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDebug_operator_shl_QLatin1String(original_self as *mut ::debug::Debug,
                                                           t as *const ::latin1_string::Latin1String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::text_stream_manipulator::TextStreamManipulator {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let m = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_QTextStreamManipulator(original_self as *mut ::debug::Debug, m as *const ::text_stream_manipulator::TextStreamManipulator) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for bool {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_bool(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for ::libc::c_char {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_char(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDebug_operator_shl_const_QByteArray_ref(original_self as *mut ::debug::Debug,
                                                                  t as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::string_ref::StringRef {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDebug_operator_shl_const_QStringRef_ref(original_self as *mut ::debug::Debug,
                                                                  t as *const ::string_ref::StringRef)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QDebug_operator_shl_const_QString_ref(original_self as *mut ::debug::Debug,
                                                               t as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_double(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl1](../struct.Debug.html#method.op_shl1) method.
  pub trait DebugOpShl1Args<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl1Args<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = ::ffi::qt_core_c_QDebug_operator_shl_const_char_ptr(original_self as *mut ::debug::Debug, t);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl1Args<'largs> for *const ::libc::c_void {
    unsafe fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = ::ffi::qt_core_c_QDebug_operator_shl_const_void_ptr(original_self as *mut ::debug::Debug, t);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl2](../struct.Debug.html#method.op_shl2) method.
  pub trait DebugOpShl2Args<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl2Args<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_float(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl2Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_int(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl2Args<'largs> for u64 {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_quint64(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl3](../struct.Debug.html#method.op_shl3) method.
  pub trait DebugOpShl3Args<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl3Args<'largs> for ::libc::c_long {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_long(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl3Args<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDebug_operator_shl_unsigned_int(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl4](../struct.Debug.html#method.op_shl4) method.
  pub trait DebugOpShl4Args<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl4Args<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_qint64(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl4Args<'largs> for ::libc::c_ulong {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDebug_operator_shl_unsigned_long(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Debug::op_shl5](../struct.Debug.html#method.op_shl5) method.
  pub trait DebugOpShl5Args<'largs> {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug;
  }
  impl<'largs> DebugOpShl5Args<'largs> for ::libc::c_short {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QDebug_operator_shl_short(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> DebugOpShl5Args<'largs> for ::libc::c_ushort {
    fn exec(self, original_self: &'largs mut ::debug::Debug) -> &'largs mut ::debug::Debug {
      let t = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QDebug_operator_shl_unsigned_short(original_self as *mut ::debug::Debug, t) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
