/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(Qt3DInput::QAbstractPhysicalDevice*)` to a Rust extern function.
  ///
  /// Use `SlotAbstractPhysicalDeviceMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAbstractPhysicalDeviceMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  impl RawSlotAbstractPhysicalDeviceMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr::custom_slot(Qt3DInput::QAbstractPhysicalDevice* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_physical_device::AbstractPhysicalDevice) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QAbstractPhysicalDevice*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::abstract_physical_device::AbstractPhysicalDevice),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr_set(self as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QAxisAccumulator::SourceAxisType)` to a Rust extern function.
  ///
  /// Use `SlotAxisAccumulatorSourceAxisTypeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType```</span>
  #[repr(C)]
  pub struct RawSlotAxisAccumulatorSourceAxisTypeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef {
    type Arguments = (&'static ::axis_accumulator::SourceAxisType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QAxisAccumulator::SourceAxisType)\0"
    }
  }
  impl RawSlotAxisAccumulatorSourceAxisTypeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType::custom_slot(Qt3DInput::QAxisAccumulator::SourceAxisType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::axis_accumulator::SourceAxisType) {
      unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType_custom_slot(self as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef, arg0 as *const ::axis_accumulator::SourceAxisType) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType::qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType::set(void (*FN_PTR)(void*, const Qt3DInput::QAxisAccumulator::SourceAxisType*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::axis_accumulator::SourceAxisType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType_set(self as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QAxis*)` to a Rust extern function.
  ///
  /// Use `SlotAxisMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAxisMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAxisMutPtr {
    type Arguments = (*mut ::axis::Axis,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QAxis*)\0"
    }
  }
  impl RawSlotAxisMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr::custom_slot(Qt3DInput::QAxis* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::axis::Axis) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAxisMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAxisMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QAxis*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::axis::Axis),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr_set(self as *mut ::slots::raw::RawSlotAxisMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAxisMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyEvent*)` to a Rust extern function.
  ///
  /// Use `SlotKeyEventMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr```</span>
  #[repr(C)]
  pub struct RawSlotKeyEventMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotKeyEventMutPtr {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl RawSlotKeyEventMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr::custom_slot(Qt3DInput::QKeyEvent* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::key_event::KeyEvent) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr_custom_slot(self as *mut ::slots::raw::RawSlotKeyEventMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotKeyEventMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QKeyEvent*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::key_event::KeyEvent),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr_set(self as *mut ::slots::raw::RawSlotKeyEventMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotKeyEventMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyboardDevice*)` to a Rust extern function.
  ///
  /// Use `SlotKeyboardDeviceMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr```</span>
  #[repr(C)]
  pub struct RawSlotKeyboardDeviceMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotKeyboardDeviceMutPtr {
    type Arguments = (*mut ::keyboard_device::KeyboardDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QKeyboardDevice*)\0"
    }
  }
  impl RawSlotKeyboardDeviceMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr::custom_slot(Qt3DInput::QKeyboardDevice* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::keyboard_device::KeyboardDevice) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr_custom_slot(self as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotKeyboardDeviceMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QKeyboardDevice*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::keyboard_device::KeyboardDevice),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr_set(self as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotKeyboardDeviceMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyboardHandler*)` to a Rust extern function.
  ///
  /// Use `SlotKeyboardHandlerMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr```</span>
  #[repr(C)]
  pub struct RawSlotKeyboardHandlerMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotKeyboardHandlerMutPtr {
    type Arguments = (*mut ::keyboard_handler::KeyboardHandler,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QKeyboardHandler*)\0"
    }
  }
  impl RawSlotKeyboardHandlerMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr::custom_slot(Qt3DInput::QKeyboardHandler* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::keyboard_handler::KeyboardHandler) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr_custom_slot(self as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotKeyboardHandlerMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QKeyboardHandler*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::keyboard_handler::KeyboardHandler),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr_set(self as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotKeyboardHandlerMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QMouseDevice*)` to a Rust extern function.
  ///
  /// Use `SlotMouseDeviceMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr```</span>
  #[repr(C)]
  pub struct RawSlotMouseDeviceMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotMouseDeviceMutPtr {
    type Arguments = (*mut ::mouse_device::MouseDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QMouseDevice*)\0"
    }
  }
  impl RawSlotMouseDeviceMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr::custom_slot(Qt3DInput::QMouseDevice* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::mouse_device::MouseDevice) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr_custom_slot(self as *mut ::slots::raw::RawSlotMouseDeviceMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotMouseDeviceMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QMouseDevice*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::mouse_device::MouseDevice),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr_set(self as *mut ::slots::raw::RawSlotMouseDeviceMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotMouseDeviceMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DInput::QMouseEvent*)` to a Rust extern function.
  ///
  /// Use `SlotMouseEventMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr```</span>
  #[repr(C)]
  pub struct RawSlotMouseEventMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotMouseEventMutPtr {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl RawSlotMouseEventMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr::custom_slot(Qt3DInput::QMouseEvent* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::mouse_event::MouseEvent) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr_custom_slot(self as *mut ::slots::raw::RawSlotMouseEventMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr::qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotMouseEventMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr::set(void (*FN_PTR)(void*, Qt3DInput::QMouseEvent*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::mouse_event::MouseEvent),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr_set(self as *mut ::slots::raw::RawSlotMouseEventMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotMouseEventMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QVector<int>&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreVectorVectorCIntRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_input_c_SlotWrapper_const_QVector_int_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreVectorVectorCIntRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QVector< int >&)\0"
    }
  }
  impl RawSlotQtCoreVectorVectorCIntRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_input_c_SlotWrapper_const_QVector_int_ref::custom_slot(const QVector<int>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::vector::VectorCInt) {
      unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_const_QVector_int_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef, arg0 as *const ::qt_core::vector::VectorCInt) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_input_c_SlotWrapper_const_QVector_int_ref::qt_3d_input_c_SlotWrapper_const_QVector_int_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreVectorVectorCIntRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_const_QVector_int_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_input_c_SlotWrapper_const_QVector_int_ref::set(void (*FN_PTR)(void*, const QVector< int >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::vector::VectorCInt),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_const_QVector_int_ref_set(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_input_c_qt_3d_input_c_SlotWrapper_const_QVector_int_ref_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType(self as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType(self as *const ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAxisMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr(self as *mut ::slots::raw::RawSlotAxisMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr(self as *const ::slots::raw::RawSlotAxisMutPtr as *mut ::slots::raw::RawSlotAxisMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotKeyEventMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr(self as *mut ::slots::raw::RawSlotKeyEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr(self as *const ::slots::raw::RawSlotKeyEventMutPtr as *mut ::slots::raw::RawSlotKeyEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotKeyboardDeviceMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr(self as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr(self as *const ::slots::raw::RawSlotKeyboardDeviceMutPtr as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotKeyboardHandlerMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr(self as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr(self as *const ::slots::raw::RawSlotKeyboardHandlerMutPtr as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotMouseDeviceMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr(self as *mut ::slots::raw::RawSlotMouseDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr(self as *const ::slots::raw::RawSlotMouseDeviceMutPtr as *mut ::slots::raw::RawSlotMouseDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotMouseEventMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr(self as *mut ::slots::raw::RawSlotMouseEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr(self as *const ::slots::raw::RawSlotMouseEventMutPtr as *mut ::slots::raw::RawSlotMouseEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_const_QVector_int_ref(self as *const ::slots::raw::RawSlotQtCoreVectorVectorCIntRef as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *const ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType(self as *const ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAxisMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr(self as *const ::slots::raw::RawSlotAxisMutPtr as *mut ::slots::raw::RawSlotAxisMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotKeyEventMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr(self as *const ::slots::raw::RawSlotKeyEventMutPtr as *mut ::slots::raw::RawSlotKeyEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotKeyboardDeviceMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr(self as *const ::slots::raw::RawSlotKeyboardDeviceMutPtr as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotKeyboardHandlerMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr(self as *const ::slots::raw::RawSlotKeyboardHandlerMutPtr as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotMouseDeviceMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr(self as *const ::slots::raw::RawSlotMouseDeviceMutPtr as *mut ::slots::raw::RawSlotMouseDeviceMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotMouseEventMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr(self as *const ::slots::raw::RawSlotMouseEventMutPtr as *mut ::slots::raw::RawSlotMouseEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_const_QVector_int_ref(self as *const ::slots::raw::RawSlotQtCoreVectorVectorCIntRef as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAbstractPhysicalDevice_ptr(self as *mut ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxisAccumulator_SourceAxisType(self as *mut ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAxisMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QAxis_ptr(self as *mut ::slots::raw::RawSlotAxisMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotKeyEventMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyEvent_ptr(self as *mut ::slots::raw::RawSlotKeyEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotKeyboardDeviceMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardDevice_ptr(self as *mut ::slots::raw::RawSlotKeyboardDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotKeyboardHandlerMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QKeyboardHandler_ptr(self as *mut ::slots::raw::RawSlotKeyboardHandlerMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotMouseDeviceMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseDevice_ptr(self as *mut ::slots::raw::RawSlotMouseDeviceMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotMouseEventMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_Qt3DInput_QMouseEvent_ptr(self as *mut ::slots::raw::RawSlotMouseEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_input_c_slots_G_static_cast_QObject_ptr_qt_3d_input_c_SlotWrapper_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(Qt3DInput::QAbstractPhysicalDevice*)` to a Rust closure.
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

pub struct SlotAbstractPhysicalDeviceMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_physical_device::AbstractPhysicalDevice) + 'a>>>,
}

impl<'a> SlotAbstractPhysicalDeviceMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_physical_device::AbstractPhysicalDevice) + 'a>
    (f: F)
     -> SlotAbstractPhysicalDeviceMutPtr<'a> {
    let mut obj = SlotAbstractPhysicalDeviceMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_physical_device::AbstractPhysicalDevice) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_physical_device::AbstractPhysicalDevice) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_physical_device_mut_ptr_callback,
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

impl<'a> Default for SlotAbstractPhysicalDeviceMutPtr<'a> {
  fn default() -> Self {
    SlotAbstractPhysicalDeviceMutPtr {
      wrapper: ::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractPhysicalDeviceMutPtr<'a> {
  type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractPhysicalDeviceMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_physical_device_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::abstract_physical_device::AbstractPhysicalDevice) {
  let func: &mut Box<FnMut(*mut ::abstract_physical_device::AbstractPhysicalDevice)> =
    unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QAxisAccumulator::SourceAxisType)` to a Rust closure.
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

pub struct SlotAxisAccumulatorSourceAxisTypeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::axis_accumulator::SourceAxisType) + 'a>>>,
}

impl<'a> SlotAxisAccumulatorSourceAxisTypeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::axis_accumulator::SourceAxisType) + 'a>
    (f: F)
     -> SlotAxisAccumulatorSourceAxisTypeRef<'a> {
    let mut obj = SlotAxisAccumulatorSourceAxisTypeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::axis_accumulator::SourceAxisType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::axis_accumulator::SourceAxisType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_axis_accumulator_source_axis_type_ref_callback,
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

impl<'a> Default for SlotAxisAccumulatorSourceAxisTypeRef<'a> {
  fn default() -> Self {
    SlotAxisAccumulatorSourceAxisTypeRef {
      wrapper: ::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAxisAccumulatorSourceAxisTypeRef<'a> {
  type Arguments = (&'static ::axis_accumulator::SourceAxisType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAxisAccumulatorSourceAxisTypeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_axis_accumulator_source_axis_type_ref_callback(data: *mut ::libc::c_void,
                                                                  arg0: *const ::axis_accumulator::SourceAxisType) {
  let func: &mut Box<FnMut(&'static ::axis_accumulator::SourceAxisType)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QAxis*)` to a Rust closure.
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

pub struct SlotAxisMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAxisMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::axis::Axis) + 'a>>>,
}

