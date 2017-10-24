/// C++ type: <span style='color: green;'>```QGraphicsSceneResizeEvent```</span>
#[repr(C)]
pub struct GraphicsSceneResizeEvent(u8);

impl GraphicsSceneResizeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QGraphicsSceneResizeEvent::QGraphicsSceneResizeEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::graphics_scene_resize_event::GraphicsSceneResizeEvent> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsSceneResizeEvent::newSize() const```</span>
  ///
  ///
  pub fn new_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_newSize_to_output(self as *const ::graphics_scene_resize_event::GraphicsSceneResizeEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSizeF QGraphicsSceneResizeEvent::oldSize() const```</span>
  ///
  ///
  pub fn old_size(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_oldSize_to_output(self as *const ::graphics_scene_resize_event::GraphicsSceneResizeEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneResizeEvent::setNewSize(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_new_size(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_setNewSize(self as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent, size as *const ::qt_core::size_f::SizeF) }
  }

  /// C++ method: <span style='color: green;'>```void QGraphicsSceneResizeEvent::setOldSize(const QSizeF& size)```</span>
  ///
  ///
  pub fn set_old_size(&mut self, size: &::qt_core::size_f::SizeF) {
    unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_setOldSize(self as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent, size as *const ::qt_core::size_f::SizeF) }
  }
}

impl ::cpp_utils::CppDeletable for ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::graphics_scene_resize_event::GraphicsSceneResizeEvent> for ::graphics_scene_event::GraphicsSceneEvent {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_dynamic_cast_QGraphicsSceneResizeEvent_ptr(self as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::graphics_scene_resize_event::GraphicsSceneResizeEvent> {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_dynamic_cast_QGraphicsSceneResizeEvent_ptr(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QEvent_ptr(self as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QEvent_ptr(self as *const ::graphics_scene_resize_event::GraphicsSceneResizeEvent as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::graphics_scene_event::GraphicsSceneEvent> for ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
fn static_cast_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_resize_event::GraphicsSceneResizeEvent as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_resize_event::GraphicsSceneResizeEvent> for ::qt_core::event::Event {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_resize_event::GraphicsSceneResizeEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::graphics_scene_resize_event::GraphicsSceneResizeEvent> for ::graphics_scene_event::GraphicsSceneEvent {
unsafe fn static_cast_mut(&mut self) -> &mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QGraphicsSceneEvent(self as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::graphics_scene_resize_event::GraphicsSceneResizeEvent {
let ffi_result = ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneResizeEvent_ptr_QGraphicsSceneEvent(self as *const ::graphics_scene_event::GraphicsSceneEvent as *mut ::graphics_scene_event::GraphicsSceneEvent);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
  type Target = ::graphics_scene_event::GraphicsSceneEvent;
  fn deref(&self) -> &::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *const ::graphics_scene_resize_event::GraphicsSceneResizeEvent as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::graphics_scene_resize_event::GraphicsSceneResizeEvent {
  fn deref_mut(&mut self) -> &mut ::graphics_scene_event::GraphicsSceneEvent {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGraphicsSceneResizeEvent_G_static_cast_QGraphicsSceneEvent_ptr(self as *mut ::graphics_scene_resize_event::GraphicsSceneResizeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
