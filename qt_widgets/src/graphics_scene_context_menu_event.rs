/// C++ type: <span style='color: green;'>```QGraphicsSceneContextMenuEvent```</span>
#[repr(C)]
pub struct GraphicsSceneContextMenuEvent(u8);

impl GraphicsSceneContextMenuEvent {
  /// C++ method: <span style='color: green;'>```QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneContextMenuEvent::QGraphicsSceneContextMenuEvent(QEvent::Type type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args)
                   -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent>
    where Args: overloading::GraphicsSceneContextMenuEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneContextMenuEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_pos_to_output(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsSceneContextMenuEvent::Reason QGraphicsSceneContextMenuEvent::reason() const```</span>
  ///
  ///
  pub fn reason(&self) -> ::graphics_scene_context_menu_event::Reason {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_reason(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneContextMenuEvent::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_scenePos_to_output(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneContextMenuEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_screenPos_to_output(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneContextMenuEvent::setPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_setPos(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneContextMenuEvent::setReason(QGraphicsSceneContextMenuEvent::Reason reason)```</span>
  ///
  ///
  pub fn set_reason(&mut self, reason: ::graphics_scene_context_menu_event::Reason) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_setReason(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, reason) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneContextMenuEvent::setScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_setScenePos(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneContextMenuEvent::setScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_setScreenPos(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent, pos as *const ::qt_core::point::Point) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QGraphicsSceneContextMenuEvent::Reason```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Reason {
  /// C++ enum variant: <span style='color: green;'>```Mouse = 0```</span>
  Mouse = 0,
  /// C++ enum variant: <span style='color: green;'>```Keyboard = 1```</span>
  Keyboard = 1,
  /// C++ enum variant: <span style='color: green;'>```Other = 2```</span>
  Other = 2,
}

impl ::cpp_utils::DynamicCast<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_dynamic_cast_QGraphicsSceneContextMenuEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_dynamic_cast_QGraphicsSceneContextMenuEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneContextMenuEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSceneContextMenuEvent::new](../struct.GraphicsSceneContextMenuEvent.html#method.new) method.
  pub trait GraphicsSceneContextMenuEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent>;
  }
  impl GraphicsSceneContextMenuEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneContextMenuEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_context_menu_event::GraphicsSceneContextMenuEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneContextMenuEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
