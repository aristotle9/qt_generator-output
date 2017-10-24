/// C++ type: <span style='color: green;'>```QApplicationStateChangeEvent```</span>
#[repr(C)]
pub struct ApplicationStateChangeEvent(u8);

impl ApplicationStateChangeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QApplicationStateChangeEvent::QApplicationStateChangeEvent(Qt::ApplicationState state)```</span>
  ///
  ///
  pub fn new(state: &::qt_core::qt::ApplicationState)
             -> ::cpp_utils::CppBox<::application_state_change_event::ApplicationStateChangeEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QApplicationStateChangeEvent_new(state as *const ::qt_core::qt::ApplicationState) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::application_state_change_event::ApplicationStateChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QApplicationStateChangeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::application_state_change_event::ApplicationStateChangeEvent {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::application_state_change_event::ApplicationStateChangeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(self as *const ::application_state_change_event::ApplicationStateChangeEvent as *mut ::application_state_change_event::ApplicationStateChangeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::application_state_change_event::ApplicationStateChangeEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::application_state_change_event::ApplicationStateChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QApplicationStateChangeEvent_ptr(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::application_state_change_event::ApplicationStateChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QApplicationStateChangeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::application_state_change_event::ApplicationStateChangeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(self as *const ::application_state_change_event::ApplicationStateChangeEvent as *mut ::application_state_change_event::ApplicationStateChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::application_state_change_event::ApplicationStateChangeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QApplicationStateChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::application_state_change_event::ApplicationStateChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
