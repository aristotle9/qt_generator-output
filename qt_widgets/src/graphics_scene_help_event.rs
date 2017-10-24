/// C++ type: <span style='color: green;'>```QGraphicsSceneHelpEvent```</span>
#[repr(C)]
pub struct GraphicsSceneHelpEvent(u8);

impl GraphicsSceneHelpEvent {
  /// C++ method: <span style='color: green;'>```QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneHelpEvent::QGraphicsSceneHelpEvent(QEvent::Type type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent>
    where Args: overloading::GraphicsSceneHelpEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneHelpEvent::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_scenePos_to_output(self as *const ::graphics_scene_help_event::GraphicsSceneHelpEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneHelpEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_screenPos_to_output(self as *const ::graphics_scene_help_event::GraphicsSceneHelpEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHelpEvent::setScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_setScenePos(self as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneHelpEvent::setScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_setScreenPos(self as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent, pos as *const ::qt_core::point::Point) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_help_event::GraphicsSceneHelpEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_help_event::GraphicsSceneHelpEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_help_event::GraphicsSceneHelpEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_dynamic_cast_QGraphicsSceneHelpEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_help_event::GraphicsSceneHelpEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_dynamic_cast_QGraphicsSceneHelpEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_help_event::GraphicsSceneHelpEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_help_event::GraphicsSceneHelpEvent as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_help_event::GraphicsSceneHelpEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_help_event::GraphicsSceneHelpEvent as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_help_event::GraphicsSceneHelpEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_help_event::GraphicsSceneHelpEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::graphics_scene_help_event::GraphicsSceneHelpEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_help_event::GraphicsSceneHelpEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_help_event::GraphicsSceneHelpEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_help_event::GraphicsSceneHelpEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneHelpEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_help_event::GraphicsSceneHelpEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_help_event::GraphicsSceneHelpEvent as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_help_event::GraphicsSceneHelpEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_help_event::GraphicsSceneHelpEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSceneHelpEvent::new](../struct.GraphicsSceneHelpEvent.html#method.new) method.
  pub trait GraphicsSceneHelpEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent>;
  }
  impl GraphicsSceneHelpEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneHelpEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_help_event::GraphicsSceneHelpEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneHelpEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
