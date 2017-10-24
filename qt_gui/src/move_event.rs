/// C++ type: <span style='color: green;'>```QMoveEvent```</span>
#[repr(C)]
pub struct MoveEvent(u8);

impl MoveEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QMoveEvent::QMoveEvent(const QPoint& pos, const QPoint& oldPos)```</span>
  ///
  ///
  pub fn new(pos: &::qt_core::point::Point,
             old_pos: &::qt_core::point::Point)
             -> ::cpp_utils::CppBox<::move_event::MoveEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMoveEvent_new(pos as *const ::qt_core::point::Point,
                                     old_pos as *const ::qt_core::point::Point)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QMoveEvent::oldPos() const```</span>
  ///
  ///
  pub fn old_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMoveEvent_oldPos(self as *const ::move_event::MoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPoint& QMoveEvent::pos() const```</span>
  ///
  ///
  pub fn pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMoveEvent_pos(self as *const ::move_event::MoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::move_event::MoveEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QMoveEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::move_event::MoveEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(self as *mut ::move_event::MoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(self as *const ::move_event::MoveEvent as *mut ::move_event::MoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::move_event::MoveEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::move_event::MoveEvent {
    let ffi_result = ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QMoveEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::move_event::MoveEvent {
    let ffi_result = ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QMoveEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::move_event::MoveEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(self as *const ::move_event::MoveEvent as *mut ::move_event::MoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::move_event::MoveEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMoveEvent_G_static_cast_QEvent_ptr(self as *mut ::move_event::MoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
