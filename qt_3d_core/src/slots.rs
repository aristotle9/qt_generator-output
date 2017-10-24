/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(float)` to a Rust extern function.
  ///
  /// Use `SlotCFloat` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_core_c_SlotWrapper_float```</span>
  #[repr(C)]
  pub struct RawSlotCFloat(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCFloat {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(float)\0"
    }
  }
  impl RawSlotCFloat {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_core_c_SlotWrapper_float::custom_slot(float arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_float) {
      unsafe {
        ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_float_custom_slot(self as *mut ::slots::raw::RawSlotCFloat, arg0)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_core_c_SlotWrapper_float::qt_3d_core_c_SlotWrapper_float()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCFloat> {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_float_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_core_c_SlotWrapper_float::set(void (*FN_PTR)(void*, float) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::libc::c_float),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_float_set(self as *mut ::slots::raw::RawSlotCFloat, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCFloat {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_float_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DCore::QEntity*)` to a Rust extern function.
  ///
  /// Use `SlotEntityMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr```</span>
  #[repr(C)]
  pub struct RawSlotEntityMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotEntityMutPtr {
    type Arguments = (*mut ::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DCore::QEntity*)\0"
    }
  }
  impl RawSlotEntityMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr::custom_slot(Qt3DCore::QEntity* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::entity::Entity) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr_custom_slot(self as *mut ::slots::raw::RawSlotEntityMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr::qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotEntityMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr::set(void (*FN_PTR)(void*, Qt3DCore::QEntity*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::entity::Entity),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr_set(self as *mut ::slots::raw::RawSlotEntityMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotEntityMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DCore::QNode::PropertyTrackingMode)` to a Rust extern function.
  ///
  /// Use `SlotNodePropertyTrackingMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode```</span>
  #[repr(C)]
  pub struct RawSlotNodePropertyTrackingMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotNodePropertyTrackingMode {
    type Arguments = (::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl RawSlotNodePropertyTrackingMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode::custom_slot(Qt3DCore::QNode::PropertyTrackingMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::node::PropertyTrackingMode) {
      unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode_custom_slot(self as *mut ::slots::raw::RawSlotNodePropertyTrackingMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode::qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotNodePropertyTrackingMode> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode::set(void (*FN_PTR)(void*, Qt3DCore::QNode::PropertyTrackingMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::node::PropertyTrackingMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode_set(self as *mut ::slots::raw::RawSlotNodePropertyTrackingMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotNodePropertyTrackingMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QQuaternion&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiQuaternionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_core_c_SlotWrapper_const_QQuaternion_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiQuaternionRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiQuaternionRef {
    type Arguments = (&'static ::qt_gui::quaternion::Quaternion,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QQuaternion&)\0"
    }
  }
  impl RawSlotQtGuiQuaternionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_core_c_SlotWrapper_const_QQuaternion_ref::custom_slot(const QQuaternion& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::quaternion::Quaternion) {
      unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiQuaternionRef, arg0 as *const ::qt_gui::quaternion::Quaternion) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_core_c_SlotWrapper_const_QQuaternion_ref::qt_3d_core_c_SlotWrapper_const_QQuaternion_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiQuaternionRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_core_c_SlotWrapper_const_QQuaternion_ref::set(void (*FN_PTR)(void*, const QQuaternion*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::quaternion::Quaternion),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref_set(self as *mut ::slots::raw::RawSlotQtGuiQuaternionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiQuaternionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QVector3D&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiVector3DRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_core_c_SlotWrapper_const_QVector3D_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiVector3DRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiVector3DRef {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QVector3D&)\0"
    }
  }
  impl RawSlotQtGuiVector3DRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_core_c_SlotWrapper_const_QVector3D_ref::custom_slot(const QVector3D& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::vector_3d::Vector3D) {
      unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QVector3D_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef, arg0 as *const ::qt_gui::vector_3d::Vector3D) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_core_c_SlotWrapper_const_QVector3D_ref::qt_3d_core_c_SlotWrapper_const_QVector3D_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVector3DRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QVector3D_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_core_c_SlotWrapper_const_QVector3D_ref::set(void (*FN_PTR)(void*, const QVector3D*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::vector_3d::Vector3D),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QVector3D_ref_set(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_core_c_qt_3d_core_c_SlotWrapper_const_QVector3D_ref_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotEntityMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr(self as *mut ::slots::raw::RawSlotEntityMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr(self as *const ::slots::raw::RawSlotEntityMutPtr as *mut ::slots::raw::RawSlotEntityMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotNodePropertyTrackingMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode(self as *mut ::slots::raw::RawSlotNodePropertyTrackingMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode(self as *const ::slots::raw::RawSlotNodePropertyTrackingMode as *mut ::slots::raw::RawSlotNodePropertyTrackingMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiQuaternionRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref(self as *mut ::slots::raw::RawSlotQtGuiQuaternionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref(self as *const ::slots::raw::RawSlotQtGuiQuaternionRef as *mut ::slots::raw::RawSlotQtGuiQuaternionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QVector3D_ref(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QVector3D_ref(self as *const ::slots::raw::RawSlotQtGuiVector3DRef as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCFloat {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_float(self as *mut ::slots::raw::RawSlotCFloat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_float(self as *const ::slots::raw::RawSlotCFloat as *mut ::slots::raw::RawSlotCFloat) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotEntityMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr(self as *const ::slots::raw::RawSlotEntityMutPtr as *mut ::slots::raw::RawSlotEntityMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotNodePropertyTrackingMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode(self as *const ::slots::raw::RawSlotNodePropertyTrackingMode as *mut ::slots::raw::RawSlotNodePropertyTrackingMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiQuaternionRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref(self as *const ::slots::raw::RawSlotQtGuiQuaternionRef as *mut ::slots::raw::RawSlotQtGuiQuaternionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiVector3DRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QVector3D_ref(self as *const ::slots::raw::RawSlotQtGuiVector3DRef as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCFloat {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_float(self as *const ::slots::raw::RawSlotCFloat as *mut ::slots::raw::RawSlotCFloat) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotEntityMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QEntity_ptr(self as *mut ::slots::raw::RawSlotEntityMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotNodePropertyTrackingMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_Qt3DCore_QNode_PropertyTrackingMode(self as *mut ::slots::raw::RawSlotNodePropertyTrackingMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiQuaternionRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QQuaternion_ref(self as *mut ::slots::raw::RawSlotQtGuiQuaternionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_const_QVector3D_ref(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCFloat {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_core_c_slots_G_static_cast_QObject_ptr_qt_3d_core_c_SlotWrapper_float(self as *mut ::slots::raw::RawSlotCFloat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(float)` to a Rust closure.
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

pub struct SlotCFloat<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCFloat>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_float) + 'a>>>,
}

impl<'a> SlotCFloat<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_float) + 'a>(f: F) -> SlotCFloat<'a> {
    let mut obj = SlotCFloat::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_float) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_float) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_float_callback,
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

impl<'a> Default for SlotCFloat<'a> {
  fn default() -> Self {
    SlotCFloat {
      wrapper: ::slots::raw::RawSlotCFloat::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCFloat<'a> {
  type Arguments = (::libc::c_float,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCFloat as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_float_callback(data: *mut ::libc::c_void, arg0: ::libc::c_float) {
  let func: &mut Box<FnMut(::libc::c_float)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DCore::QEntity*)` to a Rust closure.
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

pub struct SlotEntityMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotEntityMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::entity::Entity) + 'a>>>,
}

impl<'a> SlotEntityMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::entity::Entity) + 'a>(f: F) -> SlotEntityMutPtr<'a> {
    let mut obj = SlotEntityMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::entity::Entity) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::entity::Entity) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_entity_mut_ptr_callback,
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

impl<'a> Default for SlotEntityMutPtr<'a> {
  fn default() -> Self {
    SlotEntityMutPtr {
      wrapper: ::slots::raw::RawSlotEntityMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotEntityMutPtr<'a> {
  type Arguments = (*mut ::entity::Entity,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotEntityMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_entity_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::entity::Entity) {
  let func: &mut Box<FnMut(*mut ::entity::Entity)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DCore::QNode::PropertyTrackingMode)` to a Rust closure.
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

pub struct SlotNodePropertyTrackingMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotNodePropertyTrackingMode>,
  func: ::std::option::Option<Box<Box<FnMut(::node::PropertyTrackingMode) + 'a>>>,
}

impl<'a> SlotNodePropertyTrackingMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::node::PropertyTrackingMode) + 'a>(f: F) -> SlotNodePropertyTrackingMode<'a> {
    let mut obj = SlotNodePropertyTrackingMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::node::PropertyTrackingMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::node::PropertyTrackingMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_node_property_tracking_mode_callback,
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

impl<'a> Default for SlotNodePropertyTrackingMode<'a> {
  fn default() -> Self {
    SlotNodePropertyTrackingMode {
      wrapper: ::slots::raw::RawSlotNodePropertyTrackingMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotNodePropertyTrackingMode<'a> {
  type Arguments = (::node::PropertyTrackingMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotNodePropertyTrackingMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_node_property_tracking_mode_callback(data: *mut ::libc::c_void,
                                                        arg0: ::node::PropertyTrackingMode) {
  let func: &mut Box<FnMut(::node::PropertyTrackingMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QQuaternion&)` to a Rust closure.
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

pub struct SlotQtGuiQuaternionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiQuaternionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::quaternion::Quaternion) + 'a>>>,
}

impl<'a> SlotQtGuiQuaternionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::quaternion::Quaternion) + 'a>(f: F) -> SlotQtGuiQuaternionRef<'a> {
    let mut obj = SlotQtGuiQuaternionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::quaternion::Quaternion) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::quaternion::Quaternion) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_quaternion_ref_callback,
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

impl<'a> Default for SlotQtGuiQuaternionRef<'a> {
  fn default() -> Self {
    SlotQtGuiQuaternionRef {
      wrapper: ::slots::raw::RawSlotQtGuiQuaternionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiQuaternionRef<'a> {
  type Arguments = (&'static ::qt_gui::quaternion::Quaternion,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiQuaternionRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_quaternion_ref_callback(data: *mut ::libc::c_void,
                                                  arg0: *const ::qt_gui::quaternion::Quaternion) {
  let func: &mut Box<FnMut(&'static ::qt_gui::quaternion::Quaternion)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QVector3D&)` to a Rust closure.
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

pub struct SlotQtGuiVector3DRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVector3DRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::vector_3d::Vector3D) + 'a>>>,
}

impl<'a> SlotQtGuiVector3DRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::vector_3d::Vector3D) + 'a>(f: F) -> SlotQtGuiVector3DRef<'a> {
    let mut obj = SlotQtGuiVector3DRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::vector_3d::Vector3D) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::vector_3d::Vector3D) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_vector_3d_ref_callback,
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

impl<'a> Default for SlotQtGuiVector3DRef<'a> {
  fn default() -> Self {
    SlotQtGuiVector3DRef {
      wrapper: ::slots::raw::RawSlotQtGuiVector3DRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiVector3DRef<'a> {
  type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiVector3DRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_vector_3d_ref_callback(data: *mut ::libc::c_void,
                                                 arg0: *const ::qt_gui::vector_3d::Vector3D) {
  let func: &mut Box<FnMut(&'static ::qt_gui::vector_3d::Vector3D)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
