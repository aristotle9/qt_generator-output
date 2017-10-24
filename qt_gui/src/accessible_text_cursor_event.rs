/// C++ type: <span style='color: green;'>```QAccessibleTextCursorEvent```</span>
#[repr(C)]
pub struct AccessibleTextCursorEvent(u8);

impl AccessibleTextCursorEvent {
  /// C++ method: <span style='color: green;'>```int QAccessibleTextCursorEvent::cursorPosition() const```</span>
  ///
  ///
  pub fn cursor_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_cursorPosition(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTextCursorEvent::QAccessibleTextCursorEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, ::libc::c_int)) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QAccessibleInterface* iface, int cursorPos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, ::libc::c_int)) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextCursorEvent::QAccessibleTextCursorEvent(QObject* obj, int cursorPos)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent>
    where Args: overloading::AccessibleTextCursorEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QAccessibleTextCursorEvent::setCursorPosition(int position)```</span>
  ///
  ///
  pub fn set_cursor_position(&mut self, position: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_setCursorPosition(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent, position) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTextCursorEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_text_cursor_event::AccessibleTextCursorEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_cursor_event::AccessibleTextCursorEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_dynamic_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_cursor_event::AccessibleTextCursorEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_dynamic_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_cursor_event::AccessibleTextCursorEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
  type Target = ::accessible_event::AccessibleEvent;
  fn deref(&self) -> &::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextCursorEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleTextCursorEvent::new](../struct.AccessibleTextCursorEvent.html#method.new) method.
  pub trait AccessibleTextCursorEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent>;
  }
  impl AccessibleTextCursorEventNewArgs for (*mut ::accessible_interface::AccessibleInterface, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent> {
      let iface = self.0;
      let cursor_pos = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextCursorEvent_new_iface_cursorPos(iface, cursor_pos);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AccessibleTextCursorEventNewArgs for (*mut ::qt_core::object::Object, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_cursor_event::AccessibleTextCursorEvent> {
      let obj = self.0;
      let cursor_pos = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextCursorEvent_new_obj_cursorPos(obj, cursor_pos);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
