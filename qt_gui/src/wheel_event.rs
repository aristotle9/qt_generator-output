/// C++ type: <span style='color: green;'>```QWheelEvent```</span>
#[repr(C)]
pub struct WheelEvent(u8);

impl WheelEvent {
  /// C++ method: <span style='color: green;'>```QPoint QWheelEvent::angleDelta() const```</span>
  ///
  ///
  pub fn angle_delta(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWheelEvent_angleDelta_to_output(self as *const ::wheel_event::WheelEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QWheelEvent::delta() const```</span>
  ///
  ///
  pub fn delta(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_delta(self as *const ::wheel_event::WheelEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWheelEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWheelEvent_globalPos_to_output(self as *const ::wheel_event::WheelEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QWheelEvent::globalPosF() const```</span>
  ///
  ///
  pub fn global_pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_globalPosF(self as *const ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QWheelEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_globalX(self as *const ::wheel_event::WheelEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QWheelEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_globalY(self as *const ::wheel_event::WheelEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool QWheelEvent::inverted() const```</span>
  ///
  ///
  pub fn inverted(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_inverted(self as *const ::wheel_event::WheelEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWheelEvent::pixelDelta() const```</span>
  ///
  ///
  pub fn pixel_delta(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWheelEvent_pixelDelta_to_output(self as *const ::wheel_event::WheelEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QWheelEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWheelEvent_pos_to_output(self as *const ::wheel_event::WheelEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QWheelEvent::posF() const```</span>
  ///
  ///
  pub fn pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_posF(self as *const ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QWheelEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_x(self as *const ::wheel_event::WheelEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QWheelEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QWheelEvent_y(self as *const ::wheel_event::WheelEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::wheel_event::WheelEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QWheelEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::wheel_event::WheelEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::wheel_event::WheelEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QWheelEvent_G_dynamic_cast_QWheelEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::wheel_event::WheelEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_G_dynamic_cast_QWheelEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::wheel_event::WheelEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QEvent_ptr(self as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QEvent_ptr(self as *const ::wheel_event::WheelEvent as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::wheel_event::WheelEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(self as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(self as *const ::wheel_event::WheelEvent as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wheel_event::WheelEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wheel_event::WheelEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wheel_event::WheelEvent {
    let ffi_result = ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::wheel_event::WheelEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::wheel_event::WheelEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::wheel_event::WheelEvent {
    let ffi_result = ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QWheelEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::wheel_event::WheelEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(self as *const ::wheel_event::WheelEvent as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::wheel_event::WheelEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QWheelEvent_G_static_cast_QInputEvent_ptr(self as *mut ::wheel_event::WheelEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
