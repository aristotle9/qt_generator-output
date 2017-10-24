/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquationArguments::Blending)` to a Rust extern function.
  ///
  /// Use `SlotQt3DRenderBlendEquationArgumentsBlendingRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending```</span>
  #[repr(C)]
  pub struct RawSlotQt3DRenderBlendEquationArgumentsBlendingRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
    type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::custom_slot(Qt3DRender::QBlendEquationArguments::Blending arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_3d_render::blend_equation_arguments::Blending) {
      unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_custom_slot(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef, arg0 as *const ::qt_3d_render::blend_equation_arguments::Blending) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::set(void (*FN_PTR)(void*, const Qt3DRender::QBlendEquationArguments::Blending*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_3d_render::blend_equation_arguments::Blending),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_set(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquation::BlendFunction)` to a Rust extern function.
  ///
  /// Use `SlotQt3DRenderBlendEquationBlendFunctionRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction```</span>
  #[repr(C)]
  pub struct RawSlotQt3DRenderBlendEquationBlendFunctionRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef {
    type Arguments = (&'static ::qt_3d_render::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  impl RawSlotQt3DRenderBlendEquationBlendFunctionRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::custom_slot(Qt3DRender::QBlendEquation::BlendFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_3d_render::blend_equation::BlendFunction) {
      unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_custom_slot(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef, arg0 as *const ::qt_3d_render::blend_equation::BlendFunction) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::set(void (*FN_PTR)(void*, const Qt3DRender::QBlendEquation::BlendFunction*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_3d_render::blend_equation::BlendFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_set(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QFont&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiFontRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_extras_c_SlotWrapper_const_QFont_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiFontRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiFontRef {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QFont&)\0"
    }
  }
  impl RawSlotQtGuiFontRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_extras_c_SlotWrapper_const_QFont_ref::custom_slot(const QFont& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::font::Font) {
      unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_const_QFont_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiFontRef, arg0 as *const ::qt_gui::font::Font) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_extras_c_SlotWrapper_const_QFont_ref::qt_3d_extras_c_SlotWrapper_const_QFont_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiFontRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_const_QFont_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_extras_c_SlotWrapper_const_QFont_ref::set(void (*FN_PTR)(void*, const QFont*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::font::Font),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_const_QFont_ref_set(self as *mut ::slots::raw::RawSlotQtGuiFontRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiFontRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_const_QFont_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QVector2D)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiVector2DRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_extras_c_SlotWrapper_QVector2D```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiVector2DRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiVector2DRef {
    type Arguments = (&'static ::qt_gui::vector_2d::Vector2D,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QVector2D)\0"
    }
  }
  impl RawSlotQtGuiVector2DRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_extras_c_SlotWrapper_QVector2D::custom_slot(QVector2D arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::vector_2d::Vector2D) {
      unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_QVector2D_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiVector2DRef, arg0 as *const ::qt_gui::vector_2d::Vector2D) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_extras_c_SlotWrapper_QVector2D::qt_3d_extras_c_SlotWrapper_QVector2D()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVector2DRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_QVector2D_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_extras_c_SlotWrapper_QVector2D::set(void (*FN_PTR)(void*, const QVector2D*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::vector_2d::Vector2D),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_QVector2D_set(self as *mut ::slots::raw::RawSlotQtGuiVector2DRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiVector2DRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_extras_c_qt_3d_extras_c_SlotWrapper_QVector2D_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiVector2DRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_QVector2D(self as *mut ::slots::raw::RawSlotQtGuiVector2DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_QVector2D(self as *const ::slots::raw::RawSlotQtGuiVector2DRef as *mut ::slots::raw::RawSlotQtGuiVector2DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *const ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *const ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiFontRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_const_QFont_ref(self as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_const_QFont_ref(self as *const ::slots::raw::RawSlotQtGuiFontRef as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiVector2DRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_QVector2D(self as *const ::slots::raw::RawSlotQtGuiVector2DRef as *mut ::slots::raw::RawSlotQtGuiVector2DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *const ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *const ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiFontRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_const_QFont_ref(self as *const ::slots::raw::RawSlotQtGuiFontRef as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiVector2DRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_QVector2D(self as *mut ::slots::raw::RawSlotQtGuiVector2DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *mut ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiFontRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_extras_c_slots_G_static_cast_QObject_ptr_qt_3d_extras_c_SlotWrapper_const_QFont_ref(self as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquationArguments::Blending)` to a Rust closure.
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

pub struct SlotQt3DRenderBlendEquationArgumentsBlendingRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_3d_render::blend_equation_arguments::Blending) + 'a>>>,
}

impl<'a> SlotQt3DRenderBlendEquationArgumentsBlendingRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_3d_render::blend_equation_arguments::Blending) + 'a>
    (f: F)
     -> SlotQt3DRenderBlendEquationArgumentsBlendingRef<'a> {
    let mut obj = SlotQt3DRenderBlendEquationArgumentsBlendingRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_3d_render::blend_equation_arguments::Blending) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_3d_render::blend_equation_arguments::Blending) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_3d_render_blend_equation_arguments_blending_ref_callback,
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

