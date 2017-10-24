/// C++ type: <span style='color: green;'>```QDragEnterEvent```</span>
#[repr(C)]
pub struct DragEnterEvent(u8);

impl ::cpp_utils::CppDeletable for ::drag_enter_event::DragEnterEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDragEnterEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::drag_enter_event::DragEnterEvent> for ::drag_move_event::DragMoveEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::drag_enter_event::DragEnterEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_dynamic_cast_QDragEnterEvent_ptr_QDragMoveEvent(self as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::drag_enter_event::DragEnterEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_dynamic_cast_QDragEnterEvent_ptr_QDragMoveEvent(self as *const ::drag_move_event::DragMoveEvent as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::drag_enter_event::DragEnterEvent> for ::drop_event::DropEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::drag_enter_event::DragEnterEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_dynamic_cast_QDragEnterEvent_ptr_QDropEvent(self as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::drag_enter_event::DragEnterEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_dynamic_cast_QDragEnterEvent_ptr_QDropEvent(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::drag_enter_event::DragEnterEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QEvent_ptr(self as *mut ::drag_enter_event::DragEnterEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QEvent_ptr(self as *const ::drag_enter_event::DragEnterEvent as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::drag_move_event::DragMoveEvent> for ::drag_enter_event::DragEnterEvent {
  fn static_cast_mut(&mut self) -> &mut ::drag_move_event::DragMoveEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragMoveEvent_ptr(self as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::drag_move_event::DragMoveEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragMoveEvent_ptr(self as *const ::drag_enter_event::DragEnterEvent as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::drop_event::DropEvent> for ::drag_enter_event::DragEnterEvent {
  fn static_cast_mut(&mut self) -> &mut ::drop_event::DropEvent {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDropEvent_ptr(self as *mut ::drag_enter_event::DragEnterEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::drop_event::DropEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDropEvent_ptr(self as *const ::drag_enter_event::DragEnterEvent as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_enter_event::DragEnterEvent> for ::drag_move_event::DragMoveEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_enter_event::DragEnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QDragMoveEvent(self as *mut ::drag_move_event::DragMoveEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_enter_event::DragEnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QDragMoveEvent(self as *const ::drag_move_event::DragMoveEvent as *mut ::drag_move_event::DragMoveEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_enter_event::DragEnterEvent> for ::drop_event::DropEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_enter_event::DragEnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QDropEvent(self as *mut ::drop_event::DropEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_enter_event::DragEnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QDropEvent(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_enter_event::DragEnterEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_enter_event::DragEnterEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_enter_event::DragEnterEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragEnterEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::drag_enter_event::DragEnterEvent {
  type Target = ::drag_move_event::DragMoveEvent;
  fn deref(&self) -> &::drag_move_event::DragMoveEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragMoveEvent_ptr(self as *const ::drag_enter_event::DragEnterEvent as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::drag_enter_event::DragEnterEvent {
  fn deref_mut(&mut self) -> &mut ::drag_move_event::DragMoveEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragEnterEvent_G_static_cast_QDragMoveEvent_ptr(self as *mut ::drag_enter_event::DragEnterEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
