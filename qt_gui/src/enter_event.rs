/// C++ type: <span style='color: green;'>```QEnterEvent```</span>
#[repr(C)]
pub struct EnterEvent(u8);

impl EnterEvent {
  /// C++ method: <span style='color: green;'>```QPoint QEnterEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QEnterEvent_globalPos_to_output(self as *const ::enter_event::EnterEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QEnterEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QEnterEvent_globalX(self as *const ::enter_event::EnterEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QEnterEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QEnterEvent_globalY(self as *const ::enter_event::EnterEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QEnterEvent::localPos() const```</span>
  ///
  ///
  pub fn local_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QEnterEvent_localPos(self as *const ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QEnterEvent::QEnterEvent(const QPointF& localPos, const QPointF& windowPos, const QPointF& screenPos)```</span>
  ///
  ///
  pub fn new(local_pos: &::qt_core::point_f::PointF,
             window_pos: &::qt_core::point_f::PointF,
             screen_pos: &::qt_core::point_f::PointF)
             -> ::cpp_utils::CppBox<::enter_event::EnterEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QEnterEvent_new(local_pos as *const ::qt_core::point_f::PointF,
                                      window_pos as *const ::qt_core::point_f::PointF,
                                      screen_pos as *const ::qt_core::point_f::PointF)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QEnterEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QEnterEvent_pos_to_output(self as *const ::enter_event::EnterEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QEnterEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QEnterEvent_screenPos(self as *const ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QEnterEvent::windowPos() const```</span>
  ///
  ///
  pub fn window_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QEnterEvent_windowPos(self as *const ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QEnterEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QEnterEvent_x(self as *const ::enter_event::EnterEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QEnterEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QEnterEvent_y(self as *const ::enter_event::EnterEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::enter_event::EnterEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QEnterEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::enter_event::EnterEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(self as *mut ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(self as *const ::enter_event::EnterEvent as *mut ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::enter_event::EnterEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::enter_event::EnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEnterEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::enter_event::EnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEnterEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::enter_event::EnterEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(self as *const ::enter_event::EnterEvent as *mut ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::enter_event::EnterEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QEnterEvent_G_static_cast_QEvent_ptr(self as *mut ::enter_event::EnterEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
