/// C++ type: <span style='color: green;'>```QAccessibleStateChangeEvent```</span>
#[repr(C)]
pub struct AccessibleStateChangeEvent(u8);

impl AccessibleStateChangeEvent {
  /// C++ method: <span style='color: green;'>```QAccessible::State QAccessibleStateChangeEvent::changedStates() const```</span>
  ///
  ///
  pub fn changed_states(&self) -> ::accessible::State {
    {
      let mut object: ::accessible::State =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QAccessibleStateChangeEvent_changedStates_to_output(self as *const ::accessible_state_change_event::AccessibleStateChangeEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleStateChangeEvent::QAccessibleStateChangeEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((*mut ::accessible_interface::AccessibleInterface, &::accessible::State)) -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleStateChangeEvent::QAccessibleStateChangeEvent(QAccessibleInterface* iface, QAccessible::State state)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((*mut ::qt_core::object::Object, &::accessible::State)) -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QAccessibleStateChangeEvent::QAccessibleStateChangeEvent(QObject* obj, QAccessible::State state)```</span>
  ///
  ///
  pub unsafe fn new<Args>(args: Args)
                          -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent>
    where Args: overloading::AccessibleStateChangeEventNewArgs
  {
    args.exec()
  }
}

impl ::cpp_utils::CppDeletable for ::accessible_state_change_event::AccessibleStateChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QAccessibleStateChangeEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::accessible_state_change_event::AccessibleStateChangeEvent> for ::accessible_event::AccessibleEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::accessible_state_change_event::AccessibleStateChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_dynamic_cast_QAccessibleStateChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::accessible_state_change_event::AccessibleStateChangeEvent> {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_dynamic_cast_QAccessibleStateChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::accessible_event::AccessibleEvent> for ::accessible_state_change_event::AccessibleStateChangeEvent {
fn static_cast_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_state_change_event::AccessibleStateChangeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::accessible_event::AccessibleEvent {
let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_state_change_event::AccessibleStateChangeEvent as *mut ::accessible_state_change_event::AccessibleStateChangeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::accessible_state_change_event::AccessibleStateChangeEvent> for ::accessible_event::AccessibleEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::accessible_state_change_event::AccessibleStateChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleStateChangeEvent_ptr(self as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::accessible_state_change_event::AccessibleStateChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleStateChangeEvent_ptr(self as *const ::accessible_event::AccessibleEvent as *mut ::accessible_event::AccessibleEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::accessible_state_change_event::AccessibleStateChangeEvent {
  type Target = ::accessible_event::AccessibleEvent;
  fn deref(&self) -> &::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *const ::accessible_state_change_event::AccessibleStateChangeEvent as *mut ::accessible_state_change_event::AccessibleStateChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::accessible_state_change_event::AccessibleStateChangeEvent {
  fn deref_mut(&mut self) -> &mut ::accessible_event::AccessibleEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QAccessibleStateChangeEvent_G_static_cast_QAccessibleEvent_ptr(self as *mut ::accessible_state_change_event::AccessibleStateChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [AccessibleStateChangeEvent::new](../struct.AccessibleStateChangeEvent.html#method.new) method.
  pub trait AccessibleStateChangeEventNewArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent>;
  }
  impl<'a> AccessibleStateChangeEventNewArgs
    for (*mut ::accessible_interface::AccessibleInterface, &'a ::accessible::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent> {
      let iface = self.0;
      let state = self.1;
      let ffi_result =
        ::ffi::qt_gui_c_QAccessibleStateChangeEvent_new_iface_state(iface, state as *const ::accessible::State);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> AccessibleStateChangeEventNewArgs for (*mut ::qt_core::object::Object, &'a ::accessible::State) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::accessible_state_change_event::AccessibleStateChangeEvent> {
      let obj = self.0;
      let state = self.1;
      let ffi_result = ::ffi::qt_gui_c_QAccessibleStateChangeEvent_new_obj_state(obj,
                                                                                 state as *const ::accessible::State);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
