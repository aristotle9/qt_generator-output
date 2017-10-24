/// C++ type: <span style='color: green;'>```QScrollPrepareEvent```</span>
#[repr(C)]
pub struct ScrollPrepareEvent(u8);

impl ScrollPrepareEvent {
  /// C++ method: <span style='color: green;'>```QPointF QScrollPrepareEvent::contentPos() const```</span>
  ///
  ///
  pub fn content_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollPrepareEvent_contentPos_to_output(self as *const ::scroll_prepare_event::ScrollPrepareEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QScrollPrepareEvent::contentPosRange() const```</span>
  ///
  ///
  pub fn content_pos_range(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollPrepareEvent_contentPosRange_to_output(self as *const ::scroll_prepare_event::ScrollPrepareEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QScrollPrepareEvent::QScrollPrepareEvent(const QPointF& startPos)```</span>
  ///
  ///
  pub fn new(start_pos: &::qt_core::point_f::PointF)
             -> ::cpp_utils::CppBox<::scroll_prepare_event::ScrollPrepareEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollPrepareEvent_new(start_pos as *const ::qt_core::point_f::PointF) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QScrollPrepareEvent::setContentPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_content_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QScrollPrepareEvent_setContentPos(self as *mut ::scroll_prepare_event::ScrollPrepareEvent,
                                                        pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QScrollPrepareEvent::setContentPosRange(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_content_pos_range(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QScrollPrepareEvent_setContentPosRange(self as *mut ::scroll_prepare_event::ScrollPrepareEvent,
                                                             rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QScrollPrepareEvent::setViewportSize(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_viewport_size(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe {
      ::ffi::qt_gui_c_QScrollPrepareEvent_setViewportSize(self as *mut ::scroll_prepare_event::ScrollPrepareEvent,
                                                          size as *const ::qt_core::size_f::SizeF)
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QScrollPrepareEvent::startPos() const```</span>
  ///
  ///
  pub fn start_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollPrepareEvent_startPos_to_output(self as *const ::scroll_prepare_event::ScrollPrepareEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QScrollPrepareEvent::viewportSize() const```</span>
  ///
  ///
  pub fn viewport_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QScrollPrepareEvent_viewportSize_to_output(self as *const ::scroll_prepare_event::ScrollPrepareEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::scroll_prepare_event::ScrollPrepareEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QScrollPrepareEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::scroll_prepare_event::ScrollPrepareEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(self as *mut ::scroll_prepare_event::ScrollPrepareEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(self as *const ::scroll_prepare_event::ScrollPrepareEvent as *mut ::scroll_prepare_event::ScrollPrepareEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::scroll_prepare_event::ScrollPrepareEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::scroll_prepare_event::ScrollPrepareEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QScrollPrepareEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::scroll_prepare_event::ScrollPrepareEvent {
    let ffi_result = ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QScrollPrepareEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::scroll_prepare_event::ScrollPrepareEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(self as *const ::scroll_prepare_event::ScrollPrepareEvent as *mut ::scroll_prepare_event::ScrollPrepareEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::scroll_prepare_event::ScrollPrepareEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QScrollPrepareEvent_G_static_cast_QEvent_ptr(self as *mut ::scroll_prepare_event::ScrollPrepareEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
