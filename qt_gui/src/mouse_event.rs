/// C++ type: <span style='color: green;'>```QMouseEvent```</span>
#[repr(C)]
pub struct MouseEvent(u8);

impl MouseEvent {
  /// C++ method: <span style='color: green;'>```QPoint QMouseEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMouseEvent_globalPos_to_output(self as *const ::mouse_event::MouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QMouseEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMouseEvent_globalX(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QMouseEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMouseEvent_globalY(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QMouseEvent::localPos() const```</span>
  ///
  ///
  pub fn local_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_localPos(self as *const ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint QMouseEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMouseEvent_pos_to_output(self as *const ::mouse_event::MouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QMouseEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_screenPos(self as *const ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QMouseEvent::setLocalPos(const QPointF& localPosition)```</span>
  ///
  ///
  pub fn set_local_pos(&mut self, local_position: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QMouseEvent_setLocalPos(self as *mut ::mouse_event::MouseEvent,
                                              local_position as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QMouseEvent::windowPos() const```</span>
  ///
  ///
  pub fn window_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_windowPos(self as *const ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QMouseEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMouseEvent_x(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QMouseEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QMouseEvent_y(self as *const ::mouse_event::MouseEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::mouse_event::MouseEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QMouseEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::mouse_event::MouseEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mouse_event::MouseEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMouseEvent_G_dynamic_cast_QMouseEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mouse_event::MouseEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_G_dynamic_cast_QMouseEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::mouse_event::MouseEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QEvent_ptr(self as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QEvent_ptr(self as *const ::mouse_event::MouseEvent as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::mouse_event::MouseEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(self as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(self as *const ::mouse_event::MouseEvent as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event::MouseEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event::MouseEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_event::MouseEvent {
    let ffi_result = ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event::MouseEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event::MouseEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_event::MouseEvent {
    let ffi_result = ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QMouseEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mouse_event::MouseEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(self as *const ::mouse_event::MouseEvent as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mouse_event::MouseEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMouseEvent_G_static_cast_QInputEvent_ptr(self as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