impl<'a> SlotAxisMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::axis::Axis) + 'a>(f: F) -> SlotAxisMutPtr<'a> {
    let mut obj = SlotAxisMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::axis::Axis) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::axis::Axis) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_axis_mut_ptr_callback,
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

impl<'a> Default for SlotAxisMutPtr<'a> {
  fn default() -> Self {
    SlotAxisMutPtr {
      wrapper: ::slots::raw::RawSlotAxisMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAxisMutPtr<'a> {
  type Arguments = (*mut ::axis::Axis,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAxisMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_axis_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::axis::Axis) {
  let func: &mut Box<FnMut(*mut ::axis::Axis)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyEvent*)` to a Rust closure.
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

pub struct SlotKeyEventMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotKeyEventMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::key_event::KeyEvent) + 'a>>>,
}

impl<'a> SlotKeyEventMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::key_event::KeyEvent) + 'a>(f: F) -> SlotKeyEventMutPtr<'a> {
    let mut obj = SlotKeyEventMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::key_event::KeyEvent) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::key_event::KeyEvent) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_key_event_mut_ptr_callback,
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

impl<'a> Default for SlotKeyEventMutPtr<'a> {
  fn default() -> Self {
    SlotKeyEventMutPtr {
      wrapper: ::slots::raw::RawSlotKeyEventMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotKeyEventMutPtr<'a> {
  type Arguments = (*mut ::key_event::KeyEvent,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotKeyEventMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_key_event_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::key_event::KeyEvent) {
  let func: &mut Box<FnMut(*mut ::key_event::KeyEvent)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyboardDevice*)` to a Rust closure.
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

pub struct SlotKeyboardDeviceMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotKeyboardDeviceMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::keyboard_device::KeyboardDevice) + 'a>>>,
}

impl<'a> SlotKeyboardDeviceMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::keyboard_device::KeyboardDevice) + 'a>(f: F) -> SlotKeyboardDeviceMutPtr<'a> {
    let mut obj = SlotKeyboardDeviceMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::keyboard_device::KeyboardDevice) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::keyboard_device::KeyboardDevice) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_keyboard_device_mut_ptr_callback,
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

