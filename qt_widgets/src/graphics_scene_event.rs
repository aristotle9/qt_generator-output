/// C++ type: <span style='color: green;'>```QGraphicsSceneEvent```</span>
#[repr(C)]
pub struct GraphicsSceneEvent(u8);

impl GraphicsSceneEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneEvent::QGraphicsSceneEvent(QEvent::Type type)```</span>
  ///
  ///
  pub fn new(type_: ::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_event::GraphicsSceneEvent> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_new(type_) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneEvent::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsSceneEvent_setWidget(self as *mut ::graphics_scene_event::GraphicsSceneEvent,
                                                      widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QGraphicsSceneEvent::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_widget(self as *const ::graphics_scene_event::GraphicsSceneEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_event::GraphicsSceneEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_event::GraphicsSceneEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::graphics_scene_event::GraphicsSceneEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_event::GraphicsSceneEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
