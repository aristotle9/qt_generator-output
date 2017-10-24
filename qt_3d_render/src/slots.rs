/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::ComparisonFunction)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureComparisonFunction` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureComparisonFunction(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureComparisonFunction {
    type Arguments = (::abstract_texture::ComparisonFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::ComparisonFunction)\0"
    }
  }
  impl RawSlotAbstractTextureComparisonFunction {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction::custom_slot(Qt3DRender::QAbstractTexture::ComparisonFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::abstract_texture::ComparisonFunction) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureComparisonFunction> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture::ComparisonFunction) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::abstract_texture::ComparisonFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction_set(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureComparisonFunction {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::ComparisonMode)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureComparisonMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureComparisonMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureComparisonMode {
    type Arguments = (::abstract_texture::ComparisonMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::ComparisonMode)\0"
    }
  }
  impl RawSlotAbstractTextureComparisonMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode::custom_slot(Qt3DRender::QAbstractTexture::ComparisonMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::abstract_texture::ComparisonMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureComparisonMode> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture::ComparisonMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::abstract_texture::ComparisonMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode_set(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureComparisonMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::CubeMapFace)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureCubeMapFaceRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureCubeMapFaceRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  impl RawSlotAbstractTextureCubeMapFaceRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace::custom_slot(Qt3DRender::QAbstractTexture::CubeMapFace arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::abstract_texture::CubeMapFace) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef, arg0 as *const ::abstract_texture::CubeMapFace) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureCubeMapFaceRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace::set(void (*FN_PTR)(void*, const Qt3DRender::QAbstractTexture::CubeMapFace*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::abstract_texture::CubeMapFace),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace_set(self as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::Filter)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureFilter` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureFilter(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureFilter {
    type Arguments = (::abstract_texture::Filter,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::Filter)\0"
    }
  }
  impl RawSlotAbstractTextureFilter {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter::custom_slot(Qt3DRender::QAbstractTexture::Filter arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::abstract_texture::Filter) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureFilter, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureFilter> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture::Filter) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::abstract_texture::Filter),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter_set(self as *mut ::slots::raw::RawSlotAbstractTextureFilter, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureFilter {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture*)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureMutPtr {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl RawSlotAbstractTextureMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr::custom_slot(Qt3DRender::QAbstractTexture* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_texture::AbstractTexture) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::abstract_texture::AbstractTexture),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr_set(self as *mut ::slots::raw::RawSlotAbstractTextureMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::Status)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureStatus` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureStatus(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureStatus {
    type Arguments = (::abstract_texture::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::Status)\0"
    }
  }
  impl RawSlotAbstractTextureStatus {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status::custom_slot(Qt3DRender::QAbstractTexture::Status arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::abstract_texture::Status) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureStatus, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureStatus> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture::Status) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::abstract_texture::Status),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status_set(self as *mut ::slots::raw::RawSlotAbstractTextureStatus, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureStatus {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::TextureFormat)` to a Rust extern function.
  ///
  /// Use `SlotAbstractTextureTextureFormat` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat```</span>
  #[repr(C)]
  pub struct RawSlotAbstractTextureTextureFormat(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractTextureTextureFormat {
    type Arguments = (::abstract_texture::TextureFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAbstractTexture::TextureFormat)\0"
    }
  }
  impl RawSlotAbstractTextureTextureFormat {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat::custom_slot(Qt3DRender::QAbstractTexture::TextureFormat arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::abstract_texture::TextureFormat) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat_custom_slot(self as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat::qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureTextureFormat> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat::set(void (*FN_PTR)(void*, Qt3DRender::QAbstractTexture::TextureFormat) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::abstract_texture::TextureFormat),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat_set(self as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractTextureTextureFormat {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAlphaTest::AlphaFunction)` to a Rust extern function.
  ///
  /// Use `SlotAlphaTestAlphaFunction` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction```</span>
  #[repr(C)]
  pub struct RawSlotAlphaTestAlphaFunction(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAlphaTestAlphaFunction {
    type Arguments = (::alpha_test::AlphaFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAlphaTest::AlphaFunction)\0"
    }
  }
  impl RawSlotAlphaTestAlphaFunction {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction::custom_slot(Qt3DRender::QAlphaTest::AlphaFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::alpha_test::AlphaFunction) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction_custom_slot(self as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction::qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAlphaTestAlphaFunction> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction::set(void (*FN_PTR)(void*, Qt3DRender::QAlphaTest::AlphaFunction) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::alpha_test::AlphaFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction_set(self as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAlphaTestAlphaFunction {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute::AttributeType)` to a Rust extern function.
  ///
  /// Use `SlotAttributeAttributeType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType```</span>
  #[repr(C)]
  pub struct RawSlotAttributeAttributeType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAttributeAttributeType {
    type Arguments = (::attribute::AttributeType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAttribute::AttributeType)\0"
    }
  }
  impl RawSlotAttributeAttributeType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType::custom_slot(Qt3DRender::QAttribute::AttributeType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::attribute::AttributeType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType_custom_slot(self as *mut ::slots::raw::RawSlotAttributeAttributeType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType::qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeAttributeType> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType::set(void (*FN_PTR)(void*, Qt3DRender::QAttribute::AttributeType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::attribute::AttributeType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType_set(self as *mut ::slots::raw::RawSlotAttributeAttributeType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAttributeAttributeType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute*)` to a Rust extern function.
  ///
  /// Use `SlotAttributeMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAttributeMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAttributeMutPtr {
    type Arguments = (*mut ::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAttribute*)\0"
    }
  }
  impl RawSlotAttributeMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr::custom_slot(Qt3DRender::QAttribute* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::attribute::Attribute) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAttributeMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QAttribute*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::attribute::Attribute),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr_set(self as *mut ::slots::raw::RawSlotAttributeMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAttributeMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute::VertexBaseType)` to a Rust extern function.
  ///
  /// Use `SlotAttributeVertexBaseType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType```</span>
  #[repr(C)]
  pub struct RawSlotAttributeVertexBaseType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAttributeVertexBaseType {
    type Arguments = (::attribute::VertexBaseType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QAttribute::VertexBaseType)\0"
    }
  }
  impl RawSlotAttributeVertexBaseType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType::custom_slot(Qt3DRender::QAttribute::VertexBaseType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::attribute::VertexBaseType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType_custom_slot(self as *mut ::slots::raw::RawSlotAttributeVertexBaseType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType::qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeVertexBaseType> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType::set(void (*FN_PTR)(void*, Qt3DRender::QAttribute::VertexBaseType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::attribute::VertexBaseType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType_set(self as *mut ::slots::raw::RawSlotAttributeVertexBaseType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAttributeVertexBaseType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquationArguments::Blending)` to a Rust extern function.
  ///
  /// Use `SlotBlendEquationArgumentsBlending` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending```</span>
  #[repr(C)]
  pub struct RawSlotBlendEquationArgumentsBlending(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBlendEquationArgumentsBlending {
    type Arguments = (::blend_equation_arguments::Blending,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBlendEquationArguments::Blending)\0"
    }
  }
  impl RawSlotBlendEquationArgumentsBlending {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::custom_slot(Qt3DRender::QBlendEquationArguments::Blending arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::blend_equation_arguments::Blending) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_custom_slot(self as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBlendEquationArgumentsBlending> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending::set(void (*FN_PTR)(void*, Qt3DRender::QBlendEquationArguments::Blending) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::blend_equation_arguments::Blending),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_set(self as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBlendEquationArgumentsBlending {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBlendEquation::BlendFunction)` to a Rust extern function.
  ///
  /// Use `SlotBlendEquationBlendFunction` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction```</span>
  #[repr(C)]
  pub struct RawSlotBlendEquationBlendFunction(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBlendEquationBlendFunction {
    type Arguments = (::blend_equation::BlendFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBlendEquation::BlendFunction)\0"
    }
  }
  impl RawSlotBlendEquationBlendFunction {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::custom_slot(Qt3DRender::QBlendEquation::BlendFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::blend_equation::BlendFunction) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_custom_slot(self as *mut ::slots::raw::RawSlotBlendEquationBlendFunction, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBlendEquationBlendFunction> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction::set(void (*FN_PTR)(void*, Qt3DRender::QBlendEquation::BlendFunction) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::blend_equation::BlendFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_set(self as *mut ::slots::raw::RawSlotBlendEquationBlendFunction, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBlendEquationBlendFunction {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::AccessType)` to a Rust extern function.
  ///
  /// Use `SlotBufferAccessType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType```</span>
  #[repr(C)]
  pub struct RawSlotBufferAccessType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBufferAccessType {
    type Arguments = (::buffer::AccessType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBuffer::AccessType)\0"
    }
  }
  impl RawSlotBufferAccessType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType::custom_slot(Qt3DRender::QBuffer::AccessType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::buffer::AccessType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType_custom_slot(self as *mut ::slots::raw::RawSlotBufferAccessType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType::qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBufferAccessType> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType::set(void (*FN_PTR)(void*, Qt3DRender::QBuffer::AccessType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::buffer::AccessType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType_set(self as *mut ::slots::raw::RawSlotBufferAccessType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBufferAccessType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::BufferType)` to a Rust extern function.
  ///
  /// Use `SlotBufferBufferType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType```</span>
  #[repr(C)]
  pub struct RawSlotBufferBufferType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBufferBufferType {
    type Arguments = (::buffer::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBuffer::BufferType)\0"
    }
  }
  impl RawSlotBufferBufferType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType::custom_slot(Qt3DRender::QBuffer::BufferType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::buffer::BufferType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType_custom_slot(self as *mut ::slots::raw::RawSlotBufferBufferType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType::qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBufferBufferType> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType::set(void (*FN_PTR)(void*, Qt3DRender::QBuffer::BufferType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::buffer::BufferType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType_set(self as *mut ::slots::raw::RawSlotBufferBufferType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBufferBufferType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer*)` to a Rust extern function.
  ///
  /// Use `SlotBufferMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr```</span>
  #[repr(C)]
  pub struct RawSlotBufferMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBufferMutPtr {
    type Arguments = (*mut ::buffer::Buffer,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBuffer*)\0"
    }
  }
  impl RawSlotBufferMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr::custom_slot(Qt3DRender::QBuffer* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::buffer::Buffer) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr_custom_slot(self as *mut ::slots::raw::RawSlotBufferMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBufferMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QBuffer*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::buffer::Buffer),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr_set(self as *mut ::slots::raw::RawSlotBufferMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBufferMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::UsageType)` to a Rust extern function.
  ///
  /// Use `SlotBufferUsageType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType```</span>
  #[repr(C)]
  pub struct RawSlotBufferUsageType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotBufferUsageType {
    type Arguments = (::buffer::UsageType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QBuffer::UsageType)\0"
    }
  }
  impl RawSlotBufferUsageType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType::custom_slot(Qt3DRender::QBuffer::UsageType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::buffer::UsageType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType_custom_slot(self as *mut ::slots::raw::RawSlotBufferUsageType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType::qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotBufferUsageType> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType::set(void (*FN_PTR)(void*, Qt3DRender::QBuffer::UsageType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::buffer::UsageType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType_set(self as *mut ::slots::raw::RawSlotBufferUsageType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotBufferUsageType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(unsigned int)` to a Rust extern function.
  ///
  /// Use `SlotCUint` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_unsigned_int```</span>
  #[repr(C)]
  pub struct RawSlotCUint(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCUint {
    type Arguments = (::libc::c_uint,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(unsigned int)\0"
    }
  }
  impl RawSlotCUint {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_unsigned_int::custom_slot(unsigned int arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_uint) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_unsigned_int_custom_slot(self as *mut ::slots::raw::RawSlotCUint, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_unsigned_int::qt_3d_render_c_SlotWrapper_unsigned_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCUint> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_unsigned_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_unsigned_int::set(void (*FN_PTR)(void*, unsigned int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void, ::libc::c_uint), data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_unsigned_int_set(self as *mut ::slots::raw::RawSlotCUint,
                                                                        func,
                                                                        data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCUint {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_unsigned_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QCameraLens::ProjectionType)` to a Rust extern function.
  ///
  /// Use `SlotCameraLensProjectionTypeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType```</span>
  #[repr(C)]
  pub struct RawSlotCameraLensProjectionTypeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCameraLensProjectionTypeRef {
    type Arguments = (&'static ::camera_lens::ProjectionType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QCameraLens::ProjectionType)\0"
    }
  }
  impl RawSlotCameraLensProjectionTypeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType::custom_slot(Qt3DRender::QCameraLens::ProjectionType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::camera_lens::ProjectionType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType_custom_slot(self as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef, arg0 as *const ::camera_lens::ProjectionType) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType::qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCameraLensProjectionTypeRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType::set(void (*FN_PTR)(void*, const Qt3DRender::QCameraLens::ProjectionType*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::camera_lens::ProjectionType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType_set(self as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCameraLensProjectionTypeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QCamera*)` to a Rust extern function.
  ///
  /// Use `SlotCameraMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr```</span>
  #[repr(C)]
  pub struct RawSlotCameraMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCameraMutPtr {
    type Arguments = (*mut ::camera::Camera,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QCamera*)\0"
    }
  }
  impl RawSlotCameraMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr::custom_slot(Qt3DRender::QCamera* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::camera::Camera) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr_custom_slot(self as *mut ::slots::raw::RawSlotCameraMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCameraMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QCamera*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::camera::Camera),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr_set(self as *mut ::slots::raw::RawSlotCameraMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCameraMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QClearBuffers::BufferType)` to a Rust extern function.
  ///
  /// Use `SlotClearBuffersBufferType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType```</span>
  #[repr(C)]
  pub struct RawSlotClearBuffersBufferType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotClearBuffersBufferType {
    type Arguments = (::clear_buffers::BufferType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QClearBuffers::BufferType)\0"
    }
  }
  impl RawSlotClearBuffersBufferType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType::custom_slot(Qt3DRender::QClearBuffers::BufferType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::clear_buffers::BufferType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType_custom_slot(self as *mut ::slots::raw::RawSlotClearBuffersBufferType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType::qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotClearBuffersBufferType> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType::set(void (*FN_PTR)(void*, Qt3DRender::QClearBuffers::BufferType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::clear_buffers::BufferType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType_set(self as *mut ::slots::raw::RawSlotClearBuffersBufferType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotClearBuffersBufferType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QCullFace::CullingMode)` to a Rust extern function.
  ///
  /// Use `SlotCullFaceCullingMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode```</span>
  #[repr(C)]
  pub struct RawSlotCullFaceCullingMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCullFaceCullingMode {
    type Arguments = (::cull_face::CullingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QCullFace::CullingMode)\0"
    }
  }
  impl RawSlotCullFaceCullingMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode::custom_slot(Qt3DRender::QCullFace::CullingMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::cull_face::CullingMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode_custom_slot(self as *mut ::slots::raw::RawSlotCullFaceCullingMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCullFaceCullingMode> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode::set(void (*FN_PTR)(void*, Qt3DRender::QCullFace::CullingMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::cull_face::CullingMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode_set(self as *mut ::slots::raw::RawSlotCullFaceCullingMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCullFaceCullingMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QDepthTest::DepthFunction)` to a Rust extern function.
  ///
  /// Use `SlotDepthTestDepthFunction` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction```</span>
  #[repr(C)]
  pub struct RawSlotDepthTestDepthFunction(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotDepthTestDepthFunction {
    type Arguments = (::depth_test::DepthFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QDepthTest::DepthFunction)\0"
    }
  }
  impl RawSlotDepthTestDepthFunction {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction::custom_slot(Qt3DRender::QDepthTest::DepthFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::depth_test::DepthFunction) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction_custom_slot(self as *mut ::slots::raw::RawSlotDepthTestDepthFunction, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction::qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotDepthTestDepthFunction> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction::set(void (*FN_PTR)(void*, Qt3DRender::QDepthTest::DepthFunction) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::depth_test::DepthFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction_set(self as *mut ::slots::raw::RawSlotDepthTestDepthFunction, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotDepthTestDepthFunction {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QEffect*)` to a Rust extern function.
  ///
  /// Use `SlotEffectMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr```</span>
  #[repr(C)]
  pub struct RawSlotEffectMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotEffectMutPtr {
    type Arguments = (*mut ::effect::Effect,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QEffect*)\0"
    }
  }
  impl RawSlotEffectMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr::custom_slot(Qt3DRender::QEffect* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::effect::Effect) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr_custom_slot(self as *mut ::slots::raw::RawSlotEffectMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotEffectMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QEffect*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::effect::Effect),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr_set(self as *mut ::slots::raw::RawSlotEffectMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotEffectMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QFrameGraphNode*)` to a Rust extern function.
  ///
  /// Use `SlotFrameGraphNodeMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr```</span>
  #[repr(C)]
  pub struct RawSlotFrameGraphNodeMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotFrameGraphNodeMutPtr {
    type Arguments = (*mut ::frame_graph_node::FrameGraphNode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QFrameGraphNode*)\0"
    }
  }
  impl RawSlotFrameGraphNodeMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr::custom_slot(Qt3DRender::QFrameGraphNode* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::frame_graph_node::FrameGraphNode) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr_custom_slot(self as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotFrameGraphNodeMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QFrameGraphNode*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::frame_graph_node::FrameGraphNode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr_set(self as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotFrameGraphNodeMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QFrontFace::WindingDirection)` to a Rust extern function.
  ///
  /// Use `SlotFrontFaceWindingDirection` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection```</span>
  #[repr(C)]
  pub struct RawSlotFrontFaceWindingDirection(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotFrontFaceWindingDirection {
    type Arguments = (::front_face::WindingDirection,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QFrontFace::WindingDirection)\0"
    }
  }
  impl RawSlotFrontFaceWindingDirection {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection::custom_slot(Qt3DRender::QFrontFace::WindingDirection arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::front_face::WindingDirection) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection_custom_slot(self as *mut ::slots::raw::RawSlotFrontFaceWindingDirection, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection::qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotFrontFaceWindingDirection> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection::set(void (*FN_PTR)(void*, Qt3DRender::QFrontFace::WindingDirection) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::front_face::WindingDirection),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection_set(self as *mut ::slots::raw::RawSlotFrontFaceWindingDirection, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotFrontFaceWindingDirection {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QGeometry*)` to a Rust extern function.
  ///
  /// Use `SlotGeometryMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr```</span>
  #[repr(C)]
  pub struct RawSlotGeometryMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGeometryMutPtr {
    type Arguments = (*mut ::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QGeometry*)\0"
    }
  }
  impl RawSlotGeometryMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr::custom_slot(Qt3DRender::QGeometry* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::geometry::Geometry) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr_custom_slot(self as *mut ::slots::raw::RawSlotGeometryMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGeometryMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QGeometry*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::geometry::Geometry),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr_set(self as *mut ::slots::raw::RawSlotGeometryMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGeometryMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QGeometryRenderer::PrimitiveType)` to a Rust extern function.
  ///
  /// Use `SlotGeometryRendererPrimitiveType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType```</span>
  #[repr(C)]
  pub struct RawSlotGeometryRendererPrimitiveType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGeometryRendererPrimitiveType {
    type Arguments = (::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  impl RawSlotGeometryRendererPrimitiveType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType::custom_slot(Qt3DRender::QGeometryRenderer::PrimitiveType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::geometry_renderer::PrimitiveType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType_custom_slot(self as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType::qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGeometryRendererPrimitiveType> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType::set(void (*FN_PTR)(void*, Qt3DRender::QGeometryRenderer::PrimitiveType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::geometry_renderer::PrimitiveType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType_set(self as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGeometryRendererPrimitiveType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QGraphicsApiFilter::Api)` to a Rust extern function.
  ///
  /// Use `SlotGraphicsApiFilterApiRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api```</span>
  #[repr(C)]
  pub struct RawSlotGraphicsApiFilterApiRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGraphicsApiFilterApiRef {
    type Arguments = (&'static ::graphics_api_filter::Api,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QGraphicsApiFilter::Api)\0"
    }
  }
  impl RawSlotGraphicsApiFilterApiRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api::custom_slot(Qt3DRender::QGraphicsApiFilter::Api arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::graphics_api_filter::Api) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api_custom_slot(self as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef, arg0 as *const ::graphics_api_filter::Api) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api::qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsApiFilterApiRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api::set(void (*FN_PTR)(void*, const Qt3DRender::QGraphicsApiFilter::Api*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::graphics_api_filter::Api),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api_set(self as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGraphicsApiFilterApiRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QGraphicsApiFilter::OpenGLProfile)` to a Rust extern function.
  ///
  /// Use `SlotGraphicsApiFilterOpenglProfileRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile```</span>
  #[repr(C)]
  pub struct RawSlotGraphicsApiFilterOpenglProfileRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef {
    type Arguments = (&'static ::graphics_api_filter::OpenGLProfile,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QGraphicsApiFilter::OpenGLProfile)\0"
    }
  }
  impl RawSlotGraphicsApiFilterOpenglProfileRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile::custom_slot(Qt3DRender::QGraphicsApiFilter::OpenGLProfile arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::graphics_api_filter::OpenGLProfile) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile_custom_slot(self as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef, arg0 as *const ::graphics_api_filter::OpenGLProfile) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile::qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile::set(void (*FN_PTR)(void*, const Qt3DRender::QGraphicsApiFilter::OpenGLProfile*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::graphics_api_filter::OpenGLProfile),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile_set(self as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const Qt3DRender::QLevelOfDetailBoundingSphere&)` to a Rust extern function.
  ///
  /// Use `SlotLevelOfDetailBoundingSphereRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref```</span>
  #[repr(C)]
  pub struct RawSlotLevelOfDetailBoundingSphereRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef {
    type Arguments = (&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const Qt3DRender::QLevelOfDetailBoundingSphere&)\0"
    }
  }
  impl RawSlotLevelOfDetailBoundingSphereRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref::custom_slot(const Qt3DRender::QLevelOfDetailBoundingSphere& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref_custom_slot(self as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef, arg0 as *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref::qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotLevelOfDetailBoundingSphereRef> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref::set(void (*FN_PTR)(void*, const Qt3DRender::QLevelOfDetailBoundingSphere*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref_set(self as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QLevelOfDetail::ThresholdType)` to a Rust extern function.
  ///
  /// Use `SlotLevelOfDetailThresholdType` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType```</span>
  #[repr(C)]
  pub struct RawSlotLevelOfDetailThresholdType(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotLevelOfDetailThresholdType {
    type Arguments = (::level_of_detail::ThresholdType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QLevelOfDetail::ThresholdType)\0"
    }
  }
  impl RawSlotLevelOfDetailThresholdType {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType::custom_slot(Qt3DRender::QLevelOfDetail::ThresholdType arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::level_of_detail::ThresholdType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType_custom_slot(self as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType::qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotLevelOfDetailThresholdType> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType::set(void (*FN_PTR)(void*, Qt3DRender::QLevelOfDetail::ThresholdType) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::level_of_detail::ThresholdType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType_set(self as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotLevelOfDetailThresholdType {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QPickEvent*)` to a Rust extern function.
  ///
  /// Use `SlotPickEventMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr```</span>
  #[repr(C)]
  pub struct RawSlotPickEventMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPickEventMutPtr {
    type Arguments = (*mut ::pick_event::PickEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QPickEvent*)\0"
    }
  }
  impl RawSlotPickEventMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr::custom_slot(Qt3DRender::QPickEvent* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::pick_event::PickEvent) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr_custom_slot(self as *mut ::slots::raw::RawSlotPickEventMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPickEventMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QPickEvent*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::pick_event::PickEvent),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr_set(self as *mut ::slots::raw::RawSlotPickEventMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPickEventMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::FaceOrientationPickingMode)` to a Rust extern function.
  ///
  /// Use `SlotPickingSettingsFaceOrientationPickingModeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode```</span>
  #[repr(C)]
  pub struct RawSlotPickingSettingsFaceOrientationPickingModeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef {
    type Arguments = (&'static ::picking_settings::FaceOrientationPickingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QPickingSettings::FaceOrientationPickingMode)\0"
    }
  }
  impl RawSlotPickingSettingsFaceOrientationPickingModeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode::custom_slot(Qt3DRender::QPickingSettings::FaceOrientationPickingMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::picking_settings::FaceOrientationPickingMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode_custom_slot(self as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef, arg0 as *const ::picking_settings::FaceOrientationPickingMode) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode::set(void (*FN_PTR)(void*, const Qt3DRender::QPickingSettings::FaceOrientationPickingMode*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::picking_settings::FaceOrientationPickingMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode_set(self as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::PickMethod)` to a Rust extern function.
  ///
  /// Use `SlotPickingSettingsPickMethodRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod```</span>
  #[repr(C)]
  pub struct RawSlotPickingSettingsPickMethodRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPickingSettingsPickMethodRef {
    type Arguments = (&'static ::picking_settings::PickMethod,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QPickingSettings::PickMethod)\0"
    }
  }
  impl RawSlotPickingSettingsPickMethodRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod::custom_slot(Qt3DRender::QPickingSettings::PickMethod arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::picking_settings::PickMethod) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod_custom_slot(self as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef, arg0 as *const ::picking_settings::PickMethod) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod::qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsPickMethodRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod::set(void (*FN_PTR)(void*, const Qt3DRender::QPickingSettings::PickMethod*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::picking_settings::PickMethod),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod_set(self as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPickingSettingsPickMethodRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::PickResultMode)` to a Rust extern function.
  ///
  /// Use `SlotPickingSettingsPickResultModeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode```</span>
  #[repr(C)]
  pub struct RawSlotPickingSettingsPickResultModeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPickingSettingsPickResultModeRef {
    type Arguments = (&'static ::picking_settings::PickResultMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QPickingSettings::PickResultMode)\0"
    }
  }
  impl RawSlotPickingSettingsPickResultModeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode::custom_slot(Qt3DRender::QPickingSettings::PickResultMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::picking_settings::PickResultMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode_custom_slot(self as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef, arg0 as *const ::picking_settings::PickResultMode) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsPickResultModeRef> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode::set(void (*FN_PTR)(void*, const Qt3DRender::QPickingSettings::PickResultMode*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::picking_settings::PickResultMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode_set(self as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPickingSettingsPickResultModeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QPointSize::SizeMode)` to a Rust extern function.
  ///
  /// Use `SlotPointSizeSizeMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode```</span>
  #[repr(C)]
  pub struct RawSlotPointSizeSizeMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotPointSizeSizeMode {
    type Arguments = (::point_size::SizeMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QPointSize::SizeMode)\0"
    }
  }
  impl RawSlotPointSizeSizeMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode::custom_slot(Qt3DRender::QPointSize::SizeMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::point_size::SizeMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode_custom_slot(self as *mut ::slots::raw::RawSlotPointSizeSizeMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotPointSizeSizeMode> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode::set(void (*FN_PTR)(void*, Qt3DRender::QPointSize::SizeMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::point_size::SizeMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode_set(self as *mut ::slots::raw::RawSlotPointSizeSizeMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotPointSizeSizeMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QByteArray&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreByteArrayRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QByteArray_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreByteArrayRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreByteArrayRef {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QByteArray&)\0"
    }
  }
  impl RawSlotQtCoreByteArrayRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QByteArray_ref::custom_slot(const QByteArray& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::byte_array::ByteArray) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QByteArray_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreByteArrayRef, arg0 as *const ::qt_core::byte_array::ByteArray) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QByteArray_ref::qt_3d_render_c_SlotWrapper_const_QByteArray_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreByteArrayRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QByteArray_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QByteArray_ref::set(void (*FN_PTR)(void*, const QByteArray*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::byte_array::ByteArray),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QByteArray_ref_set(self as *mut ::slots::raw::RawSlotQtCoreByteArrayRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreByteArrayRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QByteArray_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QSize)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreSizeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_QSize```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreSizeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreSizeRef {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QSize)\0"
    }
  }
  impl RawSlotQtCoreSizeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_QSize::custom_slot(QSize arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::size::Size) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QSize_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreSizeRef, arg0 as *const ::qt_core::size::Size) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_QSize::qt_3d_render_c_SlotWrapper_QSize()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreSizeRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QSize_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_QSize::set(void (*FN_PTR)(void*, const QSize*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::size::Size),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QSize_set(self as *mut ::slots::raw::RawSlotQtCoreSizeRef,
                                                                 func,
                                                                 data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreSizeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QSize_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QStringList&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreStringListRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QStringList_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreStringListRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreStringListRef {
    type Arguments = (&'static ::qt_core::string_list::StringList,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QStringList&)\0"
    }
  }
  impl RawSlotQtCoreStringListRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QStringList_ref::custom_slot(const QStringList& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::string_list::StringList) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QStringList_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreStringListRef, arg0 as *const ::qt_core::string_list::StringList) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QStringList_ref::qt_3d_render_c_SlotWrapper_const_QStringList_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringListRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QStringList_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QStringList_ref::set(void (*FN_PTR)(void*, const QStringList*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::string_list::StringList),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QStringList_ref_set(self as *mut ::slots::raw::RawSlotQtCoreStringListRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreStringListRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QStringList_ref_delete
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
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QVector_int_ref```</span>
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
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QVector_int_ref::custom_slot(const QVector<int>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::vector::VectorCInt) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_int_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef, arg0 as *const ::qt_core::vector::VectorCInt) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QVector_int_ref::qt_3d_render_c_SlotWrapper_const_QVector_int_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreVectorVectorCIntRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_int_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QVector_int_ref::set(void (*FN_PTR)(void*, const QVector< int >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::vector::VectorCInt),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_int_ref_set(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_int_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QColor&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiColorRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QColor_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiColorRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiColorRef {
    type Arguments = (&'static ::qt_gui::color::Color,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QColor&)\0"
    }
  }
  impl RawSlotQtGuiColorRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QColor_ref::custom_slot(const QColor& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::color::Color) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QColor_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiColorRef, arg0 as *const ::qt_gui::color::Color) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QColor_ref::qt_3d_render_c_SlotWrapper_const_QColor_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiColorRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QColor_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QColor_ref::set(void (*FN_PTR)(void*, const QColor*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::color::Color),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QColor_ref_set(self as *mut ::slots::raw::RawSlotQtGuiColorRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiColorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QColor_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QMatrix4x4&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiMatrix4X4Matrix4X4Ref` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiMatrix4X4Matrix4X4Ref(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QMatrix4x4&)\0"
    }
  }
  impl RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref::custom_slot(const QMatrix4x4& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::matrix_4x4::Matrix4X4) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref, arg0 as *const ::qt_gui::matrix_4x4::Matrix4X4) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref::qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref::set(void (*FN_PTR)(void*, const QMatrix4x4*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::matrix_4x4::Matrix4X4),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref_set(self as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QVector3D)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiVector3DRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_QVector3D```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiVector3DRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiVector3DRef {
    type Arguments = (&'static ::qt_gui::vector_3d::Vector3D,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QVector3D)\0"
    }
  }
  impl RawSlotQtGuiVector3DRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_QVector3D::custom_slot(QVector3D arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::vector_3d::Vector3D) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QVector3D_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef, arg0 as *const ::qt_gui::vector_3d::Vector3D) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_QVector3D::qt_3d_render_c_SlotWrapper_QVector3D()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVector3DRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QVector3D_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_QVector3D::set(void (*FN_PTR)(void*, const QVector3D*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::vector_3d::Vector3D),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QVector3D_set(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_QVector3D_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QVector<double>&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiVectorVectorCDoubleRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QVector_double_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiVectorVectorCDoubleRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef {
    type Arguments = (&'static ::qt_gui::vector::VectorCDouble,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QVector< double >&)\0"
    }
  }
  impl RawSlotQtGuiVectorVectorCDoubleRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QVector_double_ref::custom_slot(const QVector<double>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::vector::VectorCDouble) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_double_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef, arg0 as *const ::qt_gui::vector::VectorCDouble) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QVector_double_ref::qt_3d_render_c_SlotWrapper_const_QVector_double_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_double_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QVector_double_ref::set(void (*FN_PTR)(void*, const QVector< double >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::vector::VectorCDouble),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_double_ref_set(self as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_double_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderSettings::RenderPolicy)` to a Rust extern function.
  ///
  /// Use `SlotRenderSettingsRenderPolicy` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy```</span>
  #[repr(C)]
  pub struct RawSlotRenderSettingsRenderPolicy(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotRenderSettingsRenderPolicy {
    type Arguments = (::render_settings::RenderPolicy,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QRenderSettings::RenderPolicy)\0"
    }
  }
  impl RawSlotRenderSettingsRenderPolicy {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy::custom_slot(Qt3DRender::QRenderSettings::RenderPolicy arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::render_settings::RenderPolicy) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy_custom_slot(self as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy::qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotRenderSettingsRenderPolicy> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy::set(void (*FN_PTR)(void*, Qt3DRender::QRenderSettings::RenderPolicy) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::render_settings::RenderPolicy),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy_set(self as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotRenderSettingsRenderPolicy {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTarget*)` to a Rust extern function.
  ///
  /// Use `SlotRenderTargetMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr```</span>
  #[repr(C)]
  pub struct RawSlotRenderTargetMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotRenderTargetMutPtr {
    type Arguments = (*mut ::render_target::RenderTarget,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QRenderTarget*)\0"
    }
  }
  impl RawSlotRenderTargetMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr::custom_slot(Qt3DRender::QRenderTarget* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::render_target::RenderTarget) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr_custom_slot(self as *mut ::slots::raw::RawSlotRenderTargetMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QRenderTarget*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::render_target::RenderTarget),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr_set(self as *mut ::slots::raw::RawSlotRenderTargetMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotRenderTargetMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTargetOutput::AttachmentPoint)` to a Rust extern function.
  ///
  /// Use `SlotRenderTargetOutputAttachmentPoint` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint```</span>
  #[repr(C)]
  pub struct RawSlotRenderTargetOutputAttachmentPoint(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint {
    type Arguments = (::render_target_output::AttachmentPoint,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QRenderTargetOutput::AttachmentPoint)\0"
    }
  }
  impl RawSlotRenderTargetOutputAttachmentPoint {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint::custom_slot(Qt3DRender::QRenderTargetOutput::AttachmentPoint arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::render_target_output::AttachmentPoint) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint_custom_slot(self as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint::qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetOutputAttachmentPoint> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint::set(void (*FN_PTR)(void*, Qt3DRender::QRenderTargetOutput::AttachmentPoint) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::render_target_output::AttachmentPoint),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint_set(self as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTargetOutput*)` to a Rust extern function.
  ///
  /// Use `SlotRenderTargetOutputMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr```</span>
  #[repr(C)]
  pub struct RawSlotRenderTargetOutputMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotRenderTargetOutputMutPtr {
    type Arguments = (*mut ::render_target_output::RenderTargetOutput,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QRenderTargetOutput*)\0"
    }
  }
  impl RawSlotRenderTargetOutputMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr::custom_slot(Qt3DRender::QRenderTargetOutput* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::render_target_output::RenderTargetOutput) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr_custom_slot(self as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetOutputMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QRenderTargetOutput*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::render_target_output::RenderTargetOutput),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr_set(self as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotRenderTargetOutputMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QSceneLoader::Status)` to a Rust extern function.
  ///
  /// Use `SlotSceneLoaderStatus` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status```</span>
  #[repr(C)]
  pub struct RawSlotSceneLoaderStatus(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotSceneLoaderStatus {
    type Arguments = (::scene_loader::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QSceneLoader::Status)\0"
    }
  }
  impl RawSlotSceneLoaderStatus {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status::custom_slot(Qt3DRender::QSceneLoader::Status arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::scene_loader::Status) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status_custom_slot(self as *mut ::slots::raw::RawSlotSceneLoaderStatus, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status::qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotSceneLoaderStatus> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status::set(void (*FN_PTR)(void*, Qt3DRender::QSceneLoader::Status) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::scene_loader::Status),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status_set(self as *mut ::slots::raw::RawSlotSceneLoaderStatus, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotSceneLoaderStatus {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QShaderProgram*)` to a Rust extern function.
  ///
  /// Use `SlotShaderProgramMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr```</span>
  #[repr(C)]
  pub struct RawSlotShaderProgramMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotShaderProgramMutPtr {
    type Arguments = (*mut ::shader_program::ShaderProgram,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QShaderProgram*)\0"
    }
  }
  impl RawSlotShaderProgramMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr::custom_slot(Qt3DRender::QShaderProgram* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::shader_program::ShaderProgram) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr_custom_slot(self as *mut ::slots::raw::RawSlotShaderProgramMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr::qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotShaderProgramMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr::set(void (*FN_PTR)(void*, Qt3DRender::QShaderProgram*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::shader_program::ShaderProgram),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr_set(self as *mut ::slots::raw::RawSlotShaderProgramMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotShaderProgramMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QShaderProgram::Status)` to a Rust extern function.
  ///
  /// Use `SlotShaderProgramStatus` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status```</span>
  #[repr(C)]
  pub struct RawSlotShaderProgramStatus(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotShaderProgramStatus {
    type Arguments = (::shader_program::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QShaderProgram::Status)\0"
    }
  }
  impl RawSlotShaderProgramStatus {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status::custom_slot(Qt3DRender::QShaderProgram::Status arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::shader_program::Status) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status_custom_slot(self as *mut ::slots::raw::RawSlotShaderProgramStatus, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status::qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotShaderProgramStatus> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status::set(void (*FN_PTR)(void*, Qt3DRender::QShaderProgram::Status) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::shader_program::Status),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status_set(self as *mut ::slots::raw::RawSlotShaderProgramStatus, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotShaderProgramStatus {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilOperationArguments::FaceMode)` to a Rust extern function.
  ///
  /// Use `SlotStencilOperationArgumentsFaceMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode```</span>
  #[repr(C)]
  pub struct RawSlotStencilOperationArgumentsFaceMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotStencilOperationArgumentsFaceMode {
    type Arguments = (::stencil_operation_arguments::FaceMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QStencilOperationArguments::FaceMode)\0"
    }
  }
  impl RawSlotStencilOperationArgumentsFaceMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode::custom_slot(Qt3DRender::QStencilOperationArguments::FaceMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::stencil_operation_arguments::FaceMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode_custom_slot(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStencilOperationArgumentsFaceMode> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode::set(void (*FN_PTR)(void*, Qt3DRender::QStencilOperationArguments::FaceMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::stencil_operation_arguments::FaceMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode_set(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStencilOperationArgumentsFaceMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilOperationArguments::Operation)` to a Rust extern function.
  ///
  /// Use `SlotStencilOperationArgumentsOperation` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation```</span>
  #[repr(C)]
  pub struct RawSlotStencilOperationArgumentsOperation(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotStencilOperationArgumentsOperation {
    type Arguments = (::stencil_operation_arguments::Operation,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QStencilOperationArguments::Operation)\0"
    }
  }
  impl RawSlotStencilOperationArgumentsOperation {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation::custom_slot(Qt3DRender::QStencilOperationArguments::Operation arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::stencil_operation_arguments::Operation) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation_custom_slot(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation::qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStencilOperationArgumentsOperation> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation::set(void (*FN_PTR)(void*, Qt3DRender::QStencilOperationArguments::Operation) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::stencil_operation_arguments::Operation),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation_set(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStencilOperationArgumentsOperation {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilTestArguments::StencilFaceMode)` to a Rust extern function.
  ///
  /// Use `SlotStencilTestArgumentsStencilFaceMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode```</span>
  #[repr(C)]
  pub struct RawSlotStencilTestArgumentsStencilFaceMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode {
    type Arguments = (::stencil_test_arguments::StencilFaceMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QStencilTestArguments::StencilFaceMode)\0"
    }
  }
  impl RawSlotStencilTestArgumentsStencilFaceMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode::custom_slot(Qt3DRender::QStencilTestArguments::StencilFaceMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::stencil_test_arguments::StencilFaceMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode_custom_slot(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode::set(void (*FN_PTR)(void*, Qt3DRender::QStencilTestArguments::StencilFaceMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::stencil_test_arguments::StencilFaceMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode_set(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilTestArguments::StencilFunction)` to a Rust extern function.
  ///
  /// Use `SlotStencilTestArgumentsStencilFunction` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction```</span>
  #[repr(C)]
  pub struct RawSlotStencilTestArgumentsStencilFunction(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotStencilTestArgumentsStencilFunction {
    type Arguments = (::stencil_test_arguments::StencilFunction,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QStencilTestArguments::StencilFunction)\0"
    }
  }
  impl RawSlotStencilTestArgumentsStencilFunction {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction::custom_slot(Qt3DRender::QStencilTestArguments::StencilFunction arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::stencil_test_arguments::StencilFunction) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction_custom_slot(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction::qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotStencilTestArgumentsStencilFunction> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction::set(void (*FN_PTR)(void*, Qt3DRender::QStencilTestArguments::StencilFunction) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::stencil_test_arguments::StencilFunction),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction_set(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotStencilTestArgumentsStencilFunction {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QTextureImage::Status)` to a Rust extern function.
  ///
  /// Use `SlotTextureImageStatus` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status```</span>
  #[repr(C)]
  pub struct RawSlotTextureImageStatus(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTextureImageStatus {
    type Arguments = (::texture_image::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QTextureImage::Status)\0"
    }
  }
  impl RawSlotTextureImageStatus {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status::custom_slot(Qt3DRender::QTextureImage::Status arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::texture_image::Status) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status_custom_slot(self as *mut ::slots::raw::RawSlotTextureImageStatus, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status::qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTextureImageStatus> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status::set(void (*FN_PTR)(void*, Qt3DRender::QTextureImage::Status) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::texture_image::Status),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status_set(self as *mut ::slots::raw::RawSlotTextureImageStatus, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTextureImageStatus {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt3DRender::QTextureWrapMode::WrapMode)` to a Rust extern function.
  ///
  /// Use `SlotTextureWrapModeWrapMode` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode```</span>
  #[repr(C)]
  pub struct RawSlotTextureWrapModeWrapMode(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTextureWrapModeWrapMode {
    type Arguments = (::texture_wrap_mode::WrapMode,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt3DRender::QTextureWrapMode::WrapMode)\0"
    }
  }
  impl RawSlotTextureWrapModeWrapMode {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode::custom_slot(Qt3DRender::QTextureWrapMode::WrapMode arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::texture_wrap_mode::WrapMode) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode_custom_slot(self as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode, arg0) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode::qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTextureWrapModeWrapMode> {
      let ffi_result =
        unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode::set(void (*FN_PTR)(void*, Qt3DRender::QTextureWrapMode::WrapMode) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::texture_wrap_mode::WrapMode),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode_set(self as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTextureWrapModeWrapMode {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QVector<Qt3DRender::QSortPolicy::SortType>&)` to a Rust extern function.
  ///
  /// Use `SlotVectorVectorSortPolicySortTypeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref```</span>
  #[repr(C)]
  pub struct RawSlotVectorVectorSortPolicySortTypeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef {
    type Arguments = (&'static ::vector::VectorSortPolicySortType,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QVector< Qt3DRender::QSortPolicy::SortType >&)\0"
    }
  }
  impl RawSlotVectorVectorSortPolicySortTypeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref::custom_slot(const QVector<Qt3DRender::QSortPolicy::SortType>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::vector::VectorSortPolicySortType) {
      unsafe { ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref_custom_slot(self as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef, arg0 as *const ::vector::VectorSortPolicySortType) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref::qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef> {
      let ffi_result =
        unsafe {
          ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref::set(void (*FN_PTR)(void*, const QVector< Qt3DRender::QSortPolicy::SortType >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::vector::VectorSortPolicySortType),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref_set(self as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_3d_render_c_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreSizeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QSize(self as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QSize(self as *const ::slots::raw::RawSlotQtCoreSizeRef as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QVector3D(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QVector3D(self as *const ::slots::raw::RawSlotQtGuiVector3DRef as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureComparisonFunction {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction(self as *const ::slots::raw::RawSlotAbstractTextureComparisonFunction as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureComparisonMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode(self as *const ::slots::raw::RawSlotAbstractTextureComparisonMode as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace(self as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace(self as *const ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureFilter {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter(self as *mut ::slots::raw::RawSlotAbstractTextureFilter) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter(self as *const ::slots::raw::RawSlotAbstractTextureFilter as *mut ::slots::raw::RawSlotAbstractTextureFilter) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureStatus {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status(self as *mut ::slots::raw::RawSlotAbstractTextureStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status(self as *const ::slots::raw::RawSlotAbstractTextureStatus as *mut ::slots::raw::RawSlotAbstractTextureStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureTextureFormat {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat(self as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat(self as *const ::slots::raw::RawSlotAbstractTextureTextureFormat as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractTextureMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr(self as *mut ::slots::raw::RawSlotAbstractTextureMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr(self as *const ::slots::raw::RawSlotAbstractTextureMutPtr as *mut ::slots::raw::RawSlotAbstractTextureMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAlphaTestAlphaFunction {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction(self as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction(self as *const ::slots::raw::RawSlotAlphaTestAlphaFunction as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAttributeAttributeType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType(self as *mut ::slots::raw::RawSlotAttributeAttributeType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType(self as *const ::slots::raw::RawSlotAttributeAttributeType as *mut ::slots::raw::RawSlotAttributeAttributeType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAttributeVertexBaseType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType(self as *mut ::slots::raw::RawSlotAttributeVertexBaseType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType(self as *const ::slots::raw::RawSlotAttributeVertexBaseType as *mut ::slots::raw::RawSlotAttributeVertexBaseType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAttributeMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr(self as *mut ::slots::raw::RawSlotAttributeMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr(self as *const ::slots::raw::RawSlotAttributeMutPtr as *mut ::slots::raw::RawSlotAttributeMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBlendEquationArgumentsBlending {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *const ::slots::raw::RawSlotBlendEquationArgumentsBlending as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBlendEquationBlendFunction {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *mut ::slots::raw::RawSlotBlendEquationBlendFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *const ::slots::raw::RawSlotBlendEquationBlendFunction as *mut ::slots::raw::RawSlotBlendEquationBlendFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBufferAccessType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType(self as *mut ::slots::raw::RawSlotBufferAccessType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType(self as *const ::slots::raw::RawSlotBufferAccessType as *mut ::slots::raw::RawSlotBufferAccessType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBufferBufferType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType(self as *mut ::slots::raw::RawSlotBufferBufferType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType(self as *const ::slots::raw::RawSlotBufferBufferType as *mut ::slots::raw::RawSlotBufferBufferType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBufferUsageType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType(self as *mut ::slots::raw::RawSlotBufferUsageType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType(self as *const ::slots::raw::RawSlotBufferUsageType as *mut ::slots::raw::RawSlotBufferUsageType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotBufferMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr(self as *mut ::slots::raw::RawSlotBufferMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr(self as *const ::slots::raw::RawSlotBufferMutPtr as *mut ::slots::raw::RawSlotBufferMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCameraLensProjectionTypeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType(self as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType(self as *const ::slots::raw::RawSlotCameraLensProjectionTypeRef as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCameraMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr(self as *mut ::slots::raw::RawSlotCameraMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr(self as *const ::slots::raw::RawSlotCameraMutPtr as *mut ::slots::raw::RawSlotCameraMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotClearBuffersBufferType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType(self as *mut ::slots::raw::RawSlotClearBuffersBufferType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType(self as *const ::slots::raw::RawSlotClearBuffersBufferType as *mut ::slots::raw::RawSlotClearBuffersBufferType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCullFaceCullingMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode(self as *mut ::slots::raw::RawSlotCullFaceCullingMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode(self as *const ::slots::raw::RawSlotCullFaceCullingMode as *mut ::slots::raw::RawSlotCullFaceCullingMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotDepthTestDepthFunction {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction(self as *mut ::slots::raw::RawSlotDepthTestDepthFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction(self as *const ::slots::raw::RawSlotDepthTestDepthFunction as *mut ::slots::raw::RawSlotDepthTestDepthFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotEffectMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr(self as *mut ::slots::raw::RawSlotEffectMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr(self as *const ::slots::raw::RawSlotEffectMutPtr as *mut ::slots::raw::RawSlotEffectMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotFrameGraphNodeMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr(self as *const ::slots::raw::RawSlotFrameGraphNodeMutPtr as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotFrontFaceWindingDirection {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection(self as *mut ::slots::raw::RawSlotFrontFaceWindingDirection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection(self as *const ::slots::raw::RawSlotFrontFaceWindingDirection as *mut ::slots::raw::RawSlotFrontFaceWindingDirection) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGeometryRendererPrimitiveType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType(self as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType(self as *const ::slots::raw::RawSlotGeometryRendererPrimitiveType as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGeometryMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr(self as *mut ::slots::raw::RawSlotGeometryMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr(self as *const ::slots::raw::RawSlotGeometryMutPtr as *mut ::slots::raw::RawSlotGeometryMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGraphicsApiFilterApiRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api(self as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api(self as *const ::slots::raw::RawSlotGraphicsApiFilterApiRef as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile(self as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile(self as *const ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotLevelOfDetailThresholdType {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType(self as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType(self as *const ::slots::raw::RawSlotLevelOfDetailThresholdType as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPickEventMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr(self as *mut ::slots::raw::RawSlotPickEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr(self as *const ::slots::raw::RawSlotPickEventMutPtr as *mut ::slots::raw::RawSlotPickEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode(self as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode(self as *const ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPickingSettingsPickMethodRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod(self as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod(self as *const ::slots::raw::RawSlotPickingSettingsPickMethodRef as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPickingSettingsPickResultModeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode(self as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode(self as *const ::slots::raw::RawSlotPickingSettingsPickResultModeRef as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotPointSizeSizeMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode(self as *mut ::slots::raw::RawSlotPointSizeSizeMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode(self as *const ::slots::raw::RawSlotPointSizeSizeMode as *mut ::slots::raw::RawSlotPointSizeSizeMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotRenderSettingsRenderPolicy {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy(self as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy(self as *const ::slots::raw::RawSlotRenderSettingsRenderPolicy as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint(self as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint(self as *const ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotRenderTargetOutputMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr(self as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr(self as *const ::slots::raw::RawSlotRenderTargetOutputMutPtr as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotRenderTargetMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr(self as *mut ::slots::raw::RawSlotRenderTargetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr(self as *const ::slots::raw::RawSlotRenderTargetMutPtr as *mut ::slots::raw::RawSlotRenderTargetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotSceneLoaderStatus {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status(self as *mut ::slots::raw::RawSlotSceneLoaderStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status(self as *const ::slots::raw::RawSlotSceneLoaderStatus as *mut ::slots::raw::RawSlotSceneLoaderStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotShaderProgramStatus {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status(self as *mut ::slots::raw::RawSlotShaderProgramStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status(self as *const ::slots::raw::RawSlotShaderProgramStatus as *mut ::slots::raw::RawSlotShaderProgramStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotShaderProgramMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr(self as *mut ::slots::raw::RawSlotShaderProgramMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr(self as *const ::slots::raw::RawSlotShaderProgramMutPtr as *mut ::slots::raw::RawSlotShaderProgramMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotStencilOperationArgumentsFaceMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode(self as *const ::slots::raw::RawSlotStencilOperationArgumentsFaceMode as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotStencilOperationArgumentsOperation {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation(self as *const ::slots::raw::RawSlotStencilOperationArgumentsOperation as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode(self as *const ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotStencilTestArgumentsStencilFunction {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction(self as *const ::slots::raw::RawSlotStencilTestArgumentsStencilFunction as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTextureImageStatus {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status(self as *mut ::slots::raw::RawSlotTextureImageStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status(self as *const ::slots::raw::RawSlotTextureImageStatus as *mut ::slots::raw::RawSlotTextureImageStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTextureWrapModeWrapMode {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode(self as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode(self as *const ::slots::raw::RawSlotTextureWrapModeWrapMode as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreByteArrayRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QByteArray_ref(self as *mut ::slots::raw::RawSlotQtCoreByteArrayRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QByteArray_ref(self as *const ::slots::raw::RawSlotQtCoreByteArrayRef as *mut ::slots::raw::RawSlotQtCoreByteArrayRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiColorRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QColor_ref(self as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QColor_ref(self as *const ::slots::raw::RawSlotQtGuiColorRef as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref(self as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref(self as *const ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreStringListRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QStringList_ref(self as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QStringList_ref(self as *const ::slots::raw::RawSlotQtCoreStringListRef as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref(self as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref(self as *const ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_double_ref(self as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_double_ref(self as *const ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_int_ref(self as *const ::slots::raw::RawSlotQtCoreVectorVectorCIntRef as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref(self as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref(self as *const ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCUint {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_unsigned_int(self as *mut ::slots::raw::RawSlotCUint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_unsigned_int(self as *const ::slots::raw::RawSlotCUint as *mut ::slots::raw::RawSlotCUint) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreSizeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QSize(self as *const ::slots::raw::RawSlotQtCoreSizeRef as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiVector3DRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QVector3D(self as *const ::slots::raw::RawSlotQtGuiVector3DRef as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureComparisonFunction {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction(self as *const ::slots::raw::RawSlotAbstractTextureComparisonFunction as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureComparisonMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode(self as *const ::slots::raw::RawSlotAbstractTextureComparisonMode as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace(self as *const ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureFilter {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter(self as *const ::slots::raw::RawSlotAbstractTextureFilter as *mut ::slots::raw::RawSlotAbstractTextureFilter) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureStatus {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status(self as *const ::slots::raw::RawSlotAbstractTextureStatus as *mut ::slots::raw::RawSlotAbstractTextureStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureTextureFormat {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat(self as *const ::slots::raw::RawSlotAbstractTextureTextureFormat as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractTextureMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr(self as *const ::slots::raw::RawSlotAbstractTextureMutPtr as *mut ::slots::raw::RawSlotAbstractTextureMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAlphaTestAlphaFunction {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction(self as *const ::slots::raw::RawSlotAlphaTestAlphaFunction as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAttributeAttributeType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType(self as *const ::slots::raw::RawSlotAttributeAttributeType as *mut ::slots::raw::RawSlotAttributeAttributeType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAttributeVertexBaseType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType(self as *const ::slots::raw::RawSlotAttributeVertexBaseType as *mut ::slots::raw::RawSlotAttributeVertexBaseType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAttributeMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr(self as *const ::slots::raw::RawSlotAttributeMutPtr as *mut ::slots::raw::RawSlotAttributeMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBlendEquationArgumentsBlending {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *const ::slots::raw::RawSlotBlendEquationArgumentsBlending as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBlendEquationBlendFunction {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *const ::slots::raw::RawSlotBlendEquationBlendFunction as *mut ::slots::raw::RawSlotBlendEquationBlendFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBufferAccessType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType(self as *const ::slots::raw::RawSlotBufferAccessType as *mut ::slots::raw::RawSlotBufferAccessType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBufferBufferType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType(self as *const ::slots::raw::RawSlotBufferBufferType as *mut ::slots::raw::RawSlotBufferBufferType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBufferUsageType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType(self as *const ::slots::raw::RawSlotBufferUsageType as *mut ::slots::raw::RawSlotBufferUsageType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotBufferMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr(self as *const ::slots::raw::RawSlotBufferMutPtr as *mut ::slots::raw::RawSlotBufferMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCameraLensProjectionTypeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType(self as *const ::slots::raw::RawSlotCameraLensProjectionTypeRef as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCameraMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr(self as *const ::slots::raw::RawSlotCameraMutPtr as *mut ::slots::raw::RawSlotCameraMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotClearBuffersBufferType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType(self as *const ::slots::raw::RawSlotClearBuffersBufferType as *mut ::slots::raw::RawSlotClearBuffersBufferType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCullFaceCullingMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode(self as *const ::slots::raw::RawSlotCullFaceCullingMode as *mut ::slots::raw::RawSlotCullFaceCullingMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotDepthTestDepthFunction {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction(self as *const ::slots::raw::RawSlotDepthTestDepthFunction as *mut ::slots::raw::RawSlotDepthTestDepthFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotEffectMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr(self as *const ::slots::raw::RawSlotEffectMutPtr as *mut ::slots::raw::RawSlotEffectMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotFrameGraphNodeMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr(self as *const ::slots::raw::RawSlotFrameGraphNodeMutPtr as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotFrontFaceWindingDirection {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection(self as *const ::slots::raw::RawSlotFrontFaceWindingDirection as *mut ::slots::raw::RawSlotFrontFaceWindingDirection) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGeometryRendererPrimitiveType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType(self as *const ::slots::raw::RawSlotGeometryRendererPrimitiveType as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGeometryMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr(self as *const ::slots::raw::RawSlotGeometryMutPtr as *mut ::slots::raw::RawSlotGeometryMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGraphicsApiFilterApiRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api(self as *const ::slots::raw::RawSlotGraphicsApiFilterApiRef as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile(self as *const ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotLevelOfDetailThresholdType {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType(self as *const ::slots::raw::RawSlotLevelOfDetailThresholdType as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPickEventMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr(self as *const ::slots::raw::RawSlotPickEventMutPtr as *mut ::slots::raw::RawSlotPickEventMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode(self as *const ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPickingSettingsPickMethodRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod(self as *const ::slots::raw::RawSlotPickingSettingsPickMethodRef as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPickingSettingsPickResultModeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode(self as *const ::slots::raw::RawSlotPickingSettingsPickResultModeRef as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotPointSizeSizeMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode(self as *const ::slots::raw::RawSlotPointSizeSizeMode as *mut ::slots::raw::RawSlotPointSizeSizeMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotRenderSettingsRenderPolicy {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy(self as *const ::slots::raw::RawSlotRenderSettingsRenderPolicy as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint(self as *const ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotRenderTargetOutputMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr(self as *const ::slots::raw::RawSlotRenderTargetOutputMutPtr as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotRenderTargetMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr(self as *const ::slots::raw::RawSlotRenderTargetMutPtr as *mut ::slots::raw::RawSlotRenderTargetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotSceneLoaderStatus {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status(self as *const ::slots::raw::RawSlotSceneLoaderStatus as *mut ::slots::raw::RawSlotSceneLoaderStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotShaderProgramStatus {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status(self as *const ::slots::raw::RawSlotShaderProgramStatus as *mut ::slots::raw::RawSlotShaderProgramStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotShaderProgramMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr(self as *const ::slots::raw::RawSlotShaderProgramMutPtr as *mut ::slots::raw::RawSlotShaderProgramMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStencilOperationArgumentsFaceMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode(self as *const ::slots::raw::RawSlotStencilOperationArgumentsFaceMode as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStencilOperationArgumentsOperation {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation(self as *const ::slots::raw::RawSlotStencilOperationArgumentsOperation as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode(self as *const ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotStencilTestArgumentsStencilFunction {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction(self as *const ::slots::raw::RawSlotStencilTestArgumentsStencilFunction as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTextureImageStatus {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status(self as *const ::slots::raw::RawSlotTextureImageStatus as *mut ::slots::raw::RawSlotTextureImageStatus) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTextureWrapModeWrapMode {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode(self as *const ::slots::raw::RawSlotTextureWrapModeWrapMode as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreByteArrayRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QByteArray_ref(self as *const ::slots::raw::RawSlotQtCoreByteArrayRef as *mut ::slots::raw::RawSlotQtCoreByteArrayRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiColorRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QColor_ref(self as *const ::slots::raw::RawSlotQtGuiColorRef as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref(self as *const ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreStringListRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QStringList_ref(self as *const ::slots::raw::RawSlotQtCoreStringListRef as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref(self as *const ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_double_ref(self as *const ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_int_ref(self as *const ::slots::raw::RawSlotQtCoreVectorVectorCIntRef as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref(self as *const ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCUint {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_unsigned_int(self as *const ::slots::raw::RawSlotCUint as *mut ::slots::raw::RawSlotCUint) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreSizeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QSize(self as *mut ::slots::raw::RawSlotQtCoreSizeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiVector3DRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_QVector3D(self as *mut ::slots::raw::RawSlotQtGuiVector3DRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureComparisonFunction {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonFunction(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureComparisonMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ComparisonMode(self as *mut ::slots::raw::RawSlotAbstractTextureComparisonMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_CubeMapFace(self as *mut ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureFilter {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Filter(self as *mut ::slots::raw::RawSlotAbstractTextureFilter) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureStatus {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_Status(self as *mut ::slots::raw::RawSlotAbstractTextureStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureTextureFormat {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_TextureFormat(self as *mut ::slots::raw::RawSlotAbstractTextureTextureFormat) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractTextureMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAbstractTexture_ptr(self as *mut ::slots::raw::RawSlotAbstractTextureMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAlphaTestAlphaFunction {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAlphaTest_AlphaFunction(self as *mut ::slots::raw::RawSlotAlphaTestAlphaFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAttributeAttributeType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_AttributeType(self as *mut ::slots::raw::RawSlotAttributeAttributeType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAttributeVertexBaseType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_VertexBaseType(self as *mut ::slots::raw::RawSlotAttributeVertexBaseType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAttributeMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QAttribute_ptr(self as *mut ::slots::raw::RawSlotAttributeMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBlendEquationArgumentsBlending {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquationArguments_Blending(self as *mut ::slots::raw::RawSlotBlendEquationArgumentsBlending) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBlendEquationBlendFunction {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBlendEquation_BlendFunction(self as *mut ::slots::raw::RawSlotBlendEquationBlendFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBufferAccessType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_AccessType(self as *mut ::slots::raw::RawSlotBufferAccessType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBufferBufferType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_BufferType(self as *mut ::slots::raw::RawSlotBufferBufferType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBufferUsageType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_UsageType(self as *mut ::slots::raw::RawSlotBufferUsageType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotBufferMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QBuffer_ptr(self as *mut ::slots::raw::RawSlotBufferMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCameraLensProjectionTypeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCameraLens_ProjectionType(self as *mut ::slots::raw::RawSlotCameraLensProjectionTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCameraMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCamera_ptr(self as *mut ::slots::raw::RawSlotCameraMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotClearBuffersBufferType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QClearBuffers_BufferType(self as *mut ::slots::raw::RawSlotClearBuffersBufferType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCullFaceCullingMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QCullFace_CullingMode(self as *mut ::slots::raw::RawSlotCullFaceCullingMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotDepthTestDepthFunction {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QDepthTest_DepthFunction(self as *mut ::slots::raw::RawSlotDepthTestDepthFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotEffectMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QEffect_ptr(self as *mut ::slots::raw::RawSlotEffectMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotFrameGraphNodeMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrameGraphNode_ptr(self as *mut ::slots::raw::RawSlotFrameGraphNodeMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotFrontFaceWindingDirection {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QFrontFace_WindingDirection(self as *mut ::slots::raw::RawSlotFrontFaceWindingDirection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGeometryRendererPrimitiveType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometryRenderer_PrimitiveType(self as *mut ::slots::raw::RawSlotGeometryRendererPrimitiveType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGeometryMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGeometry_ptr(self as *mut ::slots::raw::RawSlotGeometryMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGraphicsApiFilterApiRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_Api(self as *mut ::slots::raw::RawSlotGraphicsApiFilterApiRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QGraphicsApiFilter_OpenGLProfile(self as *mut ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotLevelOfDetailThresholdType {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QLevelOfDetail_ThresholdType(self as *mut ::slots::raw::RawSlotLevelOfDetailThresholdType) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPickEventMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickEvent_ptr(self as *mut ::slots::raw::RawSlotPickEventMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_FaceOrientationPickingMode(self as *mut ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPickingSettingsPickMethodRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickMethod(self as *mut ::slots::raw::RawSlotPickingSettingsPickMethodRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPickingSettingsPickResultModeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPickingSettings_PickResultMode(self as *mut ::slots::raw::RawSlotPickingSettingsPickResultModeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotPointSizeSizeMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QPointSize_SizeMode(self as *mut ::slots::raw::RawSlotPointSizeSizeMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotRenderSettingsRenderPolicy {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderSettings_RenderPolicy(self as *mut ::slots::raw::RawSlotRenderSettingsRenderPolicy) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_AttachmentPoint(self as *mut ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotRenderTargetOutputMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTargetOutput_ptr(self as *mut ::slots::raw::RawSlotRenderTargetOutputMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotRenderTargetMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QRenderTarget_ptr(self as *mut ::slots::raw::RawSlotRenderTargetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotSceneLoaderStatus {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QSceneLoader_Status(self as *mut ::slots::raw::RawSlotSceneLoaderStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotShaderProgramStatus {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_Status(self as *mut ::slots::raw::RawSlotShaderProgramStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotShaderProgramMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QShaderProgram_ptr(self as *mut ::slots::raw::RawSlotShaderProgramMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStencilOperationArgumentsFaceMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_FaceMode(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsFaceMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStencilOperationArgumentsOperation {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilOperationArguments_Operation(self as *mut ::slots::raw::RawSlotStencilOperationArgumentsOperation) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFaceMode(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotStencilTestArgumentsStencilFunction {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QStencilTestArguments_StencilFunction(self as *mut ::slots::raw::RawSlotStencilTestArgumentsStencilFunction) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTextureImageStatus {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureImage_Status(self as *mut ::slots::raw::RawSlotTextureImageStatus) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTextureWrapModeWrapMode {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_Qt3DRender_QTextureWrapMode_WrapMode(self as *mut ::slots::raw::RawSlotTextureWrapModeWrapMode) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreByteArrayRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QByteArray_ref(self as *mut ::slots::raw::RawSlotQtCoreByteArrayRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiColorRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QColor_ref(self as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QMatrix4x4_ref(self as *mut ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreStringListRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QStringList_ref(self as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_Qt3DRender_QSortPolicy_SortType_ref(self as *mut ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_double_ref(self as *mut ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreVectorVectorCIntRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_QVector_int_ref(self as *mut ::slots::raw::RawSlotQtCoreVectorVectorCIntRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_const_Qt3DRender_QLevelOfDetailBoundingSphere_ref(self as *mut ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCUint {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_3d_render_c_slots_G_static_cast_QObject_ptr_qt_3d_render_c_SlotWrapper_unsigned_int(self as *mut ::slots::raw::RawSlotCUint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::ComparisonFunction)` to a Rust closure.
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

pub struct SlotAbstractTextureComparisonFunction<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureComparisonFunction>,
  func: ::std::option::Option<Box<Box<FnMut(::abstract_texture::ComparisonFunction) + 'a>>>,
}

impl<'a> SlotAbstractTextureComparisonFunction<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::abstract_texture::ComparisonFunction) + 'a>(f: F) -> SlotAbstractTextureComparisonFunction<'a> {
    let mut obj = SlotAbstractTextureComparisonFunction::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::abstract_texture::ComparisonFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::abstract_texture::ComparisonFunction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_comparison_function_callback,
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

impl<'a> Default for SlotAbstractTextureComparisonFunction<'a> {
  fn default() -> Self {
    SlotAbstractTextureComparisonFunction {
      wrapper: ::slots::raw::RawSlotAbstractTextureComparisonFunction::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureComparisonFunction<'a> {
  type Arguments = (::abstract_texture::ComparisonFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureComparisonFunction as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_comparison_function_callback(data: *mut ::libc::c_void,
                                                                 arg0: ::abstract_texture::ComparisonFunction) {
  let func: &mut Box<FnMut(::abstract_texture::ComparisonFunction)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::ComparisonMode)` to a Rust closure.
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

pub struct SlotAbstractTextureComparisonMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureComparisonMode>,
  func: ::std::option::Option<Box<Box<FnMut(::abstract_texture::ComparisonMode) + 'a>>>,
}

impl<'a> SlotAbstractTextureComparisonMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::abstract_texture::ComparisonMode) + 'a>(f: F) -> SlotAbstractTextureComparisonMode<'a> {
    let mut obj = SlotAbstractTextureComparisonMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::abstract_texture::ComparisonMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::abstract_texture::ComparisonMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_comparison_mode_callback,
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

impl<'a> Default for SlotAbstractTextureComparisonMode<'a> {
  fn default() -> Self {
    SlotAbstractTextureComparisonMode {
      wrapper: ::slots::raw::RawSlotAbstractTextureComparisonMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureComparisonMode<'a> {
  type Arguments = (::abstract_texture::ComparisonMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureComparisonMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_comparison_mode_callback(data: *mut ::libc::c_void,
                                                             arg0: ::abstract_texture::ComparisonMode) {
  let func: &mut Box<FnMut(::abstract_texture::ComparisonMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::CubeMapFace)` to a Rust closure.
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

pub struct SlotAbstractTextureCubeMapFaceRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureCubeMapFaceRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::abstract_texture::CubeMapFace) + 'a>>>,
}

impl<'a> SlotAbstractTextureCubeMapFaceRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::abstract_texture::CubeMapFace) + 'a>(f: F) -> SlotAbstractTextureCubeMapFaceRef<'a> {
    let mut obj = SlotAbstractTextureCubeMapFaceRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::abstract_texture::CubeMapFace) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::abstract_texture::CubeMapFace) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_cube_map_face_ref_callback,
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

impl<'a> Default for SlotAbstractTextureCubeMapFaceRef<'a> {
  fn default() -> Self {
    SlotAbstractTextureCubeMapFaceRef {
      wrapper: ::slots::raw::RawSlotAbstractTextureCubeMapFaceRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureCubeMapFaceRef<'a> {
  type Arguments = (&'static ::abstract_texture::CubeMapFace,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureCubeMapFaceRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_cube_map_face_ref_callback(data: *mut ::libc::c_void,
                                                               arg0: *const ::abstract_texture::CubeMapFace) {
  let func: &mut Box<FnMut(&'static ::abstract_texture::CubeMapFace)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::Filter)` to a Rust closure.
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

pub struct SlotAbstractTextureFilter<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureFilter>,
  func: ::std::option::Option<Box<Box<FnMut(::abstract_texture::Filter) + 'a>>>,
}

impl<'a> SlotAbstractTextureFilter<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::abstract_texture::Filter) + 'a>(f: F) -> SlotAbstractTextureFilter<'a> {
    let mut obj = SlotAbstractTextureFilter::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::abstract_texture::Filter) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::abstract_texture::Filter) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_filter_callback,
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

impl<'a> Default for SlotAbstractTextureFilter<'a> {
  fn default() -> Self {
    SlotAbstractTextureFilter {
      wrapper: ::slots::raw::RawSlotAbstractTextureFilter::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureFilter<'a> {
  type Arguments = (::abstract_texture::Filter,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureFilter as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_filter_callback(data: *mut ::libc::c_void, arg0: ::abstract_texture::Filter) {
  let func: &mut Box<FnMut(::abstract_texture::Filter)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture*)` to a Rust closure.
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

pub struct SlotAbstractTextureMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_texture::AbstractTexture) + 'a>>>,
}

impl<'a> SlotAbstractTextureMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_texture::AbstractTexture) + 'a>(f: F) -> SlotAbstractTextureMutPtr<'a> {
    let mut obj = SlotAbstractTextureMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_texture::AbstractTexture) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_texture::AbstractTexture) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_mut_ptr_callback,
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

impl<'a> Default for SlotAbstractTextureMutPtr<'a> {
  fn default() -> Self {
    SlotAbstractTextureMutPtr {
      wrapper: ::slots::raw::RawSlotAbstractTextureMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureMutPtr<'a> {
  type Arguments = (*mut ::abstract_texture::AbstractTexture,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_mut_ptr_callback(data: *mut ::libc::c_void,
                                                     arg0: *mut ::abstract_texture::AbstractTexture) {
  let func: &mut Box<FnMut(*mut ::abstract_texture::AbstractTexture)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::Status)` to a Rust closure.
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

pub struct SlotAbstractTextureStatus<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureStatus>,
  func: ::std::option::Option<Box<Box<FnMut(::abstract_texture::Status) + 'a>>>,
}

impl<'a> SlotAbstractTextureStatus<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::abstract_texture::Status) + 'a>(f: F) -> SlotAbstractTextureStatus<'a> {
    let mut obj = SlotAbstractTextureStatus::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::abstract_texture::Status) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::abstract_texture::Status) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_status_callback,
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

impl<'a> Default for SlotAbstractTextureStatus<'a> {
  fn default() -> Self {
    SlotAbstractTextureStatus {
      wrapper: ::slots::raw::RawSlotAbstractTextureStatus::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureStatus<'a> {
  type Arguments = (::abstract_texture::Status,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureStatus as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_status_callback(data: *mut ::libc::c_void, arg0: ::abstract_texture::Status) {
  let func: &mut Box<FnMut(::abstract_texture::Status)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAbstractTexture::TextureFormat)` to a Rust closure.
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

pub struct SlotAbstractTextureTextureFormat<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractTextureTextureFormat>,
  func: ::std::option::Option<Box<Box<FnMut(::abstract_texture::TextureFormat) + 'a>>>,
}

impl<'a> SlotAbstractTextureTextureFormat<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::abstract_texture::TextureFormat) + 'a>(f: F) -> SlotAbstractTextureTextureFormat<'a> {
    let mut obj = SlotAbstractTextureTextureFormat::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::abstract_texture::TextureFormat) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::abstract_texture::TextureFormat) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_texture_texture_format_callback,
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

impl<'a> Default for SlotAbstractTextureTextureFormat<'a> {
  fn default() -> Self {
    SlotAbstractTextureTextureFormat {
      wrapper: ::slots::raw::RawSlotAbstractTextureTextureFormat::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractTextureTextureFormat<'a> {
  type Arguments = (::abstract_texture::TextureFormat,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractTextureTextureFormat as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_texture_texture_format_callback(data: *mut ::libc::c_void,
                                                            arg0: ::abstract_texture::TextureFormat) {
  let func: &mut Box<FnMut(::abstract_texture::TextureFormat)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAlphaTest::AlphaFunction)` to a Rust closure.
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

pub struct SlotAlphaTestAlphaFunction<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAlphaTestAlphaFunction>,
  func: ::std::option::Option<Box<Box<FnMut(::alpha_test::AlphaFunction) + 'a>>>,
}

impl<'a> SlotAlphaTestAlphaFunction<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::alpha_test::AlphaFunction) + 'a>(f: F) -> SlotAlphaTestAlphaFunction<'a> {
    let mut obj = SlotAlphaTestAlphaFunction::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::alpha_test::AlphaFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::alpha_test::AlphaFunction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_alpha_test_alpha_function_callback,
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

impl<'a> Default for SlotAlphaTestAlphaFunction<'a> {
  fn default() -> Self {
    SlotAlphaTestAlphaFunction {
      wrapper: ::slots::raw::RawSlotAlphaTestAlphaFunction::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAlphaTestAlphaFunction<'a> {
  type Arguments = (::alpha_test::AlphaFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAlphaTestAlphaFunction as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_alpha_test_alpha_function_callback(data: *mut ::libc::c_void, arg0: ::alpha_test::AlphaFunction) {
  let func: &mut Box<FnMut(::alpha_test::AlphaFunction)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute::AttributeType)` to a Rust closure.
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

pub struct SlotAttributeAttributeType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeAttributeType>,
  func: ::std::option::Option<Box<Box<FnMut(::attribute::AttributeType) + 'a>>>,
}

impl<'a> SlotAttributeAttributeType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::attribute::AttributeType) + 'a>(f: F) -> SlotAttributeAttributeType<'a> {
    let mut obj = SlotAttributeAttributeType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::attribute::AttributeType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::attribute::AttributeType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_attribute_attribute_type_callback,
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

impl<'a> Default for SlotAttributeAttributeType<'a> {
  fn default() -> Self {
    SlotAttributeAttributeType {
      wrapper: ::slots::raw::RawSlotAttributeAttributeType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAttributeAttributeType<'a> {
  type Arguments = (::attribute::AttributeType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAttributeAttributeType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_attribute_attribute_type_callback(data: *mut ::libc::c_void, arg0: ::attribute::AttributeType) {
  let func: &mut Box<FnMut(::attribute::AttributeType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute*)` to a Rust closure.
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

pub struct SlotAttributeMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::attribute::Attribute) + 'a>>>,
}

impl<'a> SlotAttributeMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::attribute::Attribute) + 'a>(f: F) -> SlotAttributeMutPtr<'a> {
    let mut obj = SlotAttributeMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::attribute::Attribute) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::attribute::Attribute) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_attribute_mut_ptr_callback,
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

impl<'a> Default for SlotAttributeMutPtr<'a> {
  fn default() -> Self {
    SlotAttributeMutPtr {
      wrapper: ::slots::raw::RawSlotAttributeMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAttributeMutPtr<'a> {
  type Arguments = (*mut ::attribute::Attribute,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAttributeMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_attribute_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::attribute::Attribute) {
  let func: &mut Box<FnMut(*mut ::attribute::Attribute)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QAttribute::VertexBaseType)` to a Rust closure.
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

pub struct SlotAttributeVertexBaseType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAttributeVertexBaseType>,
  func: ::std::option::Option<Box<Box<FnMut(::attribute::VertexBaseType) + 'a>>>,
}

impl<'a> SlotAttributeVertexBaseType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::attribute::VertexBaseType) + 'a>(f: F) -> SlotAttributeVertexBaseType<'a> {
    let mut obj = SlotAttributeVertexBaseType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::attribute::VertexBaseType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::attribute::VertexBaseType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_attribute_vertex_base_type_callback,
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

impl<'a> Default for SlotAttributeVertexBaseType<'a> {
  fn default() -> Self {
    SlotAttributeVertexBaseType {
      wrapper: ::slots::raw::RawSlotAttributeVertexBaseType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAttributeVertexBaseType<'a> {
  type Arguments = (::attribute::VertexBaseType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAttributeVertexBaseType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_attribute_vertex_base_type_callback(data: *mut ::libc::c_void, arg0: ::attribute::VertexBaseType) {
  let func: &mut Box<FnMut(::attribute::VertexBaseType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
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

pub struct SlotBlendEquationArgumentsBlending<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBlendEquationArgumentsBlending>,
  func: ::std::option::Option<Box<Box<FnMut(::blend_equation_arguments::Blending) + 'a>>>,
}

impl<'a> SlotBlendEquationArgumentsBlending<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::blend_equation_arguments::Blending) + 'a>(f: F) -> SlotBlendEquationArgumentsBlending<'a> {
    let mut obj = SlotBlendEquationArgumentsBlending::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::blend_equation_arguments::Blending) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::blend_equation_arguments::Blending) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_blend_equation_arguments_blending_callback,
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

impl<'a> Default for SlotBlendEquationArgumentsBlending<'a> {
  fn default() -> Self {
    SlotBlendEquationArgumentsBlending {
      wrapper: ::slots::raw::RawSlotBlendEquationArgumentsBlending::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBlendEquationArgumentsBlending<'a> {
  type Arguments = (::blend_equation_arguments::Blending,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBlendEquationArgumentsBlending as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_blend_equation_arguments_blending_callback(data: *mut ::libc::c_void,
                                                              arg0: ::blend_equation_arguments::Blending) {
  let func: &mut Box<FnMut(::blend_equation_arguments::Blending)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
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

pub struct SlotBlendEquationBlendFunction<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBlendEquationBlendFunction>,
  func: ::std::option::Option<Box<Box<FnMut(::blend_equation::BlendFunction) + 'a>>>,
}

impl<'a> SlotBlendEquationBlendFunction<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::blend_equation::BlendFunction) + 'a>(f: F) -> SlotBlendEquationBlendFunction<'a> {
    let mut obj = SlotBlendEquationBlendFunction::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::blend_equation::BlendFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::blend_equation::BlendFunction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_blend_equation_blend_function_callback,
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

impl<'a> Default for SlotBlendEquationBlendFunction<'a> {
  fn default() -> Self {
    SlotBlendEquationBlendFunction {
      wrapper: ::slots::raw::RawSlotBlendEquationBlendFunction::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBlendEquationBlendFunction<'a> {
  type Arguments = (::blend_equation::BlendFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBlendEquationBlendFunction as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_blend_equation_blend_function_callback(data: *mut ::libc::c_void,
                                                          arg0: ::blend_equation::BlendFunction) {
  let func: &mut Box<FnMut(::blend_equation::BlendFunction)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::AccessType)` to a Rust closure.
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

pub struct SlotBufferAccessType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBufferAccessType>,
  func: ::std::option::Option<Box<Box<FnMut(::buffer::AccessType) + 'a>>>,
}

impl<'a> SlotBufferAccessType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::buffer::AccessType) + 'a>(f: F) -> SlotBufferAccessType<'a> {
    let mut obj = SlotBufferAccessType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::buffer::AccessType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::buffer::AccessType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_buffer_access_type_callback,
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

impl<'a> Default for SlotBufferAccessType<'a> {
  fn default() -> Self {
    SlotBufferAccessType {
      wrapper: ::slots::raw::RawSlotBufferAccessType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBufferAccessType<'a> {
  type Arguments = (::buffer::AccessType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBufferAccessType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_buffer_access_type_callback(data: *mut ::libc::c_void, arg0: ::buffer::AccessType) {
  let func: &mut Box<FnMut(::buffer::AccessType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::BufferType)` to a Rust closure.
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

pub struct SlotBufferBufferType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBufferBufferType>,
  func: ::std::option::Option<Box<Box<FnMut(::buffer::BufferType) + 'a>>>,
}

impl<'a> SlotBufferBufferType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::buffer::BufferType) + 'a>(f: F) -> SlotBufferBufferType<'a> {
    let mut obj = SlotBufferBufferType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::buffer::BufferType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::buffer::BufferType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_buffer_buffer_type_callback,
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

impl<'a> Default for SlotBufferBufferType<'a> {
  fn default() -> Self {
    SlotBufferBufferType {
      wrapper: ::slots::raw::RawSlotBufferBufferType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBufferBufferType<'a> {
  type Arguments = (::buffer::BufferType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBufferBufferType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_buffer_buffer_type_callback(data: *mut ::libc::c_void, arg0: ::buffer::BufferType) {
  let func: &mut Box<FnMut(::buffer::BufferType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer*)` to a Rust closure.
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

pub struct SlotBufferMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBufferMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::buffer::Buffer) + 'a>>>,
}

impl<'a> SlotBufferMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::buffer::Buffer) + 'a>(f: F) -> SlotBufferMutPtr<'a> {
    let mut obj = SlotBufferMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::buffer::Buffer) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::buffer::Buffer) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_buffer_mut_ptr_callback,
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

impl<'a> Default for SlotBufferMutPtr<'a> {
  fn default() -> Self {
    SlotBufferMutPtr {
      wrapper: ::slots::raw::RawSlotBufferMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBufferMutPtr<'a> {
  type Arguments = (*mut ::buffer::Buffer,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBufferMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_buffer_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::buffer::Buffer) {
  let func: &mut Box<FnMut(*mut ::buffer::Buffer)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QBuffer::UsageType)` to a Rust closure.
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

pub struct SlotBufferUsageType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotBufferUsageType>,
  func: ::std::option::Option<Box<Box<FnMut(::buffer::UsageType) + 'a>>>,
}

impl<'a> SlotBufferUsageType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::buffer::UsageType) + 'a>(f: F) -> SlotBufferUsageType<'a> {
    let mut obj = SlotBufferUsageType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::buffer::UsageType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::buffer::UsageType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_buffer_usage_type_callback,
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

impl<'a> Default for SlotBufferUsageType<'a> {
  fn default() -> Self {
    SlotBufferUsageType {
      wrapper: ::slots::raw::RawSlotBufferUsageType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotBufferUsageType<'a> {
  type Arguments = (::buffer::UsageType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotBufferUsageType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_buffer_usage_type_callback(data: *mut ::libc::c_void, arg0: ::buffer::UsageType) {
  let func: &mut Box<FnMut(::buffer::UsageType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(unsigned int)` to a Rust closure.
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

pub struct SlotCUint<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCUint>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_uint) + 'a>>>,
}

impl<'a> SlotCUint<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_uint) + 'a>(f: F) -> SlotCUint<'a> {
    let mut obj = SlotCUint::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_uint) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_uint) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_uint_callback,
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

impl<'a> Default for SlotCUint<'a> {
  fn default() -> Self {
    SlotCUint {
      wrapper: ::slots::raw::RawSlotCUint::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCUint<'a> {
  type Arguments = (::libc::c_uint,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCUint as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_uint_callback(data: *mut ::libc::c_void, arg0: ::libc::c_uint) {
  let func: &mut Box<FnMut(::libc::c_uint)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QCameraLens::ProjectionType)` to a Rust closure.
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

pub struct SlotCameraLensProjectionTypeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCameraLensProjectionTypeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::camera_lens::ProjectionType) + 'a>>>,
}

impl<'a> SlotCameraLensProjectionTypeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::camera_lens::ProjectionType) + 'a>(f: F) -> SlotCameraLensProjectionTypeRef<'a> {
    let mut obj = SlotCameraLensProjectionTypeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::camera_lens::ProjectionType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::camera_lens::ProjectionType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_camera_lens_projection_type_ref_callback,
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

impl<'a> Default for SlotCameraLensProjectionTypeRef<'a> {
  fn default() -> Self {
    SlotCameraLensProjectionTypeRef {
      wrapper: ::slots::raw::RawSlotCameraLensProjectionTypeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCameraLensProjectionTypeRef<'a> {
  type Arguments = (&'static ::camera_lens::ProjectionType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCameraLensProjectionTypeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_camera_lens_projection_type_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: *const ::camera_lens::ProjectionType) {
  let func: &mut Box<FnMut(&'static ::camera_lens::ProjectionType)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QCamera*)` to a Rust closure.
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

pub struct SlotCameraMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCameraMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::camera::Camera) + 'a>>>,
}

impl<'a> SlotCameraMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::camera::Camera) + 'a>(f: F) -> SlotCameraMutPtr<'a> {
    let mut obj = SlotCameraMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::camera::Camera) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::camera::Camera) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_camera_mut_ptr_callback,
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

impl<'a> Default for SlotCameraMutPtr<'a> {
  fn default() -> Self {
    SlotCameraMutPtr {
      wrapper: ::slots::raw::RawSlotCameraMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCameraMutPtr<'a> {
  type Arguments = (*mut ::camera::Camera,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCameraMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_camera_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::camera::Camera) {
  let func: &mut Box<FnMut(*mut ::camera::Camera)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QClearBuffers::BufferType)` to a Rust closure.
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

pub struct SlotClearBuffersBufferType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotClearBuffersBufferType>,
  func: ::std::option::Option<Box<Box<FnMut(::clear_buffers::BufferType) + 'a>>>,
}

impl<'a> SlotClearBuffersBufferType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::clear_buffers::BufferType) + 'a>(f: F) -> SlotClearBuffersBufferType<'a> {
    let mut obj = SlotClearBuffersBufferType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::clear_buffers::BufferType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::clear_buffers::BufferType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_clear_buffers_buffer_type_callback,
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

impl<'a> Default for SlotClearBuffersBufferType<'a> {
  fn default() -> Self {
    SlotClearBuffersBufferType {
      wrapper: ::slots::raw::RawSlotClearBuffersBufferType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotClearBuffersBufferType<'a> {
  type Arguments = (::clear_buffers::BufferType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotClearBuffersBufferType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_clear_buffers_buffer_type_callback(data: *mut ::libc::c_void, arg0: ::clear_buffers::BufferType) {
  let func: &mut Box<FnMut(::clear_buffers::BufferType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QCullFace::CullingMode)` to a Rust closure.
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

pub struct SlotCullFaceCullingMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCullFaceCullingMode>,
  func: ::std::option::Option<Box<Box<FnMut(::cull_face::CullingMode) + 'a>>>,
}

impl<'a> SlotCullFaceCullingMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::cull_face::CullingMode) + 'a>(f: F) -> SlotCullFaceCullingMode<'a> {
    let mut obj = SlotCullFaceCullingMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::cull_face::CullingMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::cull_face::CullingMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_cull_face_culling_mode_callback,
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

impl<'a> Default for SlotCullFaceCullingMode<'a> {
  fn default() -> Self {
    SlotCullFaceCullingMode {
      wrapper: ::slots::raw::RawSlotCullFaceCullingMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCullFaceCullingMode<'a> {
  type Arguments = (::cull_face::CullingMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCullFaceCullingMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_cull_face_culling_mode_callback(data: *mut ::libc::c_void, arg0: ::cull_face::CullingMode) {
  let func: &mut Box<FnMut(::cull_face::CullingMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QDepthTest::DepthFunction)` to a Rust closure.
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

pub struct SlotDepthTestDepthFunction<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotDepthTestDepthFunction>,
  func: ::std::option::Option<Box<Box<FnMut(::depth_test::DepthFunction) + 'a>>>,
}

impl<'a> SlotDepthTestDepthFunction<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::depth_test::DepthFunction) + 'a>(f: F) -> SlotDepthTestDepthFunction<'a> {
    let mut obj = SlotDepthTestDepthFunction::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::depth_test::DepthFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::depth_test::DepthFunction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_depth_test_depth_function_callback,
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

impl<'a> Default for SlotDepthTestDepthFunction<'a> {
  fn default() -> Self {
    SlotDepthTestDepthFunction {
      wrapper: ::slots::raw::RawSlotDepthTestDepthFunction::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotDepthTestDepthFunction<'a> {
  type Arguments = (::depth_test::DepthFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotDepthTestDepthFunction as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_depth_test_depth_function_callback(data: *mut ::libc::c_void, arg0: ::depth_test::DepthFunction) {
  let func: &mut Box<FnMut(::depth_test::DepthFunction)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QEffect*)` to a Rust closure.
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

pub struct SlotEffectMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotEffectMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::effect::Effect) + 'a>>>,
}

impl<'a> SlotEffectMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::effect::Effect) + 'a>(f: F) -> SlotEffectMutPtr<'a> {
    let mut obj = SlotEffectMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::effect::Effect) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::effect::Effect) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_effect_mut_ptr_callback,
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

impl<'a> Default for SlotEffectMutPtr<'a> {
  fn default() -> Self {
    SlotEffectMutPtr {
      wrapper: ::slots::raw::RawSlotEffectMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotEffectMutPtr<'a> {
  type Arguments = (*mut ::effect::Effect,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotEffectMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_effect_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::effect::Effect) {
  let func: &mut Box<FnMut(*mut ::effect::Effect)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QFrameGraphNode*)` to a Rust closure.
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

pub struct SlotFrameGraphNodeMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotFrameGraphNodeMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::frame_graph_node::FrameGraphNode) + 'a>>>,
}

impl<'a> SlotFrameGraphNodeMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::frame_graph_node::FrameGraphNode) + 'a>(f: F) -> SlotFrameGraphNodeMutPtr<'a> {
    let mut obj = SlotFrameGraphNodeMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::frame_graph_node::FrameGraphNode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::frame_graph_node::FrameGraphNode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_frame_graph_node_mut_ptr_callback,
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

impl<'a> Default for SlotFrameGraphNodeMutPtr<'a> {
  fn default() -> Self {
    SlotFrameGraphNodeMutPtr {
      wrapper: ::slots::raw::RawSlotFrameGraphNodeMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotFrameGraphNodeMutPtr<'a> {
  type Arguments = (*mut ::frame_graph_node::FrameGraphNode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotFrameGraphNodeMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_frame_graph_node_mut_ptr_callback(data: *mut ::libc::c_void,
                                                     arg0: *mut ::frame_graph_node::FrameGraphNode) {
  let func: &mut Box<FnMut(*mut ::frame_graph_node::FrameGraphNode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QFrontFace::WindingDirection)` to a Rust closure.
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

pub struct SlotFrontFaceWindingDirection<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotFrontFaceWindingDirection>,
  func: ::std::option::Option<Box<Box<FnMut(::front_face::WindingDirection) + 'a>>>,
}

impl<'a> SlotFrontFaceWindingDirection<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::front_face::WindingDirection) + 'a>(f: F) -> SlotFrontFaceWindingDirection<'a> {
    let mut obj = SlotFrontFaceWindingDirection::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::front_face::WindingDirection) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::front_face::WindingDirection) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_front_face_winding_direction_callback,
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

impl<'a> Default for SlotFrontFaceWindingDirection<'a> {
  fn default() -> Self {
    SlotFrontFaceWindingDirection {
      wrapper: ::slots::raw::RawSlotFrontFaceWindingDirection::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotFrontFaceWindingDirection<'a> {
  type Arguments = (::front_face::WindingDirection,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotFrontFaceWindingDirection as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_front_face_winding_direction_callback(data: *mut ::libc::c_void,
                                                         arg0: ::front_face::WindingDirection) {
  let func: &mut Box<FnMut(::front_face::WindingDirection)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QGeometry*)` to a Rust closure.
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

pub struct SlotGeometryMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGeometryMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::geometry::Geometry) + 'a>>>,
}

impl<'a> SlotGeometryMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::geometry::Geometry) + 'a>(f: F) -> SlotGeometryMutPtr<'a> {
    let mut obj = SlotGeometryMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::geometry::Geometry) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::geometry::Geometry) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_geometry_mut_ptr_callback,
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

impl<'a> Default for SlotGeometryMutPtr<'a> {
  fn default() -> Self {
    SlotGeometryMutPtr {
      wrapper: ::slots::raw::RawSlotGeometryMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGeometryMutPtr<'a> {
  type Arguments = (*mut ::geometry::Geometry,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGeometryMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_geometry_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::geometry::Geometry) {
  let func: &mut Box<FnMut(*mut ::geometry::Geometry)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QGeometryRenderer::PrimitiveType)` to a Rust closure.
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

pub struct SlotGeometryRendererPrimitiveType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGeometryRendererPrimitiveType>,
  func: ::std::option::Option<Box<Box<FnMut(::geometry_renderer::PrimitiveType) + 'a>>>,
}

impl<'a> SlotGeometryRendererPrimitiveType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::geometry_renderer::PrimitiveType) + 'a>(f: F) -> SlotGeometryRendererPrimitiveType<'a> {
    let mut obj = SlotGeometryRendererPrimitiveType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::geometry_renderer::PrimitiveType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::geometry_renderer::PrimitiveType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_geometry_renderer_primitive_type_callback,
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

impl<'a> Default for SlotGeometryRendererPrimitiveType<'a> {
  fn default() -> Self {
    SlotGeometryRendererPrimitiveType {
      wrapper: ::slots::raw::RawSlotGeometryRendererPrimitiveType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGeometryRendererPrimitiveType<'a> {
  type Arguments = (::geometry_renderer::PrimitiveType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGeometryRendererPrimitiveType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_geometry_renderer_primitive_type_callback(data: *mut ::libc::c_void,
                                                             arg0: ::geometry_renderer::PrimitiveType) {
  let func: &mut Box<FnMut(::geometry_renderer::PrimitiveType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QGraphicsApiFilter::Api)` to a Rust closure.
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

pub struct SlotGraphicsApiFilterApiRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsApiFilterApiRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::graphics_api_filter::Api) + 'a>>>,
}

impl<'a> SlotGraphicsApiFilterApiRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::graphics_api_filter::Api) + 'a>(f: F) -> SlotGraphicsApiFilterApiRef<'a> {
    let mut obj = SlotGraphicsApiFilterApiRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::graphics_api_filter::Api) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::graphics_api_filter::Api) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_graphics_api_filter_api_ref_callback,
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

impl<'a> Default for SlotGraphicsApiFilterApiRef<'a> {
  fn default() -> Self {
    SlotGraphicsApiFilterApiRef {
      wrapper: ::slots::raw::RawSlotGraphicsApiFilterApiRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGraphicsApiFilterApiRef<'a> {
  type Arguments = (&'static ::graphics_api_filter::Api,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGraphicsApiFilterApiRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_graphics_api_filter_api_ref_callback(data: *mut ::libc::c_void,
                                                        arg0: *const ::graphics_api_filter::Api) {
  let func: &mut Box<FnMut(&'static ::graphics_api_filter::Api)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QGraphicsApiFilter::OpenGLProfile)` to a Rust closure.
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

pub struct SlotGraphicsApiFilterOpenglProfileRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::graphics_api_filter::OpenGLProfile) + 'a>>>,
}

impl<'a> SlotGraphicsApiFilterOpenglProfileRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::graphics_api_filter::OpenGLProfile) + 'a>
    (f: F)
     -> SlotGraphicsApiFilterOpenglProfileRef<'a> {
    let mut obj = SlotGraphicsApiFilterOpenglProfileRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::graphics_api_filter::OpenGLProfile) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::graphics_api_filter::OpenGLProfile) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_graphics_api_filter_opengl_profile_ref_callback,
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

impl<'a> Default for SlotGraphicsApiFilterOpenglProfileRef<'a> {
  fn default() -> Self {
    SlotGraphicsApiFilterOpenglProfileRef {
      wrapper: ::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGraphicsApiFilterOpenglProfileRef<'a> {
  type Arguments = (&'static ::graphics_api_filter::OpenGLProfile,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGraphicsApiFilterOpenglProfileRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_graphics_api_filter_opengl_profile_ref_callback(data: *mut ::libc::c_void, arg0: *const ::graphics_api_filter::OpenGLProfile) {
  let func: &mut Box<FnMut(&'static ::graphics_api_filter::OpenGLProfile)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const Qt3DRender::QLevelOfDetailBoundingSphere&)` to a Rust closure.
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

pub struct SlotLevelOfDetailBoundingSphereRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotLevelOfDetailBoundingSphereRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) + 'a>>>,
}

impl<'a> SlotLevelOfDetailBoundingSphereRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) + 'a>
    (f: F)
     -> SlotLevelOfDetailBoundingSphereRef<'a> {
    let mut obj = SlotLevelOfDetailBoundingSphereRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_level_of_detail_bounding_sphere_ref_callback,
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

impl<'a> Default for SlotLevelOfDetailBoundingSphereRef<'a> {
  fn default() -> Self {
    SlotLevelOfDetailBoundingSphereRef {
      wrapper: ::slots::raw::RawSlotLevelOfDetailBoundingSphereRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotLevelOfDetailBoundingSphereRef<'a> {
  type Arguments = (&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotLevelOfDetailBoundingSphereRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_level_of_detail_bounding_sphere_ref_callback(data: *mut ::libc::c_void, arg0: *const ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere) {
  let func: &mut Box<FnMut(&'static ::level_of_detail_bounding_sphere::LevelOfDetailBoundingSphere)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QLevelOfDetail::ThresholdType)` to a Rust closure.
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

pub struct SlotLevelOfDetailThresholdType<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotLevelOfDetailThresholdType>,
  func: ::std::option::Option<Box<Box<FnMut(::level_of_detail::ThresholdType) + 'a>>>,
}

impl<'a> SlotLevelOfDetailThresholdType<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::level_of_detail::ThresholdType) + 'a>(f: F) -> SlotLevelOfDetailThresholdType<'a> {
    let mut obj = SlotLevelOfDetailThresholdType::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::level_of_detail::ThresholdType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::level_of_detail::ThresholdType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_level_of_detail_threshold_type_callback,
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

impl<'a> Default for SlotLevelOfDetailThresholdType<'a> {
  fn default() -> Self {
    SlotLevelOfDetailThresholdType {
      wrapper: ::slots::raw::RawSlotLevelOfDetailThresholdType::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotLevelOfDetailThresholdType<'a> {
  type Arguments = (::level_of_detail::ThresholdType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotLevelOfDetailThresholdType as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_level_of_detail_threshold_type_callback(data: *mut ::libc::c_void,
                                                           arg0: ::level_of_detail::ThresholdType) {
  let func: &mut Box<FnMut(::level_of_detail::ThresholdType)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QPickEvent*)` to a Rust closure.
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

pub struct SlotPickEventMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPickEventMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::pick_event::PickEvent) + 'a>>>,
}

impl<'a> SlotPickEventMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::pick_event::PickEvent) + 'a>(f: F) -> SlotPickEventMutPtr<'a> {
    let mut obj = SlotPickEventMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::pick_event::PickEvent) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::pick_event::PickEvent) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_pick_event_mut_ptr_callback,
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

impl<'a> Default for SlotPickEventMutPtr<'a> {
  fn default() -> Self {
    SlotPickEventMutPtr {
      wrapper: ::slots::raw::RawSlotPickEventMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPickEventMutPtr<'a> {
  type Arguments = (*mut ::pick_event::PickEvent,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPickEventMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_pick_event_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::pick_event::PickEvent) {
  let func: &mut Box<FnMut(*mut ::pick_event::PickEvent)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::FaceOrientationPickingMode)` to a Rust closure.
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

pub struct SlotPickingSettingsFaceOrientationPickingModeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::picking_settings::FaceOrientationPickingMode) + 'a>>>,
}

impl<'a> SlotPickingSettingsFaceOrientationPickingModeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::picking_settings::FaceOrientationPickingMode) + 'a>
    (f: F)
     -> SlotPickingSettingsFaceOrientationPickingModeRef<'a> {
    let mut obj = SlotPickingSettingsFaceOrientationPickingModeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::picking_settings::FaceOrientationPickingMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::picking_settings::FaceOrientationPickingMode) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_picking_settings_face_orientation_picking_mode_ref_callback,
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

impl<'a> Default for SlotPickingSettingsFaceOrientationPickingModeRef<'a> {
  fn default() -> Self {
    SlotPickingSettingsFaceOrientationPickingModeRef {
      wrapper: ::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPickingSettingsFaceOrientationPickingModeRef<'a> {
  type Arguments = (&'static ::picking_settings::FaceOrientationPickingMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPickingSettingsFaceOrientationPickingModeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_picking_settings_face_orientation_picking_mode_ref_callback(data: *mut ::libc::c_void, arg0: *const ::picking_settings::FaceOrientationPickingMode) {
  let func: &mut Box<FnMut(&'static ::picking_settings::FaceOrientationPickingMode)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::PickMethod)` to a Rust closure.
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

pub struct SlotPickingSettingsPickMethodRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsPickMethodRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::picking_settings::PickMethod) + 'a>>>,
}

impl<'a> SlotPickingSettingsPickMethodRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::picking_settings::PickMethod) + 'a>(f: F) -> SlotPickingSettingsPickMethodRef<'a> {
    let mut obj = SlotPickingSettingsPickMethodRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::picking_settings::PickMethod) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::picking_settings::PickMethod) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_picking_settings_pick_method_ref_callback,
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

impl<'a> Default for SlotPickingSettingsPickMethodRef<'a> {
  fn default() -> Self {
    SlotPickingSettingsPickMethodRef {
      wrapper: ::slots::raw::RawSlotPickingSettingsPickMethodRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPickingSettingsPickMethodRef<'a> {
  type Arguments = (&'static ::picking_settings::PickMethod,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPickingSettingsPickMethodRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_picking_settings_pick_method_ref_callback(data: *mut ::libc::c_void,
                                                             arg0: *const ::picking_settings::PickMethod) {
  let func: &mut Box<FnMut(&'static ::picking_settings::PickMethod)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QPickingSettings::PickResultMode)` to a Rust closure.
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

pub struct SlotPickingSettingsPickResultModeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPickingSettingsPickResultModeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::picking_settings::PickResultMode) + 'a>>>,
}

impl<'a> SlotPickingSettingsPickResultModeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::picking_settings::PickResultMode) + 'a>
    (f: F)
     -> SlotPickingSettingsPickResultModeRef<'a> {
    let mut obj = SlotPickingSettingsPickResultModeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::picking_settings::PickResultMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::picking_settings::PickResultMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_picking_settings_pick_result_mode_ref_callback,
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

impl<'a> Default for SlotPickingSettingsPickResultModeRef<'a> {
  fn default() -> Self {
    SlotPickingSettingsPickResultModeRef {
      wrapper: ::slots::raw::RawSlotPickingSettingsPickResultModeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPickingSettingsPickResultModeRef<'a> {
  type Arguments = (&'static ::picking_settings::PickResultMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPickingSettingsPickResultModeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_picking_settings_pick_result_mode_ref_callback(data: *mut ::libc::c_void,
                                                                  arg0: *const ::picking_settings::PickResultMode) {
  let func: &mut Box<FnMut(&'static ::picking_settings::PickResultMode)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QPointSize::SizeMode)` to a Rust closure.
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

pub struct SlotPointSizeSizeMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotPointSizeSizeMode>,
  func: ::std::option::Option<Box<Box<FnMut(::point_size::SizeMode) + 'a>>>,
}

impl<'a> SlotPointSizeSizeMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::point_size::SizeMode) + 'a>(f: F) -> SlotPointSizeSizeMode<'a> {
    let mut obj = SlotPointSizeSizeMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::point_size::SizeMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::point_size::SizeMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_point_size_size_mode_callback,
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

impl<'a> Default for SlotPointSizeSizeMode<'a> {
  fn default() -> Self {
    SlotPointSizeSizeMode {
      wrapper: ::slots::raw::RawSlotPointSizeSizeMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotPointSizeSizeMode<'a> {
  type Arguments = (::point_size::SizeMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotPointSizeSizeMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_point_size_size_mode_callback(data: *mut ::libc::c_void, arg0: ::point_size::SizeMode) {
  let func: &mut Box<FnMut(::point_size::SizeMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QByteArray&)` to a Rust closure.
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

pub struct SlotQtCoreByteArrayRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreByteArrayRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::byte_array::ByteArray) + 'a>>>,
}

impl<'a> SlotQtCoreByteArrayRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::byte_array::ByteArray) + 'a>(f: F) -> SlotQtCoreByteArrayRef<'a> {
    let mut obj = SlotQtCoreByteArrayRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::byte_array::ByteArray) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::byte_array::ByteArray) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_byte_array_ref_callback,
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

impl<'a> Default for SlotQtCoreByteArrayRef<'a> {
  fn default() -> Self {
    SlotQtCoreByteArrayRef {
      wrapper: ::slots::raw::RawSlotQtCoreByteArrayRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreByteArrayRef<'a> {
  type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreByteArrayRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_byte_array_ref_callback(data: *mut ::libc::c_void,
                                                   arg0: *const ::qt_core::byte_array::ByteArray) {
  let func: &mut Box<FnMut(&'static ::qt_core::byte_array::ByteArray)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QSize)` to a Rust closure.
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
/// Allows to bind Qt signals with arguments `(const QStringList&)` to a Rust closure.
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

pub struct SlotQtCoreStringListRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringListRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::string_list::StringList) + 'a>>>,
}

impl<'a> SlotQtCoreStringListRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::string_list::StringList) + 'a>(f: F) -> SlotQtCoreStringListRef<'a> {
    let mut obj = SlotQtCoreStringListRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::string_list::StringList) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::string_list::StringList) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_string_list_ref_callback,
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

impl<'a> Default for SlotQtCoreStringListRef<'a> {
  fn default() -> Self {
    SlotQtCoreStringListRef {
      wrapper: ::slots::raw::RawSlotQtCoreStringListRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreStringListRef<'a> {
  type Arguments = (&'static ::qt_core::string_list::StringList,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreStringListRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_string_list_ref_callback(data: *mut ::libc::c_void,
                                                    arg0: *const ::qt_core::string_list::StringList) {
  let func: &mut Box<FnMut(&'static ::qt_core::string_list::StringList)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
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
/// Allows to bind Qt signals with arguments `(const QColor&)` to a Rust closure.
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

pub struct SlotQtGuiColorRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiColorRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::color::Color) + 'a>>>,
}

impl<'a> SlotQtGuiColorRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::color::Color) + 'a>(f: F) -> SlotQtGuiColorRef<'a> {
    let mut obj = SlotQtGuiColorRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::color::Color) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::color::Color) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_color_ref_callback,
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

impl<'a> Default for SlotQtGuiColorRef<'a> {
  fn default() -> Self {
    SlotQtGuiColorRef {
      wrapper: ::slots::raw::RawSlotQtGuiColorRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiColorRef<'a> {
  type Arguments = (&'static ::qt_gui::color::Color,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiColorRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_color_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_gui::color::Color) {
  let func: &mut Box<FnMut(&'static ::qt_gui::color::Color)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QMatrix4x4&)` to a Rust closure.
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

pub struct SlotQtGuiMatrix4X4Matrix4X4Ref<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::matrix_4x4::Matrix4X4) + 'a>>>,
}

impl<'a> SlotQtGuiMatrix4X4Matrix4X4Ref<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::matrix_4x4::Matrix4X4) + 'a>(f: F) -> SlotQtGuiMatrix4X4Matrix4X4Ref<'a> {
    let mut obj = SlotQtGuiMatrix4X4Matrix4X4Ref::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::matrix_4x4::Matrix4X4) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::matrix_4x4::Matrix4X4) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_matrix_4x4_matrix4_x4_ref_callback,
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

impl<'a> Default for SlotQtGuiMatrix4X4Matrix4X4Ref<'a> {
  fn default() -> Self {
    SlotQtGuiMatrix4X4Matrix4X4Ref {
      wrapper: ::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiMatrix4X4Matrix4X4Ref<'a> {
  type Arguments = (&'static ::qt_gui::matrix_4x4::Matrix4X4,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiMatrix4X4Matrix4X4Ref as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_matrix_4x4_matrix4_x4_ref_callback(data: *mut ::libc::c_void,
                                                             arg0: *const ::qt_gui::matrix_4x4::Matrix4X4) {
  let func: &mut Box<FnMut(&'static ::qt_gui::matrix_4x4::Matrix4X4)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QVector3D)` to a Rust closure.
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
/// Allows to bind Qt signals with arguments `(const QVector<double>&)` to a Rust closure.
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

pub struct SlotQtGuiVectorVectorCDoubleRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::vector::VectorCDouble) + 'a>>>,
}

impl<'a> SlotQtGuiVectorVectorCDoubleRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::vector::VectorCDouble) + 'a>(f: F) -> SlotQtGuiVectorVectorCDoubleRef<'a> {
    let mut obj = SlotQtGuiVectorVectorCDoubleRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::vector::VectorCDouble) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::vector::VectorCDouble) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_vector_vector_c_double_ref_callback,
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

impl<'a> Default for SlotQtGuiVectorVectorCDoubleRef<'a> {
  fn default() -> Self {
    SlotQtGuiVectorVectorCDoubleRef {
      wrapper: ::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiVectorVectorCDoubleRef<'a> {
  type Arguments = (&'static ::qt_gui::vector::VectorCDouble,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiVectorVectorCDoubleRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_vector_vector_c_double_ref_callback(data: *mut ::libc::c_void,
                                                              arg0: *const ::qt_gui::vector::VectorCDouble) {
  let func: &mut Box<FnMut(&'static ::qt_gui::vector::VectorCDouble)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderSettings::RenderPolicy)` to a Rust closure.
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

pub struct SlotRenderSettingsRenderPolicy<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotRenderSettingsRenderPolicy>,
  func: ::std::option::Option<Box<Box<FnMut(::render_settings::RenderPolicy) + 'a>>>,
}

impl<'a> SlotRenderSettingsRenderPolicy<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::render_settings::RenderPolicy) + 'a>(f: F) -> SlotRenderSettingsRenderPolicy<'a> {
    let mut obj = SlotRenderSettingsRenderPolicy::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::render_settings::RenderPolicy) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::render_settings::RenderPolicy) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_render_settings_render_policy_callback,
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

impl<'a> Default for SlotRenderSettingsRenderPolicy<'a> {
  fn default() -> Self {
    SlotRenderSettingsRenderPolicy {
      wrapper: ::slots::raw::RawSlotRenderSettingsRenderPolicy::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotRenderSettingsRenderPolicy<'a> {
  type Arguments = (::render_settings::RenderPolicy,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotRenderSettingsRenderPolicy as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_render_settings_render_policy_callback(data: *mut ::libc::c_void,
                                                          arg0: ::render_settings::RenderPolicy) {
  let func: &mut Box<FnMut(::render_settings::RenderPolicy)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTarget*)` to a Rust closure.
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

pub struct SlotRenderTargetMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::render_target::RenderTarget) + 'a>>>,
}

impl<'a> SlotRenderTargetMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::render_target::RenderTarget) + 'a>(f: F) -> SlotRenderTargetMutPtr<'a> {
    let mut obj = SlotRenderTargetMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::render_target::RenderTarget) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::render_target::RenderTarget) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_render_target_mut_ptr_callback,
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

impl<'a> Default for SlotRenderTargetMutPtr<'a> {
  fn default() -> Self {
    SlotRenderTargetMutPtr {
      wrapper: ::slots::raw::RawSlotRenderTargetMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotRenderTargetMutPtr<'a> {
  type Arguments = (*mut ::render_target::RenderTarget,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotRenderTargetMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_render_target_mut_ptr_callback(data: *mut ::libc::c_void,
                                                  arg0: *mut ::render_target::RenderTarget) {
  let func: &mut Box<FnMut(*mut ::render_target::RenderTarget)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTargetOutput::AttachmentPoint)` to a Rust closure.
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

pub struct SlotRenderTargetOutputAttachmentPoint<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetOutputAttachmentPoint>,
  func: ::std::option::Option<Box<Box<FnMut(::render_target_output::AttachmentPoint) + 'a>>>,
}

impl<'a> SlotRenderTargetOutputAttachmentPoint<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::render_target_output::AttachmentPoint) + 'a>(f: F)
                                                                     -> SlotRenderTargetOutputAttachmentPoint<'a> {
    let mut obj = SlotRenderTargetOutputAttachmentPoint::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::render_target_output::AttachmentPoint) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::render_target_output::AttachmentPoint) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_render_target_output_attachment_point_callback,
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

impl<'a> Default for SlotRenderTargetOutputAttachmentPoint<'a> {
  fn default() -> Self {
    SlotRenderTargetOutputAttachmentPoint {
      wrapper: ::slots::raw::RawSlotRenderTargetOutputAttachmentPoint::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotRenderTargetOutputAttachmentPoint<'a> {
  type Arguments = (::render_target_output::AttachmentPoint,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotRenderTargetOutputAttachmentPoint as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_render_target_output_attachment_point_callback(data: *mut ::libc::c_void,
                                                                  arg0: ::render_target_output::AttachmentPoint) {
  let func: &mut Box<FnMut(::render_target_output::AttachmentPoint)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QRenderTargetOutput*)` to a Rust closure.
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

pub struct SlotRenderTargetOutputMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotRenderTargetOutputMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::render_target_output::RenderTargetOutput) + 'a>>>,
}

impl<'a> SlotRenderTargetOutputMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::render_target_output::RenderTargetOutput) + 'a>(f: F) -> SlotRenderTargetOutputMutPtr<'a> {
    let mut obj = SlotRenderTargetOutputMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::render_target_output::RenderTargetOutput) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::render_target_output::RenderTargetOutput) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_render_target_output_mut_ptr_callback,
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

impl<'a> Default for SlotRenderTargetOutputMutPtr<'a> {
  fn default() -> Self {
    SlotRenderTargetOutputMutPtr {
      wrapper: ::slots::raw::RawSlotRenderTargetOutputMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotRenderTargetOutputMutPtr<'a> {
  type Arguments = (*mut ::render_target_output::RenderTargetOutput,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotRenderTargetOutputMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_render_target_output_mut_ptr_callback(data: *mut ::libc::c_void,
                                                         arg0: *mut ::render_target_output::RenderTargetOutput) {
  let func: &mut Box<FnMut(*mut ::render_target_output::RenderTargetOutput)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QSceneLoader::Status)` to a Rust closure.
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

pub struct SlotSceneLoaderStatus<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotSceneLoaderStatus>,
  func: ::std::option::Option<Box<Box<FnMut(::scene_loader::Status) + 'a>>>,
}

impl<'a> SlotSceneLoaderStatus<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::scene_loader::Status) + 'a>(f: F) -> SlotSceneLoaderStatus<'a> {
    let mut obj = SlotSceneLoaderStatus::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::scene_loader::Status) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::scene_loader::Status) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_scene_loader_status_callback,
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

impl<'a> Default for SlotSceneLoaderStatus<'a> {
  fn default() -> Self {
    SlotSceneLoaderStatus {
      wrapper: ::slots::raw::RawSlotSceneLoaderStatus::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotSceneLoaderStatus<'a> {
  type Arguments = (::scene_loader::Status,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotSceneLoaderStatus as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_scene_loader_status_callback(data: *mut ::libc::c_void, arg0: ::scene_loader::Status) {
  let func: &mut Box<FnMut(::scene_loader::Status)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QShaderProgram*)` to a Rust closure.
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

pub struct SlotShaderProgramMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotShaderProgramMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::shader_program::ShaderProgram) + 'a>>>,
}

impl<'a> SlotShaderProgramMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::shader_program::ShaderProgram) + 'a>(f: F) -> SlotShaderProgramMutPtr<'a> {
    let mut obj = SlotShaderProgramMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::shader_program::ShaderProgram) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::shader_program::ShaderProgram) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_shader_program_mut_ptr_callback,
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

impl<'a> Default for SlotShaderProgramMutPtr<'a> {
  fn default() -> Self {
    SlotShaderProgramMutPtr {
      wrapper: ::slots::raw::RawSlotShaderProgramMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotShaderProgramMutPtr<'a> {
  type Arguments = (*mut ::shader_program::ShaderProgram,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotShaderProgramMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_shader_program_mut_ptr_callback(data: *mut ::libc::c_void,
                                                   arg0: *mut ::shader_program::ShaderProgram) {
  let func: &mut Box<FnMut(*mut ::shader_program::ShaderProgram)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QShaderProgram::Status)` to a Rust closure.
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

pub struct SlotShaderProgramStatus<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotShaderProgramStatus>,
  func: ::std::option::Option<Box<Box<FnMut(::shader_program::Status) + 'a>>>,
}

impl<'a> SlotShaderProgramStatus<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::shader_program::Status) + 'a>(f: F) -> SlotShaderProgramStatus<'a> {
    let mut obj = SlotShaderProgramStatus::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::shader_program::Status) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::shader_program::Status) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_shader_program_status_callback,
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

impl<'a> Default for SlotShaderProgramStatus<'a> {
  fn default() -> Self {
    SlotShaderProgramStatus {
      wrapper: ::slots::raw::RawSlotShaderProgramStatus::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotShaderProgramStatus<'a> {
  type Arguments = (::shader_program::Status,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotShaderProgramStatus as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_shader_program_status_callback(data: *mut ::libc::c_void, arg0: ::shader_program::Status) {
  let func: &mut Box<FnMut(::shader_program::Status)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilOperationArguments::FaceMode)` to a Rust closure.
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

pub struct SlotStencilOperationArgumentsFaceMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStencilOperationArgumentsFaceMode>,
  func: ::std::option::Option<Box<Box<FnMut(::stencil_operation_arguments::FaceMode) + 'a>>>,
}

impl<'a> SlotStencilOperationArgumentsFaceMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::stencil_operation_arguments::FaceMode) + 'a>(f: F)
                                                                     -> SlotStencilOperationArgumentsFaceMode<'a> {
    let mut obj = SlotStencilOperationArgumentsFaceMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::stencil_operation_arguments::FaceMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::stencil_operation_arguments::FaceMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_stencil_operation_arguments_face_mode_callback,
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

impl<'a> Default for SlotStencilOperationArgumentsFaceMode<'a> {
  fn default() -> Self {
    SlotStencilOperationArgumentsFaceMode {
      wrapper: ::slots::raw::RawSlotStencilOperationArgumentsFaceMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotStencilOperationArgumentsFaceMode<'a> {
  type Arguments = (::stencil_operation_arguments::FaceMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStencilOperationArgumentsFaceMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_stencil_operation_arguments_face_mode_callback(data: *mut ::libc::c_void,
                                                                  arg0: ::stencil_operation_arguments::FaceMode) {
  let func: &mut Box<FnMut(::stencil_operation_arguments::FaceMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilOperationArguments::Operation)` to a Rust closure.
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

pub struct SlotStencilOperationArgumentsOperation<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStencilOperationArgumentsOperation>,
  func: ::std::option::Option<Box<Box<FnMut(::stencil_operation_arguments::Operation) + 'a>>>,
}

impl<'a> SlotStencilOperationArgumentsOperation<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::stencil_operation_arguments::Operation) + 'a>(f: F)
                                                                      -> SlotStencilOperationArgumentsOperation<'a> {
    let mut obj = SlotStencilOperationArgumentsOperation::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::stencil_operation_arguments::Operation) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::stencil_operation_arguments::Operation) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_stencil_operation_arguments_operation_callback,
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

impl<'a> Default for SlotStencilOperationArgumentsOperation<'a> {
  fn default() -> Self {
    SlotStencilOperationArgumentsOperation {
      wrapper: ::slots::raw::RawSlotStencilOperationArgumentsOperation::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotStencilOperationArgumentsOperation<'a> {
  type Arguments = (::stencil_operation_arguments::Operation,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStencilOperationArgumentsOperation as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_stencil_operation_arguments_operation_callback(data: *mut ::libc::c_void,
                                                                  arg0: ::stencil_operation_arguments::Operation) {
  let func: &mut Box<FnMut(::stencil_operation_arguments::Operation)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilTestArguments::StencilFaceMode)` to a Rust closure.
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

pub struct SlotStencilTestArgumentsStencilFaceMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode>,
  func: ::std::option::Option<Box<Box<FnMut(::stencil_test_arguments::StencilFaceMode) + 'a>>>,
}

impl<'a> SlotStencilTestArgumentsStencilFaceMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::stencil_test_arguments::StencilFaceMode) + 'a>
    (f: F)
     -> SlotStencilTestArgumentsStencilFaceMode<'a> {
    let mut obj = SlotStencilTestArgumentsStencilFaceMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::stencil_test_arguments::StencilFaceMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::stencil_test_arguments::StencilFaceMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_stencil_test_arguments_stencil_face_mode_callback,
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

impl<'a> Default for SlotStencilTestArgumentsStencilFaceMode<'a> {
  fn default() -> Self {
    SlotStencilTestArgumentsStencilFaceMode {
      wrapper: ::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotStencilTestArgumentsStencilFaceMode<'a> {
  type Arguments = (::stencil_test_arguments::StencilFaceMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStencilTestArgumentsStencilFaceMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_stencil_test_arguments_stencil_face_mode_callback(data: *mut ::libc::c_void, arg0: ::stencil_test_arguments::StencilFaceMode) {
  let func: &mut Box<FnMut(::stencil_test_arguments::StencilFaceMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QStencilTestArguments::StencilFunction)` to a Rust closure.
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

pub struct SlotStencilTestArgumentsStencilFunction<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotStencilTestArgumentsStencilFunction>,
  func: ::std::option::Option<Box<Box<FnMut(::stencil_test_arguments::StencilFunction) + 'a>>>,
}

impl<'a> SlotStencilTestArgumentsStencilFunction<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::stencil_test_arguments::StencilFunction) + 'a>
    (f: F)
     -> SlotStencilTestArgumentsStencilFunction<'a> {
    let mut obj = SlotStencilTestArgumentsStencilFunction::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::stencil_test_arguments::StencilFunction) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::stencil_test_arguments::StencilFunction) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_stencil_test_arguments_stencil_function_callback,
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

impl<'a> Default for SlotStencilTestArgumentsStencilFunction<'a> {
  fn default() -> Self {
    SlotStencilTestArgumentsStencilFunction {
      wrapper: ::slots::raw::RawSlotStencilTestArgumentsStencilFunction::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotStencilTestArgumentsStencilFunction<'a> {
  type Arguments = (::stencil_test_arguments::StencilFunction,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotStencilTestArgumentsStencilFunction as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_stencil_test_arguments_stencil_function_callback(data: *mut ::libc::c_void,
                                                                    arg0: ::stencil_test_arguments::StencilFunction) {
  let func: &mut Box<FnMut(::stencil_test_arguments::StencilFunction)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QTextureImage::Status)` to a Rust closure.
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

pub struct SlotTextureImageStatus<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTextureImageStatus>,
  func: ::std::option::Option<Box<Box<FnMut(::texture_image::Status) + 'a>>>,
}

impl<'a> SlotTextureImageStatus<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::texture_image::Status) + 'a>(f: F) -> SlotTextureImageStatus<'a> {
    let mut obj = SlotTextureImageStatus::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::texture_image::Status) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::texture_image::Status) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_texture_image_status_callback,
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

impl<'a> Default for SlotTextureImageStatus<'a> {
  fn default() -> Self {
    SlotTextureImageStatus {
      wrapper: ::slots::raw::RawSlotTextureImageStatus::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTextureImageStatus<'a> {
  type Arguments = (::texture_image::Status,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTextureImageStatus as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_texture_image_status_callback(data: *mut ::libc::c_void, arg0: ::texture_image::Status) {
  let func: &mut Box<FnMut(::texture_image::Status)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(Qt3DRender::QTextureWrapMode::WrapMode)` to a Rust closure.
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

pub struct SlotTextureWrapModeWrapMode<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTextureWrapModeWrapMode>,
  func: ::std::option::Option<Box<Box<FnMut(::texture_wrap_mode::WrapMode) + 'a>>>,
}

impl<'a> SlotTextureWrapModeWrapMode<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::texture_wrap_mode::WrapMode) + 'a>(f: F) -> SlotTextureWrapModeWrapMode<'a> {
    let mut obj = SlotTextureWrapModeWrapMode::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::texture_wrap_mode::WrapMode) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::texture_wrap_mode::WrapMode) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_texture_wrap_mode_wrap_mode_callback,
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

impl<'a> Default for SlotTextureWrapModeWrapMode<'a> {
  fn default() -> Self {
    SlotTextureWrapModeWrapMode {
      wrapper: ::slots::raw::RawSlotTextureWrapModeWrapMode::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTextureWrapModeWrapMode<'a> {
  type Arguments = (::texture_wrap_mode::WrapMode,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTextureWrapModeWrapMode as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_texture_wrap_mode_wrap_mode_callback(data: *mut ::libc::c_void,
                                                        arg0: ::texture_wrap_mode::WrapMode) {
  let func: &mut Box<FnMut(::texture_wrap_mode::WrapMode)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QVector<Qt3DRender::QSortPolicy::SortType>&)` to a Rust closure.
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

pub struct SlotVectorVectorSortPolicySortTypeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::vector::VectorSortPolicySortType) + 'a>>>,
}

impl<'a> SlotVectorVectorSortPolicySortTypeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::vector::VectorSortPolicySortType) + 'a>
    (f: F)
     -> SlotVectorVectorSortPolicySortTypeRef<'a> {
    let mut obj = SlotVectorVectorSortPolicySortTypeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::vector::VectorSortPolicySortType) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::vector::VectorSortPolicySortType) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_vector_vector_sort_policy_sort_type_ref_callback,
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

impl<'a> Default for SlotVectorVectorSortPolicySortTypeRef<'a> {
  fn default() -> Self {
    SlotVectorVectorSortPolicySortTypeRef {
      wrapper: ::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotVectorVectorSortPolicySortTypeRef<'a> {
  type Arguments = (&'static ::vector::VectorSortPolicySortType,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotVectorVectorSortPolicySortTypeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_vector_vector_sort_policy_sort_type_ref_callback(data: *mut ::libc::c_void,
                                                                    arg0: *const ::vector::VectorSortPolicySortType) {
  let func: &mut Box<FnMut(&'static ::vector::VectorSortPolicySortType)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