impl<'a> Default for SlotKeyboardDeviceMutPtr<'a> {
  fn default() -> Self {
    SlotKeyboardDeviceMutPtr {
      wrapper: ::slots::raw::RawSlotKeyboardDeviceMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotKeyboardDeviceMutPtr<'a> {
  type Arguments = (*mut ::keyboard_device::KeyboardDevice,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotKeyboardDeviceMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_keyboard_device_mut_ptr_callback(data: *mut ::libc::c_void,
                                                    arg0: *mut ::keyboard_device::KeyboardDevice) {
  let func: &mut Box<FnMut(*mut ::keyboard_device::KeyboardDevice)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QKeyboardHandler*)` to a Rust closure.
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

pub struct SlotKeyboardHandlerMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotKeyboardHandlerMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::keyboard_handler::KeyboardHandler) + 'a>>>,
}

impl<'a> SlotKeyboardHandlerMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::keyboard_handler::KeyboardHandler) + 'a>(f: F) -> SlotKeyboardHandlerMutPtr<'a> {
    let mut obj = SlotKeyboardHandlerMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::keyboard_handler::KeyboardHandler) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::keyboard_handler::KeyboardHandler) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_keyboard_handler_mut_ptr_callback,
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

impl<'a> Default for SlotKeyboardHandlerMutPtr<'a> {
  fn default() -> Self {
    SlotKeyboardHandlerMutPtr {
      wrapper: ::slots::raw::RawSlotKeyboardHandlerMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotKeyboardHandlerMutPtr<'a> {
  type Arguments = (*mut ::keyboard_handler::KeyboardHandler,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotKeyboardHandlerMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_keyboard_handler_mut_ptr_callback(data: *mut ::libc::c_void,
                                                     arg0: *mut ::keyboard_handler::KeyboardHandler) {
  let func: &mut Box<FnMut(*mut ::keyboard_handler::KeyboardHandler)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QMouseDevice*)` to a Rust closure.
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

pub struct SlotMouseDeviceMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotMouseDeviceMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::mouse_device::MouseDevice) + 'a>>>,
}

impl<'a> SlotMouseDeviceMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::mouse_device::MouseDevice) + 'a>(f: F) -> SlotMouseDeviceMutPtr<'a> {
    let mut obj = SlotMouseDeviceMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::mouse_device::MouseDevice) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::mouse_device::MouseDevice) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_mouse_device_mut_ptr_callback,
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

impl<'a> Default for SlotMouseDeviceMutPtr<'a> {
  fn default() -> Self {
    SlotMouseDeviceMutPtr {
      wrapper: ::slots::raw::RawSlotMouseDeviceMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotMouseDeviceMutPtr<'a> {
  type Arguments = (*mut ::mouse_device::MouseDevice,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotMouseDeviceMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_mouse_device_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::mouse_device::MouseDevice) {
  let func: &mut Box<FnMut(*mut ::mouse_device::MouseDevice)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DInput::QMouseEvent*)` to a Rust closure.
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

pub struct SlotMouseEventMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotMouseEventMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::mouse_event::MouseEvent) + 'a>>>,
}

impl<'a> SlotMouseEventMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::mouse_event::MouseEvent) + 'a>(f: F) -> SlotMouseEventMutPtr<'a> {
    let mut obj = SlotMouseEventMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::mouse_event::MouseEvent) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::mouse_event::MouseEvent) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_mouse_event_mut_ptr_callback,
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

impl<'a> Default for SlotMouseEventMutPtr<'a> {
  fn default() -> Self {
    SlotMouseEventMutPtr {
      wrapper: ::slots::raw::RawSlotMouseEventMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotMouseEventMutPtr<'a> {
  type Arguments = (*mut ::mouse_event::MouseEvent,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotMouseEventMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_mouse_event_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::mouse_event::MouseEvent) {
  let func: &mut Box<FnMut(*mut ::mouse_event::MouseEvent)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QVector<int>&)` to a Rust closure.
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

pub struct SlotQtCoreVectorVectorCIntRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreVectorVectorCIntRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::vector::VectorCInt) + 'a>>>,
}

impl<'a> SlotQtCoreVectorVectorCIntRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::vector::VectorCInt) + 'a>(f: F) -> SlotQtCoreVectorVectorCIntRef<'a> {
    let mut obj = SlotQtCoreVectorVectorCIntRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::vector::VectorCInt) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::vector::VectorCInt) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_vector_vector_c_int_ref_callback,
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

impl<'a> Default for SlotQtCoreVectorVectorCIntRef<'a> {
  fn default() -> Self {
    SlotQtCoreVectorVectorCIntRef {
      wrapper: ::slots::raw::RawSlotQtCoreVectorVectorCIntRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreVectorVectorCIntRef<'a> {
  type Arguments = (&'static ::qt_core::vector::VectorCInt,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreVectorVectorCIntRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_vector_vector_c_int_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: *const ::qt_core::vector::VectorCInt) {
  let func: &mut Box<FnMut(&'static ::qt_core::vector::VectorCInt)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
