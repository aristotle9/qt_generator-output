/// C++ type: <span style='color: green;'>```QAccessibleTextUpdateEvent```</span>
#[repr(C)]
pub struct AccessibleTextUpdateEvent(u8);

impl AccessibleTextUpdateEvent {
  /// C++ method: <span style='color: green;'>```int QAccessibleTextUpdateEvent::changePosition() const```</span>
  ///
  ///
  pub fn change_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_changePosition(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, ::libc::c_int, &::qt_core::string::String, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QAccessibleInterface* iface, int position, const QString& oldText, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, ::libc::c_int, &::qt_core::string::String, &::qt_core::string::String)) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleTextUpdateEvent::QAccessibleTextUpdateEvent(QObject* obj, int position, const QString& oldText, const QString& text)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent>
    where Args: overloading::AccessibleTextUpdateEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QString QAccessibleTextUpdateEvent::textInserted() const```</span>
  ///
  ///
  pub fn text_inserted(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_textInserted_to_output(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QAccessibleTextUpdateEvent::textRemoved() const```</span>
  ///
  ///
  pub fn text_removed(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_textRemoved_to_output(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_text_update_event::AccessibleTextUpdateEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_text_update_event::AccessibleTextUpdateEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_update_event::AccessibleTextUpdateEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_update_event::AccessibleTextUpdateEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::accessible_text_update_event::AccessibleTextUpdateEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_text_update_event::AccessibleTextUpdateEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_text_update_event::AccessibleTextUpdateEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_dynamic_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_text_update_event::AccessibleTextUpdateEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::accessible_text_cursor_event::AccessibleTextCursorEvent> for ::accessible_text_update_event::AccessibleTextUpdateEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_update_event::AccessibleTextUpdateEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_update_event::AccessibleTextUpdateEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_update_event::AccessibleTextUpdateEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleEvent(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_text_update_event::AccessibleTextUpdateEvent> for ::accessible_text_cursor_event::AccessibleTextCursorEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_text_update_event::AccessibleTextUpdateEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(self as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_text_update_event::AccessibleTextUpdateEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextUpdateEvent_ptr_QAccessibleTextCursorEvent(self as *const ::accessible_text_cursor_event::AccessibleTextCursorEvent as *mut ::accessible_text_cursor_event::AccessibleTextCursorEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_text_update_event::AccessibleTextUpdateEvent {
  type Target = ::accessible_text_cursor_event::AccessibleTextCursorEvent;
  fn deref(&self) -> &::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *const ::accessible_text_update_event::AccessibleTextUpdateEvent as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_text_update_event::AccessibleTextUpdateEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_text_cursor_event::AccessibleTextCursorEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_G_static_cast_QAccessibleTextCursorEvent_ptr(self as *mut ::accessible_text_update_event::AccessibleTextUpdateEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleTextUpdateEvent::new](../struct.AccessibleTextUpdateEvent.html#method.new) method.
  pub trait AccessibleTextUpdateEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent>;
  }
  impl<'a> AccessibleTextUpdateEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface,
                                                     ::libc::c_int,
                                                     &'a ::qt_core::string::String,
                                                     &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent> {
      let iface = self.0;
      let position = self.1;
      let old_text = self.2;
      let text = self.3;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_new_iface_position_oldText_text(iface, position, old_text as *const ::qt_core::string::String, text as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AccessibleTextUpdateEventNewArgs
    for (*mut ::qt_core::object::Object, ::libc::c_int, &'a ::qt_core::string::String, &'a ::qt_core::string::String) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_text_update_event::AccessibleTextUpdateEvent> {
      let obj = self.0;
      let position = self.1;
      let old_text = self.2;
      let text = self.3;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleTextUpdateEvent_new_obj_position_oldText_text(obj, position, old_text as *const ::qt_core::string::String, text as *const ::qt_core::string::String);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
