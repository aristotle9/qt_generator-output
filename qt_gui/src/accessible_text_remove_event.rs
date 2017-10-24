/// C++ type: <span style='color: green;'>```QAccessibleTextRemoveEvent```</span>
#[repr(C)]
pub struct AccessibleTextRemoveEvent(u8);

impl AccessibleTextRemoveEvent {
  /// C++ method: <span style='color: green;'>```int QAccessibleTextRemoveEvent::changePosition() const```</span>
  ///
  ///
  pub fn change_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_changePosition(self as *const ::accessible_text_remove_event::AccessibleTextRemoveEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, ::libc::c_int, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QAccessibleInterface* iface, int position, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, ::libc::c_int, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextRemoveEvent::QAccessibleTextRemoveEvent(QObject* obj, int position, const QString& text)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent>
    where Args: overloading::AccessibleTextRemoveEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QAccessibleTextRemoveEvent::textRemoved() const```</span>
  ///
  ///
  pub fn text_removed(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_textRemoved_to_output(self as *const ::accessible_text_remove_event::AccessibleTextRemoveEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_text_remove_event::AccessibleTextRemoveEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_text_remove_event::AccessibleTextRemoveEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_remove_event::AccessibleTextRemoveEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_remove_event::AccessibleTextRemoveEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::accessible_text_remove_event::AccessibleTextRemoveEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_remove_event::AccessibleTextRemoveEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_remove_event::AccessibleTextRemoveEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_dynamic_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_text_remove_event::AccessibleTextRemoveEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_text_remove_event::AccessibleTextRemoveEvent as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::accessible_text_cursor_event::AccessibleTextCursorEvent> for ::accessible_text_remove_event::AccessibleTextRemoveEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_remove_event::AccessibleTextRemoveEvent as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_remove_event::AccessibleTextRemoveEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_remove_event::AccessibleTextRemoveEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_remove_event::AccessibleTextRemoveEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_remove_event::AccessibleTextRemoveEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_remove_event::AccessibleTextRemoveEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_remove_event::AccessibleTextRemoveEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextRemoveEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_text_remove_event::AccessibleTextRemoveEvent {
  type Target = ::accessible_text_cursor_event::AccessibleTextCursorEvent;
  fn deref(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_remove_event::AccessibleTextRemoveEvent as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_text_remove_event::AccessibleTextRemoveEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_remove_event::AccessibleTextRemoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleTextRemoveEvent::new](../struct.AccessibleTextRemoveEvent.html#method.new) method.
  pub trait AccessibleTextRemoveEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent>;
  }
  impl<'a> AccessibleTextRemoveEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface, ::libc::c_int, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent> {
      let iface = self.0;
      let position = self.1;
      let text = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_new_iface_position_text(iface,
                                                                           position,
                                                                           text as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AccessibleTextRemoveEventNewArgs
    for (*mut ::qt_core::object::Object, ::libc::c_int, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_remove_event::AccessibleTextRemoveEvent> {
      let obj = self.0;
      let position = self.1;
      let text = self.2;
      let ffi_result =
        ::ffi::qt_gui_c_QAccessibleTextRemoveEvent_new_obj_position_text(obj,
                                                                         position,
                                                                         text as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
