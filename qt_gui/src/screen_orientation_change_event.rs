/// C++ type: <span style='color: green;'>```QScreenOrientationChangeEvent```</span>
#[repr(C)]
pub struct ScreenOrientationChangeEvent(u8);

impl ScreenOrientationChangeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QScreenOrientationChangeEvent::QScreenOrientationChangeEvent(QScreen* screen, Qt::ScreenOrientation orientation)```</span>
  ///
  ///
  pub unsafe fn new(screen: *mut ::screen::Screen,
                    orientation: &::qt_core::qt::ScreenOrientation)
                    -> ::cpp_utils::CppBox<::screen_orientation_change_event::ScreenOrientationChangeEvent> {
    let ffi_result =
      ::ffi::qt_gui_c_QScreenOrientationChangeEvent_new(screen,
                                                        orientation as *const ::qt_core::qt::ScreenOrientation);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QScreen* QScreenOrientationChangeEvent::screen() const```</span>
  ///
  ///
  pub fn screen(&self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QScreenOrientationChangeEvent_screen(self as *const ::screen_orientation_change_event::ScreenOrientationChangeEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::screen_orientation_change_event::ScreenOrientationChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QScreenOrientationChangeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::screen_orientation_change_event::ScreenOrientationChangeEvent {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::screen_orientation_change_event::ScreenOrientationChangeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(self as *const ::screen_orientation_change_event::ScreenOrientationChangeEvent as *mut ::screen_orientation_change_event::ScreenOrientationChangeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::screen_orientation_change_event::ScreenOrientationChangeEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::screen_orientation_change_event::ScreenOrientationChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QScreenOrientationChangeEvent_ptr(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::screen_orientation_change_event::ScreenOrientationChangeEvent {
let ffi_result = ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QScreenOrientationChangeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::screen_orientation_change_event::ScreenOrientationChangeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(self as *const ::screen_orientation_change_event::ScreenOrientationChangeEvent as *mut ::screen_orientation_change_event::ScreenOrientationChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::screen_orientation_change_event::ScreenOrientationChangeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScreenOrientationChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::screen_orientation_change_event::ScreenOrientationChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
