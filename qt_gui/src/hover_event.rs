/// C++ type: <span style='color: green;'>```QHoverEvent```</span>
#[repr(C)]
pub struct HoverEvent(u8);

impl HoverEvent {
  /// C++ method: <span style='color: green;'>```QPoint QHoverEvent::oldPos() const```</span>
  ///
  ///
  pub fn old_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QHoverEvent_oldPos_to_output(self as *const ::hover_event::HoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QHoverEvent::oldPosF() const```</span>
  ///
  ///
  pub fn old_pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_oldPosF(self as *const ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint QHoverEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QHoverEvent_pos_to_output(self as *const ::hover_event::HoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QHoverEvent::posF() const```</span>
  ///
  ///
  pub fn pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_posF(self as *const ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::hover_event::HoverEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QHoverEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::hover_event::HoverEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::hover_event::HoverEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHoverEvent_G_dynamic_cast_QHoverEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::hover_event::HoverEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_G_dynamic_cast_QHoverEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::hover_event::HoverEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QEvent_ptr(self as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QEvent_ptr(self as *const ::hover_event::HoverEvent as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::hover_event::HoverEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(self as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(self as *const ::hover_event::HoverEvent as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::hover_event::HoverEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::hover_event::HoverEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::hover_event::HoverEvent {
    let ffi_result = ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::hover_event::HoverEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::hover_event::HoverEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::hover_event::HoverEvent {
    let ffi_result = ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QHoverEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::hover_event::HoverEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(self as *const ::hover_event::HoverEvent as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::hover_event::HoverEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHoverEvent_G_static_cast_QInputEvent_ptr(self as *mut ::hover_event::HoverEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
