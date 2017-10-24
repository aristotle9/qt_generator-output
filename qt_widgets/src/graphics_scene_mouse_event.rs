/// C++ type: <span style='color: green;'>```QGraphicsSceneMouseEvent```</span>
#[repr(C)]
pub struct GraphicsSceneMouseEvent(u8);

impl GraphicsSceneMouseEvent {
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::buttonDownPos(Qt::MouseButton button) const```</span>
  ///
  ///
  pub fn button_down_pos(&self, button: &::qt_core::qt::MouseButton) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownPos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::buttonDownScenePos(Qt::MouseButton button) const```</span>
  ///
  ///
  pub fn button_down_scene_pos(&self, button: &::qt_core::qt::MouseButton) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScenePos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneMouseEvent::buttonDownScreenPos(Qt::MouseButton button) const```</span>
  ///
  ///
  pub fn button_down_screen_pos(&self, button: &::qt_core::qt::MouseButton) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_buttonDownScreenPos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::lastPos() const```</span>
  ///
  ///
  pub fn last_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_lastPos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::lastScenePos() const```</span>
  ///
  ///
  pub fn last_scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_lastScenePos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneMouseEvent::lastScreenPos() const```</span>
  ///
  ///
  pub fn last_screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_lastScreenPos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneMouseEvent::QGraphicsSceneMouseEvent(QEvent::Type type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent>
    where Args: overloading::GraphicsSceneMouseEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_pos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneMouseEvent::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_scenePos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneMouseEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_screenPos_to_output(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setButton(Qt::MouseButton button)```</span>
  ///
  ///
  pub fn set_button(&mut self, button: &::qt_core::qt::MouseButton) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setButton(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setButtonDownPos(Qt::MouseButton button, const QPointF& pos)```</span>
  ///
  ///
  pub fn set_button_down_pos(&mut self, button: &::qt_core::qt::MouseButton, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setButtonDownScenePos(Qt::MouseButton button, const QPointF& pos)```</span>
  ///
  ///
  pub fn set_button_down_scene_pos(&mut self, button: &::qt_core::qt::MouseButton, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScenePos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setButtonDownScreenPos(Qt::MouseButton button, const QPoint& pos)```</span>
  ///
  ///
  pub fn set_button_down_screen_pos(&mut self, button: &::qt_core::qt::MouseButton, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setButtonDownScreenPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, button as *const ::qt_core::qt::MouseButton, pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setLastPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_last_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setLastPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setLastScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_last_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setLastScenePos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setLastScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_last_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setLastScreenPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setScenePos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setScreenPos(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneMouseEvent::setSource(Qt::MouseEventSource source)```</span>
  ///
  ///
  pub fn set_source(&mut self, source: &::qt_core::qt::MouseEventSource) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_setSource(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent, source as *const ::qt_core::qt::MouseEventSource) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_mouse_event::GraphicsSceneMouseEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_dynamic_cast_QGraphicsSceneMouseEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_mouse_event::GraphicsSceneMouseEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_dynamic_cast_QGraphicsSceneMouseEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_mouse_event::GraphicsSceneMouseEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_mouse_event::GraphicsSceneMouseEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneMouseEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_mouse_event::GraphicsSceneMouseEvent as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_mouse_event::GraphicsSceneMouseEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_mouse_event::GraphicsSceneMouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSceneMouseEvent::new](../struct.GraphicsSceneMouseEvent.html#method.new) method.
  pub trait GraphicsSceneMouseEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent>;
  }
  impl GraphicsSceneMouseEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneMouseEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_mouse_event::GraphicsSceneMouseEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneMouseEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
