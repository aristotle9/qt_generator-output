/// Binding Qt signals to Rust extern functions.
///
/// Types in this module to connect Qt signals with certain argument types to a Rust extern function with payload. Raw slots expose low level C++ API and are used to implement the closure slots located in the parent module. Raw slots are less convenient but slightly faster than closure slots.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots::raw` module.
pub mod raw {
  /// Allows to bind Qt signals with arguments `(QAbstractButton*)` to a Rust extern function.
  ///
  /// Use `SlotAbstractButtonMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QAbstractButton_ptr```</span>
  #[repr(C)]
  pub struct RawSlotAbstractButtonMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractButtonMutPtr {
    type Arguments = (*mut ::abstract_button::AbstractButton,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractButton*)\0"
    }
  }
  impl RawSlotAbstractButtonMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QAbstractButton_ptr::custom_slot(QAbstractButton* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_button::AbstractButton) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_custom_slot(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QAbstractButton_ptr::qt_widgets_c_SlotWrapper_QAbstractButton_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractButtonMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QAbstractButton_ptr::set(void (*FN_PTR)(void*, QAbstractButton*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::abstract_button::AbstractButton),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_set(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractButtonMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAbstractButton*, bool)` to a Rust extern function.
  ///
  /// Use `SlotAbstractButtonMutPtrBool` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool```</span>
  #[repr(C)]
  pub struct RawSlotAbstractButtonMutPtrBool(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotAbstractButtonMutPtrBool {
    type Arguments = (*mut ::abstract_button::AbstractButton, bool);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAbstractButton*,bool)\0"
    }
  }
  impl RawSlotAbstractButtonMutPtrBool {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool::custom_slot(QAbstractButton* arg0, bool arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::abstract_button::AbstractButton, arg1: bool) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool_custom_slot(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool::qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractButtonMutPtrBool> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool::set(void (*FN_PTR)(void*, QAbstractButton*, bool) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::abstract_button::AbstractButton,
                                          bool),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool_set(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotAbstractButtonMutPtrBool {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QAction*)` to a Rust extern function.
  ///
  /// Use `SlotActionMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QAction_ptr```</span>
  #[repr(C)]
  pub struct RawSlotActionMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotActionMutPtr {
    type Arguments = (*mut ::action::Action,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QAction*)\0"
    }
  }
  impl RawSlotActionMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QAction_ptr::custom_slot(QAction* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::action::Action) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAction_ptr_custom_slot(self as *mut ::slots::raw::RawSlotActionMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QAction_ptr::qt_widgets_c_SlotWrapper_QAction_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotActionMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAction_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QAction_ptr::set(void (*FN_PTR)(void*, QAction*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::action::Action),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAction_ptr_set(self as *mut ::slots::raw::RawSlotActionMutPtr,
                                                                   func,
                                                                   data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotActionMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QAction_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, bool)` to a Rust extern function.
  ///
  /// Use `SlotCIntBool` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_int_bool```</span>
  #[repr(C)]
  pub struct RawSlotCIntBool(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCIntBool {
    type Arguments = (::libc::c_int, bool);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,bool)\0"
    }
  }
  impl RawSlotCIntBool {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_int_bool::custom_slot(int arg0, bool arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: bool) {
      unsafe {
        ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_bool_custom_slot(self as *mut ::slots::raw::RawSlotCIntBool,
                                                                          arg0,
                                                                          arg1)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_int_bool::qt_widgets_c_SlotWrapper_int_bool()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntBool> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_bool_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_int_bool::set(void (*FN_PTR)(void*, int, bool) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, ::libc::c_int, bool),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_bool_set(self as *mut ::slots::raw::RawSlotCIntBool, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntBool {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_bool_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, int, int, int)` to a Rust extern function.
  ///
  /// Use `SlotCIntCIntCIntCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_int_int_int_int```</span>
  #[repr(C)]
  pub struct RawSlotCIntCIntCIntCInt(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCIntCIntCIntCInt {
    type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,int,int,int)\0"
    }
  }
  impl RawSlotCIntCIntCIntCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_int_int_int_int::custom_slot(int arg0, int arg1, int arg2, int arg3)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: ::libc::c_int, arg2: ::libc::c_int, arg3: ::libc::c_int) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_int_int_int_custom_slot(self as *mut ::slots::raw::RawSlotCIntCIntCIntCInt, arg0, arg1, arg2, arg3) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_int_int_int_int::qt_widgets_c_SlotWrapper_int_int_int_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCIntCIntCInt> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_int_int_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_int_int_int_int::set(void (*FN_PTR)(void*, int, int, int, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          ::libc::c_int,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_int_int_int_set(self as *mut ::slots::raw::RawSlotCIntCIntCIntCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntCIntCIntCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_int_int_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(int, Qt::SortOrder)` to a Rust extern function.
  ///
  /// Use `SlotCIntQtCoreQtSortOrderRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_int_Qt_SortOrder```</span>
  #[repr(C)]
  pub struct RawSlotCIntQtCoreQtSortOrderRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef {
    type Arguments = (::libc::c_int, &'static ::qt_core::qt::SortOrder);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(int,Qt::SortOrder)\0"
    }
  }
  impl RawSlotCIntQtCoreQtSortOrderRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_int_Qt_SortOrder::custom_slot(int arg0, Qt::SortOrder arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::libc::c_int, arg1: &::qt_core::qt::SortOrder) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_Qt_SortOrder_custom_slot(self as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef, arg0, arg1 as *const ::qt_core::qt::SortOrder) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_int_Qt_SortOrder::qt_widgets_c_SlotWrapper_int_Qt_SortOrder()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotCIntQtCoreQtSortOrderRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_Qt_SortOrder_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_int_Qt_SortOrder::set(void (*FN_PTR)(void*, int, const Qt::SortOrder*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          ::libc::c_int,
                                          *const ::qt_core::qt::SortOrder),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_Qt_SortOrder_set(self as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_int_Qt_SortOrder_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QDockWidget*)` to a Rust extern function.
  ///
  /// Use `SlotDockWidgetMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QDockWidget_ptr```</span>
  #[repr(C)]
  pub struct RawSlotDockWidgetMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotDockWidgetMutPtr {
    type Arguments = (*mut ::dock_widget::DockWidget,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QDockWidget*)\0"
    }
  }
  impl RawSlotDockWidgetMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QDockWidget_ptr::custom_slot(QDockWidget* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::dock_widget::DockWidget) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QDockWidget_ptr_custom_slot(self as *mut ::slots::raw::RawSlotDockWidgetMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QDockWidget_ptr::qt_widgets_c_SlotWrapper_QDockWidget_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotDockWidgetMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QDockWidget_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QDockWidget_ptr::set(void (*FN_PTR)(void*, QDockWidget*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::dock_widget::DockWidget),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QDockWidget_ptr_set(self as *mut ::slots::raw::RawSlotDockWidgetMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotDockWidgetMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QDockWidget_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QGraphicsItem*)` to a Rust extern function.
  ///
  /// Use `SlotGraphicsItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QGraphicsItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotGraphicsItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGraphicsItemMutPtr {
    type Arguments = (*mut ::graphics_item::GraphicsItem,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QGraphicsItem*)\0"
    }
  }
  impl RawSlotGraphicsItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr::custom_slot(QGraphicsItem* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::graphics_item::GraphicsItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr::qt_widgets_c_SlotWrapper_QGraphicsItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr::set(void (*FN_PTR)(void*, QGraphicsItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::graphics_item::GraphicsItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_set(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGraphicsItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QGraphicsItem*, QGraphicsItem*)` to a Rust extern function.
  ///
  /// Use `SlotGraphicsItemMutPtrGraphicsItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotGraphicsItemMutPtrGraphicsItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
    type Arguments = (*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QGraphicsItem*,QGraphicsItem*)\0"
    }
  }
  impl RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr::custom_slot(QGraphicsItem* arg0, QGraphicsItem* arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self,
                              arg0: *mut ::graphics_item::GraphicsItem,
                              arg1: *mut ::graphics_item::GraphicsItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr::qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr::set(void (*FN_PTR)(void*, QGraphicsItem*, QGraphicsItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::graphics_item::GraphicsItem,
                                          *mut ::graphics_item::GraphicsItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_set(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QGraphicsItem*, QGraphicsItem*, Qt::FocusReason)` to a Rust extern function.
  ///
  /// Use `SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason```</span>
  #[repr(C)]
  pub struct RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
  type Arguments = (*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem, &'static ::qt_core::qt::FocusReason);
  fn object(&self) -> &::qt_core::object::Object {
    ::cpp_utils::StaticCast::static_cast(self)
  }
  fn receiver_id() -> &'static [u8] {
    b"1custom_slot(QGraphicsItem*,QGraphicsItem*,Qt::FocusReason)\0"
  }
}
  impl RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason::custom_slot(QGraphicsItem* arg0, QGraphicsItem* arg1, Qt::FocusReason arg2)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self,
                              arg0: *mut ::graphics_item::GraphicsItem,
                              arg1: *mut ::graphics_item::GraphicsItem,
                              arg2: &::qt_core::qt::FocusReason) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason_custom_slot(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef, arg0, arg1, arg2 as *const ::qt_core::qt::FocusReason)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason::qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason()```</span>
    ///
    ///
    pub fn new
      ()
        -> ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef>
    {
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason::set(void (*FN_PTR)(void*, QGraphicsItem*, QGraphicsItem*, const Qt::FocusReason*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::graphics_item::GraphicsItem,
                                          *mut ::graphics_item::GraphicsItem,
                                          *const ::qt_core::qt::FocusReason),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason_set(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QList<QRectF>&)` to a Rust extern function.
  ///
  /// Use `SlotListListQtCoreRectFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QList_QRectF_ref```</span>
  #[repr(C)]
  pub struct RawSlotListListQtCoreRectFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotListListQtCoreRectFRef {
    type Arguments = (&'static ::list::ListQtCoreRectF,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QList< QRectF >&)\0"
    }
  }
  impl RawSlotListListQtCoreRectFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QList_QRectF_ref::custom_slot(const QList<QRectF>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::list::ListQtCoreRectF) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref_custom_slot(self as *mut ::slots::raw::RawSlotListListQtCoreRectFRef, arg0 as *const ::list::ListQtCoreRectF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QList_QRectF_ref::qt_widgets_c_SlotWrapper_const_QList_QRectF_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotListListQtCoreRectFRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QList_QRectF_ref::set(void (*FN_PTR)(void*, const QList< QRectF >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::list::ListQtCoreRectF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref_set(self as *mut ::slots::raw::RawSlotListListQtCoreRectFRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotListListQtCoreRectFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QListWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotListWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QListWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotListWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotListWidgetItemMutPtr {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QListWidgetItem*)\0"
    }
  }
  impl RawSlotListWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr::custom_slot(QListWidgetItem* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::list_widget_item::ListWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr::qt_widgets_c_SlotWrapper_QListWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotListWidgetItemMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr::set(void (*FN_PTR)(void*, QListWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::list_widget_item::ListWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotListWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QListWidgetItem*, QListWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotListWidgetItemMutPtrListWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotListWidgetItemMutPtrListWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
    type Arguments = (*mut ::list_widget_item::ListWidgetItem, *mut ::list_widget_item::ListWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QListWidgetItem*,QListWidgetItem*)\0"
    }
  }
  impl RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr::custom_slot(QListWidgetItem* arg0, QListWidgetItem* arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self,
                              arg0: *mut ::list_widget_item::ListWidgetItem,
                              arg1: *mut ::list_widget_item::ListWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr::qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr::set(void (*FN_PTR)(void*, QListWidgetItem*, QListWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::list_widget_item::ListWidgetItem,
                                          *mut ::list_widget_item::ListWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QMdiSubWindow*)` to a Rust extern function.
  ///
  /// Use `SlotMdiSubWindowMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr```</span>
  #[repr(C)]
  pub struct RawSlotMdiSubWindowMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotMdiSubWindowMutPtr {
    type Arguments = (*mut ::mdi_sub_window::MdiSubWindow,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QMdiSubWindow*)\0"
    }
  }
  impl RawSlotMdiSubWindowMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr::custom_slot(QMdiSubWindow* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::mdi_sub_window::MdiSubWindow) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr_custom_slot(self as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr::qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotMdiSubWindowMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr::set(void (*FN_PTR)(void*, QMdiSubWindow*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::mdi_sub_window::MdiSubWindow),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr_set(self as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotMdiSubWindowMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QDate&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreDateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QDate_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreDateRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreDateRef {
    type Arguments = (&'static ::qt_core::date::Date,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QDate&)\0"
    }
  }
  impl RawSlotQtCoreDateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QDate_ref::custom_slot(const QDate& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::date::Date) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDate_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreDateRef, arg0 as *const ::qt_core::date::Date) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QDate_ref::qt_widgets_c_SlotWrapper_const_QDate_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreDateRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDate_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QDate_ref::set(void (*FN_PTR)(void*, const QDate*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::date::Date),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDate_ref_set(self as *mut ::slots::raw::RawSlotQtCoreDateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreDateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDate_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QDateTime&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreDateTimeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QDateTime_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreDateTimeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreDateTimeRef {
    type Arguments = (&'static ::qt_core::date_time::DateTime,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QDateTime&)\0"
    }
  }
  impl RawSlotQtCoreDateTimeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QDateTime_ref::custom_slot(const QDateTime& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::date_time::DateTime) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDateTime_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreDateTimeRef, arg0 as *const ::qt_core::date_time::DateTime) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QDateTime_ref::qt_widgets_c_SlotWrapper_const_QDateTime_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreDateTimeRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDateTime_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QDateTime_ref::set(void (*FN_PTR)(void*, const QDateTime*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::date_time::DateTime),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDateTime_ref_set(self as *mut ::slots::raw::RawSlotQtCoreDateTimeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreDateTimeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QDateTime_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QFlags<QGraphicsBlurEffect::BlurHint>)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreFlagsGraphicsBlurEffectBlurHint` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
    type Arguments = (::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QFlags< QGraphicsBlurEffect::BlurHint >)\0"
    }
  }
  impl RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint::custom_slot(QFlags<QGraphicsBlurEffect::BlurHint> arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: ::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint, arg0.to_int() as ::libc::c_uint) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint::qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint::set(void (*FN_PTR)(void*, unsigned int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self, func: extern "C" fn(*mut ::libc::c_void, ::libc::c_uint), data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint_set(self as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QList<QModelIndex>&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreListListModelIndexRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreListListModelIndexRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreListListModelIndexRef {
    type Arguments = (&'static ::qt_core::list::ListModelIndex,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QList< QModelIndex >&)\0"
    }
  }
  impl RawSlotQtCoreListListModelIndexRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref::custom_slot(const QList<QModelIndex>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::list::ListModelIndex) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef, arg0 as *const ::qt_core::list::ListModelIndex) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref::qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreListListModelIndexRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref::set(void (*FN_PTR)(void*, const QList< QModelIndex >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::list::ListModelIndex),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref_set(self as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreListListModelIndexRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QList<QUrl>&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreListListUrlRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QList_QUrl_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreListListUrlRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreListListUrlRef {
    type Arguments = (&'static ::qt_core::list::ListUrl,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QList< QUrl >&)\0"
    }
  }
  impl RawSlotQtCoreListListUrlRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QList_QUrl_ref::custom_slot(const QList<QUrl>& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::list::ListUrl) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreListListUrlRef, arg0 as *const ::qt_core::list::ListUrl) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QList_QUrl_ref::qt_widgets_c_SlotWrapper_const_QList_QUrl_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreListListUrlRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QList_QUrl_ref::set(void (*FN_PTR)(void*, const QList< QUrl >*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::list::ListUrl),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref_set(self as *mut ::slots::raw::RawSlotQtCoreListListUrlRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreListListUrlRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QPointF&)` to a Rust extern function.
  ///
  /// Use `SlotQtCorePointFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QPointF_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCorePointFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCorePointFRef {
    type Arguments = (&'static ::qt_core::point_f::PointF,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QPointF&)\0"
    }
  }
  impl RawSlotQtCorePointFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QPointF_ref::custom_slot(const QPointF& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::point_f::PointF) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPointF_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCorePointFRef, arg0 as *const ::qt_core::point_f::PointF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QPointF_ref::qt_widgets_c_SlotWrapper_const_QPointF_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCorePointFRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPointF_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QPointF_ref::set(void (*FN_PTR)(void*, const QPointF*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::point_f::PointF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPointF_ref_set(self as *mut ::slots::raw::RawSlotQtCorePointFRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCorePointFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPointF_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QPoint&)` to a Rust extern function.
  ///
  /// Use `SlotQtCorePointRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QPoint_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCorePointRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCorePointRef {
    type Arguments = (&'static ::qt_core::point::Point,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QPoint&)\0"
    }
  }
  impl RawSlotQtCorePointRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QPoint_ref::custom_slot(const QPoint& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::point::Point) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPoint_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCorePointRef, arg0 as *const ::qt_core::point::Point) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QPoint_ref::qt_widgets_c_SlotWrapper_const_QPoint_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCorePointRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPoint_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QPoint_ref::set(void (*FN_PTR)(void*, const QPoint*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::point::Point),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPoint_ref_set(self as *mut ::slots::raw::RawSlotQtCorePointRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCorePointRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QPoint_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::DockWidgetArea)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtDockWidgetAreaRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_Qt_DockWidgetArea```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtDockWidgetAreaRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef {
    type Arguments = (&'static ::qt_core::qt::DockWidgetArea,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::DockWidgetArea)\0"
    }
  }
  impl RawSlotQtCoreQtDockWidgetAreaRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_Qt_DockWidgetArea::custom_slot(Qt::DockWidgetArea arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::DockWidgetArea) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef, arg0 as *const ::qt_core::qt::DockWidgetArea) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_Qt_DockWidgetArea::qt_widgets_c_SlotWrapper_Qt_DockWidgetArea()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_Qt_DockWidgetArea::set(void (*FN_PTR)(void*, const Qt::DockWidgetArea*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::DockWidgetArea),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea_set(self as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(Qt::ToolButtonStyle)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreQtToolButtonStyleRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreQtToolButtonStyleRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef {
    type Arguments = (&'static ::qt_core::qt::ToolButtonStyle,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(Qt::ToolButtonStyle)\0"
    }
  }
  impl RawSlotQtCoreQtToolButtonStyleRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle::custom_slot(Qt::ToolButtonStyle arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::qt::ToolButtonStyle) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef, arg0 as *const ::qt_core::qt::ToolButtonStyle) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle::qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtToolButtonStyleRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle::set(void (*FN_PTR)(void*, const Qt::ToolButtonStyle*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::qt::ToolButtonStyle),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle_set(self as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QRect)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QRect```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectRef {
    type Arguments = (&'static ::qt_core::rect::Rect,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QRect)\0"
    }
  }
  impl RawSlotQtCoreRectRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QRect::custom_slot(QRect arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::rect::Rect) {
      unsafe {
        ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectRef, arg0 as *const ::qt_core::rect::Rect)
      }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QRect::qt_widgets_c_SlotWrapper_QRect()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QRect::set(void (*FN_PTR)(void*, const QRect*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::rect::Rect),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_set(self as *mut ::slots::raw::RawSlotQtCoreRectRef,
                                                             func,
                                                             data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QRect&, int)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectRefCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QRect_ref_int```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectRefCInt(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectRefCInt {
    type Arguments = (&'static ::qt_core::rect::Rect, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QRect&,int)\0"
    }
  }
  impl RawSlotQtCoreRectRefCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QRect_ref_int::custom_slot(const QRect& arg0, int arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::rect::Rect, arg1: ::libc::c_int) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QRect_ref_int_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectRefCInt, arg0 as *const ::qt_core::rect::Rect, arg1) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QRect_ref_int::qt_widgets_c_SlotWrapper_const_QRect_ref_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefCInt> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QRect_ref_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QRect_ref_int::set(void (*FN_PTR)(void*, const QRect*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::rect::Rect,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QRect_ref_int_set(self as *mut ::slots::raw::RawSlotQtCoreRectRefCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectRefCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QRect_ref_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QRect, QPointF)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectRefQtCorePointFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QRect_QPointF```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectRefQtCorePointFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef {
    type Arguments = (&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QRect,QPointF)\0"
    }
  }
  impl RawSlotQtCoreRectRefQtCorePointFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QRect_QPointF::custom_slot(QRect arg0, QPointF arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::rect::Rect, arg1: &::qt_core::point_f::PointF) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef, arg0 as *const ::qt_core::rect::Rect, arg1 as *const ::qt_core::point_f::PointF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QRect_QPointF::qt_widgets_c_SlotWrapper_QRect_QPointF()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QRect_QPointF::set(void (*FN_PTR)(void*, const QRect*, const QPointF*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::rect::Rect,
                                          *const ::qt_core::point_f::PointF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_set(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QRect, QPointF, QPointF)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
    type Arguments = (&'static ::qt_core::rect::Rect,
     &'static ::qt_core::point_f::PointF,
     &'static ::qt_core::point_f::PointF);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QRect,QPointF,QPointF)\0"
    }
  }
  impl RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF::custom_slot(QRect arg0, QPointF arg1, QPointF arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::qt_core::rect::Rect,
                       arg1: &::qt_core::point_f::PointF,
                       arg2: &::qt_core::point_f::PointF) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef, arg0 as *const ::qt_core::rect::Rect, arg1 as *const ::qt_core::point_f::PointF, arg2 as *const ::qt_core::point_f::PointF) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF::qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF::set(void (*FN_PTR)(void*, const QRect*, const QPointF*, const QPointF*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::rect::Rect,
                                          *const ::qt_core::point_f::PointF,
                                          *const ::qt_core::point_f::PointF),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF_set(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF_delete
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
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QStringList_ref```</span>
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
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QStringList_ref::custom_slot(const QStringList& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::string_list::StringList) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QStringList_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreStringListRef, arg0 as *const ::qt_core::string_list::StringList) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QStringList_ref::qt_widgets_c_SlotWrapper_const_QStringList_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringListRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QStringList_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QStringList_ref::set(void (*FN_PTR)(void*, const QStringList*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::string_list::StringList),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QStringList_ref_set(self as *mut ::slots::raw::RawSlotQtCoreStringListRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreStringListRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QStringList_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QString&, const QString&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreStringRefQtCoreStringRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreStringRefQtCoreStringRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef {
    type Arguments = (&'static ::qt_core::string::String, &'static ::qt_core::string::String);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QString&,const QString&)\0"
    }
  }
  impl RawSlotQtCoreStringRefQtCoreStringRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref::custom_slot(const QString& arg0, const QString& arg1)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::string::String, arg1: &::qt_core::string::String) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef, arg0 as *const ::qt_core::string::String, arg1 as *const ::qt_core::string::String) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref::qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref::set(void (*FN_PTR)(void*, const QString*, const QString*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::string::String,
                                          *const ::qt_core::string::String),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_set(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QString&, const QString&, const QString&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
    type Arguments = (&'static ::qt_core::string::String,
     &'static ::qt_core::string::String,
     &'static ::qt_core::string::String);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QString&,const QString&,const QString&)\0"
    }
  }
  impl RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref::custom_slot(const QString& arg0, const QString& arg1, const QString& arg2)```</span>
    ///
    ///
    pub fn custom_slot(&mut self,
                       arg0: &::qt_core::string::String,
                       arg1: &::qt_core::string::String,
                       arg2: &::qt_core::string::String) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef, arg0 as *const ::qt_core::string::String, arg1 as *const ::qt_core::string::String, arg2 as *const ::qt_core::string::String) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref::qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef> {
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref_new()
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref::set(void (*FN_PTR)(void*, const QString*, const QString*, const QString*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_core::string::String,
                                          *const ::qt_core::string::String,
                                          *const ::qt_core::string::String),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref_set(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QTime&)` to a Rust extern function.
  ///
  /// Use `SlotQtCoreTimeRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QTime_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtCoreTimeRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtCoreTimeRef {
    type Arguments = (&'static ::qt_core::time::Time,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QTime&)\0"
    }
  }
  impl RawSlotQtCoreTimeRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QTime_ref::custom_slot(const QTime& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_core::time::Time) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTime_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtCoreTimeRef, arg0 as *const ::qt_core::time::Time) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QTime_ref::qt_widgets_c_SlotWrapper_const_QTime_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreTimeRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTime_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QTime_ref::set(void (*FN_PTR)(void*, const QTime*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_core::time::Time),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTime_ref_set(self as *mut ::slots::raw::RawSlotQtCoreTimeRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtCoreTimeRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTime_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QBrush&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiBrushRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QBrush_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiBrushRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiBrushRef {
    type Arguments = (&'static ::qt_gui::brush::Brush,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QBrush&)\0"
    }
  }
  impl RawSlotQtGuiBrushRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QBrush_ref::custom_slot(const QBrush& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::brush::Brush) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QBrush_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiBrushRef, arg0 as *const ::qt_gui::brush::Brush) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QBrush_ref::qt_widgets_c_SlotWrapper_const_QBrush_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiBrushRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QBrush_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QBrush_ref::set(void (*FN_PTR)(void*, const QBrush*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::brush::Brush),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QBrush_ref_set(self as *mut ::slots::raw::RawSlotQtGuiBrushRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiBrushRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QBrush_ref_delete
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
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QColor_ref```</span>
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
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QColor_ref::custom_slot(const QColor& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::color::Color) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QColor_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiColorRef, arg0 as *const ::qt_gui::color::Color) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QColor_ref::qt_widgets_c_SlotWrapper_const_QColor_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiColorRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QColor_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QColor_ref::set(void (*FN_PTR)(void*, const QColor*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::color::Color),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QColor_ref_set(self as *mut ::slots::raw::RawSlotQtGuiColorRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiColorRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QColor_ref_delete
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
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QFont_ref```</span>
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
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QFont_ref::custom_slot(const QFont& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::font::Font) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QFont_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiFontRef, arg0 as *const ::qt_gui::font::Font) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QFont_ref::qt_widgets_c_SlotWrapper_const_QFont_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiFontRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QFont_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QFont_ref::set(void (*FN_PTR)(void*, const QFont*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::font::Font),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QFont_ref_set(self as *mut ::slots::raw::RawSlotQtGuiFontRef,
                                                                       func,
                                                                       data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiFontRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QFont_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QIcon&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiIconRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QIcon_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiIconRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiIconRef {
    type Arguments = (&'static ::qt_gui::icon::Icon,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QIcon&)\0"
    }
  }
  impl RawSlotQtGuiIconRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QIcon_ref::custom_slot(const QIcon& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::icon::Icon) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QIcon_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiIconRef, arg0 as *const ::qt_gui::icon::Icon) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QIcon_ref::qt_widgets_c_SlotWrapper_const_QIcon_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiIconRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QIcon_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QIcon_ref::set(void (*FN_PTR)(void*, const QIcon*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::qt_gui::icon::Icon),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QIcon_ref_set(self as *mut ::slots::raw::RawSlotQtGuiIconRef,
                                                                       func,
                                                                       data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiIconRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QIcon_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QKeySequence&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiKeySequenceRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QKeySequence_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiKeySequenceRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiKeySequenceRef {
    type Arguments = (&'static ::qt_gui::key_sequence::KeySequence,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QKeySequence&)\0"
    }
  }
  impl RawSlotQtGuiKeySequenceRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QKeySequence_ref::custom_slot(const QKeySequence& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::key_sequence::KeySequence) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QKeySequence_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef, arg0 as *const ::qt_gui::key_sequence::KeySequence) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QKeySequence_ref::qt_widgets_c_SlotWrapper_const_QKeySequence_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiKeySequenceRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QKeySequence_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QKeySequence_ref::set(void (*FN_PTR)(void*, const QKeySequence*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_gui::key_sequence::KeySequence),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QKeySequence_ref_set(self as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiKeySequenceRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QKeySequence_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QTextCharFormat&)` to a Rust extern function.
  ///
  /// Use `SlotQtGuiTextCharFormatRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref```</span>
  #[repr(C)]
  pub struct RawSlotQtGuiTextCharFormatRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotQtGuiTextCharFormatRef {
    type Arguments = (&'static ::qt_gui::text_char_format::TextCharFormat,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QTextCharFormat&)\0"
    }
  }
  impl RawSlotQtGuiTextCharFormatRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref::custom_slot(const QTextCharFormat& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::qt_gui::text_char_format::TextCharFormat) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref_custom_slot(self as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef, arg0 as *const ::qt_gui::text_char_format::TextCharFormat) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref::qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiTextCharFormatRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref::set(void (*FN_PTR)(void*, const QTextCharFormat*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::qt_gui::text_char_format::TextCharFormat),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref_set(self as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotQtGuiTextCharFormatRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(const QScrollerProperties&)` to a Rust extern function.
  ///
  /// Use `SlotScrollerPropertiesRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref```</span>
  #[repr(C)]
  pub struct RawSlotScrollerPropertiesRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotScrollerPropertiesRef {
    type Arguments = (&'static ::scroller_properties::ScrollerProperties,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(const QScrollerProperties&)\0"
    }
  }
  impl RawSlotScrollerPropertiesRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref::custom_slot(const QScrollerProperties& arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::scroller_properties::ScrollerProperties) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref_custom_slot(self as *mut ::slots::raw::RawSlotScrollerPropertiesRef, arg0 as *const ::scroller_properties::ScrollerProperties) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref::qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotScrollerPropertiesRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref::set(void (*FN_PTR)(void*, const QScrollerProperties*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::scroller_properties::ScrollerProperties),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref_set(self as *mut ::slots::raw::RawSlotScrollerPropertiesRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotScrollerPropertiesRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QScroller::State)` to a Rust extern function.
  ///
  /// Use `SlotScrollerStateRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QScroller_State```</span>
  #[repr(C)]
  pub struct RawSlotScrollerStateRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotScrollerStateRef {
    type Arguments = (&'static ::scroller::State,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QScroller::State)\0"
    }
  }
  impl RawSlotScrollerStateRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QScroller_State::custom_slot(QScroller::State arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::scroller::State) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QScroller_State_custom_slot(self as *mut ::slots::raw::RawSlotScrollerStateRef, arg0 as *const ::scroller::State) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QScroller_State::qt_widgets_c_SlotWrapper_QScroller_State()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotScrollerStateRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QScroller_State_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QScroller_State::set(void (*FN_PTR)(void*, const QScroller::State*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *const ::scroller::State),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QScroller_State_set(self as *mut ::slots::raw::RawSlotScrollerStateRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotScrollerStateRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QScroller_State_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QSystemTrayIcon::ActivationReason)` to a Rust extern function.
  ///
  /// Use `SlotSystemTrayIconActivationReasonRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason```</span>
  #[repr(C)]
  pub struct RawSlotSystemTrayIconActivationReasonRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotSystemTrayIconActivationReasonRef {
    type Arguments = (&'static ::system_tray_icon::ActivationReason,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QSystemTrayIcon::ActivationReason)\0"
    }
  }
  impl RawSlotSystemTrayIconActivationReasonRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason::custom_slot(QSystemTrayIcon::ActivationReason arg0)```</span>
    ///
    ///
    pub fn custom_slot(&mut self, arg0: &::system_tray_icon::ActivationReason) {
      unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason_custom_slot(self as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef, arg0 as *const ::system_tray_icon::ActivationReason) }
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason::qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotSystemTrayIconActivationReasonRef> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason::set(void (*FN_PTR)(void*, const QSystemTrayIcon::ActivationReason*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *const ::system_tray_icon::ActivationReason),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason_set(self as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotSystemTrayIconActivationReasonRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTableWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotTableWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotTableWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTableWidgetItemMutPtr {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTableWidgetItem*)\0"
    }
  }
  impl RawSlotTableWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr::custom_slot(QTableWidgetItem* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::table_widget_item::TableWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr::qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTableWidgetItemMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr::set(void (*FN_PTR)(void*, QTableWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::table_widget_item::TableWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTableWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTableWidgetItem*, QTableWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotTableWidgetItemMutPtrTableWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
    type Arguments = (*mut ::table_widget_item::TableWidgetItem, *mut ::table_widget_item::TableWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTableWidgetItem*,QTableWidgetItem*)\0"
    }
  }
  impl RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr::custom_slot(QTableWidgetItem* arg0, QTableWidgetItem* arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self,
                              arg0: *mut ::table_widget_item::TableWidgetItem,
                              arg1: *mut ::table_widget_item::TableWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr::qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr::set(void (*FN_PTR)(void*, QTableWidgetItem*, QTableWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::table_widget_item::TableWidgetItem,
                                          *mut ::table_widget_item::TableWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTreeWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotTreeWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotTreeWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTreeWidgetItemMutPtr {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTreeWidgetItem*)\0"
    }
  }
  impl RawSlotTreeWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr::custom_slot(QTreeWidgetItem* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::tree_widget_item::TreeWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr::qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr::set(void (*FN_PTR)(void*, QTreeWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::tree_widget_item::TreeWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTreeWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTreeWidgetItem*, int)` to a Rust extern function.
  ///
  /// Use `SlotTreeWidgetItemMutPtrCInt` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int```</span>
  #[repr(C)]
  pub struct RawSlotTreeWidgetItemMutPtrCInt(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTreeWidgetItem*,int)\0"
    }
  }
  impl RawSlotTreeWidgetItemMutPtrCInt {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int::custom_slot(QTreeWidgetItem* arg0, int arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::tree_widget_item::TreeWidgetItem, arg1: ::libc::c_int) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int_custom_slot(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int::qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtrCInt> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int::set(void (*FN_PTR)(void*, QTreeWidgetItem*, int) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::tree_widget_item::TreeWidgetItem,
                                          ::libc::c_int),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int_set(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QTreeWidgetItem*, QTreeWidgetItem*)` to a Rust extern function.
  ///
  /// Use `SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr```</span>
  #[repr(C)]
  pub struct RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
    type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QTreeWidgetItem*,QTreeWidgetItem*)\0"
    }
  }
  impl RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr::custom_slot(QTreeWidgetItem* arg0, QTreeWidgetItem* arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self,
                              arg0: *mut ::tree_widget_item::TreeWidgetItem,
                              arg1: *mut ::tree_widget_item::TreeWidgetItem) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr_custom_slot(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr::qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr::set(void (*FN_PTR)(void*, QTreeWidgetItem*, QTreeWidgetItem*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::tree_widget_item::TreeWidgetItem,
                                          *mut ::tree_widget_item::TreeWidgetItem),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr_set(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QUndoStack*)` to a Rust extern function.
  ///
  /// Use `SlotUndoStackMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QUndoStack_ptr```</span>
  #[repr(C)]
  pub struct RawSlotUndoStackMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotUndoStackMutPtr {
    type Arguments = (*mut ::undo_stack::UndoStack,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QUndoStack*)\0"
    }
  }
  impl RawSlotUndoStackMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QUndoStack_ptr::custom_slot(QUndoStack* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::undo_stack::UndoStack) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QUndoStack_ptr_custom_slot(self as *mut ::slots::raw::RawSlotUndoStackMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QUndoStack_ptr::qt_widgets_c_SlotWrapper_QUndoStack_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotUndoStackMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QUndoStack_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QUndoStack_ptr::set(void (*FN_PTR)(void*, QUndoStack*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::undo_stack::UndoStack),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QUndoStack_ptr_set(self as *mut ::slots::raw::RawSlotUndoStackMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotUndoStackMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QUndoStack_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QWidget*)` to a Rust extern function.
  ///
  /// Use `SlotWidgetMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QWidget_ptr```</span>
  #[repr(C)]
  pub struct RawSlotWidgetMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotWidgetMutPtr {
    type Arguments = (*mut ::widget::Widget,);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QWidget*)\0"
    }
  }
  impl RawSlotWidgetMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QWidget_ptr::custom_slot(QWidget* arg0)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::widget::Widget) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_custom_slot(self as *mut ::slots::raw::RawSlotWidgetMutPtr, arg0)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QWidget_ptr::qt_widgets_c_SlotWrapper_QWidget_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QWidget_ptr::set(void (*FN_PTR)(void*, QWidget*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void, *mut ::widget::Widget),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_set(self as *mut ::slots::raw::RawSlotWidgetMutPtr,
                                                                   func,
                                                                   data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotWidgetMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QWidget*, QAbstractItemDelegate::EndEditHint)` to a Rust extern function.
  ///
  /// Use `SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint```</span>
  #[repr(C)]
  pub struct RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
    type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QWidget*,QAbstractItemDelegate::EndEditHint)\0"
    }
  }
  impl RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint::custom_slot(QWidget* arg0, QAbstractItemDelegate::EndEditHint arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::widget::Widget, arg1: &::abstract_item_delegate::EndEditHint) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint_custom_slot(self as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef, arg0, arg1 as *const ::abstract_item_delegate::EndEditHint)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint::qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef> {
      let ffi_result =
        unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint::set(void (*FN_PTR)(void*, QWidget*, const QAbstractItemDelegate::EndEditHint*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::widget::Widget,
                                          *const ::abstract_item_delegate::EndEditHint),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint_set(self as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint_delete
    }
  }

  /// Allows to bind Qt signals with arguments `(QWidget*, QWidget*)` to a Rust extern function.
  ///
  /// Use `SlotWidgetMutPtrWidgetMutPtr` to bind signals to a Rust closure instead.
  ///
  /// Create an object using `new()` and bind your function and payload using `set()`. The function will receive the payload as its first arguments, and the rest of arguments will be values passed through the Qt connection system. Use `connect()` method of a `qt_core::connection::Signal` object to connect the signal to this slot. The callback function will be executed each time the slot is invoked until source signals are disconnected or the slot object is destroyed.
  ///
  /// If `set()` was not called, slot invokation has no effect.
  ///
  /// C++ type: <span style='color: green;'>```qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr```</span>
  #[repr(C)]
  pub struct RawSlotWidgetMutPtrWidgetMutPtr(u8);

  impl ::qt_core::connection::Receiver for ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr {
    type Arguments = (*mut ::widget::Widget, *mut ::widget::Widget);
    fn object(&self) -> &::qt_core::object::Object {
      ::cpp_utils::StaticCast::static_cast(self)
    }
    fn receiver_id() -> &'static [u8] {
      b"1custom_slot(QWidget*,QWidget*)\0"
    }
  }
  impl RawSlotWidgetMutPtrWidgetMutPtr {
    /// Executes the callback function, as if the slot was invoked with these arguments. Does nothing if no callback function was set.
    ///
    /// C++ method: <span style='color: green;'>```virtual [slot] void qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr::custom_slot(QWidget* arg0, QWidget* arg1)```</span>
    ///
    ///
    pub unsafe fn custom_slot(&mut self, arg0: *mut ::widget::Widget, arg1: *mut ::widget::Widget) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr_custom_slot(self as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr, arg0, arg1)
    }

    /// Constructs a new object.
    ///
    /// C++ method: <span style='color: green;'>```virtual [constructor] void qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr::qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr()```</span>
    ///
    ///
    pub fn new() -> ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr> {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr_new() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }

    /// Sets `func` as the callback function and `data` as the payload. When the slot is invoked, `func(data)` will be called. Note that it may happen at any time and in any thread.
    ///
    /// C++ method: <span style='color: green;'>```virtual void qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr::set(void (*FN_PTR)(void*, QWidget*, QWidget*) func, void* data)```</span>
    ///
    ///
    pub unsafe fn set(&mut self,
                      func: extern "C" fn(*mut ::libc::c_void,
                                          *mut ::widget::Widget,
                                          *mut ::widget::Widget),
                      data: *mut ::libc::c_void) {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr_set(self as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr, func, data)
    }
  }

  impl ::cpp_utils::CppDeletable for ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr {
    fn deleter() -> ::cpp_utils::Deleter<Self> {
      ::ffi::qt_widgets_c_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr_delete
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractButtonMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr(self as *const ::slots::raw::RawSlotAbstractButtonMutPtr as *mut ::slots::raw::RawSlotAbstractButtonMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotAbstractButtonMutPtrBool {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool(self as *const ::slots::raw::RawSlotAbstractButtonMutPtrBool as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotActionMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAction_ptr(self as *mut ::slots::raw::RawSlotActionMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAction_ptr(self as *const ::slots::raw::RawSlotActionMutPtr as *mut ::slots::raw::RawSlotActionMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotDockWidgetMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QDockWidget_ptr(self as *mut ::slots::raw::RawSlotDockWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QDockWidget_ptr(self as *const ::slots::raw::RawSlotDockWidgetMutPtr as *mut ::slots::raw::RawSlotDockWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint(self as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint(self as *const ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGraphicsItemMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr(self as *const ::slots::raw::RawSlotGraphicsItemMutPtr as *mut ::slots::raw::RawSlotGraphicsItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr(self as *const ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason(self as *const ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotListWidgetItemMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr(self as *const ::slots::raw::RawSlotListWidgetItemMutPtr as *mut ::slots::raw::RawSlotListWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr(self as *const ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotMdiSubWindowMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr(self as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr(self as *const ::slots::raw::RawSlotMdiSubWindowMutPtr as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect(self as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect(self as *const ::slots::raw::RawSlotQtCoreRectRef as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF(self as *const ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF(self as *const ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotScrollerStateRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QScroller_State(self as *mut ::slots::raw::RawSlotScrollerStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QScroller_State(self as *const ::slots::raw::RawSlotScrollerStateRef as *mut ::slots::raw::RawSlotScrollerStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotSystemTrayIconActivationReasonRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason(self as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason(self as *const ::slots::raw::RawSlotSystemTrayIconActivationReasonRef as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTableWidgetItemMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr(self as *const ::slots::raw::RawSlotTableWidgetItemMutPtr as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr(self as *const ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTreeWidgetItemMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtr as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotUndoStackMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QUndoStack_ptr(self as *mut ::slots::raw::RawSlotUndoStackMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QUndoStack_ptr(self as *const ::slots::raw::RawSlotUndoStackMutPtr as *mut ::slots::raw::RawSlotUndoStackMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotWidgetMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr(self as *mut ::slots::raw::RawSlotWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr(self as *const ::slots::raw::RawSlotWidgetMutPtr as *mut ::slots::raw::RawSlotWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint(self as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint(self as *const ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr(self as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr(self as *const ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea(self as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea(self as *const ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle(self as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle(self as *const ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiBrushRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QBrush_ref(self as *mut ::slots::raw::RawSlotQtGuiBrushRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QBrush_ref(self as *const ::slots::raw::RawSlotQtGuiBrushRef as *mut ::slots::raw::RawSlotQtGuiBrushRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiColorRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QColor_ref(self as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QColor_ref(self as *const ::slots::raw::RawSlotQtGuiColorRef as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreDateTimeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDateTime_ref(self as *mut ::slots::raw::RawSlotQtCoreDateTimeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDateTime_ref(self as *const ::slots::raw::RawSlotQtCoreDateTimeRef as *mut ::slots::raw::RawSlotQtCoreDateTimeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreDateRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDate_ref(self as *mut ::slots::raw::RawSlotQtCoreDateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDate_ref(self as *const ::slots::raw::RawSlotQtCoreDateRef as *mut ::slots::raw::RawSlotQtCoreDateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiFontRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QFont_ref(self as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QFont_ref(self as *const ::slots::raw::RawSlotQtGuiFontRef as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiIconRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QIcon_ref(self as *mut ::slots::raw::RawSlotQtGuiIconRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QIcon_ref(self as *const ::slots::raw::RawSlotQtGuiIconRef as *mut ::slots::raw::RawSlotQtGuiIconRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiKeySequenceRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QKeySequence_ref(self as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QKeySequence_ref(self as *const ::slots::raw::RawSlotQtGuiKeySequenceRef as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreListListModelIndexRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref(self as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref(self as *const ::slots::raw::RawSlotQtCoreListListModelIndexRef as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotListListQtCoreRectFRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref(self as *mut ::slots::raw::RawSlotListListQtCoreRectFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref(self as *const ::slots::raw::RawSlotListListQtCoreRectFRef as *mut ::slots::raw::RawSlotListListQtCoreRectFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreListListUrlRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref(self as *mut ::slots::raw::RawSlotQtCoreListListUrlRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref(self as *const ::slots::raw::RawSlotQtCoreListListUrlRef as *mut ::slots::raw::RawSlotQtCoreListListUrlRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCorePointFRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPointF_ref(self as *mut ::slots::raw::RawSlotQtCorePointFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPointF_ref(self as *const ::slots::raw::RawSlotQtCorePointFRef as *mut ::slots::raw::RawSlotQtCorePointFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCorePointRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPoint_ref(self as *mut ::slots::raw::RawSlotQtCorePointRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPoint_ref(self as *const ::slots::raw::RawSlotQtCorePointRef as *mut ::slots::raw::RawSlotQtCorePointRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreRectRefCInt {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QRect_ref_int(self as *mut ::slots::raw::RawSlotQtCoreRectRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QRect_ref_int(self as *const ::slots::raw::RawSlotQtCoreRectRefCInt as *mut ::slots::raw::RawSlotQtCoreRectRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotScrollerPropertiesRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref(self as *mut ::slots::raw::RawSlotScrollerPropertiesRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref(self as *const ::slots::raw::RawSlotScrollerPropertiesRef as *mut ::slots::raw::RawSlotScrollerPropertiesRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreStringListRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QStringList_ref(self as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QStringList_ref(self as *const ::slots::raw::RawSlotQtCoreStringListRef as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref(self as *const ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref(self as *const ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtGuiTextCharFormatRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref(self as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref(self as *const ::slots::raw::RawSlotQtGuiTextCharFormatRef as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotQtCoreTimeRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTime_ref(self as *mut ::slots::raw::RawSlotQtCoreTimeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTime_ref(self as *const ::slots::raw::RawSlotQtCoreTimeRef as *mut ::slots::raw::RawSlotQtCoreTimeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_Qt_SortOrder(self as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_Qt_SortOrder(self as *const ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCIntBool {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_bool(self as *mut ::slots::raw::RawSlotCIntBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_bool(self as *const ::slots::raw::RawSlotCIntBool as *mut ::slots::raw::RawSlotCIntBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::slots::raw::RawSlotCIntCIntCIntCInt {
    fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_int_int_int(self as *mut ::slots::raw::RawSlotCIntCIntCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }

    fn static_cast(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_int_int_int(self as *const ::slots::raw::RawSlotCIntCIntCIntCInt as *mut ::slots::raw::RawSlotCIntCIntCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractButtonMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr(self as *const ::slots::raw::RawSlotAbstractButtonMutPtr as *mut ::slots::raw::RawSlotAbstractButtonMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotAbstractButtonMutPtrBool {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool(self as *const ::slots::raw::RawSlotAbstractButtonMutPtrBool as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotActionMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAction_ptr(self as *const ::slots::raw::RawSlotActionMutPtr as *mut ::slots::raw::RawSlotActionMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotDockWidgetMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QDockWidget_ptr(self as *const ::slots::raw::RawSlotDockWidgetMutPtr as *mut ::slots::raw::RawSlotDockWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint(self as *const ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGraphicsItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr(self as *const ::slots::raw::RawSlotGraphicsItemMutPtr as *mut ::slots::raw::RawSlotGraphicsItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr(self as *const ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason(self as *const ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotListWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr(self as *const ::slots::raw::RawSlotListWidgetItemMutPtr as *mut ::slots::raw::RawSlotListWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr(self as *const ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotMdiSubWindowMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr(self as *const ::slots::raw::RawSlotMdiSubWindowMutPtr as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect(self as *const ::slots::raw::RawSlotQtCoreRectRef as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF(self as *const ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF(self as *const ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotScrollerStateRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QScroller_State(self as *const ::slots::raw::RawSlotScrollerStateRef as *mut ::slots::raw::RawSlotScrollerStateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotSystemTrayIconActivationReasonRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason(self as *const ::slots::raw::RawSlotSystemTrayIconActivationReasonRef as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTableWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr(self as *const ::slots::raw::RawSlotTableWidgetItemMutPtr as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr(self as *const ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTreeWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtr as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int(self as *const ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotUndoStackMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QUndoStack_ptr(self as *const ::slots::raw::RawSlotUndoStackMutPtr as *mut ::slots::raw::RawSlotUndoStackMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotWidgetMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr(self as *const ::slots::raw::RawSlotWidgetMutPtr as *mut ::slots::raw::RawSlotWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint(self as *const ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr(self as *const ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea(self as *const ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle(self as *const ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiBrushRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QBrush_ref(self as *const ::slots::raw::RawSlotQtGuiBrushRef as *mut ::slots::raw::RawSlotQtGuiBrushRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiColorRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QColor_ref(self as *const ::slots::raw::RawSlotQtGuiColorRef as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreDateTimeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDateTime_ref(self as *const ::slots::raw::RawSlotQtCoreDateTimeRef as *mut ::slots::raw::RawSlotQtCoreDateTimeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreDateRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDate_ref(self as *const ::slots::raw::RawSlotQtCoreDateRef as *mut ::slots::raw::RawSlotQtCoreDateRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiFontRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QFont_ref(self as *const ::slots::raw::RawSlotQtGuiFontRef as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiIconRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QIcon_ref(self as *const ::slots::raw::RawSlotQtGuiIconRef as *mut ::slots::raw::RawSlotQtGuiIconRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiKeySequenceRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QKeySequence_ref(self as *const ::slots::raw::RawSlotQtGuiKeySequenceRef as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreListListModelIndexRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref(self as *const ::slots::raw::RawSlotQtCoreListListModelIndexRef as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotListListQtCoreRectFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref(self as *const ::slots::raw::RawSlotListListQtCoreRectFRef as *mut ::slots::raw::RawSlotListListQtCoreRectFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreListListUrlRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref(self as *const ::slots::raw::RawSlotQtCoreListListUrlRef as *mut ::slots::raw::RawSlotQtCoreListListUrlRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCorePointFRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPointF_ref(self as *const ::slots::raw::RawSlotQtCorePointFRef as *mut ::slots::raw::RawSlotQtCorePointFRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCorePointRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPoint_ref(self as *const ::slots::raw::RawSlotQtCorePointRef as *mut ::slots::raw::RawSlotQtCorePointRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreRectRefCInt {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QRect_ref_int(self as *const ::slots::raw::RawSlotQtCoreRectRefCInt as *mut ::slots::raw::RawSlotQtCoreRectRefCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotScrollerPropertiesRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref(self as *const ::slots::raw::RawSlotScrollerPropertiesRef as *mut ::slots::raw::RawSlotScrollerPropertiesRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreStringListRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QStringList_ref(self as *const ::slots::raw::RawSlotQtCoreStringListRef as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref(self as *const ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref(self as *const ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtGuiTextCharFormatRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref(self as *const ::slots::raw::RawSlotQtGuiTextCharFormatRef as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotQtCoreTimeRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTime_ref(self as *const ::slots::raw::RawSlotQtCoreTimeRef as *mut ::slots::raw::RawSlotQtCoreTimeRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_Qt_SortOrder(self as *const ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntBool {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_bool(self as *const ::slots::raw::RawSlotCIntBool as *mut ::slots::raw::RawSlotCIntBool) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::Deref for ::slots::raw::RawSlotCIntCIntCIntCInt {
    type Target = ::qt_core::object::Object;
    fn deref(&self) -> &::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_int_int_int(self as *const ::slots::raw::RawSlotCIntCIntCIntCInt as *mut ::slots::raw::RawSlotCIntCIntCIntCInt) };
      unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractButtonMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotAbstractButtonMutPtrBool {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAbstractButton_ptr_bool(self as *mut ::slots::raw::RawSlotAbstractButtonMutPtrBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotActionMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QAction_ptr(self as *mut ::slots::raw::RawSlotActionMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotDockWidgetMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QDockWidget_ptr(self as *mut ::slots::raw::RawSlotDockWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QFlags_QGraphicsBlurEffect_BlurHint(self as *mut ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGraphicsItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QGraphicsItem_ptr_QGraphicsItem_ptr_Qt_FocusReason(self as *mut ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotListWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QListWidgetItem_ptr_QListWidgetItem_ptr(self as *mut ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotMdiSubWindowMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QMdiSubWindow_ptr(self as *mut ::slots::raw::RawSlotMdiSubWindowMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect(self as *mut ::slots::raw::RawSlotQtCoreRectRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QRect_QPointF_QPointF(self as *mut ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotScrollerStateRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QScroller_State(self as *mut ::slots::raw::RawSlotScrollerStateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotSystemTrayIconActivationReasonRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QSystemTrayIcon_ActivationReason(self as *mut ::slots::raw::RawSlotSystemTrayIconActivationReasonRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTableWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTableWidgetItem_ptr_QTableWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTreeWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_QTreeWidgetItem_ptr(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QTreeWidgetItem_ptr_int(self as *mut ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotUndoStackMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QUndoStack_ptr(self as *mut ::slots::raw::RawSlotUndoStackMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotWidgetMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr(self as *mut ::slots::raw::RawSlotWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QAbstractItemDelegate_EndEditHint(self as *mut ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_QWidget_ptr_QWidget_ptr(self as *mut ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_DockWidgetArea(self as *mut ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_Qt_ToolButtonStyle(self as *mut ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiBrushRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QBrush_ref(self as *mut ::slots::raw::RawSlotQtGuiBrushRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiColorRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QColor_ref(self as *mut ::slots::raw::RawSlotQtGuiColorRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreDateTimeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDateTime_ref(self as *mut ::slots::raw::RawSlotQtCoreDateTimeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreDateRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QDate_ref(self as *mut ::slots::raw::RawSlotQtCoreDateRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiFontRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QFont_ref(self as *mut ::slots::raw::RawSlotQtGuiFontRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiIconRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QIcon_ref(self as *mut ::slots::raw::RawSlotQtGuiIconRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiKeySequenceRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QKeySequence_ref(self as *mut ::slots::raw::RawSlotQtGuiKeySequenceRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreListListModelIndexRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QModelIndex_ref(self as *mut ::slots::raw::RawSlotQtCoreListListModelIndexRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotListListQtCoreRectFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QRectF_ref(self as *mut ::slots::raw::RawSlotListListQtCoreRectFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreListListUrlRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QList_QUrl_ref(self as *mut ::slots::raw::RawSlotQtCoreListListUrlRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCorePointFRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPointF_ref(self as *mut ::slots::raw::RawSlotQtCorePointFRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCorePointRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QPoint_ref(self as *mut ::slots::raw::RawSlotQtCorePointRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreRectRefCInt {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QRect_ref_int(self as *mut ::slots::raw::RawSlotQtCoreRectRefCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotScrollerPropertiesRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QScrollerProperties_ref(self as *mut ::slots::raw::RawSlotScrollerPropertiesRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreStringListRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QStringList_ref(self as *mut ::slots::raw::RawSlotQtCoreStringListRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QString_ref_const_QString_ref_const_QString_ref(self as *mut ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtGuiTextCharFormatRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTextCharFormat_ref(self as *mut ::slots::raw::RawSlotQtGuiTextCharFormatRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotQtCoreTimeRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_const_QTime_ref(self as *mut ::slots::raw::RawSlotQtCoreTimeRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_Qt_SortOrder(self as *mut ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntBool {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_bool(self as *mut ::slots::raw::RawSlotCIntBool) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

  impl ::std::ops::DerefMut for ::slots::raw::RawSlotCIntCIntCIntCInt {
    fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
      let ffi_result = unsafe { ::ffi::qt_widgets_c_slots_G_static_cast_QObject_ptr_qt_widgets_c_SlotWrapper_int_int_int_int(self as *mut ::slots::raw::RawSlotCIntCIntCIntCInt) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }

}

/// Allows to bind Qt signals with arguments `(QAbstractButton*)` to a Rust closure.
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

pub struct SlotAbstractButtonMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractButtonMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_button::AbstractButton) + 'a>>>,
}

impl<'a> SlotAbstractButtonMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_button::AbstractButton) + 'a>(f: F) -> SlotAbstractButtonMutPtr<'a> {
    let mut obj = SlotAbstractButtonMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_button::AbstractButton) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_button::AbstractButton) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_button_mut_ptr_callback,
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

impl<'a> Default for SlotAbstractButtonMutPtr<'a> {
  fn default() -> Self {
    SlotAbstractButtonMutPtr {
      wrapper: ::slots::raw::RawSlotAbstractButtonMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractButtonMutPtr<'a> {
  type Arguments = (*mut ::abstract_button::AbstractButton,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractButtonMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_button_mut_ptr_callback(data: *mut ::libc::c_void,
                                                    arg0: *mut ::abstract_button::AbstractButton) {
  let func: &mut Box<FnMut(*mut ::abstract_button::AbstractButton)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QAbstractButton*, bool)` to a Rust closure.
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

pub struct SlotAbstractButtonMutPtrBool<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotAbstractButtonMutPtrBool>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::abstract_button::AbstractButton, bool) + 'a>>>,
}

impl<'a> SlotAbstractButtonMutPtrBool<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::abstract_button::AbstractButton, bool) + 'a>(f: F) -> SlotAbstractButtonMutPtrBool<'a> {
    let mut obj = SlotAbstractButtonMutPtrBool::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::abstract_button::AbstractButton, bool) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::abstract_button::AbstractButton, bool) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_abstract_button_mut_ptr_bool_callback,
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

impl<'a> Default for SlotAbstractButtonMutPtrBool<'a> {
  fn default() -> Self {
    SlotAbstractButtonMutPtrBool {
      wrapper: ::slots::raw::RawSlotAbstractButtonMutPtrBool::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotAbstractButtonMutPtrBool<'a> {
  type Arguments = (*mut ::abstract_button::AbstractButton, bool);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotAbstractButtonMutPtrBool as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_abstract_button_mut_ptr_bool_callback(data: *mut ::libc::c_void,
                                                         arg0: *mut ::abstract_button::AbstractButton,
                                                         arg1: bool) {
  let func: &mut Box<FnMut(*mut ::abstract_button::AbstractButton, bool)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QAction*)` to a Rust closure.
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

pub struct SlotActionMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotActionMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::action::Action) + 'a>>>,
}

impl<'a> SlotActionMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::action::Action) + 'a>(f: F) -> SlotActionMutPtr<'a> {
    let mut obj = SlotActionMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::action::Action) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::action::Action) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_action_mut_ptr_callback,
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

impl<'a> Default for SlotActionMutPtr<'a> {
  fn default() -> Self {
    SlotActionMutPtr {
      wrapper: ::slots::raw::RawSlotActionMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotActionMutPtr<'a> {
  type Arguments = (*mut ::action::Action,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotActionMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_action_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::action::Action) {
  let func: &mut Box<FnMut(*mut ::action::Action)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(int, bool)` to a Rust closure.
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

pub struct SlotCIntBool<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntBool>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, bool) + 'a>>>,
}

impl<'a> SlotCIntBool<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, bool) + 'a>(f: F) -> SlotCIntBool<'a> {
    let mut obj = SlotCIntBool::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, bool) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, bool) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_bool_callback,
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

impl<'a> Default for SlotCIntBool<'a> {
  fn default() -> Self {
    SlotCIntBool {
      wrapper: ::slots::raw::RawSlotCIntBool::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCIntBool<'a> {
  type Arguments = (::libc::c_int, bool);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntBool as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_bool_callback(data: *mut ::libc::c_void, arg0: ::libc::c_int, arg1: bool) {
  let func: &mut Box<FnMut(::libc::c_int, bool)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(int, int, int, int)` to a Rust closure.
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

pub struct SlotCIntCIntCIntCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntCIntCIntCInt>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotCIntCIntCIntCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>(f: F)
                                                                                        -> SlotCIntCIntCIntCInt<'a> {
    let mut obj = SlotCIntCIntCIntCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_c_int_c_int_c_int_callback,
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

impl<'a> Default for SlotCIntCIntCIntCInt<'a> {
  fn default() -> Self {
    SlotCIntCIntCIntCInt {
      wrapper: ::slots::raw::RawSlotCIntCIntCIntCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCIntCIntCIntCInt<'a> {
  type Arguments = (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntCIntCIntCInt as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_c_int_c_int_c_int_callback(data: *mut ::libc::c_void,
                                                    arg0: ::libc::c_int,
                                                    arg1: ::libc::c_int,
                                                    arg2: ::libc::c_int,
                                                    arg3: ::libc::c_int) {
  let func: &mut Box<FnMut(::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)> =
    unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1, arg2, arg3);
}
/// Allows to bind Qt signals with arguments `(int, Qt::SortOrder)` to a Rust closure.
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

pub struct SlotCIntQtCoreQtSortOrderRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotCIntQtCoreQtSortOrderRef>,
  func: ::std::option::Option<Box<Box<FnMut(::libc::c_int, &'static ::qt_core::qt::SortOrder) + 'a>>>,
}

impl<'a> SlotCIntQtCoreQtSortOrderRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::libc::c_int, &'static ::qt_core::qt::SortOrder) + 'a>(f: F)
                                                                              -> SlotCIntQtCoreQtSortOrderRef<'a> {
    let mut obj = SlotCIntQtCoreQtSortOrderRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::libc::c_int, &'static ::qt_core::qt::SortOrder) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::libc::c_int, &'static ::qt_core::qt::SortOrder) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_c_int_qt_core_qt_sort_order_ref_callback,
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

impl<'a> Default for SlotCIntQtCoreQtSortOrderRef<'a> {
  fn default() -> Self {
    SlotCIntQtCoreQtSortOrderRef {
      wrapper: ::slots::raw::RawSlotCIntQtCoreQtSortOrderRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotCIntQtCoreQtSortOrderRef<'a> {
  type Arguments = (::libc::c_int, &'static ::qt_core::qt::SortOrder);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotCIntQtCoreQtSortOrderRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_c_int_qt_core_qt_sort_order_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: ::libc::c_int,
                                                            arg1: *const ::qt_core::qt::SortOrder) {
  let func: &mut Box<FnMut(::libc::c_int, &'static ::qt_core::qt::SortOrder)> = unsafe { ::std::mem::transmute(data) };
  func(arg0,
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QDockWidget*)` to a Rust closure.
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

pub struct SlotDockWidgetMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotDockWidgetMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::dock_widget::DockWidget) + 'a>>>,
}

impl<'a> SlotDockWidgetMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::dock_widget::DockWidget) + 'a>(f: F) -> SlotDockWidgetMutPtr<'a> {
    let mut obj = SlotDockWidgetMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::dock_widget::DockWidget) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::dock_widget::DockWidget) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_dock_widget_mut_ptr_callback,
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

impl<'a> Default for SlotDockWidgetMutPtr<'a> {
  fn default() -> Self {
    SlotDockWidgetMutPtr {
      wrapper: ::slots::raw::RawSlotDockWidgetMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotDockWidgetMutPtr<'a> {
  type Arguments = (*mut ::dock_widget::DockWidget,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotDockWidgetMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_dock_widget_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::dock_widget::DockWidget) {
  let func: &mut Box<FnMut(*mut ::dock_widget::DockWidget)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QGraphicsItem*)` to a Rust closure.
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

pub struct SlotGraphicsItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::graphics_item::GraphicsItem) + 'a>>>,
}

impl<'a> SlotGraphicsItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::graphics_item::GraphicsItem) + 'a>(f: F) -> SlotGraphicsItemMutPtr<'a> {
    let mut obj = SlotGraphicsItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::graphics_item::GraphicsItem) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::graphics_item::GraphicsItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_graphics_item_mut_ptr_callback,
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

impl<'a> Default for SlotGraphicsItemMutPtr<'a> {
  fn default() -> Self {
    SlotGraphicsItemMutPtr {
      wrapper: ::slots::raw::RawSlotGraphicsItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGraphicsItemMutPtr<'a> {
  type Arguments = (*mut ::graphics_item::GraphicsItem,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGraphicsItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_graphics_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                  arg0: *mut ::graphics_item::GraphicsItem) {
  let func: &mut Box<FnMut(*mut ::graphics_item::GraphicsItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QGraphicsItem*, QGraphicsItem*)` to a Rust closure.
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

pub struct SlotGraphicsItemMutPtrGraphicsItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem) + 'a>>>,
}

impl<'a> SlotGraphicsItemMutPtrGraphicsItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::graphics_item::GraphicsItem,
                   *mut ::graphics_item::GraphicsItem) + 'a>
    (f: F)
     -> SlotGraphicsItemMutPtrGraphicsItemMutPtr<'a> {
    let mut obj = SlotGraphicsItemMutPtrGraphicsItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::graphics_item::GraphicsItem,
                   *mut ::graphics_item::GraphicsItem) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_graphics_item_mut_ptr_graphics_item_mut_ptr_callback,
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

impl<'a> Default for SlotGraphicsItemMutPtrGraphicsItemMutPtr<'a> {
  fn default() -> Self {
    SlotGraphicsItemMutPtrGraphicsItemMutPtr {
      wrapper: ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGraphicsItemMutPtrGraphicsItemMutPtr<'a> {
  type Arguments = (*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_graphics_item_mut_ptr_graphics_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                                        arg0: *mut ::graphics_item::GraphicsItem,
                                                                        arg1: *mut ::graphics_item::GraphicsItem) {
  let func: &mut Box<FnMut(*mut ::graphics_item::GraphicsItem,
                           *mut ::graphics_item::GraphicsItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QGraphicsItem*, QGraphicsItem*, Qt::FocusReason)` to a Rust closure.
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

pub struct SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem, &'static ::qt_core::qt::FocusReason) + 'a>>>,
}

impl<'a> SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::graphics_item::GraphicsItem,
                   *mut ::graphics_item::GraphicsItem,
                   &'static ::qt_core::qt::FocusReason) + 'a>
    (f: F)
     -> SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef<'a> {
    let mut obj = SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::graphics_item::GraphicsItem,
                   *mut ::graphics_item::GraphicsItem,
                   &'static ::qt_core::qt::FocusReason) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::graphics_item::GraphicsItem, *mut ::graphics_item::GraphicsItem, &'static ::qt_core::qt::FocusReason) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_graphics_item_mut_ptr_graphics_item_mut_ptr_qt_core_qt_focus_reason_ref_callback,
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

impl<'a> Default for SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef<'a> {
  fn default() -> Self {
    SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef {
      wrapper: ::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef<'a> {
  type Arguments = (*mut ::graphics_item::GraphicsItem,
   *mut ::graphics_item::GraphicsItem,
   &'static ::qt_core::qt::FocusReason);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotGraphicsItemMutPtrGraphicsItemMutPtrQtCoreQtFocusReasonRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_graphics_item_mut_ptr_graphics_item_mut_ptr_qt_core_qt_focus_reason_ref_callback(data: *mut ::libc::c_void, arg0: *mut ::graphics_item::GraphicsItem, arg1: *mut ::graphics_item::GraphicsItem, arg2: *const ::qt_core::qt::FocusReason) {
  let func: &mut Box<FnMut(*mut ::graphics_item::GraphicsItem,
                           *mut ::graphics_item::GraphicsItem,
                           &'static ::qt_core::qt::FocusReason)> = unsafe { ::std::mem::transmute(data) };
  func(arg0,
       arg1,
       unsafe { arg2.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QList<QRectF>&)` to a Rust closure.
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

pub struct SlotListListQtCoreRectFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotListListQtCoreRectFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::list::ListQtCoreRectF) + 'a>>>,
}

impl<'a> SlotListListQtCoreRectFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::list::ListQtCoreRectF) + 'a>(f: F) -> SlotListListQtCoreRectFRef<'a> {
    let mut obj = SlotListListQtCoreRectFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::list::ListQtCoreRectF) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::list::ListQtCoreRectF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_list_list_qt_core_rect_f_ref_callback,
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

impl<'a> Default for SlotListListQtCoreRectFRef<'a> {
  fn default() -> Self {
    SlotListListQtCoreRectFRef {
      wrapper: ::slots::raw::RawSlotListListQtCoreRectFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotListListQtCoreRectFRef<'a> {
  type Arguments = (&'static ::list::ListQtCoreRectF,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotListListQtCoreRectFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_list_list_qt_core_rect_f_ref_callback(data: *mut ::libc::c_void,
                                                         arg0: *const ::list::ListQtCoreRectF) {
  let func: &mut Box<FnMut(&'static ::list::ListQtCoreRectF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QListWidgetItem*)` to a Rust closure.
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

pub struct SlotListWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotListWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::list_widget_item::ListWidgetItem) + 'a>>>,
}

impl<'a> SlotListWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::list_widget_item::ListWidgetItem) + 'a>(f: F) -> SlotListWidgetItemMutPtr<'a> {
    let mut obj = SlotListWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::list_widget_item::ListWidgetItem) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::list_widget_item::ListWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_list_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotListWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotListWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotListWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotListWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::list_widget_item::ListWidgetItem,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotListWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_list_widget_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                     arg0: *mut ::list_widget_item::ListWidgetItem) {
  let func: &mut Box<FnMut(*mut ::list_widget_item::ListWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QListWidgetItem*, QListWidgetItem*)` to a Rust closure.
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

pub struct SlotListWidgetItemMutPtrListWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::list_widget_item::ListWidgetItem, *mut ::list_widget_item::ListWidgetItem) + 'a>>>,
}

impl<'a> SlotListWidgetItemMutPtrListWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::list_widget_item::ListWidgetItem,
                   *mut ::list_widget_item::ListWidgetItem) + 'a>
    (f: F)
     -> SlotListWidgetItemMutPtrListWidgetItemMutPtr<'a> {
    let mut obj = SlotListWidgetItemMutPtrListWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::list_widget_item::ListWidgetItem,
                   *mut ::list_widget_item::ListWidgetItem) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::list_widget_item::ListWidgetItem, *mut ::list_widget_item::ListWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_list_widget_item_mut_ptr_list_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotListWidgetItemMutPtrListWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotListWidgetItemMutPtrListWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotListWidgetItemMutPtrListWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::list_widget_item::ListWidgetItem, *mut ::list_widget_item::ListWidgetItem);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotListWidgetItemMutPtrListWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_list_widget_item_mut_ptr_list_widget_item_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::list_widget_item::ListWidgetItem, arg1: *mut ::list_widget_item::ListWidgetItem) {
  let func: &mut Box<FnMut(*mut ::list_widget_item::ListWidgetItem,
                           *mut ::list_widget_item::ListWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QMdiSubWindow*)` to a Rust closure.
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

pub struct SlotMdiSubWindowMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotMdiSubWindowMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::mdi_sub_window::MdiSubWindow) + 'a>>>,
}

impl<'a> SlotMdiSubWindowMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::mdi_sub_window::MdiSubWindow) + 'a>(f: F) -> SlotMdiSubWindowMutPtr<'a> {
    let mut obj = SlotMdiSubWindowMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::mdi_sub_window::MdiSubWindow) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::mdi_sub_window::MdiSubWindow) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_mdi_sub_window_mut_ptr_callback,
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

impl<'a> Default for SlotMdiSubWindowMutPtr<'a> {
  fn default() -> Self {
    SlotMdiSubWindowMutPtr {
      wrapper: ::slots::raw::RawSlotMdiSubWindowMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotMdiSubWindowMutPtr<'a> {
  type Arguments = (*mut ::mdi_sub_window::MdiSubWindow,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotMdiSubWindowMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_mdi_sub_window_mut_ptr_callback(data: *mut ::libc::c_void,
                                                   arg0: *mut ::mdi_sub_window::MdiSubWindow) {
  let func: &mut Box<FnMut(*mut ::mdi_sub_window::MdiSubWindow)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(const QDate&)` to a Rust closure.
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

pub struct SlotQtCoreDateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreDateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::date::Date) + 'a>>>,
}

impl<'a> SlotQtCoreDateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::date::Date) + 'a>(f: F) -> SlotQtCoreDateRef<'a> {
    let mut obj = SlotQtCoreDateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::date::Date) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::date::Date) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_date_ref_callback,
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

impl<'a> Default for SlotQtCoreDateRef<'a> {
  fn default() -> Self {
    SlotQtCoreDateRef {
      wrapper: ::slots::raw::RawSlotQtCoreDateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreDateRef<'a> {
  type Arguments = (&'static ::qt_core::date::Date,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreDateRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_date_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::date::Date) {
  let func: &mut Box<FnMut(&'static ::qt_core::date::Date)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QDateTime&)` to a Rust closure.
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

pub struct SlotQtCoreDateTimeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreDateTimeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::date_time::DateTime) + 'a>>>,
}

impl<'a> SlotQtCoreDateTimeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::date_time::DateTime) + 'a>(f: F) -> SlotQtCoreDateTimeRef<'a> {
    let mut obj = SlotQtCoreDateTimeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::date_time::DateTime) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::date_time::DateTime) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_date_time_ref_callback,
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

impl<'a> Default for SlotQtCoreDateTimeRef<'a> {
  fn default() -> Self {
    SlotQtCoreDateTimeRef {
      wrapper: ::slots::raw::RawSlotQtCoreDateTimeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreDateTimeRef<'a> {
  type Arguments = (&'static ::qt_core::date_time::DateTime,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreDateTimeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_date_time_ref_callback(data: *mut ::libc::c_void,
                                                  arg0: *const ::qt_core::date_time::DateTime) {
  let func: &mut Box<FnMut(&'static ::qt_core::date_time::DateTime)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QFlags<QGraphicsBlurEffect::BlurHint>)` to a Rust closure.
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

pub struct SlotQtCoreFlagsGraphicsBlurEffectBlurHint<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint>,
  func: ::std::option::Option<Box<Box<FnMut(::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) + 'a>>>,
}

impl<'a> SlotQtCoreFlagsGraphicsBlurEffectBlurHint<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) + 'a>
    (f: F)
     -> SlotQtCoreFlagsGraphicsBlurEffectBlurHint<'a> {
    let mut obj = SlotQtCoreFlagsGraphicsBlurEffectBlurHint::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_flags_graphics_blur_effect_blur_hint_callback,
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

impl<'a> Default for SlotQtCoreFlagsGraphicsBlurEffectBlurHint<'a> {
  fn default() -> Self {
    SlotQtCoreFlagsGraphicsBlurEffectBlurHint {
      wrapper: ::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreFlagsGraphicsBlurEffectBlurHint<'a> {
  type Arguments = (::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreFlagsGraphicsBlurEffectBlurHint as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_flags_graphics_blur_effect_blur_hint_callback(data: *mut ::libc::c_void,
                                                                         arg0: ::libc::c_uint) {
  let func: &mut Box<FnMut(::qt_core::flags::Flags<::graphics_blur_effect::BlurHint>)> =
    unsafe { ::std::mem::transmute(data) };
  func(::qt_core::flags::Flags::from_int(arg0 as i32));
}
/// Allows to bind Qt signals with arguments `(const QList<QModelIndex>&)` to a Rust closure.
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

pub struct SlotQtCoreListListModelIndexRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreListListModelIndexRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::list::ListModelIndex) + 'a>>>,
}

impl<'a> SlotQtCoreListListModelIndexRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::list::ListModelIndex) + 'a>(f: F) -> SlotQtCoreListListModelIndexRef<'a> {
    let mut obj = SlotQtCoreListListModelIndexRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::list::ListModelIndex) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::list::ListModelIndex) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_list_list_model_index_ref_callback,
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

impl<'a> Default for SlotQtCoreListListModelIndexRef<'a> {
  fn default() -> Self {
    SlotQtCoreListListModelIndexRef {
      wrapper: ::slots::raw::RawSlotQtCoreListListModelIndexRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreListListModelIndexRef<'a> {
  type Arguments = (&'static ::qt_core::list::ListModelIndex,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreListListModelIndexRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_list_list_model_index_ref_callback(data: *mut ::libc::c_void,
                                                              arg0: *const ::qt_core::list::ListModelIndex) {
  let func: &mut Box<FnMut(&'static ::qt_core::list::ListModelIndex)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QList<QUrl>&)` to a Rust closure.
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

pub struct SlotQtCoreListListUrlRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreListListUrlRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::list::ListUrl) + 'a>>>,
}

impl<'a> SlotQtCoreListListUrlRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::list::ListUrl) + 'a>(f: F) -> SlotQtCoreListListUrlRef<'a> {
    let mut obj = SlotQtCoreListListUrlRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::list::ListUrl) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::list::ListUrl) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_list_list_url_ref_callback,
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

impl<'a> Default for SlotQtCoreListListUrlRef<'a> {
  fn default() -> Self {
    SlotQtCoreListListUrlRef {
      wrapper: ::slots::raw::RawSlotQtCoreListListUrlRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreListListUrlRef<'a> {
  type Arguments = (&'static ::qt_core::list::ListUrl,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreListListUrlRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_list_list_url_ref_callback(data: *mut ::libc::c_void,
                                                      arg0: *const ::qt_core::list::ListUrl) {
  let func: &mut Box<FnMut(&'static ::qt_core::list::ListUrl)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QPointF&)` to a Rust closure.
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

pub struct SlotQtCorePointFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCorePointFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::point_f::PointF) + 'a>>>,
}

impl<'a> SlotQtCorePointFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::point_f::PointF) + 'a>(f: F) -> SlotQtCorePointFRef<'a> {
    let mut obj = SlotQtCorePointFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::point_f::PointF) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::point_f::PointF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_point_f_ref_callback,
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

impl<'a> Default for SlotQtCorePointFRef<'a> {
  fn default() -> Self {
    SlotQtCorePointFRef {
      wrapper: ::slots::raw::RawSlotQtCorePointFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCorePointFRef<'a> {
  type Arguments = (&'static ::qt_core::point_f::PointF,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCorePointFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_point_f_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::point_f::PointF) {
  let func: &mut Box<FnMut(&'static ::qt_core::point_f::PointF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QPoint&)` to a Rust closure.
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

pub struct SlotQtCorePointRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCorePointRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::point::Point) + 'a>>>,
}

impl<'a> SlotQtCorePointRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::point::Point) + 'a>(f: F) -> SlotQtCorePointRef<'a> {
    let mut obj = SlotQtCorePointRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::point::Point) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::point::Point) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_point_ref_callback,
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

impl<'a> Default for SlotQtCorePointRef<'a> {
  fn default() -> Self {
    SlotQtCorePointRef {
      wrapper: ::slots::raw::RawSlotQtCorePointRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCorePointRef<'a> {
  type Arguments = (&'static ::qt_core::point::Point,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCorePointRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_point_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::point::Point) {
  let func: &mut Box<FnMut(&'static ::qt_core::point::Point)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::DockWidgetArea)` to a Rust closure.
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

pub struct SlotQtCoreQtDockWidgetAreaRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::DockWidgetArea) + 'a>>>,
}

impl<'a> SlotQtCoreQtDockWidgetAreaRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::DockWidgetArea) + 'a>(f: F) -> SlotQtCoreQtDockWidgetAreaRef<'a> {
    let mut obj = SlotQtCoreQtDockWidgetAreaRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::DockWidgetArea) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::DockWidgetArea) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_dock_widget_area_ref_callback,
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

impl<'a> Default for SlotQtCoreQtDockWidgetAreaRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtDockWidgetAreaRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtDockWidgetAreaRef<'a> {
  type Arguments = (&'static ::qt_core::qt::DockWidgetArea,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtDockWidgetAreaRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_dock_widget_area_ref_callback(data: *mut ::libc::c_void,
                                                            arg0: *const ::qt_core::qt::DockWidgetArea) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::DockWidgetArea)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(Qt::ToolButtonStyle)` to a Rust closure.
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

pub struct SlotQtCoreQtToolButtonStyleRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreQtToolButtonStyleRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::qt::ToolButtonStyle) + 'a>>>,
}

impl<'a> SlotQtCoreQtToolButtonStyleRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::qt::ToolButtonStyle) + 'a>(f: F) -> SlotQtCoreQtToolButtonStyleRef<'a> {
    let mut obj = SlotQtCoreQtToolButtonStyleRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::qt::ToolButtonStyle) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::qt::ToolButtonStyle) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_qt_tool_button_style_ref_callback,
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

impl<'a> Default for SlotQtCoreQtToolButtonStyleRef<'a> {
  fn default() -> Self {
    SlotQtCoreQtToolButtonStyleRef {
      wrapper: ::slots::raw::RawSlotQtCoreQtToolButtonStyleRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreQtToolButtonStyleRef<'a> {
  type Arguments = (&'static ::qt_core::qt::ToolButtonStyle,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreQtToolButtonStyleRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_qt_tool_button_style_ref_callback(data: *mut ::libc::c_void,
                                                             arg0: *const ::qt_core::qt::ToolButtonStyle) {
  let func: &mut Box<FnMut(&'static ::qt_core::qt::ToolButtonStyle)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QRect)` to a Rust closure.
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
/// Allows to bind Qt signals with arguments `(const QRect&, int)` to a Rust closure.
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

pub struct SlotQtCoreRectRefCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefCInt>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::rect::Rect, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotQtCoreRectRefCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::rect::Rect, ::libc::c_int) + 'a>(f: F) -> SlotQtCoreRectRefCInt<'a> {
    let mut obj = SlotQtCoreRectRefCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::rect::Rect, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::rect::Rect, ::libc::c_int) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_rect_ref_c_int_callback,
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

impl<'a> Default for SlotQtCoreRectRefCInt<'a> {
  fn default() -> Self {
    SlotQtCoreRectRefCInt {
      wrapper: ::slots::raw::RawSlotQtCoreRectRefCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRectRefCInt<'a> {
  type Arguments = (&'static ::qt_core::rect::Rect, ::libc::c_int);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRectRefCInt as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_rect_ref_c_int_callback(data: *mut ::libc::c_void,
                                                   arg0: *const ::qt_core::rect::Rect,
                                                   arg1: ::libc::c_int) {
  let func: &mut Box<FnMut(&'static ::qt_core::rect::Rect, ::libc::c_int)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       arg1);
}
/// Allows to bind Qt signals with arguments `(QRect, QPointF)` to a Rust closure.
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

pub struct SlotQtCoreRectRefQtCorePointFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF) + 'a>>>,
}

impl<'a> SlotQtCoreRectRefQtCorePointFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::rect::Rect,
                   &'static ::qt_core::point_f::PointF) + 'a>
    (f: F)
     -> SlotQtCoreRectRefQtCorePointFRef<'a> {
    let mut obj = SlotQtCoreRectRefQtCorePointFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::rect::Rect,
                   &'static ::qt_core::point_f::PointF) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_rect_ref_qt_core_point_f_ref_callback,
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

impl<'a> Default for SlotQtCoreRectRefQtCorePointFRef<'a> {
  fn default() -> Self {
    SlotQtCoreRectRefQtCorePointFRef {
      wrapper: ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRectRefQtCorePointFRef<'a> {
  type Arguments = (&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRectRefQtCorePointFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_rect_ref_qt_core_point_f_ref_callback(data: *mut ::libc::c_void,
                                                                 arg0: *const ::qt_core::rect::Rect,
                                                                 arg1: *const ::qt_core::point_f::PointF) {
  let func: &mut Box<FnMut(&'static ::qt_core::rect::Rect,
                           &'static ::qt_core::point_f::PointF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QRect, QPointF, QPointF)` to a Rust closure.
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

pub struct SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF, &'static ::qt_core::point_f::PointF) + 'a>>>,
}

impl<'a> SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::rect::Rect,
                   &'static ::qt_core::point_f::PointF,
                   &'static ::qt_core::point_f::PointF) + 'a>
    (f: F)
     -> SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef<'a> {
    let mut obj = SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::rect::Rect,
                   &'static ::qt_core::point_f::PointF,
                   &'static ::qt_core::point_f::PointF) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::rect::Rect, &'static ::qt_core::point_f::PointF, &'static ::qt_core::point_f::PointF) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_rect_ref_qt_core_point_f_ref_qt_core_point_f_ref_callback,
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

impl<'a> Default for SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef<'a> {
  fn default() -> Self {
    SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef {
      wrapper: ::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreRectRefQtCorePointFRefQtCorePointFRef<'a> {
  type Arguments = (&'static ::qt_core::rect::Rect,
   &'static ::qt_core::point_f::PointF,
   &'static ::qt_core::point_f::PointF);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreRectRefQtCorePointFRefQtCorePointFRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_rect_ref_qt_core_point_f_ref_qt_core_point_f_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::rect::Rect, arg1: *const ::qt_core::point_f::PointF, arg2: *const ::qt_core::point_f::PointF) {
  let func: &mut Box<FnMut(&'static ::qt_core::rect::Rect,
                           &'static ::qt_core::point_f::PointF,
                           &'static ::qt_core::point_f::PointF)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg2.as_ref() }.expect("Attempted to convert null pointer to reference"));
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
/// Allows to bind Qt signals with arguments `(const QString&, const QString&)` to a Rust closure.
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

pub struct SlotQtCoreStringRefQtCoreStringRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::string::String, &'static ::qt_core::string::String) + 'a>>>,
}

impl<'a> SlotQtCoreStringRefQtCoreStringRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::string::String,
                   &'static ::qt_core::string::String) + 'a>
    (f: F)
     -> SlotQtCoreStringRefQtCoreStringRef<'a> {
    let mut obj = SlotQtCoreStringRefQtCoreStringRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::string::String,
                   &'static ::qt_core::string::String) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::string::String, &'static ::qt_core::string::String) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_string_ref_qt_core_string_ref_callback,
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

impl<'a> Default for SlotQtCoreStringRefQtCoreStringRef<'a> {
  fn default() -> Self {
    SlotQtCoreStringRefQtCoreStringRef {
      wrapper: ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreStringRefQtCoreStringRef<'a> {
  type Arguments = (&'static ::qt_core::string::String, &'static ::qt_core::string::String);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreStringRefQtCoreStringRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_string_ref_qt_core_string_ref_callback(data: *mut ::libc::c_void,
                                                                  arg0: *const ::qt_core::string::String,
                                                                  arg1: *const ::qt_core::string::String) {
  let func: &mut Box<FnMut(&'static ::qt_core::string::String,
                           &'static ::qt_core::string::String)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QString&, const QString&, const QString&)` to a Rust closure.
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

pub struct SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::string::String, &'static ::qt_core::string::String, &'static ::qt_core::string::String) + 'a>>>,
}

impl<'a> SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::string::String,
                   &'static ::qt_core::string::String,
                   &'static ::qt_core::string::String) + 'a>
    (f: F)
     -> SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef<'a> {
    let mut obj = SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::string::String,
                   &'static ::qt_core::string::String,
                   &'static ::qt_core::string::String) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::string::String, &'static ::qt_core::string::String, &'static ::qt_core::string::String) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_string_ref_qt_core_string_ref_qt_core_string_ref_callback,
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

impl<'a> Default for SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef<'a> {
  fn default() -> Self {
    SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef {
      wrapper: ::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreStringRefQtCoreStringRefQtCoreStringRef<'a> {
  type Arguments = (&'static ::qt_core::string::String,
   &'static ::qt_core::string::String,
   &'static ::qt_core::string::String);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreStringRefQtCoreStringRefQtCoreStringRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_string_ref_qt_core_string_ref_qt_core_string_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::string::String, arg1: *const ::qt_core::string::String, arg2: *const ::qt_core::string::String) {
  let func: &mut Box<FnMut(&'static ::qt_core::string::String,
                           &'static ::qt_core::string::String,
                           &'static ::qt_core::string::String)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"),
       unsafe { arg2.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QTime&)` to a Rust closure.
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

pub struct SlotQtCoreTimeRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtCoreTimeRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_core::time::Time) + 'a>>>,
}

impl<'a> SlotQtCoreTimeRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_core::time::Time) + 'a>(f: F) -> SlotQtCoreTimeRef<'a> {
    let mut obj = SlotQtCoreTimeRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_core::time::Time) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_core::time::Time) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_core_time_ref_callback,
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

impl<'a> Default for SlotQtCoreTimeRef<'a> {
  fn default() -> Self {
    SlotQtCoreTimeRef {
      wrapper: ::slots::raw::RawSlotQtCoreTimeRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtCoreTimeRef<'a> {
  type Arguments = (&'static ::qt_core::time::Time,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtCoreTimeRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_core_time_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_core::time::Time) {
  let func: &mut Box<FnMut(&'static ::qt_core::time::Time)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QBrush&)` to a Rust closure.
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

pub struct SlotQtGuiBrushRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiBrushRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::brush::Brush) + 'a>>>,
}

impl<'a> SlotQtGuiBrushRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::brush::Brush) + 'a>(f: F) -> SlotQtGuiBrushRef<'a> {
    let mut obj = SlotQtGuiBrushRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::brush::Brush) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::brush::Brush) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_brush_ref_callback,
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

impl<'a> Default for SlotQtGuiBrushRef<'a> {
  fn default() -> Self {
    SlotQtGuiBrushRef {
      wrapper: ::slots::raw::RawSlotQtGuiBrushRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiBrushRef<'a> {
  type Arguments = (&'static ::qt_gui::brush::Brush,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiBrushRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_brush_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_gui::brush::Brush) {
  let func: &mut Box<FnMut(&'static ::qt_gui::brush::Brush)> = unsafe { ::std::mem::transmute(data) };
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
/// Allows to bind Qt signals with arguments `(const QIcon&)` to a Rust closure.
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

pub struct SlotQtGuiIconRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiIconRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::icon::Icon) + 'a>>>,
}

impl<'a> SlotQtGuiIconRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::icon::Icon) + 'a>(f: F) -> SlotQtGuiIconRef<'a> {
    let mut obj = SlotQtGuiIconRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::icon::Icon) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::icon::Icon) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_icon_ref_callback,
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

impl<'a> Default for SlotQtGuiIconRef<'a> {
  fn default() -> Self {
    SlotQtGuiIconRef {
      wrapper: ::slots::raw::RawSlotQtGuiIconRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiIconRef<'a> {
  type Arguments = (&'static ::qt_gui::icon::Icon,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiIconRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_icon_ref_callback(data: *mut ::libc::c_void, arg0: *const ::qt_gui::icon::Icon) {
  let func: &mut Box<FnMut(&'static ::qt_gui::icon::Icon)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QKeySequence&)` to a Rust closure.
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

pub struct SlotQtGuiKeySequenceRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiKeySequenceRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::key_sequence::KeySequence) + 'a>>>,
}

impl<'a> SlotQtGuiKeySequenceRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::key_sequence::KeySequence) + 'a>(f: F) -> SlotQtGuiKeySequenceRef<'a> {
    let mut obj = SlotQtGuiKeySequenceRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::key_sequence::KeySequence) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::key_sequence::KeySequence) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_key_sequence_ref_callback,
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

impl<'a> Default for SlotQtGuiKeySequenceRef<'a> {
  fn default() -> Self {
    SlotQtGuiKeySequenceRef {
      wrapper: ::slots::raw::RawSlotQtGuiKeySequenceRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiKeySequenceRef<'a> {
  type Arguments = (&'static ::qt_gui::key_sequence::KeySequence,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiKeySequenceRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_key_sequence_ref_callback(data: *mut ::libc::c_void,
                                                    arg0: *const ::qt_gui::key_sequence::KeySequence) {
  let func: &mut Box<FnMut(&'static ::qt_gui::key_sequence::KeySequence)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QTextCharFormat&)` to a Rust closure.
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

pub struct SlotQtGuiTextCharFormatRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotQtGuiTextCharFormatRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::qt_gui::text_char_format::TextCharFormat) + 'a>>>,
}

impl<'a> SlotQtGuiTextCharFormatRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::qt_gui::text_char_format::TextCharFormat) + 'a>(f: F)
                                                                                 -> SlotQtGuiTextCharFormatRef<'a> {
    let mut obj = SlotQtGuiTextCharFormatRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::qt_gui::text_char_format::TextCharFormat) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::qt_gui::text_char_format::TextCharFormat) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_qt_gui_text_char_format_ref_callback,
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

impl<'a> Default for SlotQtGuiTextCharFormatRef<'a> {
  fn default() -> Self {
    SlotQtGuiTextCharFormatRef {
      wrapper: ::slots::raw::RawSlotQtGuiTextCharFormatRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotQtGuiTextCharFormatRef<'a> {
  type Arguments = (&'static ::qt_gui::text_char_format::TextCharFormat,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotQtGuiTextCharFormatRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_qt_gui_text_char_format_ref_callback(data: *mut ::libc::c_void,
                                                        arg0: *const ::qt_gui::text_char_format::TextCharFormat) {
  let func: &mut Box<FnMut(&'static ::qt_gui::text_char_format::TextCharFormat)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(const QScrollerProperties&)` to a Rust closure.
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

pub struct SlotScrollerPropertiesRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotScrollerPropertiesRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::scroller_properties::ScrollerProperties) + 'a>>>,
}

impl<'a> SlotScrollerPropertiesRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::scroller_properties::ScrollerProperties) + 'a>(f: F) -> SlotScrollerPropertiesRef<'a> {
    let mut obj = SlotScrollerPropertiesRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::scroller_properties::ScrollerProperties) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::scroller_properties::ScrollerProperties) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_scroller_properties_ref_callback,
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

impl<'a> Default for SlotScrollerPropertiesRef<'a> {
  fn default() -> Self {
    SlotScrollerPropertiesRef {
      wrapper: ::slots::raw::RawSlotScrollerPropertiesRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotScrollerPropertiesRef<'a> {
  type Arguments = (&'static ::scroller_properties::ScrollerProperties,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotScrollerPropertiesRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_scroller_properties_ref_callback(data: *mut ::libc::c_void,
                                                    arg0: *const ::scroller_properties::ScrollerProperties) {
  let func: &mut Box<FnMut(&'static ::scroller_properties::ScrollerProperties)> =
    unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QScroller::State)` to a Rust closure.
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

pub struct SlotScrollerStateRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotScrollerStateRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::scroller::State) + 'a>>>,
}

impl<'a> SlotScrollerStateRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::scroller::State) + 'a>(f: F) -> SlotScrollerStateRef<'a> {
    let mut obj = SlotScrollerStateRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::scroller::State) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::scroller::State) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_scroller_state_ref_callback,
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

impl<'a> Default for SlotScrollerStateRef<'a> {
  fn default() -> Self {
    SlotScrollerStateRef {
      wrapper: ::slots::raw::RawSlotScrollerStateRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotScrollerStateRef<'a> {
  type Arguments = (&'static ::scroller::State,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotScrollerStateRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_scroller_state_ref_callback(data: *mut ::libc::c_void, arg0: *const ::scroller::State) {
  let func: &mut Box<FnMut(&'static ::scroller::State)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QSystemTrayIcon::ActivationReason)` to a Rust closure.
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

pub struct SlotSystemTrayIconActivationReasonRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotSystemTrayIconActivationReasonRef>,
  func: ::std::option::Option<Box<Box<FnMut(&'static ::system_tray_icon::ActivationReason) + 'a>>>,
}

impl<'a> SlotSystemTrayIconActivationReasonRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(&'static ::system_tray_icon::ActivationReason) + 'a>
    (f: F)
     -> SlotSystemTrayIconActivationReasonRef<'a> {
    let mut obj = SlotSystemTrayIconActivationReasonRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(&'static ::system_tray_icon::ActivationReason) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(&'static ::system_tray_icon::ActivationReason) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_system_tray_icon_activation_reason_ref_callback,
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

impl<'a> Default for SlotSystemTrayIconActivationReasonRef<'a> {
  fn default() -> Self {
    SlotSystemTrayIconActivationReasonRef {
      wrapper: ::slots::raw::RawSlotSystemTrayIconActivationReasonRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotSystemTrayIconActivationReasonRef<'a> {
  type Arguments = (&'static ::system_tray_icon::ActivationReason,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotSystemTrayIconActivationReasonRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_system_tray_icon_activation_reason_ref_callback(data: *mut ::libc::c_void, arg0: *const ::system_tray_icon::ActivationReason) {
  let func: &mut Box<FnMut(&'static ::system_tray_icon::ActivationReason)> = unsafe { ::std::mem::transmute(data) };
  func(unsafe { arg0.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QTableWidgetItem*)` to a Rust closure.
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

pub struct SlotTableWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTableWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::table_widget_item::TableWidgetItem) + 'a>>>,
}

impl<'a> SlotTableWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::table_widget_item::TableWidgetItem) + 'a>(f: F) -> SlotTableWidgetItemMutPtr<'a> {
    let mut obj = SlotTableWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::table_widget_item::TableWidgetItem) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::table_widget_item::TableWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_table_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotTableWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotTableWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotTableWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTableWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::table_widget_item::TableWidgetItem,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTableWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_table_widget_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                      arg0: *mut ::table_widget_item::TableWidgetItem) {
  let func: &mut Box<FnMut(*mut ::table_widget_item::TableWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QTableWidgetItem*, QTableWidgetItem*)` to a Rust closure.
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

pub struct SlotTableWidgetItemMutPtrTableWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::table_widget_item::TableWidgetItem, *mut ::table_widget_item::TableWidgetItem) + 'a>>>,
}

impl<'a> SlotTableWidgetItemMutPtrTableWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::table_widget_item::TableWidgetItem,
                   *mut ::table_widget_item::TableWidgetItem) + 'a>
    (f: F)
     -> SlotTableWidgetItemMutPtrTableWidgetItemMutPtr<'a> {
    let mut obj = SlotTableWidgetItemMutPtrTableWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::table_widget_item::TableWidgetItem,
                   *mut ::table_widget_item::TableWidgetItem) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::table_widget_item::TableWidgetItem, *mut ::table_widget_item::TableWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_table_widget_item_mut_ptr_table_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotTableWidgetItemMutPtrTableWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotTableWidgetItemMutPtrTableWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTableWidgetItemMutPtrTableWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::table_widget_item::TableWidgetItem, *mut ::table_widget_item::TableWidgetItem);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTableWidgetItemMutPtrTableWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_table_widget_item_mut_ptr_table_widget_item_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::table_widget_item::TableWidgetItem, arg1: *mut ::table_widget_item::TableWidgetItem) {
  let func: &mut Box<FnMut(*mut ::table_widget_item::TableWidgetItem,
                           *mut ::table_widget_item::TableWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QTreeWidgetItem*)` to a Rust closure.
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

pub struct SlotTreeWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem) + 'a>>>,
}

impl<'a> SlotTreeWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem) + 'a>(f: F) -> SlotTreeWidgetItemMutPtr<'a> {
    let mut obj = SlotTreeWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_tree_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotTreeWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotTreeWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotTreeWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTreeWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::tree_widget_item::TreeWidgetItem,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTreeWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_tree_widget_item_mut_ptr_callback(data: *mut ::libc::c_void,
                                                     arg0: *mut ::tree_widget_item::TreeWidgetItem) {
  let func: &mut Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QTreeWidgetItem*, int)` to a Rust closure.
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

pub struct SlotTreeWidgetItemMutPtrCInt<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtrCInt>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) + 'a>>>,
}

impl<'a> SlotTreeWidgetItemMutPtrCInt<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) + 'a>
    (f: F)
     -> SlotTreeWidgetItemMutPtrCInt<'a> {
    let mut obj = SlotTreeWidgetItemMutPtrCInt::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) + 'a>> =
      Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_tree_widget_item_mut_ptr_c_int_callback,
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

impl<'a> Default for SlotTreeWidgetItemMutPtrCInt<'a> {
  fn default() -> Self {
    SlotTreeWidgetItemMutPtrCInt {
      wrapper: ::slots::raw::RawSlotTreeWidgetItemMutPtrCInt::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTreeWidgetItemMutPtrCInt<'a> {
  type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTreeWidgetItemMutPtrCInt as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_tree_widget_item_mut_ptr_c_int_callback(data: *mut ::libc::c_void,
                                                           arg0: *mut ::tree_widget_item::TreeWidgetItem,
                                                           arg1: ::libc::c_int) {
  let func: &mut Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)> =
    unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QTreeWidgetItem*, QTreeWidgetItem*)` to a Rust closure.
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

pub struct SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem) + 'a>>>,
}

impl<'a> SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem,
                   *mut ::tree_widget_item::TreeWidgetItem) + 'a>
    (f: F)
     -> SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr<'a> {
    let mut obj = SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::tree_widget_item::TreeWidgetItem,
                   *mut ::tree_widget_item::TreeWidgetItem) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_tree_widget_item_mut_ptr_tree_widget_item_mut_ptr_callback,
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

impl<'a> Default for SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr<'a> {
  fn default() -> Self {
    SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr {
      wrapper: ::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr<'a> {
  type Arguments = (*mut ::tree_widget_item::TreeWidgetItem, *mut ::tree_widget_item::TreeWidgetItem);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotTreeWidgetItemMutPtrTreeWidgetItemMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_tree_widget_item_mut_ptr_tree_widget_item_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::tree_widget_item::TreeWidgetItem, arg1: *mut ::tree_widget_item::TreeWidgetItem) {
  let func: &mut Box<FnMut(*mut ::tree_widget_item::TreeWidgetItem,
                           *mut ::tree_widget_item::TreeWidgetItem)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
/// Allows to bind Qt signals with arguments `(QUndoStack*)` to a Rust closure.
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

pub struct SlotUndoStackMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotUndoStackMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::undo_stack::UndoStack) + 'a>>>,
}

impl<'a> SlotUndoStackMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::undo_stack::UndoStack) + 'a>(f: F) -> SlotUndoStackMutPtr<'a> {
    let mut obj = SlotUndoStackMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::undo_stack::UndoStack) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::undo_stack::UndoStack) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_undo_stack_mut_ptr_callback,
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

impl<'a> Default for SlotUndoStackMutPtr<'a> {
  fn default() -> Self {
    SlotUndoStackMutPtr {
      wrapper: ::slots::raw::RawSlotUndoStackMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotUndoStackMutPtr<'a> {
  type Arguments = (*mut ::undo_stack::UndoStack,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotUndoStackMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_undo_stack_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::undo_stack::UndoStack) {
  let func: &mut Box<FnMut(*mut ::undo_stack::UndoStack)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QWidget*)` to a Rust closure.
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

pub struct SlotWidgetMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::widget::Widget) + 'a>>>,
}

impl<'a> SlotWidgetMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::widget::Widget) + 'a>(f: F) -> SlotWidgetMutPtr<'a> {
    let mut obj = SlotWidgetMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::widget::Widget) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::widget::Widget) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_widget_mut_ptr_callback,
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

impl<'a> Default for SlotWidgetMutPtr<'a> {
  fn default() -> Self {
    SlotWidgetMutPtr {
      wrapper: ::slots::raw::RawSlotWidgetMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotWidgetMutPtr<'a> {
  type Arguments = (*mut ::widget::Widget,);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotWidgetMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_widget_mut_ptr_callback(data: *mut ::libc::c_void, arg0: *mut ::widget::Widget) {
  let func: &mut Box<FnMut(*mut ::widget::Widget)> = unsafe { ::std::mem::transmute(data) };
  func(arg0);
}
/// Allows to bind Qt signals with arguments `(QWidget*, QAbstractItemDelegate::EndEditHint)` to a Rust closure.
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

pub struct SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint) + 'a>>>,
}

impl<'a> SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::widget::Widget,
                   &'static ::abstract_item_delegate::EndEditHint) + 'a>
    (f: F)
     -> SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef<'a> {
    let mut obj = SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::widget::Widget,
                   &'static ::abstract_item_delegate::EndEditHint) + 'a>
    (&mut self,
     f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_widget_mut_ptr_abstract_item_delegate_end_edit_hint_ref_callback,
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

impl<'a> Default for SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef<'a> {
  fn default() -> Self {
    SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef {
      wrapper: ::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotWidgetMutPtrAbstractItemDelegateEndEditHintRef<'a> {
  type Arguments = (*mut ::widget::Widget, &'static ::abstract_item_delegate::EndEditHint);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotWidgetMutPtrAbstractItemDelegateEndEditHintRef as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_widget_mut_ptr_abstract_item_delegate_end_edit_hint_ref_callback(data: *mut ::libc::c_void, arg0: *mut ::widget::Widget, arg1: *const ::abstract_item_delegate::EndEditHint) {
  let func: &mut Box<FnMut(*mut ::widget::Widget,
                           &'static ::abstract_item_delegate::EndEditHint)> = unsafe { ::std::mem::transmute(data) };
  func(arg0,
       unsafe { arg1.as_ref() }.expect("Attempted to convert null pointer to reference"));
}
/// Allows to bind Qt signals with arguments `(QWidget*, QWidget*)` to a Rust closure.
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

pub struct SlotWidgetMutPtrWidgetMutPtr<'a> {
  wrapper: ::cpp_utils::CppBox<::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr>,
  func: ::std::option::Option<Box<Box<FnMut(*mut ::widget::Widget, *mut ::widget::Widget) + 'a>>>,
}

impl<'a> SlotWidgetMutPtrWidgetMutPtr<'a> {
  /// Constructs a new object.
  pub fn new<F: FnMut(*mut ::widget::Widget, *mut ::widget::Widget) + 'a>(f: F) -> SlotWidgetMutPtrWidgetMutPtr<'a> {
    let mut obj = SlotWidgetMutPtrWidgetMutPtr::default();
    obj.set(f);
    obj
  }

  /// Sets `f` as the callback closure. If `set()` is called again, previous closure is dropped.
  pub fn set<F: FnMut(*mut ::widget::Widget, *mut ::widget::Widget) + 'a>(&mut self, f: F) {
    self.clear();
    let mut func_box: Box<Box<FnMut(*mut ::widget::Widget, *mut ::widget::Widget) + 'a>> = Box::new(Box::new(f));
    unsafe {
      self.wrapper.set(slot_widget_mut_ptr_widget_mut_ptr_callback,
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

impl<'a> Default for SlotWidgetMutPtrWidgetMutPtr<'a> {
  fn default() -> Self {
    SlotWidgetMutPtrWidgetMutPtr {
      wrapper: ::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr::new(),
      func: None,
    }
  }
}

impl<'a> ::qt_core::connection::Receiver for SlotWidgetMutPtrWidgetMutPtr<'a> {
  type Arguments = (*mut ::widget::Widget, *mut ::widget::Widget);
  fn object(&self) -> &::qt_core::object::Object {
    ::qt_core::connection::Receiver::object(self.wrapper.as_ref())
  }
  fn receiver_id() -> &'static [u8] {
    <::slots::raw::RawSlotWidgetMutPtrWidgetMutPtr as ::qt_core::connection::Receiver>::receiver_id()
  }
}

extern "C" fn slot_widget_mut_ptr_widget_mut_ptr_callback(data: *mut ::libc::c_void,
                                                          arg0: *mut ::widget::Widget,
                                                          arg1: *mut ::widget::Widget) {
  let func: &mut Box<FnMut(*mut ::widget::Widget, *mut ::widget::Widget)> = unsafe { ::std::mem::transmute(data) };
  func(arg0, arg1);
}