impl<'a> Default for SlotQt3DRenderBlendEquationArgumentsBlendingRef<'a> {
  fn default() -> Self {
    SlotQt3DRenderBlendEquationArgumentsBlendingRef {
      wrapper: ::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQt3DRenderBlendEquationArgumentsBlendingRef<'a> {
  type Arguments = (&'static ::qt_3d_render::blend_equation_arguments::Blending,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQt3DRenderBlendEquationArgumentsBlendingRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_3d_render_blend_equation_arguments_blending_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_3d_render::blend_equation_arguments::Blending) {
  let func: &mut Box<FnMut(&'static ::qt_3d_render::blend_equation_arguments::Blending)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquation::BlendFunction)` to a Rust closure.
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

pub struct SlotQt3DRenderBlendEquationBlendFunctionRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_3d_render::blend_equation::BlendFunction) + 'a>>>,
}

impl<'a> SlotQt3DRenderBlendEquationBlendFunctionRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_3d_render::blend_equation::BlendFunction) + 'a>
    (f: F)
     -> SlotQt3DRenderBlendEquationBlendFunctionRef<'a> {
    let mut obj = SlotQt3DRenderBlendEquationBlendFunctionRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_3d_render::blend_equation::BlendFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_3d_render::blend_equation::BlendFunction) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_3d_render_blend_equation_blend_function_ref_callback,
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

impl<'a> Default for SlotQt3DRenderBlendEquationBlendFunctionRef<'a> {
  fn default() -> Self {
    SlotQt3DRenderBlendEquationBlendFunctionRef {
      wrapper: ::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQt3DRenderBlendEquationBlendFunctionRef<'a> {
  type Arguments = (&'static ::qt_3d_render::blend_equation::BlendFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQt3DRenderBlendEquationBlendFunctionRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_3d_render_blend_equation_blend_function_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_3d_render::blend_equation::BlendFunction) {
  let func: &mut Box<FnMut(&'static ::qt_3d_render::blend_equation::BlendFunction)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QFont&)` to a Rust closure.
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

pub struct SlotQtGuiFontRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiFontRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::font::Font) + 'a>>>,
}

impl<'a> SlotQtGuiFontRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::font::Font) + 'a>(f: F) -> SlotQtGuiFontRef<'a> {
    let mut obj = SlotQtGuiFontRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::font::Font) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::font::Font) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_font_ref_callback,
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

impl<'a> Default for SlotQtGuiFontRef<'a> {
  fn default() -> Self {
    SlotQtGuiFontRef {
      wrapper: ::slots::raw::RawSlotQtGuiFontRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiFontRef<'a> {
  type Arguments = (&'static ::qt_gui::font::Font,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiFontRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_font_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_gui::font::Font) {
  let func: &mut Box<FnMut(&'static ::qt_gui::font::Font)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QVector2D)` to a Rust closure.
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

pub struct SlotQtGuiVector2DRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVector2DRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::vector_2d::Vector2D) + 'a>>>,
}

impl<'a> SlotQtGuiVector2DRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::vector_2d::Vector2D) + 'a>(f: F) -> SlotQtGuiVector2DRef<'a> {
    let mut obj = SlotQtGuiVector2DRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::vector_2d::Vector2D) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::vector_2d::Vector2D) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_vector_2d_ref_callback,
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

impl<'a> Default for SlotQtGuiVector2DRef<'a> {
  fn default() -> Self {
    SlotQtGuiVector2DRef {
      wrapper: ::slots::raw::RawSlotQtGuiVector2DRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiVector2DRef<'a> {
  type Arguments = (&'static ::qt_gui::vector_2d::Vector2D,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiVector2DRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_vector_2d_ref_callback(data: *mut ::libc::c_void,
                                                 arg0: *const ::qt_gui::vector_2d::Vector2D) {
  let func: &mut Box<FnMut(&'static ::qt_gui::vector_2d::Vector2D)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
