/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(QAbstractAnimation::Direction)` to a Rust extern function.
  ///
  /// Use `SlotAbstractAnimationDirectionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QAbstractAnimation_Direction```</span>
  #[repr(C)]
  pub struct RawSlotAbstractAnimationDirectionRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotAbstractAnimationDirectionRef {
    type Arguments = (&'static ::abstract_animation::Direction,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractAnimation::Direction)\0"
    }
  }
  impl RawSlotAbstractAnimationDirectionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QAbstractAnimation_Direction::custom_slot(QAbstractAnimation::Direction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::abstract_animation::Direction) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_Direction_custom_slot(self as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef, arg0 as *const ::abstract_animation::Direction) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QAbstractAnimation_Direction::qt_core_c_SlotWrapper_QAbstractAnimation_Direction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationDirectionRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_Direction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QAbstractAnimation_Direction::set(void (*FN_PTR)(void*, const QAbstractAnimation::Direction*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::abstract_animation::Direction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_Direction_set(self as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractAnimationDirectionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_Direction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAbstractAnimation*)` to a Rust extern function.
  ///
  /// Use `SlotAbstractAnimationMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QAbstractAnimation_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAbstractAnimationMutPtr(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotAbstractAnimationMutPtr {
    type Arguments = (*mut ::abstract_animation::AbstractAnimation,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractAnimation*)\0"
    }
  }
  impl RawSlotAbstractAnimationMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QAbstractAnimation_ptr::custom_slot(QAbstractAnimation* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_animation::AbstractAnimation) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QAbstractAnimation_ptr::qt_core_c_SlotWrapper_QAbstractAnimation_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QAbstractAnimation_ptr::set(void (*FN_PTR)(void*, QAbstractAnimation*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::abstract_animation::AbstractAnimation),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_ptr_set(self as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractAnimationMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAbstractAnimation::State)` to a Rust extern function.
  ///
  /// Use `SlotAbstractAnimationStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QAbstractAnimation_State```</span>
  #[repr(C)]
  pub struct RawSlotAbstractAnimationStateRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotAbstractAnimationStateRef {
    type Arguments = (&'static ::abstract_animation::State,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractAnimation::State)\0"
    }
  }
  impl RawSlotAbstractAnimationStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QAbstractAnimation_State::custom_slot(QAbstractAnimation::State arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::abstract_animation::State) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_custom_slot(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRef, arg0 as *const ::abstract_animation::State) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QAbstractAnimation_State::qt_core_c_SlotWrapper_QAbstractAnimation_State()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationStateRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QAbstractAnimation_State::set(void (*FN_PTR)(void*, const QAbstractAnimation::State*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::abstract_animation::State),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_set(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractAnimationStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAbstractAnimation::State, QAbstractAnimation::State)` to a Rust extern function.
  ///
  /// Use `SlotAbstractAnimationStateRefAbstractAnimationStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State```</span>
  #[repr(C)]
  pub struct RawSlotAbstractAnimationStateRefAbstractAnimationStateRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
    type Arguments = (&'static ::abstract_animation::State, &'static ::abstract_animation::State);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractAnimation::State,QAbstractAnimation::State)\0"
    }
  }
  impl RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State::custom_slot(QAbstractAnimation::State arg0, QAbstractAnimation::State arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::abstract_animation::State, arg1: &::abstract_animation::State) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State_custom_slot(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef, arg0 as *const ::abstract_animation::State, arg1 as *const ::abstract_animation::State) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State::qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef> {
      let ffi_result =
        unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State::set(void (*FN_PTR)(void*, const QAbstractAnimation::State*, const QAbstractAnimation::State*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::abstract_animation::State,
                                          *const ::abstract_animation::State),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State_set(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAbstractItemModel*)` to a Rust extern function.
  ///
  /// Use `SlotAbstractItemModelMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QAbstractItemModel_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAbstractItemModelMutPtr(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotAbstractItemModelMutPtr {
    type Arguments = (*mut ::abstract_item_model::AbstractItemModel,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractItemModel*)\0"
    }
  }
  impl RawSlotAbstractItemModelMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QAbstractItemModel_ptr::custom_slot(QAbstractItemModel* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_item_model::AbstractItemModel) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractItemModel_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QAbstractItemModel_ptr::qt_core_c_SlotWrapper_QAbstractItemModel_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractItemModelMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractItemModel_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QAbstractItemModel_ptr::set(void (*FN_PTR)(void*, QAbstractItemModel*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::abstract_item_model::AbstractItemModel),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractItemModel_ptr_set(self as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractItemModelMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QAbstractItemModel_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(bool)` to a Rust extern function.
  ///
  /// Use `SlotBool` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_bool```</span>
  #[repr(C)]
  pub struct RawSlotBool(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotBool {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(bool)\0"
    }
  }
  impl RawSlotBool {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_bool::custom_slot(bool arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: bool) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_bool_custom_slot(self as *mut ::slots::raw::RawSlotBool, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_bool::qt_core_c_SlotWrapper_bool()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBool> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_bool_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_bool::set(void (*FN_PTR)(void*, bool) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void, bool), data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_bool_set(self as *mut ::slots::raw::RawSlotBool, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBool {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_bool_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(double)` to a Rust extern function.
  ///
  /// Use `SlotCDouble` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_double```</span>
  #[repr(C)]
  pub struct RawSlotCDouble(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotCDouble {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(double)\0"
    }
  }
  impl RawSlotCDouble {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_double::custom_slot(double arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_double) {
      unsafe {
        ::ffi::qt_core_c_qt_core_c_SlotWrapper_double_custom_slot(self as *mut ::slots::raw::RawSlotCDouble, arg0)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_double::qt_core_c_SlotWrapper_double()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCDouble> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_double_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_double::set(void (*FN_PTR)(void*, double) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::libc::c_double),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_double_set(self as *mut ::slots::raw::RawSlotCDouble, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCDouble {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_double_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int)` to a Rust extern function.
  ///
  /// Use `SlotCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_int```</span>
  #[repr(C)]
  pub struct RawSlotCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotCInt {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int)\0"
    }
  }
  impl RawSlotCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_int::custom_slot(int arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_custom_slot(self as *mut ::slots::raw::RawSlotCInt, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_int::qt_core_c_SlotWrapper_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCInt> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_int::set(void (*FN_PTR)(void*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void, ::libc::c_int), data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_set(self as *mut ::slots::raw::RawSlotCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, qint64)` to a Rust extern function.
  ///
  /// Use `SlotCIntI64` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_int_qint64```</span>
  #[repr(C)]
  pub struct RawSlotCIntI64(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotCIntI64 {
    type Arguments = (::libc::c_int, i64);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,qint64)\0"
    }
  }
  impl RawSlotCIntI64 {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_int_qint64::custom_slot(int arg0, qint64 arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: i64) {
      unsafe {
        ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_qint64_custom_slot(self as *mut ::slots::raw::RawSlotCIntI64,
                                                                      arg0,
                                                                      arg1)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_int_qint64::qt_core_c_SlotWrapper_int_qint64()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntI64> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_qint64_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_int_qint64::set(void (*FN_PTR)(void*, int, qint64) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::libc::c_int, i64),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_qint64_set(self as *mut ::slots::raw::RawSlotCIntI64, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntI64 {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_qint64_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, QProcess::ExitStatus)` to a Rust extern function.
  ///
  /// Use `SlotCIntProcessExitStatusRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_int_QProcess_ExitStatus```</span>
  #[repr(C)]
  pub struct RawSlotCIntProcessExitStatusRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotCIntProcessExitStatusRef {
    type Arguments = (::libc::c_int, &'static ::process::ExitStatus);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,QProcess::ExitStatus)\0"
    }
  }
  impl RawSlotCIntProcessExitStatusRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_int_QProcess_ExitStatus::custom_slot(int arg0, QProcess::ExitStatus arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: &::process::ExitStatus) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_QProcess_ExitStatus_custom_slot(self as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef, arg0, arg1 as *const ::process::ExitStatus) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_int_QProcess_ExitStatus::qt_core_c_SlotWrapper_int_QProcess_ExitStatus()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntProcessExitStatusRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_QProcess_ExitStatus_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_int_QProcess_ExitStatus::set(void (*FN_PTR)(void*, int, const QProcess::ExitStatus*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::libc::c_int,
                                          *const ::process::ExitStatus),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_QProcess_ExitStatus_set(self as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntProcessExitStatusRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_int_QProcess_ExitStatus_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(qint64)` to a Rust extern function.
  ///
  /// Use `SlotI64` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_qint64```</span>
  #[repr(C)]
  pub struct RawSlotI64(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotI64 {
    type Arguments = (i64,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(qint64)\0"
    }
  }
  impl RawSlotI64 {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_qint64::custom_slot(qint64 arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: i64) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_qint64_custom_slot(self as *mut ::slots::raw::RawSlotI64, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_qint64::qt_core_c_SlotWrapper_qint64()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotI64> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_qint64_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_qint64::set(void (*FN_PTR)(void*, qint64) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void, i64), data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_qint64_set(self as *mut ::slots::raw::RawSlotI64, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotI64 {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_qint64_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QItemSelection&)` to a Rust extern function.
  ///
  /// Use `SlotItemSelectionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QItemSelection_ref```</span>
  #[repr(C)]
  pub struct RawSlotItemSelectionRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotItemSelectionRef {
    type Arguments = (&'static ::item_selection::ItemSelection,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QItemSelection&)\0"
    }
  }
  impl RawSlotItemSelectionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QItemSelection_ref::custom_slot(const QItemSelection& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::item_selection::ItemSelection) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_custom_slot(self as *mut ::slots::raw::RawSlotItemSelectionRef, arg0 as *const ::item_selection::ItemSelection) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QItemSelection_ref::qt_core_c_SlotWrapper_const_QItemSelection_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotItemSelectionRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QItemSelection_ref::set(void (*FN_PTR)(void*, const QItemSelection*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::item_selection::ItemSelection),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_set(self as *mut ::slots::raw::RawSlotItemSelectionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotItemSelectionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QItemSelection&, const QItemSelection&)` to a Rust extern function.
  ///
  /// Use `SlotItemSelectionRefItemSelectionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref```</span>
  #[repr(C)]
  pub struct RawSlotItemSelectionRefItemSelectionRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotItemSelectionRefItemSelectionRef {
    type Arguments = (&'static ::item_selection::ItemSelection, &'static ::item_selection::ItemSelection);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QItemSelection&,const QItemSelection&)\0"
    }
  }
  impl RawSlotItemSelectionRefItemSelectionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref::custom_slot(const QItemSelection& arg0, const QItemSelection& arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::item_selection::ItemSelection, arg1: &::item_selection::ItemSelection) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref_custom_slot(self as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef, arg0 as *const ::item_selection::ItemSelection, arg1 as *const ::item_selection::ItemSelection) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref::qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotItemSelectionRefItemSelectionRef> {
      let ffi_result =
        unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref::set(void (*FN_PTR)(void*, const QItemSelection*, const QItemSelection*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::item_selection::ItemSelection,
                                          *const ::item_selection::ItemSelection),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref_set(self as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotItemSelectionRefItemSelectionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QList<QPersistentModelIndex>&)` to a Rust extern function.
  ///
  /// Use `SlotListListPersistentModelIndexRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref```</span>
  #[repr(C)]
  pub struct RawSlotListListPersistentModelIndexRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotListListPersistentModelIndexRef {
    type Arguments = (&'static ::list::ListPersistentModelIndex,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QList< QPersistentModelIndex >&)\0"
    }
  }
  impl RawSlotListListPersistentModelIndexRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref::custom_slot(const QList<QPersistentModelIndex>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::list::ListPersistentModelIndex) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_custom_slot(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef, arg0 as *const ::list::ListPersistentModelIndex) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref::qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotListListPersistentModelIndexRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref::set(void (*FN_PTR)(void*, const QList< QPersistentModelIndex >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::list::ListPersistentModelIndex),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_set(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotListListPersistentModelIndexRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QList<QPersistentModelIndex>&, QAbstractItemModel::LayoutChangeHint)` to a Rust extern function.
  ///
  /// Use `SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint```</span>
  #[repr(C)]
  pub struct RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
  type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
  fn object(&self) -> &::object::Object {
    ::cpp_utils::StaticCast::static_cast(self)
  }
  fn receiver_id() -> &'static [u8] {
    b"1custom_slot(const QList< QPersistentModelIndex >&,QAbstractItemModel::LayoutChangeHint)\0"
  }
}
  impl RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint::custom_slot(const QList<QPersistentModelIndex>& arg0, QAbstractItemModel::LayoutChangeHint arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::list::ListPersistentModelIndex,
                       arg1: &::abstract_item_model::LayoutChangeHint) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint_custom_slot(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef, arg0 as *const ::list::ListPersistentModelIndex, arg1 as *const ::abstract_item_model::LayoutChangeHint) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint::qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint()```</span>
    ///
    ///
    pub fn new
      ()
        -> ::cpp_utils::CppBox<::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef>
    {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint::set(void (*FN_PTR)(void*, const QList< QPersistentModelIndex >*, const QAbstractItemModel::LayoutChangeHint*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::list::ListPersistentModelIndex,
                                          *const ::abstract_item_model::LayoutChangeHint),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint_set(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
fn deleter() -> ::cpp_utils::Deleter<Self> {
  ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint_delete
}
}

  /// Allows to bind Qt signals with arguments `(const QModelIndex&)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRef {
    type Arguments = (&'static ::model_index::ModelIndex,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&)\0"
    }
  }
  impl RawSlotModelIndexRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref::custom_slot(const QModelIndex& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::model_index::ModelIndex) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRef, arg0 as *const ::model_index::ModelIndex) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref::qt_core_c_SlotWrapper_const_QModelIndex_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref::set(void (*FN_PTR)(void*, const QModelIndex*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::model_index::ModelIndex),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_set(self as *mut ::slots::raw::RawSlotModelIndexRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, int)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_int```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefCInt {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,int)\0"
    }
  }
  impl RawSlotModelIndexRefCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int::custom_slot(const QModelIndex& arg0, int arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::model_index::ModelIndex, arg1: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefCInt, arg0 as *const ::model_index::ModelIndex, arg1) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int::qt_core_c_SlotWrapper_const_QModelIndex_ref_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCInt> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_int::set(void (*FN_PTR)(void*, const QModelIndex*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_set(self as *mut ::slots::raw::RawSlotModelIndexRefCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefCIntCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefCIntCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefCIntCInt {
    type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,int,int)\0"
    }
  }
  impl RawSlotModelIndexRefCIntCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int::custom_slot(const QModelIndex& arg0, int arg1, int arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::model_index::ModelIndex, arg1: ::libc::c_int, arg2: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt, arg0 as *const ::model_index::ModelIndex, arg1, arg2) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int::qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCInt> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int::set(void (*FN_PTR)(void*, const QModelIndex*, int, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          ::libc::c_int,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_set(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefCIntCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int, const QModelIndex&)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefCIntCIntModelIndexRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefCIntCIntModelIndexRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,int,int,const QModelIndex&)\0"
    }
  }
  impl RawSlotModelIndexRefCIntCIntModelIndexRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref::custom_slot(const QModelIndex& arg0, int arg1, int arg2, const QModelIndex& arg3)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::model_index::ModelIndex,
                       arg1: ::libc::c_int,
                       arg2: ::libc::c_int,
                       arg3: &::model_index::ModelIndex) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef, arg0 as *const ::model_index::ModelIndex, arg1, arg2, arg3 as *const ::model_index::ModelIndex) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref::qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef> {
      let ffi_result =
        unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref::set(void (*FN_PTR)(void*, const QModelIndex*, int, int, const QModelIndex*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          *const ::model_index::ModelIndex),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_set(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int, const QModelIndex&, int)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefCIntCIntModelIndexRefCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefCIntCIntModelIndexRefCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    type Arguments = (&'static ::model_index::ModelIndex,
     ::libc::c_int,
     ::libc::c_int,
     &'static ::model_index::ModelIndex,
     ::libc::c_int);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,int,int,const QModelIndex&,int)\0"
    }
  }
  impl RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int::custom_slot(const QModelIndex& arg0, int arg1, int arg2, const QModelIndex& arg3, int arg4)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::model_index::ModelIndex,
                       arg1: ::libc::c_int,
                       arg2: ::libc::c_int,
                       arg3: &::model_index::ModelIndex,
                       arg4: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt, arg0 as *const ::model_index::ModelIndex, arg1, arg2, arg3 as *const ::model_index::ModelIndex, arg4) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int::qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt> {
      let ffi_result =
        unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int::set(void (*FN_PTR)(void*, const QModelIndex*, int, int, const QModelIndex*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          *const ::model_index::ModelIndex,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int_set(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, const QModelIndex&)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefModelIndexRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefModelIndexRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefModelIndexRef {
    type Arguments = (&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,const QModelIndex&)\0"
    }
  }
  impl RawSlotModelIndexRefModelIndexRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref::custom_slot(const QModelIndex& arg0, const QModelIndex& arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::model_index::ModelIndex, arg1: &::model_index::ModelIndex) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef, arg0 as *const ::model_index::ModelIndex, arg1 as *const ::model_index::ModelIndex) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref::qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefModelIndexRef> {
      let ffi_result =
        unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref::set(void (*FN_PTR)(void*, const QModelIndex*, const QModelIndex*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          *const ::model_index::ModelIndex),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_set(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefModelIndexRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QModelIndex&, const QModelIndex&, const QVector<int>&)` to a Rust extern function.
  ///
  /// Use `SlotModelIndexRefModelIndexRefVectorVectorCIntRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref```</span>
  #[repr(C)]
  pub struct RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
    type Arguments = (&'static ::model_index::ModelIndex,
     &'static ::model_index::ModelIndex,
     &'static ::vector::VectorCInt);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QModelIndex&,const QModelIndex&,const QVector< int >&)\0"
    }
  }
  impl RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref::custom_slot(const QModelIndex& arg0, const QModelIndex& arg1, const QVector<int>& arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::model_index::ModelIndex,
                       arg1: &::model_index::ModelIndex,
                       arg2: &::vector::VectorCInt) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref_custom_slot(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef, arg0 as *const ::model_index::ModelIndex, arg1 as *const ::model_index::ModelIndex, arg2 as *const ::vector::VectorCInt) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref::qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef> {
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref::set(void (*FN_PTR)(void*, const QModelIndex*, const QModelIndex*, const QVector< int >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::model_index::ModelIndex,
                                          *const ::model_index::ModelIndex,
                                          *const ::vector::VectorCInt),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref_set(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `()` to a Rust extern function.
  ///
  /// Use `SlotNoArgs` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_no_args```</span>
  #[repr(C)]
  pub struct RawSlotNoArgs(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotNoArgs {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot()\0"
    }
  }
  impl RawSlotNoArgs {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_no_args::custom_slot()```</span>
    ///
    ///
    pub fn custom_slot(&mut self) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_no_args_custom_slot(self as *mut ::slots::raw::RawSlotNoArgs) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_no_args::qt_core_c_SlotWrapper_no_args()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotNoArgs> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_no_args_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_no_args::set(void (*FN_PTR)(void*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void), data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_no_args_set(self as *mut ::slots::raw::RawSlotNoArgs, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotNoArgs {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_no_args_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QObject*)` to a Rust extern function.
  ///
  /// Use `SlotObjectMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QObject_ptr```</span>
  #[repr(C)]
  pub struct RawSlotObjectMutPtr(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotObjectMutPtr {
    type Arguments = (*mut ::object::Object,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QObject*)\0"
    }
  }
  impl RawSlotObjectMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QObject_ptr::custom_slot(QObject* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::object::Object) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QObject_ptr_custom_slot(self as *mut ::slots::raw::RawSlotObjectMutPtr,
                                                                     arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QObject_ptr::qt_core_c_SlotWrapper_QObject_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotObjectMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QObject_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QObject_ptr::set(void (*FN_PTR)(void*, QObject*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::object::Object),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QObject_ptr_set(self as *mut ::slots::raw::RawSlotObjectMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotObjectMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QObject_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QProcess::ProcessError)` to a Rust extern function.
  ///
  /// Use `SlotProcessProcessErrorRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QProcess_ProcessError```</span>
  #[repr(C)]
  pub struct RawSlotProcessProcessErrorRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotProcessProcessErrorRef {
    type Arguments = (&'static ::process::ProcessError,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QProcess::ProcessError)\0"
    }
  }
  impl RawSlotProcessProcessErrorRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QProcess_ProcessError::custom_slot(QProcess::ProcessError arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::process::ProcessError) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessError_custom_slot(self as *mut ::slots::raw::RawSlotProcessProcessErrorRef, arg0 as *const ::process::ProcessError) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QProcess_ProcessError::qt_core_c_SlotWrapper_QProcess_ProcessError()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotProcessProcessErrorRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessError_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QProcess_ProcessError::set(void (*FN_PTR)(void*, const QProcess::ProcessError*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::process::ProcessError),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessError_set(self as *mut ::slots::raw::RawSlotProcessProcessErrorRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotProcessProcessErrorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessError_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QProcess::ProcessState)` to a Rust extern function.
  ///
  /// Use `SlotProcessProcessStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QProcess_ProcessState```</span>
  #[repr(C)]
  pub struct RawSlotProcessProcessStateRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotProcessProcessStateRef {
    type Arguments = (&'static ::process::ProcessState,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QProcess::ProcessState)\0"
    }
  }
  impl RawSlotProcessProcessStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QProcess_ProcessState::custom_slot(QProcess::ProcessState arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::process::ProcessState) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessState_custom_slot(self as *mut ::slots::raw::RawSlotProcessProcessStateRef, arg0 as *const ::process::ProcessState) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QProcess_ProcessState::qt_core_c_SlotWrapper_QProcess_ProcessState()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotProcessProcessStateRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessState_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QProcess_ProcessState::set(void (*FN_PTR)(void*, const QProcess::ProcessState*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::process::ProcessState),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessState_set(self as *mut ::slots::raw::RawSlotProcessProcessStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotProcessProcessStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QProcess_ProcessState_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::Orientation)` to a Rust extern function.
  ///
  /// Use `SlotQtOrientationRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_Qt_Orientation```</span>
  #[repr(C)]
  pub struct RawSlotQtOrientationRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotQtOrientationRef {
    type Arguments = (&'static ::qt::Orientation,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::Orientation)\0"
    }
  }
  impl RawSlotQtOrientationRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_Qt_Orientation::custom_slot(Qt::Orientation arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt::Orientation) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_custom_slot(self as *mut ::slots::raw::RawSlotQtOrientationRef, arg0 as *const ::qt::Orientation) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_Qt_Orientation::qt_core_c_SlotWrapper_Qt_Orientation()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_Qt_Orientation::set(void (*FN_PTR)(void*, const Qt::Orientation*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt::Orientation),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_set(self as *mut ::slots::raw::RawSlotQtOrientationRef,
                                                                func,
                                                                data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtOrientationRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::Orientation, int)` to a Rust extern function.
  ///
  /// Use `SlotQtOrientationRefCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_Qt_Orientation_int```</span>
  #[repr(C)]
  pub struct RawSlotQtOrientationRefCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotQtOrientationRefCInt {
    type Arguments = (&'static ::qt::Orientation, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::Orientation,int)\0"
    }
  }
  impl RawSlotQtOrientationRefCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_Qt_Orientation_int::custom_slot(Qt::Orientation arg0, int arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt::Orientation, arg1: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_custom_slot(self as *mut ::slots::raw::RawSlotQtOrientationRefCInt, arg0 as *const ::qt::Orientation, arg1) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_Qt_Orientation_int::qt_core_c_SlotWrapper_Qt_Orientation_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRefCInt> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_Qt_Orientation_int::set(void (*FN_PTR)(void*, const Qt::Orientation*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt::Orientation, ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_set(self as *mut ::slots::raw::RawSlotQtOrientationRefCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtOrientationRefCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::Orientation, int, int)` to a Rust extern function.
  ///
  /// Use `SlotQtOrientationRefCIntCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_Qt_Orientation_int_int```</span>
  #[repr(C)]
  pub struct RawSlotQtOrientationRefCIntCInt(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotQtOrientationRefCIntCInt {
    type Arguments = (&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::Orientation,int,int)\0"
    }
  }
  impl RawSlotQtOrientationRefCIntCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_Qt_Orientation_int_int::custom_slot(Qt::Orientation arg0, int arg1, int arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt::Orientation, arg1: ::libc::c_int, arg2: ::libc::c_int) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_int_custom_slot(self as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt, arg0 as *const ::qt::Orientation, arg1, arg2) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_Qt_Orientation_int_int::qt_core_c_SlotWrapper_Qt_Orientation_int_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRefCIntCInt> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_Qt_Orientation_int_int::set(void (*FN_PTR)(void*, const Qt::Orientation*, int, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt::Orientation,
                                          ::libc::c_int,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_int_set(self as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtOrientationRefCIntCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_Qt_Orientation_int_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QString&)` to a Rust extern function.
  ///
  /// Use `SlotStringRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QString_ref```</span>
  #[repr(C)]
  pub struct RawSlotStringRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotStringRef {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QString&)\0"
    }
  }
  impl RawSlotStringRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QString_ref::custom_slot(const QString& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::string::String) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QString_ref_custom_slot(self as *mut ::slots::raw::RawSlotStringRef, arg0 as *const ::string::String) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QString_ref::qt_core_c_SlotWrapper_const_QString_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStringRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QString_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QString_ref::set(void (*FN_PTR)(void*, const QString*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::string::String),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QString_ref_set(self as *mut ::slots::raw::RawSlotStringRef,
                                                                   func,
                                                                   data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStringRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QString_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTimeLine::State)` to a Rust extern function.
  ///
  /// Use `SlotTimeLineStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_QTimeLine_State```</span>
  #[repr(C)]
  pub struct RawSlotTimeLineStateRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotTimeLineStateRef {
    type Arguments = (&'static ::time_line::State,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTimeLine::State)\0"
    }
  }
  impl RawSlotTimeLineStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_QTimeLine_State::custom_slot(QTimeLine::State arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::time_line::State) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QTimeLine_State_custom_slot(self as *mut ::slots::raw::RawSlotTimeLineStateRef, arg0 as *const ::time_line::State) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_QTimeLine_State::qt_core_c_SlotWrapper_QTimeLine_State()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTimeLineStateRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_QTimeLine_State_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_QTimeLine_State::set(void (*FN_PTR)(void*, const QTimeLine::State*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::time_line::State),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QTimeLine_State_set(self as *mut ::slots::raw::RawSlotTimeLineStateRef,
                                                                 func,
                                                                 data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTimeLineStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_QTimeLine_State_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QVariant&)` to a Rust extern function.
  ///
  /// Use `SlotVariantRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_core_c_SlotWrapper_const_QVariant_ref```</span>
  #[repr(C)]
  pub struct RawSlotVariantRef(u8);

  impl ::connection::Receiver for ::slots::raw::RawSlotVariantRef {
    type Arguments = (&'static ::variant::Variant,);
    fn object(&self) -> &::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QVariant&)\0"
    }
  }
  impl RawSlotVariantRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_core_c_SlotWrapper_const_QVariant_ref::custom_slot(const QVariant& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::variant::Variant) {
      unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QVariant_ref_custom_slot(self as *mut ::slots::raw::RawSlotVariantRef, arg0 as *const ::variant::Variant) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_core_c_SlotWrapper_const_QVariant_ref::qt_core_c_SlotWrapper_const_QVariant_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotVariantRef> {
      let ffi_result = unsafe { ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QVariant_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_core_c_SlotWrapper_const_QVariant_ref::set(void (*FN_PTR)(void*, const QVariant*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::variant::Variant),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QVariant_ref_set(self as *mut ::slots::raw::RawSlotVariantRef,
                                                                    func,
                                                                    data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotVariantRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_core_c_qt_core_c_SlotWrapper_const_QVariant_ref_delete
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotAbstractAnimationDirectionRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_Direction(self as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_Direction(self as *const ::slots::raw::RawSlotAbstractAnimationDirectionRef as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotAbstractAnimationStateRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State(self as *const ::slots::raw::RawSlotAbstractAnimationStateRef as *mut ::slots::raw::RawSlotAbstractAnimationStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
fn static_cast_mut(&mut self) -> &mut ::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State(self as *const ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotAbstractAnimationMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_ptr(self as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_ptr(self as *const ::slots::raw::RawSlotAbstractAnimationMutPtr as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotAbstractItemModelMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractItemModel_ptr(self as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractItemModel_ptr(self as *const ::slots::raw::RawSlotAbstractItemModelMutPtr as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotObjectMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QObject_ptr(self as *mut ::slots::raw::RawSlotObjectMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QObject_ptr(self as *const ::slots::raw::RawSlotObjectMutPtr as *mut ::slots::raw::RawSlotObjectMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotProcessProcessErrorRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessError(self as *mut ::slots::raw::RawSlotProcessProcessErrorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessError(self as *const ::slots::raw::RawSlotProcessProcessErrorRef as *mut ::slots::raw::RawSlotProcessProcessErrorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotProcessProcessStateRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessState(self as *mut ::slots::raw::RawSlotProcessProcessStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessState(self as *const ::slots::raw::RawSlotProcessProcessStateRef as *mut ::slots::raw::RawSlotProcessProcessStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotTimeLineStateRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QTimeLine_State(self as *mut ::slots::raw::RawSlotTimeLineStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QTimeLine_State(self as *const ::slots::raw::RawSlotTimeLineStateRef as *mut ::slots::raw::RawSlotTimeLineStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotQtOrientationRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation(self as *mut ::slots::raw::RawSlotQtOrientationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation(self as *const ::slots::raw::RawSlotQtOrientationRef as *mut ::slots::raw::RawSlotQtOrientationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotQtOrientationRefCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int(self as *mut ::slots::raw::RawSlotQtOrientationRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int(self as *const ::slots::raw::RawSlotQtOrientationRefCInt as *mut ::slots::raw::RawSlotQtOrientationRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotQtOrientationRefCIntCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int_int(self as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int_int(self as *const ::slots::raw::RawSlotQtOrientationRefCIntCInt as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotBool {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_bool(self as *mut ::slots::raw::RawSlotBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_bool(self as *const ::slots::raw::RawSlotBool as *mut ::slots::raw::RawSlotBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotItemSelectionRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref(self as *mut ::slots::raw::RawSlotItemSelectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref(self as *const ::slots::raw::RawSlotItemSelectionRef as *mut ::slots::raw::RawSlotItemSelectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotItemSelectionRefItemSelectionRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref(self as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref(self as *const ::slots::raw::RawSlotItemSelectionRefItemSelectionRef as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotListListPersistentModelIndexRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref(self as *const ::slots::raw::RawSlotListListPersistentModelIndexRef as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
fn static_cast_mut(&mut self) -> &mut ::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint(self as *const ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefModelIndexRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRefModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
fn static_cast_mut(&mut self) -> &mut ::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref(self as *const ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int(self as *mut ::slots::raw::RawSlotModelIndexRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int(self as *const ::slots::raw::RawSlotModelIndexRefCInt as *mut ::slots::raw::RawSlotModelIndexRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefCIntCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int(self as *const ::slots::raw::RawSlotModelIndexRefCIntCInt as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int(self as *const ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotStringRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QString_ref(self as *mut ::slots::raw::RawSlotStringRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QString_ref(self as *const ::slots::raw::RawSlotStringRef as *mut ::slots::raw::RawSlotStringRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotVariantRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QVariant_ref(self as *mut ::slots::raw::RawSlotVariantRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QVariant_ref(self as *const ::slots::raw::RawSlotVariantRef as *mut ::slots::raw::RawSlotVariantRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotCDouble {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_double(self as *mut ::slots::raw::RawSlotCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_double(self as *const ::slots::raw::RawSlotCDouble as *mut ::slots::raw::RawSlotCDouble) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotCInt {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int(self as *mut ::slots::raw::RawSlotCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int(self as *const ::slots::raw::RawSlotCInt as *mut ::slots::raw::RawSlotCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotCIntProcessExitStatusRef {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_QProcess_ExitStatus(self as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_QProcess_ExitStatus(self as *const ::slots::raw::RawSlotCIntProcessExitStatusRef as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotCIntI64 {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_qint64(self as *mut ::slots::raw::RawSlotCIntI64) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_qint64(self as *const ::slots::raw::RawSlotCIntI64 as *mut ::slots::raw::RawSlotCIntI64) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotNoArgs {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_no_args(self as *mut ::slots::raw::RawSlotNoArgs) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_no_args(self as *const ::slots::raw::RawSlotNoArgs as *mut ::slots::raw::RawSlotNoArgs) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::object::Object> for ::slots::raw::RawSlotI64 {
    fn static_cast_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_qint64(self as *mut ::slots::raw::RawSlotI64) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_qint64(self as *const ::slots::raw::RawSlotI64 as *mut ::slots::raw::RawSlotI64) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractAnimationDirectionRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_Direction(self as *const ::slots::raw::RawSlotAbstractAnimationDirectionRef as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractAnimationStateRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State(self as *const ::slots::raw::RawSlotAbstractAnimationStateRef as *mut ::slots::raw::RawSlotAbstractAnimationStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State(self as *const ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractAnimationMutPtr {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_ptr(self as *const ::slots::raw::RawSlotAbstractAnimationMutPtr as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractItemModelMutPtr {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractItemModel_ptr(self as *const ::slots::raw::RawSlotAbstractItemModelMutPtr as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotObjectMutPtr {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QObject_ptr(self as *const ::slots::raw::RawSlotObjectMutPtr as *mut ::slots::raw::RawSlotObjectMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotProcessProcessErrorRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessError(self as *const ::slots::raw::RawSlotProcessProcessErrorRef as *mut ::slots::raw::RawSlotProcessProcessErrorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotProcessProcessStateRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessState(self as *const ::slots::raw::RawSlotProcessProcessStateRef as *mut ::slots::raw::RawSlotProcessProcessStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTimeLineStateRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QTimeLine_State(self as *const ::slots::raw::RawSlotTimeLineStateRef as *mut ::slots::raw::RawSlotTimeLineStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtOrientationRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation(self as *const ::slots::raw::RawSlotQtOrientationRef as *mut ::slots::raw::RawSlotQtOrientationRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtOrientationRefCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int(self as *const ::slots::raw::RawSlotQtOrientationRefCInt as *mut ::slots::raw::RawSlotQtOrientationRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtOrientationRefCIntCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int_int(self as *const ::slots::raw::RawSlotQtOrientationRefCIntCInt as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBool {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_bool(self as *const ::slots::raw::RawSlotBool as *mut ::slots::raw::RawSlotBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotItemSelectionRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref(self as *const ::slots::raw::RawSlotItemSelectionRef as *mut ::slots::raw::RawSlotItemSelectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotItemSelectionRefItemSelectionRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref(self as *const ::slots::raw::RawSlotItemSelectionRefItemSelectionRef as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotListListPersistentModelIndexRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref(self as *const ::slots::raw::RawSlotListListPersistentModelIndexRef as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint(self as *const ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefModelIndexRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRefModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref(self as *const ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int(self as *const ::slots::raw::RawSlotModelIndexRefCInt as *mut ::slots::raw::RawSlotModelIndexRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefCIntCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int(self as *const ::slots::raw::RawSlotModelIndexRefCIntCInt as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref(self as *const ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int(self as *const ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStringRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QString_ref(self as *const ::slots::raw::RawSlotStringRef as *mut ::slots::raw::RawSlotStringRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotVariantRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QVariant_ref(self as *const ::slots::raw::RawSlotVariantRef as *mut ::slots::raw::RawSlotVariantRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCDouble {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_double(self as *const ::slots::raw::RawSlotCDouble as *mut ::slots::raw::RawSlotCDouble) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCInt {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int(self as *const ::slots::raw::RawSlotCInt as *mut ::slots::raw::RawSlotCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntProcessExitStatusRef {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_QProcess_ExitStatus(self as *const ::slots::raw::RawSlotCIntProcessExitStatusRef as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntI64 {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_qint64(self as *const ::slots::raw::RawSlotCIntI64 as *mut ::slots::raw::RawSlotCIntI64) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotNoArgs {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_no_args(self as *const ::slots::raw::RawSlotNoArgs as *mut ::slots::raw::RawSlotNoArgs) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotI64 {
    type Target = ::object::Object;
    fn deref(&self) -> &::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_qint64(self as *const ::slots::raw::RawSlotI64 as *mut ::slots::raw::RawSlotI64) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractAnimationDirectionRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_Direction(self as *mut ::slots::raw::RawSlotAbstractAnimationDirectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractAnimationStateRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_State_QAbstractAnimation_State(self as *mut ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractAnimationMutPtr {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractAnimation_ptr(self as *mut ::slots::raw::RawSlotAbstractAnimationMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractItemModelMutPtr {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QAbstractItemModel_ptr(self as *mut ::slots::raw::RawSlotAbstractItemModelMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotObjectMutPtr {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QObject_ptr(self as *mut ::slots::raw::RawSlotObjectMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotProcessProcessErrorRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessError(self as *mut ::slots::raw::RawSlotProcessProcessErrorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotProcessProcessStateRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QProcess_ProcessState(self as *mut ::slots::raw::RawSlotProcessProcessStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTimeLineStateRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_QTimeLine_State(self as *mut ::slots::raw::RawSlotTimeLineStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtOrientationRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation(self as *mut ::slots::raw::RawSlotQtOrientationRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtOrientationRefCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int(self as *mut ::slots::raw::RawSlotQtOrientationRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtOrientationRefCIntCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_Qt_Orientation_int_int(self as *mut ::slots::raw::RawSlotQtOrientationRefCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBool {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_bool(self as *mut ::slots::raw::RawSlotBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotItemSelectionRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref(self as *mut ::slots::raw::RawSlotItemSelectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotItemSelectionRefItemSelectionRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QItemSelection_ref_const_QItemSelection_ref(self as *mut ::slots::raw::RawSlotItemSelectionRefItemSelectionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotListListPersistentModelIndexRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
fn deref_mut(&mut self) -> &mut ::object::Object {
let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QList_QPersistentModelIndex_ref_QAbstractItemModel_LayoutChangeHint(self as *mut ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefModelIndexRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_const_QModelIndex_ref_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int(self as *mut ::slots::raw::RawSlotModelIndexRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefCIntCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QModelIndex_ref_int_int_const_QModelIndex_ref_int(self as *mut ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStringRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QString_ref(self as *mut ::slots::raw::RawSlotStringRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotVariantRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_const_QVariant_ref(self as *mut ::slots::raw::RawSlotVariantRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCDouble {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_double(self as *mut ::slots::raw::RawSlotCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCInt {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int(self as *mut ::slots::raw::RawSlotCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntProcessExitStatusRef {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_QProcess_ExitStatus(self as *mut ::slots::raw::RawSlotCIntProcessExitStatusRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntI64 {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_int_qint64(self as *mut ::slots::raw::RawSlotCIntI64) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotNoArgs {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_no_args(self as *mut ::slots::raw::RawSlotNoArgs) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotI64 {
    fn deref_mut(&mut self) -> &mut ::object::Object {
      let ffi_result = unsafe { ::ffi::qt_core_c_slots_G_static_cast_QObject_ptr_qt_core_c_SlotWrapper_qint64(self as *mut ::slots::raw::RawSlotI64) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(QAbstractAnimation::Direction)` to a Rust closure.
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

pub struct SlotAbstractAnimationDirectionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationDirectionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::abstract_animation::Direction) + 'a>>>,
}

impl<'a> SlotAbstractAnimationDirectionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::abstract_animation::Direction) + 'a>(f: F) -> SlotAbstractAnimationDirectionRef<'a> {
    let mut obj = SlotAbstractAnimationDirectionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::abstract_animation::Direction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::abstract_animation::Direction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_animation_direction_ref_callback,
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

impl<'a> Default for SlotAbstractAnimationDirectionRef<'a> {
  fn default() -> Self {
    SlotAbstractAnimationDirectionRef {
      wrapper: ::slots::raw::RawSlotAbstractAnimationDirectionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotAbstractAnimationDirectionRef<'a> {
  type Arguments = (&'static ::abstract_animation::Direction,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractAnimationDirectionRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_animation_direction_ref_callback(data: *mut ::libc::c_void,
                                                             arg0: *const ::abstract_animation::Direction) {
  let func: &mut Box<FnMut(&'static ::abstract_animation::Direction)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QAbstractAnimation*)` to a Rust closure.
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

pub struct SlotAbstractAnimationMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_animation::AbstractAnimation) + 'a>>>,
}

impl<'a> SlotAbstractAnimationMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_animation::AbstractAnimation) + 'a>(f: F) -> SlotAbstractAnimationMutPtr<'a> {
    let mut obj = SlotAbstractAnimationMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_animation::AbstractAnimation) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_animation::AbstractAnimation) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_animation_mut_ptr_callback,
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

impl<'a> Default for SlotAbstractAnimationMutPtr<'a> {
  fn default() -> Self {
    SlotAbstractAnimationMutPtr {
      wrapper: ::slots::raw::RawSlotAbstractAnimationMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotAbstractAnimationMutPtr<'a> {
  type Arguments = (*mut ::abstract_animation::AbstractAnimation,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractAnimationMutPtr as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_animation_mut_ptr_callback(data: *mut ::libc::c_void,
                                                       arg0: *mut ::abstract_animation::AbstractAnimation) {
  let func: &mut Box<FnMut(*mut ::abstract_animation::AbstractAnimation)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QAbstractAnimation::State)` to a Rust closure.
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

pub struct SlotAbstractAnimationStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::abstract_animation::State) + 'a>>>,
}

impl<'a> SlotAbstractAnimationStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::abstract_animation::State) + 'a>(f: F) -> SlotAbstractAnimationStateRef<'a> {
    let mut obj = SlotAbstractAnimationStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::abstract_animation::State) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::abstract_animation::State) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_animation_state_ref_callback,
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

impl<'a> Default for SlotAbstractAnimationStateRef<'a> {
  fn default() -> Self {
    SlotAbstractAnimationStateRef {
      wrapper: ::slots::raw::RawSlotAbstractAnimationStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotAbstractAnimationStateRef<'a> {
  type Arguments = (&'static ::abstract_animation::State,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractAnimationStateRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_animation_state_ref_callback(data: *mut ::libc::c_void,
                                                         arg0: *const ::abstract_animation::State) {
  let func: &mut Box<FnMut(&'static ::abstract_animation::State)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QAbstractAnimation::State, QAbstractAnimation::State)` to a Rust closure.
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

pub struct SlotAbstractAnimationStateRefAbstractAnimationStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::abstract_animation::State, &'static ::abstract_animation::State) + 'a>>>,
}

impl<'a> SlotAbstractAnimationStateRefAbstractAnimationStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::abstract_animation::State,
                   &'static ::abstract_animation::State) + 'a>
    (f: F)
     -> SlotAbstractAnimationStateRefAbstractAnimationStateRef<'a> {
    let mut obj = SlotAbstractAnimationStateRefAbstractAnimationStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::abstract_animation::State,
                   &'static ::abstract_animation::State) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::abstract_animation::State, &'static ::abstract_animation::State) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_animation_state_ref_abstract_animation_state_ref_callback,
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

impl<'a> Default for SlotAbstractAnimationStateRefAbstractAnimationStateRef<'a> {
  fn default() -> Self {
    SlotAbstractAnimationStateRefAbstractAnimationStateRef {
      wrapper: ::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotAbstractAnimationStateRefAbstractAnimationStateRef<'a> {
  type Arguments = (&'static ::abstract_animation::State, &'static ::abstract_animation::State);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractAnimationStateRefAbstractAnimationStateRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_animation_state_ref_abstract_animation_state_ref_callback(data: *mut ::libc::c_void, arg0: *const ::abstract_animation::State, arg1: *const ::abstract_animation::State) {
  let func: &mut Box<FnMut(&'static ::abstract_animation::State,
                           &'static ::abstract_animation::State)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QAbstractItemModel*)` to a Rust closure.
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

pub struct SlotAbstractItemModelMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractItemModelMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_item_model::AbstractItemModel) + 'a>>>,
}

impl<'a> SlotAbstractItemModelMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_item_model::AbstractItemModel) + 'a>(f: F) -> SlotAbstractItemModelMutPtr<'a> {
    let mut obj = SlotAbstractItemModelMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_item_model::AbstractItemModel) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_item_model::AbstractItemModel) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_item_model_mut_ptr_callback,
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

impl<'a> Default for SlotAbstractItemModelMutPtr<'a> {
  fn default() -> Self {
    SlotAbstractItemModelMutPtr {
      wrapper: ::slots::raw::RawSlotAbstractItemModelMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotAbstractItemModelMutPtr<'a> {
  type Arguments = (*mut ::abstract_item_model::AbstractItemModel,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractItemModelMutPtr as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_item_model_mut_ptr_callback(data: *mut ::libc::c_void,
                                                        arg0: *mut ::abstract_item_model::AbstractItemModel) {
  let func: &mut Box<FnMut(*mut ::abstract_item_model::AbstractItemModel)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(bool)` to a Rust closure.
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

pub struct SlotBool<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBool>,
  func: ::std::option::Option<Box<Box<FnMut(bool) + 'a>>>,
}

impl<'a> SlotBool<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(bool) + 'a>(f: F) -> SlotBool<'a> {
    let mut obj = SlotBool::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(bool) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(bool) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_bool_callback, ::std::mem::transmute(func_box.as_mut()));
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

impl<'a> Default for SlotBool<'a> {
  fn default() -> Self {
    SlotBool {
      wrapper: ::slots::raw::RawSlotBool::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotBool<'a> {
  type Arguments = (bool,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBool as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_bool_callback(data: *mut ::libc::c_void, arg0: bool) {
  let func: &mut Box<FnMut(bool)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(double)` to a Rust closure.
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

pub struct SlotCDouble<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCDouble>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_double) + 'a>>>,
}

impl<'a> SlotCDouble<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_double) + 'a>(f: F) -> SlotCDouble<'a> {
    let mut obj = SlotCDouble::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_double) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_double) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_double_callback,
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

impl<'a> Default for SlotCDouble<'a> {
  fn default() -> Self {
    SlotCDouble {
      wrapper: ::slots::raw::RawSlotCDouble::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotCDouble<'a> {
  type Arguments = (::libc::c_double,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCDouble as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_double_callback(data: *mut ::libc::c_void, arg0: ::libc::c_double) {
  let func: &mut Box<FnMut(::libc::c_double)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(int)` to a Rust closure.
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

pub struct SlotCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCInt>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int) + 'a>>>,
}

impl<'a> SlotCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int) + 'a>(f: F) -> SlotCInt<'a> {
    let mut obj = SlotCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_callback,
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

impl<'a> Default for SlotCInt<'a> {
  fn default() -> Self {
    SlotCInt {
      wrapper: ::slots::raw::RawSlotCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotCInt<'a> {
  type Arguments = (::libc::c_int,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_callback(data: *mut ::libc::c_void, arg0: ::libc::c_int) {
  let func: &mut Box<FnMut(::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(int, qint64)` to a Rust closure.
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

pub struct SlotCIntI64<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntI64>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, i64) + 'a>>>,
}

impl<'a> SlotCIntI64<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, i64) + 'a>(f: F) -> SlotCIntI64<'a> {
    let mut obj = SlotCIntI64::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, i64) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, i64) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_i64_callback,
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

impl<'a> Default for SlotCIntI64<'a> {
  fn default() -> Self {
    SlotCIntI64 {
      wrapper: ::slots::raw::RawSlotCIntI64::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotCIntI64<'a> {
  type Arguments = (::libc::c_int, i64);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntI64 as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_i64_callback(data: *mut ::libc::c_void, arg0: ::libc::c_int, arg1: i64) {
  let func: &mut Box<FnMut(::libc::c_int, i64)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(int, QProcess::ExitStatus)` to a Rust closure.
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

pub struct SlotCIntProcessExitStatusRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntProcessExitStatusRef>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, &'static ::process::ExitStatus) + 'a>>>,
}

impl<'a> SlotCIntProcessExitStatusRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, &'static ::process::ExitStatus) + 'a>(f: F) -> SlotCIntProcessExitStatusRef<'a> {
    let mut obj = SlotCIntProcessExitStatusRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, &'static ::process::ExitStatus) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, &'static ::process::ExitStatus) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_process_exit_status_ref_callback,
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

impl<'a> Default for SlotCIntProcessExitStatusRef<'a> {
  fn default() -> Self {
    SlotCIntProcessExitStatusRef {
      wrapper: ::slots::raw::RawSlotCIntProcessExitStatusRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotCIntProcessExitStatusRef<'a> {
  type Arguments = (::libc::c_int, &'static ::process::ExitStatus);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntProcessExitStatusRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_process_exit_status_ref_callback(data: *mut ::libc::c_void,
                                                          arg0: ::libc::c_int,
                                                          arg1: *const ::process::ExitStatus) {
  let func: &mut Box<FnMut(::libc::c_int, &'static ::process::ExitStatus)> = unsafe { ::std::mem::transmute(data) };
  func(arg0,
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(qint64)` to a Rust closure.
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

pub struct SlotI64<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotI64>,
  func: ::std::option::Option<Box<Box<FnMut(i64) + 'a>>>,
}

impl<'a> SlotI64<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(i64) + 'a>(f: F) -> SlotI64<'a> {
    let mut obj = SlotI64::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(i64) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(i64) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_i64_callback, ::std::mem::transmute(func_box.as_mut()));
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

impl<'a> Default for SlotI64<'a> {
  fn default() -> Self {
    SlotI64 {
      wrapper: ::slots::raw::RawSlotI64::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotI64<'a> {
  type Arguments = (i64,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotI64 as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_i64_callback(data: *mut ::libc::c_void, arg0: i64) {
  let func: &mut Box<FnMut(i64)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QItemSelection&)` to a Rust closure.
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

pub struct SlotItemSelectionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotItemSelectionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::item_selection::ItemSelection) + 'a>>>,
}

impl<'a> SlotItemSelectionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::item_selection::ItemSelection) + 'a>(f: F) -> SlotItemSelectionRef<'a> {
    let mut obj = SlotItemSelectionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::item_selection::ItemSelection) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::item_selection::ItemSelection) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_item_selection_ref_callback,
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

impl<'a> Default for SlotItemSelectionRef<'a> {
  fn default() -> Self {
    SlotItemSelectionRef {
      wrapper: ::slots::raw::RawSlotItemSelectionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotItemSelectionRef<'a> {
  type Arguments = (&'static ::item_selection::ItemSelection,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotItemSelectionRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_item_selection_ref_callback(data: *mut ::libc::c_void,
                                               arg0: *const ::item_selection::ItemSelection) {
  let func: &mut Box<FnMut(&'static ::item_selection::ItemSelection)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QItemSelection&, const QItemSelection&)` to a Rust closure.
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

pub struct SlotItemSelectionRefItemSelectionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotItemSelectionRefItemSelectionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::item_selection::ItemSelection, &'static ::item_selection::ItemSelection) + 'a>>>,
}

impl<'a> SlotItemSelectionRefItemSelectionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::item_selection::ItemSelection,
                   &'static ::item_selection::ItemSelection) + 'a>
    (f: F)
     -> SlotItemSelectionRefItemSelectionRef<'a> {
    let mut obj = SlotItemSelectionRefItemSelectionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::item_selection::ItemSelection,
                   &'static ::item_selection::ItemSelection) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::item_selection::ItemSelection, &'static ::item_selection::ItemSelection) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_item_selection_ref_item_selection_ref_callback,
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

impl<'a> Default for SlotItemSelectionRefItemSelectionRef<'a> {
  fn default() -> Self {
    SlotItemSelectionRefItemSelectionRef {
      wrapper: ::slots::raw::RawSlotItemSelectionRefItemSelectionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotItemSelectionRefItemSelectionRef<'a> {
  type Arguments = (&'static ::item_selection::ItemSelection, &'static ::item_selection::ItemSelection);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotItemSelectionRefItemSelectionRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_item_selection_ref_item_selection_ref_callback(data: *mut ::libc::c_void,
                                                                  arg0: *const ::item_selection::ItemSelection,
                                                                  arg1: *const ::item_selection::ItemSelection) {
  let func: &mut Box<FnMut(&'static ::item_selection::ItemSelection,
                           &'static ::item_selection::ItemSelection)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QList<QPersistentModelIndex>&)` to a Rust closure.
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

pub struct SlotListListPersistentModelIndexRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotListListPersistentModelIndexRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::list::ListPersistentModelIndex) + 'a>>>,
}

impl<'a> SlotListListPersistentModelIndexRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::list::ListPersistentModelIndex) + 'a>(f: F)
                                                                       -> SlotListListPersistentModelIndexRef<'a> {
    let mut obj = SlotListListPersistentModelIndexRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::list::ListPersistentModelIndex) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::list::ListPersistentModelIndex) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_list_list_persistent_model_index_ref_callback,
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

impl<'a> Default for SlotListListPersistentModelIndexRef<'a> {
  fn default() -> Self {
    SlotListListPersistentModelIndexRef {
      wrapper: ::slots::raw::RawSlotListListPersistentModelIndexRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotListListPersistentModelIndexRef<'a> {
  type Arguments = (&'static ::list::ListPersistentModelIndex,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotListListPersistentModelIndexRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_list_list_persistent_model_index_ref_callback(data: *mut ::libc::c_void,
                                                                 arg0: *const ::list::ListPersistentModelIndex) {
  let func: &mut Box<FnMut(&'static ::list::ListPersistentModelIndex)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QList<QPersistentModelIndex>&, QAbstractItemModel::LayoutChangeHint)` to a Rust closure.
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

pub struct SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint) + 'a>>>,
}

impl<'a> SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::list::ListPersistentModelIndex,
                   &'static ::abstract_item_model::LayoutChangeHint) + 'a>
    (f: F)
     -> SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef<'a> {
    let mut obj = SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::list::ListPersistentModelIndex,
                   &'static ::abstract_item_model::LayoutChangeHint) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_list_list_persistent_model_index_ref_abstract_item_model_layout_change_hint_ref_callback,
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

impl<'a> Default for SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef<'a> {
  fn default() -> Self {
    SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef {
      wrapper: ::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef<'a> {
  type Arguments = (&'static ::list::ListPersistentModelIndex, &'static ::abstract_item_model::LayoutChangeHint);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotListListPersistentModelIndexRefAbstractItemModelLayoutChangeHintRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_list_list_persistent_model_index_ref_abstract_item_model_layout_change_hint_ref_callback(data: *mut ::libc::c_void, arg0: *const ::list::ListPersistentModelIndex, arg1: *const ::abstract_item_model::LayoutChangeHint) {
  let func: &mut Box<FnMut(&'static ::list::ListPersistentModelIndex,
                           &'static ::abstract_item_model::LayoutChangeHint)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&)` to a Rust closure.
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

pub struct SlotModelIndexRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex) + 'a>>>,
}

impl<'a> SlotModelIndexRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex) + 'a>(f: F) -> SlotModelIndexRef<'a> {
    let mut obj = SlotModelIndexRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_callback,
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

impl<'a> Default for SlotModelIndexRef<'a> {
  fn default() -> Self {
    SlotModelIndexRef {
      wrapper: ::slots::raw::RawSlotModelIndexRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRef<'a> {
  type Arguments = (&'static ::model_index::ModelIndex,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_callback(data: *mut ::libc::c_void, arg0: *const ::model_index::ModelIndex) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, int)` to a Rust closure.
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

pub struct SlotModelIndexRefCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotModelIndexRefCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex, ::libc::c_int) + 'a>(f: F) -> SlotModelIndexRefCInt<'a> {
    let mut obj = SlotModelIndexRefCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_c_int_callback,
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

impl<'a> Default for SlotModelIndexRefCInt<'a> {
  fn default() -> Self {
    SlotModelIndexRefCInt {
      wrapper: ::slots::raw::RawSlotModelIndexRefCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefCInt<'a> {
  type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_c_int_callback(data: *mut ::libc::c_void,
                                                  arg0: *const ::model_index::ModelIndex,
                                                  arg1: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1);
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int)` to a Rust closure.
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

pub struct SlotModelIndexRefCIntCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotModelIndexRefCIntCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int) + 'a>
    (f: F)
     -> SlotModelIndexRefCIntCInt<'a> {
    let mut obj = SlotModelIndexRefCIntCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_c_int_c_int_callback,
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

impl<'a> Default for SlotModelIndexRefCIntCInt<'a> {
  fn default() -> Self {
    SlotModelIndexRefCIntCInt {
      wrapper: ::slots::raw::RawSlotModelIndexRefCIntCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefCIntCInt<'a> {
  type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefCIntCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_c_int_c_int_callback(data: *mut ::libc::c_void,
                                                        arg0: *const ::model_index::ModelIndex,
                                                        arg1: ::libc::c_int,
                                                        arg2: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex,
                           ::libc::c_int,
                           ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1,
       arg2);
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int, const QModelIndex&)` to a Rust closure.
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

pub struct SlotModelIndexRefCIntCIntModelIndexRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int, &'static ::model_index::ModelIndex) + 'a>>>,
}

impl<'a> SlotModelIndexRefCIntCIntModelIndexRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int,
                   &'static ::model_index::ModelIndex) + 'a>
    (f: F)
     -> SlotModelIndexRefCIntCIntModelIndexRef<'a> {
    let mut obj = SlotModelIndexRefCIntCIntModelIndexRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int,
                   &'static ::model_index::ModelIndex) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int, &'static ::model_index::ModelIndex) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_c_int_c_int_model_index_ref_callback,
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

impl<'a> Default for SlotModelIndexRefCIntCIntModelIndexRef<'a> {
  fn default() -> Self {
    SlotModelIndexRefCIntCIntModelIndexRef {
      wrapper: ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefCIntCIntModelIndexRef<'a> {
  type Arguments = (&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int, &'static ::model_index::ModelIndex);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_c_int_c_int_model_index_ref_callback(data: *mut ::libc::c_void,
                                                                        arg0: *const ::model_index::ModelIndex,
                                                                        arg1: ::libc::c_int,
                                                                        arg2: ::libc::c_int,
                                                                        arg3: *const ::model_index::ModelIndex) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex,
                           ::libc::c_int,
                           ::libc::c_int,
                           &'static ::model_index::ModelIndex)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1,
       arg2,
       unsafe { arg3.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, int, int, const QModelIndex&, int)` to a Rust closure.
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

pub struct SlotModelIndexRefCIntCIntModelIndexRefCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int, &'static ::model_index::ModelIndex, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotModelIndexRefCIntCIntModelIndexRefCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int,
                   &'static ::model_index::ModelIndex,
                   ::libc::c_int) + 'a>
    (f: F)
     -> SlotModelIndexRefCIntCIntModelIndexRefCInt<'a> {
    let mut obj = SlotModelIndexRefCIntCIntModelIndexRefCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex,
                   ::libc::c_int,
                   ::libc::c_int,
                   &'static ::model_index::ModelIndex,
                   ::libc::c_int) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, ::libc::c_int, ::libc::c_int, &'static ::model_index::ModelIndex, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_c_int_c_int_model_index_ref_c_int_callback,
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

impl<'a> Default for SlotModelIndexRefCIntCIntModelIndexRefCInt<'a> {
  fn default() -> Self {
    SlotModelIndexRefCIntCIntModelIndexRefCInt {
      wrapper: ::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefCIntCIntModelIndexRefCInt<'a> {
  type Arguments = (&'static ::model_index::ModelIndex,
   ::libc::c_int,
   ::libc::c_int,
   &'static ::model_index::ModelIndex,
   ::libc::c_int);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefCIntCIntModelIndexRefCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_c_int_c_int_model_index_ref_c_int_callback(data: *mut ::libc::c_void, arg0: *const ::model_index::ModelIndex, arg1: ::libc::c_int, arg2: ::libc::c_int, arg3: *const ::model_index::ModelIndex, arg4: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex,
                           ::libc::c_int,
                           ::libc::c_int,
                           &'static ::model_index::ModelIndex,
                           ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1,
       arg2,
       unsafe { arg3.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg4);
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, const QModelIndex&)` to a Rust closure.
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

pub struct SlotModelIndexRefModelIndexRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefModelIndexRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex) + 'a>>>,
}

impl<'a> SlotModelIndexRefModelIndexRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex,
                   &'static ::model_index::ModelIndex) + 'a>
    (f: F)
     -> SlotModelIndexRefModelIndexRef<'a> {
    let mut obj = SlotModelIndexRefModelIndexRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex,
                   &'static ::model_index::ModelIndex) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_model_index_ref_callback,
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

impl<'a> Default for SlotModelIndexRefModelIndexRef<'a> {
  fn default() -> Self {
    SlotModelIndexRefModelIndexRef {
      wrapper: ::slots::raw::RawSlotModelIndexRefModelIndexRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefModelIndexRef<'a> {
  type Arguments = (&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefModelIndexRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_model_index_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: *const ::model_index::ModelIndex,
                                                            arg1: *const ::model_index::ModelIndex) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex,
                           &'static ::model_index::ModelIndex)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QModelIndex&, const QModelIndex&, const QVector<int>&)` to a Rust closure.
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

pub struct SlotModelIndexRefModelIndexRefVectorVectorCIntRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex, &'static ::vector::VectorCInt) + 'a>>>,
}

impl<'a> SlotModelIndexRefModelIndexRefVectorVectorCIntRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::model_index::ModelIndex,
                   &'static ::model_index::ModelIndex,
                   &'static ::vector::VectorCInt) + 'a>
    (f: F)
     -> SlotModelIndexRefModelIndexRefVectorVectorCIntRef<'a> {
    let mut obj = SlotModelIndexRefModelIndexRefVectorVectorCIntRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::model_index::ModelIndex,
                   &'static ::model_index::ModelIndex,
                   &'static ::vector::VectorCInt) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::model_index::ModelIndex, &'static ::model_index::ModelIndex, &'static ::vector::VectorCInt) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_model_index_ref_model_index_ref_vector_vector_c_int_ref_callback,
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

impl<'a> Default for SlotModelIndexRefModelIndexRefVectorVectorCIntRef<'a> {
  fn default() -> Self {
    SlotModelIndexRefModelIndexRefVectorVectorCIntRef {
      wrapper: ::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotModelIndexRefModelIndexRefVectorVectorCIntRef<'a> {
  type Arguments = (&'static ::model_index::ModelIndex,
   &'static ::model_index::ModelIndex,
   &'static ::vector::VectorCInt);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotModelIndexRefModelIndexRefVectorVectorCIntRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_model_index_ref_model_index_ref_vector_vector_c_int_ref_callback(data: *mut ::libc::c_void, arg0: *const ::model_index::ModelIndex, arg1: *const ::model_index::ModelIndex, arg2: *const ::vector::VectorCInt) {
  let func: &mut Box<FnMut(&'static ::model_index::ModelIndex,
                           &'static ::model_index::ModelIndex,
                           &'static ::vector::VectorCInt)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg2.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `()` to a Rust closure.
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

pub struct SlotNoArgs<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotNoArgs>,
  func: ::std::option::Option<Box<Box<FnMut() + 'a>>>,
}

impl<'a> SlotNoArgs<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut() + 'a>(f: F) -> SlotNoArgs<'a> {
    let mut obj = SlotNoArgs::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut() + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut() + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_no_args_callback,
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

impl<'a> Default for SlotNoArgs<'a> {
  fn default() -> Self {
    SlotNoArgs {
      wrapper: ::slots::raw::RawSlotNoArgs::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotNoArgs<'a> {
  type Arguments = ();
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotNoArgs as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_no_args_callback(data: *mut ::libc::c_void) {
  let func: &mut Box<FnMut()> = unsafe { ::std::mem::transmute(data) };
  func();
}
/// Allows to bind Qt signals with arguments `(QObject*)` to a Rust closure.
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

pub struct SlotObjectMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotObjectMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::object::Object) + 'a>>>,
}

impl<'a> SlotObjectMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::object::Object) + 'a>(f: F) -> SlotObjectMutPtr<'a> {
    let mut obj = SlotObjectMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::object::Object) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::object::Object) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_object_mut_ptr_callback,
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

impl<'a> Default for SlotObjectMutPtr<'a> {
  fn default() -> Self {
    SlotObjectMutPtr {
      wrapper: ::slots::raw::RawSlotObjectMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotObjectMutPtr<'a> {
  type Arguments = (*mut ::object::Object,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotObjectMutPtr as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_object_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::object::Object) {
  let func: &mut Box<FnMut(*mut ::object::Object)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QProcess::ProcessError)` to a Rust closure.
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

pub struct SlotProcessProcessErrorRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotProcessProcessErrorRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::process::ProcessError) + 'a>>>,
}

impl<'a> SlotProcessProcessErrorRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::process::ProcessError) + 'a>(f: F) -> SlotProcessProcessErrorRef<'a> {
    let mut obj = SlotProcessProcessErrorRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::process::ProcessError) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::process::ProcessError) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_process_process_error_ref_callback,
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

impl<'a> Default for SlotProcessProcessErrorRef<'a> {
  fn default() -> Self {
    SlotProcessProcessErrorRef {
      wrapper: ::slots::raw::RawSlotProcessProcessErrorRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotProcessProcessErrorRef<'a> {
  type Arguments = (&'static ::process::ProcessError,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotProcessProcessErrorRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_process_process_error_ref_callback(data: *mut ::libc::c_void,
                                                      arg0: *const ::process::ProcessError) {
  let func: &mut Box<FnMut(&'static ::process::ProcessError)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QProcess::ProcessState)` to a Rust closure.
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

pub struct SlotProcessProcessStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotProcessProcessStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::process::ProcessState) + 'a>>>,
}

impl<'a> SlotProcessProcessStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::process::ProcessState) + 'a>(f: F) -> SlotProcessProcessStateRef<'a> {
    let mut obj = SlotProcessProcessStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::process::ProcessState) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::process::ProcessState) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_process_process_state_ref_callback,
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

impl<'a> Default for SlotProcessProcessStateRef<'a> {
  fn default() -> Self {
    SlotProcessProcessStateRef {
      wrapper: ::slots::raw::RawSlotProcessProcessStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotProcessProcessStateRef<'a> {
  type Arguments = (&'static ::process::ProcessState,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotProcessProcessStateRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_process_process_state_ref_callback(data: *mut ::libc::c_void,
                                                      arg0: *const ::process::ProcessState) {
  let func: &mut Box<FnMut(&'static ::process::ProcessState)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::Orientation)` to a Rust closure.
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

pub struct SlotQtOrientationRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt::Orientation) + 'a>>>,
}

impl<'a> SlotQtOrientationRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt::Orientation) + 'a>(f: F) -> SlotQtOrientationRef<'a> {
    let mut obj = SlotQtOrientationRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt::Orientation) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt::Orientation) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_orientation_ref_callback,
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

impl<'a> Default for SlotQtOrientationRef<'a> {
  fn default() -> Self {
    SlotQtOrientationRef {
      wrapper: ::slots::raw::RawSlotQtOrientationRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotQtOrientationRef<'a> {
  type Arguments = (&'static ::qt::Orientation,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtOrientationRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_orientation_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt::Orientation) {
  let func: &mut Box<FnMut(&'static ::qt::Orientation)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::Orientation, int)` to a Rust closure.
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

pub struct SlotQtOrientationRefCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRefCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt::Orientation, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotQtOrientationRefCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt::Orientation, ::libc::c_int) + 'a>(f: F) -> SlotQtOrientationRefCInt<'a> {
    let mut obj = SlotQtOrientationRefCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt::Orientation, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt::Orientation, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_orientation_ref_c_int_callback,
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

impl<'a> Default for SlotQtOrientationRefCInt<'a> {
  fn default() -> Self {
    SlotQtOrientationRefCInt {
      wrapper: ::slots::raw::RawSlotQtOrientationRefCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotQtOrientationRefCInt<'a> {
  type Arguments = (&'static ::qt::Orientation, ::libc::c_int);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtOrientationRefCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_orientation_ref_c_int_callback(data: *mut ::libc::c_void,
                                                     arg0: *const ::qt::Orientation,
                                                     arg1: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::qt::Orientation, ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1);
}
/// Allows to bind Qt signals with arguments `(Qt::Orientation, int, int)` to a Rust closure.
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

pub struct SlotQtOrientationRefCIntCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtOrientationRefCIntCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotQtOrientationRefCIntCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int) + 'a>
    (f: F)
     -> SlotQtOrientationRefCIntCInt<'a> {
    let mut obj = SlotQtOrientationRefCIntCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_orientation_ref_c_int_c_int_callback,
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

impl<'a> Default for SlotQtOrientationRefCIntCInt<'a> {
  fn default() -> Self {
    SlotQtOrientationRefCIntCInt {
      wrapper: ::slots::raw::RawSlotQtOrientationRefCIntCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotQtOrientationRefCIntCInt<'a> {
  type Arguments = (&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtOrientationRefCIntCInt as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_orientation_ref_c_int_c_int_callback(data: *mut ::libc::c_void,
                                                           arg0: *const ::qt::Orientation,
                                                           arg1: ::libc::c_int,
                                                           arg2: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::qt::Orientation, ::libc::c_int, ::libc::c_int)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1,
       arg2);
}
/// Allows to bind Qt signals with arguments `(const QString&)` to a Rust closure.
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

pub struct SlotStringRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStringRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::string::String) + 'a>>>,
}

impl<'a> SlotStringRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::string::String) + 'a>(f: F) -> SlotStringRef<'a> {
    let mut obj = SlotStringRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::string::String) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::string::String) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_string_ref_callback,
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

impl<'a> Default for SlotStringRef<'a> {
  fn default() -> Self {
    SlotStringRef {
      wrapper: ::slots::raw::RawSlotStringRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotStringRef<'a> {
  type Arguments = (&'static ::string::String,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStringRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_string_ref_callback(data: *mut ::libc::c_void, arg0: *const ::string::String) {
  let func: &mut Box<FnMut(&'static ::string::String)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QTimeLine::State)` to a Rust closure.
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

pub struct SlotTimeLineStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTimeLineStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::time_line::State) + 'a>>>,
}

impl<'a> SlotTimeLineStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::time_line::State) + 'a>(f: F) -> SlotTimeLineStateRef<'a> {
    let mut obj = SlotTimeLineStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::time_line::State) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::time_line::State) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_time_line_state_ref_callback,
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

impl<'a> Default for SlotTimeLineStateRef<'a> {
  fn default() -> Self {
    SlotTimeLineStateRef {
      wrapper: ::slots::raw::RawSlotTimeLineStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotTimeLineStateRef<'a> {
  type Arguments = (&'static ::time_line::State,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTimeLineStateRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_time_line_state_ref_callback(data: *mut ::libc::c_void, arg0: *const ::time_line::State) {
  let func: &mut Box<FnMut(&'static ::time_line::State)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QVariant&)` to a Rust closure.
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

pub struct SlotVariantRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotVariantRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::variant::Variant) + 'a>>>,
}

impl<'a> SlotVariantRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::variant::Variant) + 'a>(f: F) -> SlotVariantRef<'a> {
    let mut obj = SlotVariantRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::variant::Variant) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::variant::Variant) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_variant_ref_callback,
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

impl<'a> Default for SlotVariantRef<'a> {
  fn default() -> Self {
    SlotVariantRef {
      wrapper: ::slots::raw::RawSlotVariantRef::new(),
      func: None,
    }
  }
}

impl<'a> ::connection::Receiver for SlotVariantRef<'a> {
  type Arguments = (&'static ::variant::Variant,);
  fn object(&self) -> &::object::Object {
    ::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotVariantRef as ::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_variant_ref_callback(data: *mut ::libc::c_void, arg0: *const ::variant::Variant) {
  let func: &mut Box<FnMut(&'static ::variant::Variant)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
