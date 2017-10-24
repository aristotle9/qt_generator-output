/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(int, int)` to a Rust extern function.
  ///
  /// Use `SlotCIntCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_int_int```</span>
  #[repr(C)]
  pub struct RawSlotCIntCInt(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCIntCInt {
    type Arguments = (::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,int)\0"
    }
  }
  impl RawSlotCIntCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_int_int::custom_slot(int arg0, int arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: ::libc::c_int) {
      unsafe {
        ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_custom_slot(self as *mut ::slots::raw::RawSlotCIntCInt, arg0, arg1)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_int_int::qt_gui_c_SlotWrapper_int_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCInt> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_int_int::set(void (*FN_PTR)(void*, int, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::libc::c_int, ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_set(self as *mut ::slots::raw::RawSlotCIntCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, int, int)` to a Rust extern function.
  ///
  /// Use `SlotCIntCIntCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_int_int_int```</span>
  #[repr(C)]
  pub struct RawSlotCIntCIntCInt(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCIntCIntCInt {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,int,int)\0"
    }
  }
  impl RawSlotCIntCIntCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_int_int_int::custom_slot(int arg0, int arg1, int arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: ::libc::c_int, arg2: ::libc::c_int) {
      unsafe {
        ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_int_custom_slot(self as *mut ::slots::raw::RawSlotCIntCIntCInt,
                                                                     arg0,
                                                                     arg1,
                                                                     arg2)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_int_int_int::qt_gui_c_SlotWrapper_int_int_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCIntCInt> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_int_int_int::set(void (*FN_PTR)(void*, int, int, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_int_set(self as *mut ::slots::raw::RawSlotCIntCIntCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntCIntCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_int_int_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QClipboard::Mode)` to a Rust extern function.
  ///
  /// Use `SlotClipboardModeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QClipboard_Mode```</span>
  #[repr(C)]
  pub struct RawSlotClipboardModeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotClipboardModeRef {
    type Arguments = (&'static ::clipboard::Mode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QClipboard::Mode)\0"
    }
  }
  impl RawSlotClipboardModeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QClipboard_Mode::custom_slot(QClipboard::Mode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::clipboard::Mode) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QClipboard_Mode_custom_slot(self as *mut ::slots::raw::RawSlotClipboardModeRef, arg0 as *const ::clipboard::Mode) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QClipboard_Mode::qt_gui_c_SlotWrapper_QClipboard_Mode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotClipboardModeRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QClipboard_Mode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QClipboard_Mode::set(void (*FN_PTR)(void*, const QClipboard::Mode*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::clipboard::Mode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QClipboard_Mode_set(self as *mut ::slots::raw::RawSlotClipboardModeRef,
                                                               func,
                                                               data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotClipboardModeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QClipboard_Mode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QDoubleValidator::Notation)` to a Rust extern function.
  ///
  /// Use `SlotDoubleValidatorNotationRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QDoubleValidator_Notation```</span>
  #[repr(C)]
  pub struct RawSlotDoubleValidatorNotationRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotDoubleValidatorNotationRef {
    type Arguments = (&'static ::double_validator::Notation,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QDoubleValidator::Notation)\0"
    }
  }
  impl RawSlotDoubleValidatorNotationRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QDoubleValidator_Notation::custom_slot(QDoubleValidator::Notation arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::double_validator::Notation) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QDoubleValidator_Notation_custom_slot(self as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef, arg0 as *const ::double_validator::Notation) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QDoubleValidator_Notation::qt_gui_c_SlotWrapper_QDoubleValidator_Notation()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotDoubleValidatorNotationRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QDoubleValidator_Notation_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QDoubleValidator_Notation::set(void (*FN_PTR)(void*, const QDoubleValidator::Notation*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::double_validator::Notation),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QDoubleValidator_Notation_set(self as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotDoubleValidatorNotationRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QDoubleValidator_Notation_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QImageReader::ImageReaderError)` to a Rust extern function.
  ///
  /// Use `SlotImageReaderImageReaderErrorRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QImageReader_ImageReaderError```</span>
  #[repr(C)]
  pub struct RawSlotImageReaderImageReaderErrorRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotImageReaderImageReaderErrorRef {
    type Arguments = (&'static ::image_reader::ImageReaderError,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QImageReader::ImageReaderError)\0"
    }
  }
  impl RawSlotImageReaderImageReaderErrorRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QImageReader_ImageReaderError::custom_slot(QImageReader::ImageReaderError arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::image_reader::ImageReaderError) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError_custom_slot(self as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef, arg0 as *const ::image_reader::ImageReaderError) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QImageReader_ImageReaderError::qt_gui_c_SlotWrapper_QImageReader_ImageReaderError()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotImageReaderImageReaderErrorRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QImageReader_ImageReaderError::set(void (*FN_PTR)(void*, const QImageReader::ImageReaderError*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::image_reader::ImageReaderError),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError_set(self as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotImageReaderImageReaderErrorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QMovie::MovieState)` to a Rust extern function.
  ///
  /// Use `SlotMovieMovieStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QMovie_MovieState```</span>
  #[repr(C)]
  pub struct RawSlotMovieMovieStateRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotMovieMovieStateRef {
    type Arguments = (&'static ::movie::MovieState,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QMovie::MovieState)\0"
    }
  }
  impl RawSlotMovieMovieStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QMovie_MovieState::custom_slot(QMovie::MovieState arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::movie::MovieState) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QMovie_MovieState_custom_slot(self as *mut ::slots::raw::RawSlotMovieMovieStateRef, arg0 as *const ::movie::MovieState) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QMovie_MovieState::qt_gui_c_SlotWrapper_QMovie_MovieState()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotMovieMovieStateRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QMovie_MovieState_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QMovie_MovieState::set(void (*FN_PTR)(void*, const QMovie::MovieState*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::movie::MovieState),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QMovie_MovieState_set(self as *mut ::slots::raw::RawSlotMovieMovieStateRef,
                                                                 func,
                                                                 data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotMovieMovieStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QMovie_MovieState_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QOpenGLDebugMessage&)` to a Rust extern function.
  ///
  /// Use `SlotOpenglDebugMessageRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref```</span>
  #[repr(C)]
  pub struct RawSlotOpenglDebugMessageRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotOpenglDebugMessageRef {
    type Arguments = (&'static ::opengl_debug_message::OpenGLDebugMessage,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QOpenGLDebugMessage&)\0"
    }
  }
  impl RawSlotOpenglDebugMessageRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref::custom_slot(const QOpenGLDebugMessage& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::opengl_debug_message::OpenGLDebugMessage) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref_custom_slot(self as *mut ::slots::raw::RawSlotOpenglDebugMessageRef, arg0 as *const ::opengl_debug_message::OpenGLDebugMessage) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref::qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotOpenglDebugMessageRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref::set(void (*FN_PTR)(void*, const QOpenGLDebugMessage*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::opengl_debug_message::OpenGLDebugMessage),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref_set(self as *mut ::slots::raw::RawSlotOpenglDebugMessageRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotOpenglDebugMessageRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QPalette&)` to a Rust extern function.
  ///
  /// Use `SlotPaletteRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QPalette_ref```</span>
  #[repr(C)]
  pub struct RawSlotPaletteRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPaletteRef {
    type Arguments = (&'static ::palette::Palette,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QPalette&)\0"
    }
  }
  impl RawSlotPaletteRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QPalette_ref::custom_slot(const QPalette& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::palette::Palette) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QPalette_ref_custom_slot(self as *mut ::slots::raw::RawSlotPaletteRef, arg0 as *const ::palette::Palette) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QPalette_ref::qt_gui_c_SlotWrapper_const_QPalette_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPaletteRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QPalette_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QPalette_ref::set(void (*FN_PTR)(void*, const QPalette*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::palette::Palette),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QPalette_ref_set(self as *mut ::slots::raw::RawSlotPaletteRef,
                                                                  func,
                                                                  data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPaletteRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QPalette_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::ApplicationState)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtApplicationStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_ApplicationState```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtApplicationStateRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtApplicationStateRef {
    type Arguments = (&'static ::qt_core::qt::ApplicationState,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::ApplicationState)\0"
    }
  }
  impl RawSlotQtCoreQtApplicationStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_ApplicationState::custom_slot(Qt::ApplicationState arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::ApplicationState) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ApplicationState_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef, arg0 as *const ::qt_core::qt::ApplicationState) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_ApplicationState::qt_gui_c_SlotWrapper_Qt_ApplicationState()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtApplicationStateRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ApplicationState_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_ApplicationState::set(void (*FN_PTR)(void*, const Qt::ApplicationState*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::ApplicationState),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ApplicationState_set(self as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtApplicationStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ApplicationState_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::DropAction)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtDropActionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_DropAction```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtDropActionRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtDropActionRef {
    type Arguments = (&'static ::qt_core::qt::DropAction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::DropAction)\0"
    }
  }
  impl RawSlotQtCoreQtDropActionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_DropAction::custom_slot(Qt::DropAction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::DropAction) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_DropAction_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef, arg0 as *const ::qt_core::qt::DropAction) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_DropAction::qt_gui_c_SlotWrapper_Qt_DropAction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtDropActionRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_DropAction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_DropAction::set(void (*FN_PTR)(void*, const Qt::DropAction*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::DropAction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_DropAction_set(self as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef,
                                                             func,
                                                             data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtDropActionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_DropAction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::LayoutDirection)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtLayoutDirectionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_LayoutDirection```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtLayoutDirectionRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef {
    type Arguments = (&'static ::qt_core::qt::LayoutDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::LayoutDirection)\0"
    }
  }
  impl RawSlotQtCoreQtLayoutDirectionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_LayoutDirection::custom_slot(Qt::LayoutDirection arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::LayoutDirection) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_LayoutDirection_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef, arg0 as *const ::qt_core::qt::LayoutDirection) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_LayoutDirection::qt_gui_c_SlotWrapper_Qt_LayoutDirection()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtLayoutDirectionRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_LayoutDirection_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_LayoutDirection::set(void (*FN_PTR)(void*, const Qt::LayoutDirection*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::LayoutDirection),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_LayoutDirection_set(self as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_LayoutDirection_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::ScreenOrientation)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtScreenOrientationRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_ScreenOrientation```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtScreenOrientationRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtScreenOrientationRef {
    type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::ScreenOrientation)\0"
    }
  }
  impl RawSlotQtCoreQtScreenOrientationRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_ScreenOrientation::custom_slot(Qt::ScreenOrientation arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::ScreenOrientation) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ScreenOrientation_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef, arg0 as *const ::qt_core::qt::ScreenOrientation) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_ScreenOrientation::qt_gui_c_SlotWrapper_Qt_ScreenOrientation()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtScreenOrientationRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ScreenOrientation_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_ScreenOrientation::set(void (*FN_PTR)(void*, const Qt::ScreenOrientation*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::ScreenOrientation),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ScreenOrientation_set(self as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtScreenOrientationRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_ScreenOrientation_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::TabFocusBehavior)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtTabFocusBehaviorRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_TabFocusBehavior```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtTabFocusBehaviorRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef {
    type Arguments = (&'static ::qt_core::qt::TabFocusBehavior,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::TabFocusBehavior)\0"
    }
  }
  impl RawSlotQtCoreQtTabFocusBehaviorRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_TabFocusBehavior::custom_slot(Qt::TabFocusBehavior arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::TabFocusBehavior) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef, arg0 as *const ::qt_core::qt::TabFocusBehavior) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_TabFocusBehavior::qt_gui_c_SlotWrapper_Qt_TabFocusBehavior()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_TabFocusBehavior::set(void (*FN_PTR)(void*, const Qt::TabFocusBehavior*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::TabFocusBehavior),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior_set(self as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::WindowModality)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtWindowModalityRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_WindowModality```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtWindowModalityRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtWindowModalityRef {
    type Arguments = (&'static ::qt_core::qt::WindowModality,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::WindowModality)\0"
    }
  }
  impl RawSlotQtCoreQtWindowModalityRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_WindowModality::custom_slot(Qt::WindowModality arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::WindowModality) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowModality_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef, arg0 as *const ::qt_core::qt::WindowModality) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_WindowModality::qt_gui_c_SlotWrapper_Qt_WindowModality()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtWindowModalityRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowModality_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_WindowModality::set(void (*FN_PTR)(void*, const Qt::WindowModality*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::WindowModality),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowModality_set(self as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtWindowModalityRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowModality_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::WindowState)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtWindowStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_Qt_WindowState```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtWindowStateRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtWindowStateRef {
    type Arguments = (&'static ::qt_core::qt::WindowState,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::WindowState)\0"
    }
  }
  impl RawSlotQtCoreQtWindowStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_Qt_WindowState::custom_slot(Qt::WindowState arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::WindowState) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowState_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef, arg0 as *const ::qt_core::qt::WindowState) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_Qt_WindowState::qt_gui_c_SlotWrapper_Qt_WindowState()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtWindowStateRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowState_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_Qt_WindowState::set(void (*FN_PTR)(void*, const Qt::WindowState*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::WindowState),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowState_set(self as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtWindowStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_Qt_WindowState_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QRectF&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QRectF_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectFRef {
    type Arguments = (&'static ::qt_core::rect_f::RectF,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QRectF&)\0"
    }
  }
  impl RawSlotQtCoreRectFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QRectF_ref::custom_slot(const QRectF& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::rect_f::RectF) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRectF_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectFRef, arg0 as *const ::qt_core::rect_f::RectF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QRectF_ref::qt_gui_c_SlotWrapper_const_QRectF_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectFRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRectF_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QRectF_ref::set(void (*FN_PTR)(void*, const QRectF*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::rect_f::RectF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRectF_ref_set(self as *mut ::slots::raw::RawSlotQtCoreRectFRef,
                                                                func,
                                                                data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRectF_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QRect&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QRect_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectRef {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QRect&)\0"
    }
  }
  impl RawSlotQtCoreRectRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QRect_ref::custom_slot(const QRect& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::rect::Rect) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRect_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectRef, arg0 as *const ::qt_core::rect::Rect) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QRect_ref::qt_gui_c_SlotWrapper_const_QRect_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRect_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QRect_ref::set(void (*FN_PTR)(void*, const QRect*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::rect::Rect),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRect_ref_set(self as *mut ::slots::raw::RawSlotQtCoreRectRef,
                                                               func,
                                                               data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRect_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QRegExp&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRegExpRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QRegExp_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRegExpRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRegExpRef {
    type Arguments = (&'static ::qt_core::reg_exp::RegExp,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QRegExp&)\0"
    }
  }
  impl RawSlotQtCoreRegExpRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QRegExp_ref::custom_slot(const QRegExp& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::reg_exp::RegExp) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegExp_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRegExpRef, arg0 as *const ::qt_core::reg_exp::RegExp) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QRegExp_ref::qt_gui_c_SlotWrapper_const_QRegExp_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRegExpRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegExp_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QRegExp_ref::set(void (*FN_PTR)(void*, const QRegExp*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::reg_exp::RegExp),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegExp_ref_set(self as *mut ::slots::raw::RawSlotQtCoreRegExpRef,
                                                                 func,
                                                                 data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRegExpRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegExp_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QRegularExpression&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRegularExpressionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QRegularExpression_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRegularExpressionRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRegularExpressionRef {
    type Arguments = (&'static ::qt_core::regular_expression::RegularExpression,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QRegularExpression&)\0"
    }
  }
  impl RawSlotQtCoreRegularExpressionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QRegularExpression_ref::custom_slot(const QRegularExpression& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::regular_expression::RegularExpression) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegularExpression_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef, arg0 as *const ::qt_core::regular_expression::RegularExpression) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QRegularExpression_ref::qt_gui_c_SlotWrapper_const_QRegularExpression_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRegularExpressionRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegularExpression_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QRegularExpression_ref::set(void (*FN_PTR)(void*, const QRegularExpression*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::regular_expression::RegularExpression),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegularExpression_ref_set(self as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRegularExpressionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QRegularExpression_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QSizeF&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreSizeFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QSizeF_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreSizeFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreSizeFRef {
    type Arguments = (&'static ::qt_core::size_f::SizeF,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QSizeF&)\0"
    }
  }
  impl RawSlotQtCoreSizeFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QSizeF_ref::custom_slot(const QSizeF& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::size_f::SizeF) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSizeF_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreSizeFRef, arg0 as *const ::qt_core::size_f::SizeF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QSizeF_ref::qt_gui_c_SlotWrapper_const_QSizeF_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreSizeFRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSizeF_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QSizeF_ref::set(void (*FN_PTR)(void*, const QSizeF*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::size_f::SizeF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSizeF_ref_set(self as *mut ::slots::raw::RawSlotQtCoreSizeFRef,
                                                                func,
                                                                data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreSizeFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSizeF_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QSize&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreSizeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QSize_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreSizeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreSizeRef {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QSize&)\0"
    }
  }
  impl RawSlotQtCoreSizeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QSize_ref::custom_slot(const QSize& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::size::Size) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSize_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreSizeRef, arg0 as *const ::qt_core::size::Size) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QSize_ref::qt_gui_c_SlotWrapper_const_QSize_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreSizeRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSize_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QSize_ref::set(void (*FN_PTR)(void*, const QSize*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::size::Size),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSize_ref_set(self as *mut ::slots::raw::RawSlotQtCoreSizeRef,
                                                               func,
                                                               data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreSizeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QSize_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QUrl&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreUrlRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QUrl_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreUrlRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreUrlRef {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QUrl&)\0"
    }
  }
  impl RawSlotQtCoreUrlRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QUrl_ref::custom_slot(const QUrl& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::url::Url) {
      unsafe {
        ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QUrl_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreUrlRef, arg0 as *const ::qt_core::url::Url)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QUrl_ref::qt_gui_c_SlotWrapper_const_QUrl_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreUrlRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QUrl_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QUrl_ref::set(void (*FN_PTR)(void*, const QUrl*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::url::Url),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QUrl_ref_set(self as *mut ::slots::raw::RawSlotQtCoreUrlRef,
                                                              func,
                                                              data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreUrlRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QUrl_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QScreen*)` to a Rust extern function.
  ///
  /// Use `SlotScreenMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QScreen_ptr```</span>
  #[repr(C)]
  pub struct RawSlotScreenMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotScreenMutPtr {
    type Arguments = (*mut ::screen::Screen,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QScreen*)\0"
    }
  }
  impl RawSlotScreenMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QScreen_ptr::custom_slot(QScreen* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::screen::Screen) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QScreen_ptr_custom_slot(self as *mut ::slots::raw::RawSlotScreenMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QScreen_ptr::qt_gui_c_SlotWrapper_QScreen_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotScreenMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QScreen_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QScreen_ptr::set(void (*FN_PTR)(void*, QScreen*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::screen::Screen),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QScreen_ptr_set(self as *mut ::slots::raw::RawSlotScreenMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotScreenMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QScreen_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QSessionManager&)` to a Rust extern function.
  ///
  /// Use `SlotSessionManagerMutRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QSessionManager_ref```</span>
  #[repr(C)]
  pub struct RawSlotSessionManagerMutRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotSessionManagerMutRef {
    type Arguments = (&'static mut ::session_manager::SessionManager,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QSessionManager&)\0"
    }
  }
  impl RawSlotSessionManagerMutRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QSessionManager_ref::custom_slot(QSessionManager& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &mut ::session_manager::SessionManager) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QSessionManager_ref_custom_slot(self as *mut ::slots::raw::RawSlotSessionManagerMutRef, arg0 as *mut ::session_manager::SessionManager) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QSessionManager_ref::qt_gui_c_SlotWrapper_QSessionManager_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotSessionManagerMutRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QSessionManager_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QSessionManager_ref::set(void (*FN_PTR)(void*, QSessionManager*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::session_manager::SessionManager),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QSessionManager_ref_set(self as *mut ::slots::raw::RawSlotSessionManagerMutRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotSessionManagerMutRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QSessionManager_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QStandardItem*)` to a Rust extern function.
  ///
  /// Use `SlotStandardItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QStandardItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotStandardItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotStandardItemMutPtr {
    type Arguments = (*mut ::standard_item::StandardItem,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QStandardItem*)\0"
    }
  }
  impl RawSlotStandardItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QStandardItem_ptr::custom_slot(QStandardItem* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::standard_item::StandardItem) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QStandardItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotStandardItemMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QStandardItem_ptr::qt_gui_c_SlotWrapper_QStandardItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStandardItemMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QStandardItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QStandardItem_ptr::set(void (*FN_PTR)(void*, QStandardItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::standard_item::StandardItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QStandardItem_ptr_set(self as *mut ::slots::raw::RawSlotStandardItemMutPtr,
                                                                 func,
                                                                 data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStandardItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QStandardItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QTextBlock&)` to a Rust extern function.
  ///
  /// Use `SlotTextBlockRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QTextBlock_ref```</span>
  #[repr(C)]
  pub struct RawSlotTextBlockRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTextBlockRef {
    type Arguments = (&'static ::text_block::TextBlock,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QTextBlock&)\0"
    }
  }
  impl RawSlotTextBlockRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QTextBlock_ref::custom_slot(const QTextBlock& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::text_block::TextBlock) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextBlock_ref_custom_slot(self as *mut ::slots::raw::RawSlotTextBlockRef, arg0 as *const ::text_block::TextBlock) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QTextBlock_ref::qt_gui_c_SlotWrapper_const_QTextBlock_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTextBlockRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextBlock_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QTextBlock_ref::set(void (*FN_PTR)(void*, const QTextBlock*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::text_block::TextBlock),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextBlock_ref_set(self as *mut ::slots::raw::RawSlotTextBlockRef,
                                                                    func,
                                                                    data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTextBlockRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextBlock_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QTextCursor&)` to a Rust extern function.
  ///
  /// Use `SlotTextCursorRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_const_QTextCursor_ref```</span>
  #[repr(C)]
  pub struct RawSlotTextCursorRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTextCursorRef {
    type Arguments = (&'static ::text_cursor::TextCursor,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QTextCursor&)\0"
    }
  }
  impl RawSlotTextCursorRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_const_QTextCursor_ref::custom_slot(const QTextCursor& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::text_cursor::TextCursor) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextCursor_ref_custom_slot(self as *mut ::slots::raw::RawSlotTextCursorRef, arg0 as *const ::text_cursor::TextCursor) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_const_QTextCursor_ref::qt_gui_c_SlotWrapper_const_QTextCursor_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTextCursorRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextCursor_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_const_QTextCursor_ref::set(void (*FN_PTR)(void*, const QTextCursor*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::text_cursor::TextCursor),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextCursor_ref_set(self as *mut ::slots::raw::RawSlotTextCursorRef,
                                                                     func,
                                                                     data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTextCursorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_const_QTextCursor_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QWindow*)` to a Rust extern function.
  ///
  /// Use `SlotWindowMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QWindow_ptr```</span>
  #[repr(C)]
  pub struct RawSlotWindowMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotWindowMutPtr {
    type Arguments = (*mut ::window::Window,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QWindow*)\0"
    }
  }
  impl RawSlotWindowMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QWindow_ptr::custom_slot(QWindow* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::window::Window) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_ptr_custom_slot(self as *mut ::slots::raw::RawSlotWindowMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QWindow_ptr::qt_gui_c_SlotWrapper_QWindow_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotWindowMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QWindow_ptr::set(void (*FN_PTR)(void*, QWindow*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::window::Window),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_ptr_set(self as *mut ::slots::raw::RawSlotWindowMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotWindowMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QWindow::Visibility)` to a Rust extern function.
  ///
  /// Use `SlotWindowVisibilityRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_gui_c_SlotWrapper_QWindow_Visibility```</span>
  #[repr(C)]
  pub struct RawSlotWindowVisibilityRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotWindowVisibilityRef {
    type Arguments = (&'static ::window::Visibility,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QWindow::Visibility)\0"
    }
  }
  impl RawSlotWindowVisibilityRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_gui_c_SlotWrapper_QWindow_Visibility::custom_slot(QWindow::Visibility arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::window::Visibility) {
      unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_Visibility_custom_slot(self as *mut ::slots::raw::RawSlotWindowVisibilityRef, arg0 as *const ::window::Visibility) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_gui_c_SlotWrapper_QWindow_Visibility::qt_gui_c_SlotWrapper_QWindow_Visibility()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotWindowVisibilityRef> {
      let ffi_result = unsafe { ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_Visibility_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_gui_c_SlotWrapper_QWindow_Visibility::set(void (*FN_PTR)(void*, const QWindow::Visibility*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::window::Visibility),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_Visibility_set(self as *mut ::slots::raw::RawSlotWindowVisibilityRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotWindowVisibilityRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_gui_c_qt_gui_c_SlotWrapper_QWindow_Visibility_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotClipboardModeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QClipboard_Mode(self as *mut ::slots::raw::RawSlotClipboardModeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QClipboard_Mode(self as *const ::slots::raw::RawSlotClipboardModeRef as *mut ::slots::raw::RawSlotClipboardModeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotDoubleValidatorNotationRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QDoubleValidator_Notation(self as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QDoubleValidator_Notation(self as *const ::slots::raw::RawSlotDoubleValidatorNotationRef as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotImageReaderImageReaderErrorRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError(self as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError(self as *const ::slots::raw::RawSlotImageReaderImageReaderErrorRef as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotMovieMovieStateRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QMovie_MovieState(self as *mut ::slots::raw::RawSlotMovieMovieStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QMovie_MovieState(self as *const ::slots::raw::RawSlotMovieMovieStateRef as *mut ::slots::raw::RawSlotMovieMovieStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotScreenMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QScreen_ptr(self as *mut ::slots::raw::RawSlotScreenMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QScreen_ptr(self as *const ::slots::raw::RawSlotScreenMutPtr as *mut ::slots::raw::RawSlotScreenMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotSessionManagerMutRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QSessionManager_ref(self as *mut ::slots::raw::RawSlotSessionManagerMutRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QSessionManager_ref(self as *const ::slots::raw::RawSlotSessionManagerMutRef as *mut ::slots::raw::RawSlotSessionManagerMutRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotStandardItemMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QStandardItem_ptr(self as *mut ::slots::raw::RawSlotStandardItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QStandardItem_ptr(self as *const ::slots::raw::RawSlotStandardItemMutPtr as *mut ::slots::raw::RawSlotStandardItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotWindowVisibilityRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_Visibility(self as *mut ::slots::raw::RawSlotWindowVisibilityRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_Visibility(self as *const ::slots::raw::RawSlotWindowVisibilityRef as *mut ::slots::raw::RawSlotWindowVisibilityRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotWindowMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_ptr(self as *mut ::slots::raw::RawSlotWindowMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_ptr(self as *const ::slots::raw::RawSlotWindowMutPtr as *mut ::slots::raw::RawSlotWindowMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtApplicationStateRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ApplicationState(self as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ApplicationState(self as *const ::slots::raw::RawSlotQtCoreQtApplicationStateRef as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtDropActionRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_DropAction(self as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_DropAction(self as *const ::slots::raw::RawSlotQtCoreQtDropActionRef as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_LayoutDirection(self as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_LayoutDirection(self as *const ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtScreenOrientationRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ScreenOrientation(self as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ScreenOrientation(self as *const ::slots::raw::RawSlotQtCoreQtScreenOrientationRef as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior(self as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior(self as *const ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtWindowModalityRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowModality(self as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowModality(self as *const ::slots::raw::RawSlotQtCoreQtWindowModalityRef as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtWindowStateRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowState(self as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowState(self as *const ::slots::raw::RawSlotQtCoreQtWindowStateRef as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotOpenglDebugMessageRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref(self as *mut ::slots::raw::RawSlotOpenglDebugMessageRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref(self as *const ::slots::raw::RawSlotOpenglDebugMessageRef as *mut ::slots::raw::RawSlotOpenglDebugMessageRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPaletteRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QPalette_ref(self as *mut ::slots::raw::RawSlotPaletteRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QPalette_ref(self as *const ::slots::raw::RawSlotPaletteRef as *mut ::slots::raw::RawSlotPaletteRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectFRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRectF_ref(self as *mut ::slots::raw::RawSlotQtCoreRectFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRectF_ref(self as *const ::slots::raw::RawSlotQtCoreRectFRef as *mut ::slots::raw::RawSlotQtCoreRectFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRect_ref(self as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRect_ref(self as *const ::slots::raw::RawSlotQtCoreRectRef as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRegExpRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegExp_ref(self as *mut ::slots::raw::RawSlotQtCoreRegExpRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegExp_ref(self as *const ::slots::raw::RawSlotQtCoreRegExpRef as *mut ::slots::raw::RawSlotQtCoreRegExpRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRegularExpressionRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegularExpression_ref(self as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegularExpression_ref(self as *const ::slots::raw::RawSlotQtCoreRegularExpressionRef as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreSizeFRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSizeF_ref(self as *mut ::slots::raw::RawSlotQtCoreSizeFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSizeF_ref(self as *const ::slots::raw::RawSlotQtCoreSizeFRef as *mut ::slots::raw::RawSlotQtCoreSizeFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreSizeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSize_ref(self as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSize_ref(self as *const ::slots::raw::RawSlotQtCoreSizeRef as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTextBlockRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextBlock_ref(self as *mut ::slots::raw::RawSlotTextBlockRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextBlock_ref(self as *const ::slots::raw::RawSlotTextBlockRef as *mut ::slots::raw::RawSlotTextBlockRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTextCursorRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextCursor_ref(self as *mut ::slots::raw::RawSlotTextCursorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextCursor_ref(self as *const ::slots::raw::RawSlotTextCursorRef as *mut ::slots::raw::RawSlotTextCursorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreUrlRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QUrl_ref(self as *mut ::slots::raw::RawSlotQtCoreUrlRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QUrl_ref(self as *const ::slots::raw::RawSlotQtCoreUrlRef as *mut ::slots::raw::RawSlotQtCoreUrlRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCIntCInt {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int(self as *mut ::slots::raw::RawSlotCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int(self as *const ::slots::raw::RawSlotCIntCInt as *mut ::slots::raw::RawSlotCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCIntCIntCInt {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int_int(self as *mut ::slots::raw::RawSlotCIntCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int_int(self as *const ::slots::raw::RawSlotCIntCIntCInt as *mut ::slots::raw::RawSlotCIntCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotClipboardModeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QClipboard_Mode(self as *const ::slots::raw::RawSlotClipboardModeRef as *mut ::slots::raw::RawSlotClipboardModeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotDoubleValidatorNotationRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QDoubleValidator_Notation(self as *const ::slots::raw::RawSlotDoubleValidatorNotationRef as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotImageReaderImageReaderErrorRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError(self as *const ::slots::raw::RawSlotImageReaderImageReaderErrorRef as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotMovieMovieStateRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QMovie_MovieState(self as *const ::slots::raw::RawSlotMovieMovieStateRef as *mut ::slots::raw::RawSlotMovieMovieStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotScreenMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QScreen_ptr(self as *const ::slots::raw::RawSlotScreenMutPtr as *mut ::slots::raw::RawSlotScreenMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotSessionManagerMutRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QSessionManager_ref(self as *const ::slots::raw::RawSlotSessionManagerMutRef as *mut ::slots::raw::RawSlotSessionManagerMutRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStandardItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QStandardItem_ptr(self as *const ::slots::raw::RawSlotStandardItemMutPtr as *mut ::slots::raw::RawSlotStandardItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotWindowVisibilityRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_Visibility(self as *const ::slots::raw::RawSlotWindowVisibilityRef as *mut ::slots::raw::RawSlotWindowVisibilityRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotWindowMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_ptr(self as *const ::slots::raw::RawSlotWindowMutPtr as *mut ::slots::raw::RawSlotWindowMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtApplicationStateRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ApplicationState(self as *const ::slots::raw::RawSlotQtCoreQtApplicationStateRef as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtDropActionRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_DropAction(self as *const ::slots::raw::RawSlotQtCoreQtDropActionRef as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_LayoutDirection(self as *const ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtScreenOrientationRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ScreenOrientation(self as *const ::slots::raw::RawSlotQtCoreQtScreenOrientationRef as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior(self as *const ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtWindowModalityRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowModality(self as *const ::slots::raw::RawSlotQtCoreQtWindowModalityRef as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtWindowStateRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowState(self as *const ::slots::raw::RawSlotQtCoreQtWindowStateRef as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotOpenglDebugMessageRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref(self as *const ::slots::raw::RawSlotOpenglDebugMessageRef as *mut ::slots::raw::RawSlotOpenglDebugMessageRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPaletteRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QPalette_ref(self as *const ::slots::raw::RawSlotPaletteRef as *mut ::slots::raw::RawSlotPaletteRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRectF_ref(self as *const ::slots::raw::RawSlotQtCoreRectFRef as *mut ::slots::raw::RawSlotQtCoreRectFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRect_ref(self as *const ::slots::raw::RawSlotQtCoreRectRef as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRegExpRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegExp_ref(self as *const ::slots::raw::RawSlotQtCoreRegExpRef as *mut ::slots::raw::RawSlotQtCoreRegExpRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRegularExpressionRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegularExpression_ref(self as *const ::slots::raw::RawSlotQtCoreRegularExpressionRef as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreSizeFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSizeF_ref(self as *const ::slots::raw::RawSlotQtCoreSizeFRef as *mut ::slots::raw::RawSlotQtCoreSizeFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreSizeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSize_ref(self as *const ::slots::raw::RawSlotQtCoreSizeRef as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTextBlockRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextBlock_ref(self as *const ::slots::raw::RawSlotTextBlockRef as *mut ::slots::raw::RawSlotTextBlockRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTextCursorRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextCursor_ref(self as *const ::slots::raw::RawSlotTextCursorRef as *mut ::slots::raw::RawSlotTextCursorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreUrlRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QUrl_ref(self as *const ::slots::raw::RawSlotQtCoreUrlRef as *mut ::slots::raw::RawSlotQtCoreUrlRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntCInt {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int(self as *const ::slots::raw::RawSlotCIntCInt as *mut ::slots::raw::RawSlotCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntCIntCInt {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int_int(self as *const ::slots::raw::RawSlotCIntCIntCInt as *mut ::slots::raw::RawSlotCIntCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotClipboardModeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QClipboard_Mode(self as *mut ::slots::raw::RawSlotClipboardModeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotDoubleValidatorNotationRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QDoubleValidator_Notation(self as *mut ::slots::raw::RawSlotDoubleValidatorNotationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotImageReaderImageReaderErrorRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QImageReader_ImageReaderError(self as *mut ::slots::raw::RawSlotImageReaderImageReaderErrorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotMovieMovieStateRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QMovie_MovieState(self as *mut ::slots::raw::RawSlotMovieMovieStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotScreenMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QScreen_ptr(self as *mut ::slots::raw::RawSlotScreenMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotSessionManagerMutRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QSessionManager_ref(self as *mut ::slots::raw::RawSlotSessionManagerMutRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStandardItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QStandardItem_ptr(self as *mut ::slots::raw::RawSlotStandardItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotWindowVisibilityRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_Visibility(self as *mut ::slots::raw::RawSlotWindowVisibilityRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotWindowMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_QWindow_ptr(self as *mut ::slots::raw::RawSlotWindowMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtApplicationStateRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ApplicationState(self as *mut ::slots::raw::RawSlotQtCoreQtApplicationStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtDropActionRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_DropAction(self as *mut ::slots::raw::RawSlotQtCoreQtDropActionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_LayoutDirection(self as *mut ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtScreenOrientationRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_ScreenOrientation(self as *mut ::slots::raw::RawSlotQtCoreQtScreenOrientationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_TabFocusBehavior(self as *mut ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtWindowModalityRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowModality(self as *mut ::slots::raw::RawSlotQtCoreQtWindowModalityRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtWindowStateRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_Qt_WindowState(self as *mut ::slots::raw::RawSlotQtCoreQtWindowStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotOpenglDebugMessageRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QOpenGLDebugMessage_ref(self as *mut ::slots::raw::RawSlotOpenglDebugMessageRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPaletteRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QPalette_ref(self as *mut ::slots::raw::RawSlotPaletteRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRectF_ref(self as *mut ::slots::raw::RawSlotQtCoreRectFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRect_ref(self as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRegExpRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegExp_ref(self as *mut ::slots::raw::RawSlotQtCoreRegExpRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRegularExpressionRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QRegularExpression_ref(self as *mut ::slots::raw::RawSlotQtCoreRegularExpressionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreSizeFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSizeF_ref(self as *mut ::slots::raw::RawSlotQtCoreSizeFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreSizeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QSize_ref(self as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTextBlockRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextBlock_ref(self as *mut ::slots::raw::RawSlotTextBlockRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTextCursorRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QTextCursor_ref(self as *mut ::slots::raw::RawSlotTextCursorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreUrlRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_const_QUrl_ref(self as *mut ::slots::raw::RawSlotQtCoreUrlRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntCInt {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int(self as *mut ::slots::raw::RawSlotCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntCIntCInt {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_gui_c_slots_G_static_cast_QObject_ptr_qt_gui_c_SlotWrapper_int_int_int(self as *mut ::slots::raw::RawSlotCIntCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(int, int)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotCIntCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCInt>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotCIntCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, ::libc::c_int) + 'a>(f: F) -> SlotCIntCInt<'a> {
    let mut obj = SlotCIntCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_c_int_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotCIntCInt<'a> {
  fn default() -> Self {
    SlotCIntCInt {
      wrapper: ::slots::raw::RawSlotCIntCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCIntCInt<'a> {
  type Arguments = (::libc::c_int, ::libc::c_int);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntCInt as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_c_int_callback(data: *mut ::libc::c_void, arg0: ::libc::c_int, arg1: ::libc::c_int) {
  let func: &mut Box<FnMut(::libc::c_int, ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(int, int, int)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotCIntCIntCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCIntCInt>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotCIntCIntCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>(f: F) -> SlotCIntCIntCInt<'a> {
    let mut obj = SlotCIntCIntCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_c_int_c_int_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotCIntCIntCInt<'a> {
  fn default() -> Self {
    SlotCIntCIntCInt {
      wrapper: ::slots::raw::RawSlotCIntCIntCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCIntCIntCInt<'a> {
  type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntCIntCInt as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_c_int_c_int_callback(data: *mut ::libc::c_void,
                                              arg0: ::libc::c_int,
                                              arg1: ::libc::c_int,
                                              arg2: ::libc::c_int) {
  let func: &mut Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1, arg2);
}
/// Allows to bind Qt signals with arguments `(QClipboard::Mode)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotClipboardModeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotClipboardModeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::clipboard::Mode) + 'a>>>,
}

impl<'a> SlotClipboardModeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::clipboard::Mode) + 'a>(f: F) -> SlotClipboardModeRef<'a> {
    let mut obj = SlotClipboardModeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::clipboard::Mode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::clipboard::Mode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_clipboard_mode_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotClipboardModeRef<'a> {
  fn default() -> Self {
    SlotClipboardModeRef {
      wrapper: ::slots::raw::RawSlotClipboardModeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotClipboardModeRef<'a> {
  type Arguments = (&'static ::clipboard::Mode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotClipboardModeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_clipboard_mode_ref_callback(data: *mut ::libc::c_void, arg0: *const ::clipboard::Mode) {
  let func: &mut Box<FnMut(&'static ::clipboard::Mode)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QDoubleValidator::Notation)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotDoubleValidatorNotationRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotDoubleValidatorNotationRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::double_validator::Notation) + 'a>>>,
}

impl<'a> SlotDoubleValidatorNotationRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::double_validator::Notation) + 'a>(f: F) -> SlotDoubleValidatorNotationRef<'a> {
    let mut obj = SlotDoubleValidatorNotationRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::double_validator::Notation) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::double_validator::Notation) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_double_validator_notation_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotDoubleValidatorNotationRef<'a> {
  fn default() -> Self {
    SlotDoubleValidatorNotationRef {
      wrapper: ::slots::raw::RawSlotDoubleValidatorNotationRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotDoubleValidatorNotationRef<'a> {
  type Arguments = (&'static ::double_validator::Notation,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotDoubleValidatorNotationRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_double_validator_notation_ref_callback(data: *mut ::libc::c_void,
                                                          arg0: *const ::double_validator::Notation) {
  let func: &mut Box<FnMut(&'static ::double_validator::Notation)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QImageReader::ImageReaderError)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotImageReaderImageReaderErrorRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotImageReaderImageReaderErrorRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::image_reader::ImageReaderError) + 'a>>>,
}

impl<'a> SlotImageReaderImageReaderErrorRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::image_reader::ImageReaderError) + 'a>(f: F) -> SlotImageReaderImageReaderErrorRef<'a> {
    let mut obj = SlotImageReaderImageReaderErrorRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::image_reader::ImageReaderError) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::image_reader::ImageReaderError) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_image_reader_image_reader_error_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotImageReaderImageReaderErrorRef<'a> {
  fn default() -> Self {
    SlotImageReaderImageReaderErrorRef {
      wrapper: ::slots::raw::RawSlotImageReaderImageReaderErrorRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotImageReaderImageReaderErrorRef<'a> {
  type Arguments = (&'static ::image_reader::ImageReaderError,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotImageReaderImageReaderErrorRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_image_reader_image_reader_error_ref_callback(data: *mut ::libc::c_void,
                                                                arg0: *const ::image_reader::ImageReaderError) {
  let func: &mut Box<FnMut(&'static ::image_reader::ImageReaderError)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QMovie::MovieState)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotMovieMovieStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotMovieMovieStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::movie::MovieState) + 'a>>>,
}

impl<'a> SlotMovieMovieStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::movie::MovieState) + 'a>(f: F) -> SlotMovieMovieStateRef<'a> {
    let mut obj = SlotMovieMovieStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::movie::MovieState) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::movie::MovieState) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_movie_movie_state_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotMovieMovieStateRef<'a> {
  fn default() -> Self {
    SlotMovieMovieStateRef {
      wrapper: ::slots::raw::RawSlotMovieMovieStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotMovieMovieStateRef<'a> {
  type Arguments = (&'static ::movie::MovieState,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotMovieMovieStateRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_movie_movie_state_ref_callback(data: *mut ::libc::c_void, arg0: *const ::movie::MovieState) {
  let func: &mut Box<FnMut(&'static ::movie::MovieState)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QOpenGLDebugMessage&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotOpenglDebugMessageRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotOpenglDebugMessageRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::opengl_debug_message::OpenGLDebugMessage) + 'a>>>,
}

impl<'a> SlotOpenglDebugMessageRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::opengl_debug_message::OpenGLDebugMessage) + 'a>(f: F)
                                                                                 -> SlotOpenglDebugMessageRef<'a> {
    let mut obj = SlotOpenglDebugMessageRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::opengl_debug_message::OpenGLDebugMessage) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::opengl_debug_message::OpenGLDebugMessage) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_opengl_debug_message_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotOpenglDebugMessageRef<'a> {
  fn default() -> Self {
    SlotOpenglDebugMessageRef {
      wrapper: ::slots::raw::RawSlotOpenglDebugMessageRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotOpenglDebugMessageRef<'a> {
  type Arguments = (&'static ::opengl_debug_message::OpenGLDebugMessage,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotOpenglDebugMessageRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_opengl_debug_message_ref_callback(data: *mut ::libc::c_void,
                                                     arg0: *const ::opengl_debug_message::OpenGLDebugMessage) {
  let func: &mut Box<FnMut(&'static ::opengl_debug_message::OpenGLDebugMessage)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QPalette&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotPaletteRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPaletteRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::palette::Palette) + 'a>>>,
}

impl<'a> SlotPaletteRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::palette::Palette) + 'a>(f: F) -> SlotPaletteRef<'a> {
    let mut obj = SlotPaletteRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::palette::Palette) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::palette::Palette) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_palette_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotPaletteRef<'a> {
  fn default() -> Self {
    SlotPaletteRef {
      wrapper: ::slots::raw::RawSlotPaletteRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPaletteRef<'a> {
  type Arguments = (&'static ::palette::Palette,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPaletteRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_palette_ref_callback(data: *mut ::libc::c_void, arg0: *const ::palette::Palette) {
  let func: &mut Box<FnMut(&'static ::palette::Palette)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::ApplicationState)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtApplicationStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtApplicationStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::ApplicationState) + 'a>>>,
}

impl<'a> SlotQtCoreQtApplicationStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::ApplicationState) + 'a>(f: F) -> SlotQtCoreQtApplicationStateRef<'a> {
    let mut obj = SlotQtCoreQtApplicationStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::ApplicationState) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::ApplicationState) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_application_state_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtApplicationStateRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtApplicationStateRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtApplicationStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtApplicationStateRef<'a> {
  type Arguments = (&'static ::qt_core::qt::ApplicationState,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtApplicationStateRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_application_state_ref_callback(data: *mut ::libc::c_void,
                                                             arg0: *const ::qt_core::qt::ApplicationState) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::ApplicationState)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::DropAction)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtDropActionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtDropActionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::DropAction) + 'a>>>,
}

impl<'a> SlotQtCoreQtDropActionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::DropAction) + 'a>(f: F) -> SlotQtCoreQtDropActionRef<'a> {
    let mut obj = SlotQtCoreQtDropActionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::DropAction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::DropAction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_drop_action_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtDropActionRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtDropActionRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtDropActionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtDropActionRef<'a> {
  type Arguments = (&'static ::qt_core::qt::DropAction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtDropActionRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_drop_action_ref_callback(data: *mut ::libc::c_void,
                                                       arg0: *const ::qt_core::qt::DropAction) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::DropAction)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::LayoutDirection)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtLayoutDirectionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtLayoutDirectionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::LayoutDirection) + 'a>>>,
}

impl<'a> SlotQtCoreQtLayoutDirectionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::LayoutDirection) + 'a>(f: F) -> SlotQtCoreQtLayoutDirectionRef<'a> {
    let mut obj = SlotQtCoreQtLayoutDirectionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::LayoutDirection) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::LayoutDirection) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_layout_direction_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtLayoutDirectionRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtLayoutDirectionRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtLayoutDirectionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtLayoutDirectionRef<'a> {
  type Arguments = (&'static ::qt_core::qt::LayoutDirection,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtLayoutDirectionRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_layout_direction_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: *const ::qt_core::qt::LayoutDirection) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::LayoutDirection)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::ScreenOrientation)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtScreenOrientationRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtScreenOrientationRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::ScreenOrientation) + 'a>>>,
}

impl<'a> SlotQtCoreQtScreenOrientationRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::ScreenOrientation) + 'a>(f: F) -> SlotQtCoreQtScreenOrientationRef<'a> {
    let mut obj = SlotQtCoreQtScreenOrientationRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::ScreenOrientation) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::ScreenOrientation) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_screen_orientation_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtScreenOrientationRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtScreenOrientationRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtScreenOrientationRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtScreenOrientationRef<'a> {
  type Arguments = (&'static ::qt_core::qt::ScreenOrientation,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtScreenOrientationRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_screen_orientation_ref_callback(data: *mut ::libc::c_void,
                                                              arg0: *const ::qt_core::qt::ScreenOrientation) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::ScreenOrientation)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::TabFocusBehavior)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtTabFocusBehaviorRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::TabFocusBehavior) + 'a>>>,
}

impl<'a> SlotQtCoreQtTabFocusBehaviorRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::TabFocusBehavior) + 'a>(f: F) -> SlotQtCoreQtTabFocusBehaviorRef<'a> {
    let mut obj = SlotQtCoreQtTabFocusBehaviorRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::TabFocusBehavior) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::TabFocusBehavior) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_tab_focus_behavior_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtTabFocusBehaviorRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtTabFocusBehaviorRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtTabFocusBehaviorRef<'a> {
  type Arguments = (&'static ::qt_core::qt::TabFocusBehavior,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtTabFocusBehaviorRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_tab_focus_behavior_ref_callback(data: *mut ::libc::c_void,
                                                              arg0: *const ::qt_core::qt::TabFocusBehavior) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::TabFocusBehavior)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::WindowModality)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtWindowModalityRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtWindowModalityRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::WindowModality) + 'a>>>,
}

impl<'a> SlotQtCoreQtWindowModalityRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::WindowModality) + 'a>(f: F) -> SlotQtCoreQtWindowModalityRef<'a> {
    let mut obj = SlotQtCoreQtWindowModalityRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::WindowModality) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::WindowModality) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_window_modality_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtWindowModalityRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtWindowModalityRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtWindowModalityRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtWindowModalityRef<'a> {
  type Arguments = (&'static ::qt_core::qt::WindowModality,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtWindowModalityRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_window_modality_ref_callback(data: *mut ::libc::c_void,
                                                           arg0: *const ::qt_core::qt::WindowModality) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::WindowModality)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::WindowState)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreQtWindowStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtWindowStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::WindowState) + 'a>>>,
}

impl<'a> SlotQtCoreQtWindowStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::WindowState) + 'a>(f: F) -> SlotQtCoreQtWindowStateRef<'a> {
    let mut obj = SlotQtCoreQtWindowStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::WindowState) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::WindowState) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_window_state_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreQtWindowStateRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtWindowStateRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtWindowStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtWindowStateRef<'a> {
  type Arguments = (&'static ::qt_core::qt::WindowState,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtWindowStateRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_window_state_ref_callback(data: *mut ::libc::c_void,
                                                        arg0: *const ::qt_core::qt::WindowState) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::WindowState)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QRectF&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreRectFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::rect_f::RectF) + 'a>>>,
}

impl<'a> SlotQtCoreRectFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::rect_f::RectF) + 'a>(f: F) -> SlotQtCoreRectFRef<'a> {
    let mut obj = SlotQtCoreRectFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::rect_f::RectF) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::rect_f::RectF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_rect_f_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreRectFRef<'a> {
  fn default() -> Self {
    SlotQtCoreRectFRef {
      wrapper: ::slots::raw::RawSlotQtCoreRectFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRectFRef<'a> {
  type Arguments = (&'static ::qt_core::rect_f::RectF,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRectFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_rect_f_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::rect_f::RectF) {
  let func: &mut Box<FnMut(&'static ::qt_core::rect_f::RectF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QRect&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreRectRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::rect::Rect) + 'a>>>,
}

impl<'a> SlotQtCoreRectRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::rect::Rect) + 'a>(f: F) -> SlotQtCoreRectRef<'a> {
    let mut obj = SlotQtCoreRectRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::rect::Rect) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::rect::Rect) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_rect_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreRectRef<'a> {
  fn default() -> Self {
    SlotQtCoreRectRef {
      wrapper: ::slots::raw::RawSlotQtCoreRectRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRectRef<'a> {
  type Arguments = (&'static ::qt_core::rect::Rect,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRectRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_rect_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::rect::Rect) {
  let func: &mut Box<FnMut(&'static ::qt_core::rect::Rect)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QRegExp&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreRegExpRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRegExpRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::reg_exp::RegExp) + 'a>>>,
}

impl<'a> SlotQtCoreRegExpRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::reg_exp::RegExp) + 'a>(f: F) -> SlotQtCoreRegExpRef<'a> {
    let mut obj = SlotQtCoreRegExpRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::reg_exp::RegExp) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::reg_exp::RegExp) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_reg_exp_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreRegExpRef<'a> {
  fn default() -> Self {
    SlotQtCoreRegExpRef {
      wrapper: ::slots::raw::RawSlotQtCoreRegExpRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRegExpRef<'a> {
  type Arguments = (&'static ::qt_core::reg_exp::RegExp,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRegExpRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_reg_exp_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::reg_exp::RegExp) {
  let func: &mut Box<FnMut(&'static ::qt_core::reg_exp::RegExp)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QRegularExpression&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreRegularExpressionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRegularExpressionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::regular_expression::RegularExpression) + 'a>>>,
}

impl<'a> SlotQtCoreRegularExpressionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::regular_expression::RegularExpression) + 'a>
    (f: F)
     -> SlotQtCoreRegularExpressionRef<'a> {
    let mut obj = SlotQtCoreRegularExpressionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::regular_expression::RegularExpression) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::regular_expression::RegularExpression) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_regular_expression_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreRegularExpressionRef<'a> {
  fn default() -> Self {
    SlotQtCoreRegularExpressionRef {
      wrapper: ::slots::raw::RawSlotQtCoreRegularExpressionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRegularExpressionRef<'a> {
  type Arguments = (&'static ::qt_core::regular_expression::RegularExpression,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRegularExpressionRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_regular_expression_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::regular_expression::RegularExpression) {
  let func: &mut Box<FnMut(&'static ::qt_core::regular_expression::RegularExpression)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QSizeF&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreSizeFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreSizeFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::size_f::SizeF) + 'a>>>,
}

impl<'a> SlotQtCoreSizeFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::size_f::SizeF) + 'a>(f: F) -> SlotQtCoreSizeFRef<'a> {
    let mut obj = SlotQtCoreSizeFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::size_f::SizeF) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::size_f::SizeF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_size_f_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreSizeFRef<'a> {
  fn default() -> Self {
    SlotQtCoreSizeFRef {
      wrapper: ::slots::raw::RawSlotQtCoreSizeFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreSizeFRef<'a> {
  type Arguments = (&'static ::qt_core::size_f::SizeF,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreSizeFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_size_f_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::size_f::SizeF) {
  let func: &mut Box<FnMut(&'static ::qt_core::size_f::SizeF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QSize&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreSizeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreSizeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::size::Size) + 'a>>>,
}

impl<'a> SlotQtCoreSizeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::size::Size) + 'a>(f: F) -> SlotQtCoreSizeRef<'a> {
    let mut obj = SlotQtCoreSizeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::size::Size) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::size::Size) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_size_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreSizeRef<'a> {
  fn default() -> Self {
    SlotQtCoreSizeRef {
      wrapper: ::slots::raw::RawSlotQtCoreSizeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreSizeRef<'a> {
  type Arguments = (&'static ::qt_core::size::Size,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreSizeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_size_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::size::Size) {
  let func: &mut Box<FnMut(&'static ::qt_core::size::Size)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QUrl&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotQtCoreUrlRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreUrlRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::url::Url) + 'a>>>,
}

impl<'a> SlotQtCoreUrlRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::url::Url) + 'a>(f: F) -> SlotQtCoreUrlRef<'a> {
    let mut obj = SlotQtCoreUrlRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::url::Url) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::url::Url) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_url_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotQtCoreUrlRef<'a> {
  fn default() -> Self {
    SlotQtCoreUrlRef {
      wrapper: ::slots::raw::RawSlotQtCoreUrlRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreUrlRef<'a> {
  type Arguments = (&'static ::qt_core::url::Url,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreUrlRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_url_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::url::Url) {
  let func: &mut Box<FnMut(&'static ::qt_core::url::Url)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QScreen*)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotScreenMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotScreenMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::screen::Screen) + 'a>>>,
}

impl<'a> SlotScreenMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::screen::Screen) + 'a>(f: F) -> SlotScreenMutPtr<'a> {
    let mut obj = SlotScreenMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::screen::Screen) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::screen::Screen) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_screen_mut_ptr_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotScreenMutPtr<'a> {
  fn default() -> Self {
    SlotScreenMutPtr {
      wrapper: ::slots::raw::RawSlotScreenMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotScreenMutPtr<'a> {
  type Arguments = (*mut ::screen::Screen,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotScreenMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_screen_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::screen::Screen) {
  let func: &mut Box<FnMut(*mut ::screen::Screen)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QSessionManager&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotSessionManagerMutRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotSessionManagerMutRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static mut ::session_manager::SessionManager) + 'a>>>,
}

impl<'a> SlotSessionManagerMutRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static mut ::session_manager::SessionManager) + 'a>(f: F) -> SlotSessionManagerMutRef<'a> {
    let mut obj = SlotSessionManagerMutRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static mut ::session_manager::SessionManager) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static mut ::session_manager::SessionManager) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_session_manager_mut_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotSessionManagerMutRef<'a> {
  fn default() -> Self {
    SlotSessionManagerMutRef {
      wrapper: ::slots::raw::RawSlotSessionManagerMutRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotSessionManagerMutRef<'a> {
  type Arguments = (&'static mut ::session_manager::SessionManager,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotSessionManagerMutRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_session_manager_mut_ref_callback(data: *mut ::libc::c_void,
                                                    arg0: *mut ::session_manager::SessionManager) {
  let func: &mut Box<FnMut(&'static mut ::session_manager::SessionManager)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_mut() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QStandardItem*)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotStandardItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStandardItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::standard_item::StandardItem) + 'a>>>,
}

impl<'a> SlotStandardItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::standard_item::StandardItem) + 'a>(f: F) -> SlotStandardItemMutPtr<'a> {
    let mut obj = SlotStandardItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::standard_item::StandardItem) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::standard_item::StandardItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_standard_item_mut_ptr_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotStandardItemMutPtr<'a> {
  fn default() -> Self {
    SlotStandardItemMutPtr {
      wrapper: ::slots::raw::RawSlotStandardItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotStandardItemMutPtr<'a> {
  type Arguments = (*mut ::standard_item::StandardItem,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStandardItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_standard_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                  arg0: *mut ::standard_item::StandardItem) {
  let func: &mut Box<FnMut(*mut ::standard_item::StandardItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QTextBlock&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotTextBlockRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTextBlockRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::text_block::TextBlock) + 'a>>>,
}

impl<'a> SlotTextBlockRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::text_block::TextBlock) + 'a>(f: F) -> SlotTextBlockRef<'a> {
    let mut obj = SlotTextBlockRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::text_block::TextBlock) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::text_block::TextBlock) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_text_block_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotTextBlockRef<'a> {
  fn default() -> Self {
    SlotTextBlockRef {
      wrapper: ::slots::raw::RawSlotTextBlockRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTextBlockRef<'a> {
  type Arguments = (&'static ::text_block::TextBlock,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTextBlockRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_text_block_ref_callback(data: *mut ::libc::c_void, arg0: *const ::text_block::TextBlock) {
  let func: &mut Box<FnMut(&'static ::text_block::TextBlock)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QTextCursor&)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotTextCursorRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTextCursorRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::text_cursor::TextCursor) + 'a>>>,
}

impl<'a> SlotTextCursorRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::text_cursor::TextCursor) + 'a>(f: F) -> SlotTextCursorRef<'a> {
    let mut obj = SlotTextCursorRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::text_cursor::TextCursor) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::text_cursor::TextCursor) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_text_cursor_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotTextCursorRef<'a> {
  fn default() -> Self {
    SlotTextCursorRef {
      wrapper: ::slots::raw::RawSlotTextCursorRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTextCursorRef<'a> {
  type Arguments = (&'static ::text_cursor::TextCursor,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTextCursorRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_text_cursor_ref_callback(data: *mut ::libc::c_void, arg0: *const ::text_cursor::TextCursor) {
  let func: &mut Box<FnMut(&'static ::text_cursor::TextCursor)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QWindow*)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotWindowMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotWindowMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::window::Window) + 'a>>>,
}

impl<'a> SlotWindowMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::window::Window) + 'a>(f: F) -> SlotWindowMutPtr<'a> {
    let mut obj = SlotWindowMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::window::Window) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::window::Window) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_window_mut_ptr_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotWindowMutPtr<'a> {
  fn default() -> Self {
    SlotWindowMutPtr {
      wrapper: ::slots::raw::RawSlotWindowMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotWindowMutPtr<'a> {
  type Arguments = (*mut ::window::Window,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotWindowMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_window_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::window::Window) {
  let func: &mut Box<FnMut(*mut ::window::Window)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QWindow::Visibility)` to a Rust closure.
///
/// Create an object using `new()` and bind your closure using `set()`.
/// The closure will be called with the signal's arguments when the slot is invoked.
/// Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal
/// to this slot. The closure will be executed each time the slot is invoked
/// until source signals are disconnected or the slot object is destroyed.
///
/// The slot object takes ownership of the passed closure. If `set()` is called again,
/// previously set closure is dropped. Make sure that the slot object does not outlive
/// objects referenced by the closure.
///
/// If `set()` was not called, slot invokation has no effect.

pub struct SlotWindowVisibilityRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotWindowVisibilityRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::window::Visibility) + 'a>>>,
}

impl<'a> SlotWindowVisibilityRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::window::Visibility) + 'a>(f: F) -> SlotWindowVisibilityRef<'a> {
    let mut obj = SlotWindowVisibilityRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::window::Visibility) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::window::Visibility) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_window_visibility_ref_callback,
                       ::std::mem::transmute(func_box.as_mut()));
    }
    self.func = Some(func_box);
  }

  /// Drops the previously set closure, if any. After this, slot invokation will have no effect
  /// until a new closure is set.
  pub fn clear(&mut self) {
    if self.func.is_some() {
      unsafe {
        self.wrapper.set(::std::mem::transmute(0usize), ::std::ptr::null_mut());
      }
      self.func = None;
    }
  }
}

impl<'a> Default for SlotWindowVisibilityRef<'a> {
  fn default() -> Self {
    SlotWindowVisibilityRef {
      wrapper: ::slots::raw::RawSlotWindowVisibilityRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotWindowVisibilityRef<'a> {
  type Arguments = (&'static ::window::Visibility,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotWindowVisibilityRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_window_visibility_ref_callback(data: *mut ::libc::c_void, arg0: *const ::window::Visibility) {
  let func: &mut Box<FnMut(&'static ::window::Visibility)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
