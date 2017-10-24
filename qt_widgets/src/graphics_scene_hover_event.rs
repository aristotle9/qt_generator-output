/// C++ type: <span style='color: green;'>```QGraphicsSceneHoverEvent```</span>
#[repr(C)]
pub struct GraphicsSceneHoverEvent(u8);

impl GraphicsSceneHoverEvent {
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneHoverEvent::lastPos() const```</span>
  ///
  ///
  pub fn last_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_lastPos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneHoverEvent::lastScenePos() const```</span>
  ///
  ///
  pub fn last_scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_lastScenePos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneHoverEvent::lastScreenPos() const```</span>
  ///
  ///
  pub fn last_screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_lastScreenPos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneHoverEvent::QGraphicsSceneHoverEvent(QEvent::Type type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent>
    where Args: overloading::GraphicsSceneHoverEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneHoverEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_pos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneHoverEvent::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_scenePos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneHoverEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_screenPos_to_output(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setLastPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_last_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setLastPos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setLastScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_last_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setLastScenePos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setLastScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_last_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setLastScreenPos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setPos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setScenePos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHoverEvent::setScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_setScreenPos(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent, pos as *const ::qt_core::point::Point) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_hover_event::GraphicsSceneHoverEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_dynamic_cast_QGraphicsSceneHoverEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_hover_event::GraphicsSceneHoverEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_dynamic_cast_QGraphicsSceneHoverEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_hover_event::GraphicsSceneHoverEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_hover_event::GraphicsSceneHoverEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_hover_event::GraphicsSceneHoverEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_hover_event::GraphicsSceneHoverEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneHoverEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_hover_event::GraphicsSceneHoverEvent as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_hover_event::GraphicsSceneHoverEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_hover_event::GraphicsSceneHoverEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSceneHoverEvent::new](../struct.GraphicsSceneHoverEvent.html#method.new) method.
  pub trait GraphicsSceneHoverEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent>;
  }
  impl GraphicsSceneHoverEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneHoverEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_hover_event::GraphicsSceneHoverEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHoverEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
