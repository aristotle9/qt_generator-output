/// C++ type: <span style='color: green;'>```QAccessibleTextSelectionEvent```</span>
#[repr(C)]
pub struct AccessibleTextSelectionEvent(u8);

impl AccessibleTextSelectionEvent {
  /// C++ method: <span style='color: green;'>```QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QAccessibleInterface* iface, int start, int end)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextSelectionEvent::QAccessibleTextSelectionEvent(QObject* obj, int start, int end)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args)
                          -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent>
    where Args: overloading::AccessibleTextSelectionEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QAccessibleTextSelectionEvent::selectionEnd() const```</span>
  ///
  ///
  pub fn selection_end(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_selectionEnd(self as *const ::accessible_text_selection_event::AccessibleTextSelectionEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QAccessibleTextSelectionEvent::selectionStart() const```</span>
  ///
  ///
  pub fn selection_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_selectionStart(self as *const ::accessible_text_selection_event::AccessibleTextSelectionEvent) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleTextSelectionEvent::setSelection(int start, int end)```</span>
  ///
  ///
  pub fn set_selection(&mut self, start: ::libc::c_int, end: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_setSelection(self as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent, start, end) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_text_selection_event::AccessibleTextSelectionEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_text_selection_event::AccessibleTextSelectionEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_selection_event::AccessibleTextSelectionEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_selection_event::AccessibleTextSelectionEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::accessible_text_selection_event::AccessibleTextSelectionEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_selection_event::AccessibleTextSelectionEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_selection_event::AccessibleTextSelectionEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_dynamic_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_text_selection_event::AccessibleTextSelectionEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_text_selection_event::AccessibleTextSelectionEvent as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::accessible_text_cursor_event::AccessibleTextCursorEvent> for ::accessible_text_selection_event::AccessibleTextSelectionEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_selection_event::AccessibleTextSelectionEvent as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_selection_event::AccessibleTextSelectionEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_selection_event::AccessibleTextSelectionEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_selection_event::AccessibleTextSelectionEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_selection_event::AccessibleTextSelectionEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_selection_event::AccessibleTextSelectionEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_selection_event::AccessibleTextSelectionEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextSelectionEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_text_selection_event::AccessibleTextSelectionEvent {
  type Target = ::accessible_text_cursor_event::AccessibleTextCursorEvent;
  fn deref(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_selection_event::AccessibleTextSelectionEvent as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_text_selection_event::AccessibleTextSelectionEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_selection_event::AccessibleTextSelectionEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleTextSelectionEvent::new](../struct.AccessibleTextSelectionEvent.html#method.new) method.
  pub trait AccessibleTextSelectionEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent>;
  }
  impl AccessibleTextSelectionEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent> {
      let iface = self.0;
      let start = self.1;
      let end = self.2;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_new_iface_start_end(iface, start, end);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl AccessibleTextSelectionEventNewArgs for (*mut ::qt_core::object::Object, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_selection_event::AccessibleTextSelectionEvent> {
      let obj = self.0;
      let start = self.1;
      let end = self.2;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextSelectionEvent_new_obj_start_end(obj, start, end);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
