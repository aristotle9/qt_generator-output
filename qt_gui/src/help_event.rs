/// C++ type: <span style='color: green;'>```QHelpEvent```</span>
#[repr(C)]
pub struct HelpEvent(u8);

impl HelpEvent {
  /// C++ method: <span style='color: green;'>```const QPoint& QHelpEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHelpEvent_globalPos(self as *const ::help_event::HelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QHelpEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QHelpEvent_globalX(self as *const ::help_event::HelpEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QHelpEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QHelpEvent_globalY(self as *const ::help_event::HelpEvent) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QHelpEvent::QHelpEvent(QEvent::Type type, const QPoint& pos, const QPoint& globalPos)```</span>
  ///
  ///
  pub fn new(type_: ::qt_core::event::Type,
             pos: &::qt_core::point::Point,
             global_pos: &::qt_core::point::Point)
             -> ::cpp_utils::CppBox<::help_event::HelpEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QHelpEvent_new(type_,
                                     pos as *const ::qt_core::point::Point,
                                     global_pos as *const ::qt_core::point::Point)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QHelpEvent::pos() const```</span>
  ///
  ///
  pub fn pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHelpEvent_pos(self as *const ::help_event::HelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QHelpEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QHelpEvent_x(self as *const ::help_event::HelpEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QHelpEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QHelpEvent_y(self as *const ::help_event::HelpEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::help_event::HelpEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QHelpEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::help_event::HelpEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(self as *mut ::help_event::HelpEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(self as *const ::help_event::HelpEvent as *mut ::help_event::HelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::help_event::HelpEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::help_event::HelpEvent {
    let ffi_result = ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QHelpEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::help_event::HelpEvent {
    let ffi_result = ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QHelpEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::help_event::HelpEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(self as *const ::help_event::HelpEvent as *mut ::help_event::HelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::help_event::HelpEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHelpEvent_G_static_cast_QEvent_ptr(self as *mut ::help_event::HelpEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
