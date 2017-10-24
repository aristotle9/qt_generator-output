/// C++ type: <span style='color: green;'>```QAccessibleEvent```</span>
#[repr(C)]
pub struct AccessibleEvent(u8);

impl AccessibleEvent {
  /// C++ method: <span style='color: green;'>```virtual QAccessibleInterface* QAccessibleEvent::accessibleInterface() const```</span>
  ///
  ///
  pub fn accessible_interface(&self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe { ::ffi::qt_gui_c_QAccessibleEvent_accessibleInterface(self as *const ::accessible_event::AccessibleEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QAccessibleEvent::child() const```</span>
  ///
  ///
  pub fn child(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QAccessibleEvent_child(self as *const ::accessible_event::AccessibleEvent) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleEvent::QAccessibleEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, &::accessible::Event)) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleEvent::QAccessibleEvent(QAccessibleInterface* iface, QAccessible::Event typ)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, &::accessible::Event)) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleEvent::QAccessibleEvent(QObject* obj, QAccessible::Event typ)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent>
    where Args: overloading::AccessibleEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QObject* QAccessibleEvent::object() const```</span>
  ///
  ///
  pub fn object(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QAccessibleEvent_object(self as *const ::accessible_event::AccessibleEvent) }
  }

  /// C++ method: <span style='color: green;'>```void QAccessibleEvent::setChild(int chld)```</span>
  ///
  ///
  pub fn set_child(&mut self, chld: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QAccessibleEvent_setChild(self as *mut ::accessible_event::AccessibleEvent, chld) }
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_event::AccessibleEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleEvent_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleEvent::new](../struct.AccessibleEvent.html#method.new) method.
  pub trait AccessibleEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent>;
  }
  impl<'a> AccessibleEventNewArgs for (*mut ::accessible_interface::AccessibleInterface, &'a ::accessible::Event) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent> {
      let iface = self.0;
      let typ = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleEvent_new_iface_typ(iface, typ as *const ::accessible::Event);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AccessibleEventNewArgs for (*mut ::qt_core::object::Object, &'a ::accessible::Event) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_event::AccessibleEvent> {
      let obj = self.0;
      let typ = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleEvent_new_obj_typ(obj, typ as *const ::accessible::Event);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
