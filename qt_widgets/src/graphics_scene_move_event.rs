/// C++ type: <span style='color: green;'>```QGraphicsSceneMoveEvent```</span>
#[repr(C)]
pub struct GraphicsSceneMoveEvent(u8);

impl GraphicsSceneMoveEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneMoveEvent::QGraphicsSceneMoveEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_scene_move_event::GraphicsSceneMoveEvent> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMoveEvent::newPos() const```</span>
  ///
  ///
  pub fn new_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_newPos_to_output(self as *const ::graphics_scene_move_event::GraphicsSceneMoveEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMoveEvent::oldPos() const```</span>
  ///
  ///
  pub fn old_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_oldPos_to_output(self as *const ::graphics_scene_move_event::GraphicsSceneMoveEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMoveEvent::setNewPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_new_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_setNewPos(self as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMoveEvent::setOldPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_old_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_setOldPos(self as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent, pos as *const ::qt_core::point_f::PointF) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_move_event::GraphicsSceneMoveEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_move_event::GraphicsSceneMoveEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_move_event::GraphicsSceneMoveEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_dynamic_cast_QGraphicsSceneMoveEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_move_event::GraphicsSceneMoveEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_dynamic_cast_QGraphicsSceneMoveEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_move_event::GraphicsSceneMoveEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_move_event::GraphicsSceneMoveEvent as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_move_event::GraphicsSceneMoveEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_move_event::GraphicsSceneMoveEvent as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_move_event::GraphicsSceneMoveEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_move_event::GraphicsSceneMoveEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scene_move_event::GraphicsSceneMoveEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_move_event::GraphicsSceneMoveEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_move_event::GraphicsSceneMoveEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_move_event::GraphicsSceneMoveEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneMoveEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_move_event::GraphicsSceneMoveEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_move_event::GraphicsSceneMoveEvent as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_move_event::GraphicsSceneMoveEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMoveEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_move_event::GraphicsSceneMoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
