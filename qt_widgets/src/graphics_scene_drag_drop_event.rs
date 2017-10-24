/// C++ type: <span style='color: green;'>```QGraphicsSceneDragDropEvent```</span>
#[repr(C)]
pub struct GraphicsSceneDragDropEvent(u8);

impl GraphicsSceneDragDropEvent {
  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::acceptProposedAction()```</span>
  ///
  ///
  pub fn accept_proposed_action(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_acceptProposedAction(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QMimeData* QGraphicsSceneDragDropEvent::mimeData() const```</span>
  ///
  ///
  pub fn mime_data(&self) -> *const ::qt_core::mime_data::MimeData {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_mimeData(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneDragDropEvent::QGraphicsSceneDragDropEvent(QEvent::Type type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent>
    where Args: overloading::GraphicsSceneDragDropEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneDragDropEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_pos_to_output(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QGraphicsSceneDragDropEvent::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_scenePos_to_output(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPoint QGraphicsSceneDragDropEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_screenPos_to_output(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setDropAction(Qt::DropAction action)```</span>
  ///
  ///
  pub fn set_drop_action(&mut self, action: &::qt_core::qt::DropAction) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setDropAction(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, action as *const ::qt_core::qt::DropAction) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setMimeData(const QMimeData* data)```</span>
  ///
  ///
  pub unsafe fn set_mime_data(&mut self, data: *const ::qt_core::mime_data::MimeData) {
    ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setMimeData(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, data)
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setPos(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setProposedAction(Qt::DropAction action)```</span>
  ///
  ///
  pub fn set_proposed_action(&mut self, action: &::qt_core::qt::DropAction) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setProposedAction(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, action as *const ::qt_core::qt::DropAction) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setScenePos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setScenePos(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setScreenPos(const QPoint& pos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, pos: &::qt_core::point::Point) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setScreenPos(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, pos as *const ::qt_core::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneDragDropEvent::setSource(QWidget* source)```</span>
  ///
  ///
  pub unsafe fn set_source(&mut self, source: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_setSource(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent, source)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QGraphicsSceneDragDropEvent::source() const```</span>
  ///
  ///
  pub fn source(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_source(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_dynamic_cast_QGraphicsSceneDragDropEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_dynamic_cast_QGraphicsSceneDragDropEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::event::Event {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneDragDropEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [GraphicsSceneDragDropEvent::new](../struct.GraphicsSceneDragDropEvent.html#method.new) method.
  pub trait GraphicsSceneDragDropEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent>;
  }
  impl GraphicsSceneDragDropEventNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> {

      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl GraphicsSceneDragDropEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::graphics_scene_drag_drop_event::GraphicsSceneDragDropEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneDragDropEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
