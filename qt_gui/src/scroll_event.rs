/// C++ type: <span style='color: green;'>```QScrollEvent```</span>
#[repr(C)]
pub struct ScrollEvent(u8);

impl ScrollEvent {
  /// C++ method: <span style='color: green;'>```QPointF QScrollEvent::contentPos() const```</span>
  ///
  ///
  pub fn content_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollEvent_contentPos_to_output(self as *const ::scroll_event::ScrollEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QScrollEvent::QScrollEvent(const QPointF& contentPos, const QPointF& overshoot, QScrollEvent::ScrollState scrollState)```</span>
  ///
  ///
  pub fn new(content_pos: &::qt_core::point_f::PointF,
             overshoot: &::qt_core::point_f::PointF,
             scroll_state: ::scroll_event::ScrollState)
             -> ::cpp_utils::CppBox<::scroll_event::ScrollEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QScrollEvent_new(content_pos as *const ::qt_core::point_f::PointF,
                                       overshoot as *const ::qt_core::point_f::PointF,
                                       scroll_state)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QScrollEvent::overshootDistance() const```</span>
  ///
  ///
  pub fn overshoot_distance(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollEvent_overshootDistance_to_output(self as *const ::scroll_event::ScrollEvent,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QScrollEvent::ScrollState QScrollEvent::scrollState() const```</span>
  ///
  ///
  pub fn scroll_state(&self) -> ::scroll_event::ScrollState {
    unsafe { ::ffi::qt_gui_c_QScrollEvent_scrollState(self as *const ::scroll_event::ScrollEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::scroll_event::ScrollEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QScrollEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QScrollEvent::ScrollState```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ScrollState {
  /// C++ enum variant: <span style='color: green;'>```ScrollStarted = 0```</span>
  Started = 0,
  /// C++ enum variant: <span style='color: green;'>```ScrollUpdated = 1```</span>
  Updated = 1,
  /// C++ enum variant: <span style='color: green;'>```ScrollFinished = 2```</span>
  Finished = 2,
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::scroll_event::ScrollEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(self as *mut ::scroll_event::ScrollEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(self as *const ::scroll_event::ScrollEvent as *mut ::scroll_event::ScrollEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_event::ScrollEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_event::ScrollEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QScrollEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_event::ScrollEvent {
    let ffi_result = ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QScrollEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scroll_event::ScrollEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(self as *const ::scroll_event::ScrollEvent as *mut ::scroll_event::ScrollEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scroll_event::ScrollEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QScrollEvent_G_static_cast_QEvent_ptr(self as *mut ::scroll_event::ScrollEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
