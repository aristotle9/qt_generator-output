/// C++ type: <span style='color: green;'>```QDropEvent```</span>
#[repr(C)]
pub struct DropEvent(u8);

impl DropEvent {
  /// C++ method: <span style='color: green;'>```void QDropEvent::acceptProposedAction()```</span>
  ///
  ///
  pub fn accept_proposed_action(&mut self) {
    unsafe { ::ffi::qt_gui_c_QDropEvent_acceptProposedAction(self as *mut ::drop_event::DropEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QMimeData* QDropEvent::mimeData() const```</span>
  ///
  ///
  pub fn mime_data(&self) -> *const ::qt_core::mime_data::MimeData {
    unsafe { ::ffi::qt_gui_c_QDropEvent_mimeData(self as *const ::drop_event::DropEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QDropEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QDropEvent_pos_to_output(self as *const ::drop_event::DropEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QDropEvent::posF() const```</span>
  ///
  ///
  pub fn pos_f<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDropEvent_posF(self as *const ::drop_event::DropEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QDropEvent::setDropAction(Qt::DropAction action)```</span>
  ///
  ///
  pub fn set_drop_action(&mut self, action: &::qt_core::qt::DropAction) {
    unsafe {
      ::ffi::qt_gui_c_QDropEvent_setDropAction(self as *mut ::drop_event::DropEvent,
                                               action as *const ::qt_core::qt::DropAction)
    }
  }

  /// C++ method: <span style='color: green;'>```QObject* QDropEvent::source() const```</span>
  ///
  ///
  pub fn source(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QDropEvent_source(self as *const ::drop_event::DropEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::drop_event::DropEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDropEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::drop_event::DropEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(self as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drop_event::DropEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drop_event::DropEvent {
    let ffi_result = ::ffi::qt_gui_c_QDropEvent_G_static_cast_QDropEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drop_event::DropEvent {
    let ffi_result = ::ffi::qt_gui_c_QDropEvent_G_static_cast_QDropEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::drop_event::DropEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::drop_event::DropEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDropEvent_G_static_cast_QEvent_ptr(self as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
