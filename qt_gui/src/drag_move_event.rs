/// C++ type: <span style='color: green;'>```QDragMoveEvent```</span>
#[repr(C)]
pub struct DragMoveEvent(u8);

impl DragMoveEvent {
  /// C++ method: <span style='color: green;'>```QDragMoveEvent::accept```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn accept(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDragMoveEvent::accept()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn accept(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDragMoveEvent::accept(const QRect& r)```</span>
  ///
  ///
  pub fn accept<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DragMoveEventAcceptArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QDragMoveEvent::answerRect() const```</span>
  ///
  ///
  pub fn answer_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QDragMoveEvent_answerRect_to_output(self as *const ::drag_move_event::DragMoveEvent,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDragMoveEvent::ignore```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ignore(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDragMoveEvent::ignore()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ignore(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDragMoveEvent::ignore(const QRect& r)```</span>
  ///
  ///
  pub fn ignore<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DragMoveEventIgnoreArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::drag_move_event::DragMoveEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDragMoveEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::drag_move_event::DragMoveEvent> for ::drop_event::DropEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::drag_move_event::DragMoveEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_dynamic_cast_QDragMoveEvent_ptr(self as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::drag_move_event::DragMoveEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_dynamic_cast_QDragMoveEvent_ptr(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::drag_move_event::DragMoveEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QEvent_ptr(self as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QEvent_ptr(self as *const ::drag_move_event::DragMoveEvent as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::drop_event::DropEvent> for ::drag_move_event::DragMoveEvent {
  fn static_cast_mut(&mut self) -> &mut ::drop_event::DropEvent {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(self as *mut ::drag_move_event::DragMoveEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::drop_event::DropEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(self as *const ::drag_move_event::DragMoveEvent as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_move_event::DragMoveEvent> for ::drop_event::DropEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_move_event::DragMoveEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QDropEvent(self as *mut ::drop_event::DropEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_move_event::DragMoveEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QDropEvent(self as *const ::drop_event::DropEvent as *mut ::drop_event::DropEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_move_event::DragMoveEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_move_event::DragMoveEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_move_event::DragMoveEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDragMoveEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::drag_move_event::DragMoveEvent {
  type Target = ::drop_event::DropEvent;
  fn deref(&self) -> &::drop_event::DropEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(self as *const ::drag_move_event::DragMoveEvent as *mut ::drag_move_event::DragMoveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::drag_move_event::DragMoveEvent {
  fn deref_mut(&mut self) -> &mut ::drop_event::DropEvent {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragMoveEvent_G_static_cast_QDropEvent_ptr(self as *mut ::drag_move_event::DragMoveEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DragMoveEvent::accept](../struct.DragMoveEvent.html#method.accept) method.
  pub trait DragMoveEventAcceptArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> ();
  }
  impl<'largs> DragMoveEventAcceptArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> () {

      unsafe { ::ffi::qt_gui_c_QDragMoveEvent_accept_no_args(original_self as *mut ::drag_move_event::DragMoveEvent) }
    }
  }
  impl<'largs> DragMoveEventAcceptArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QDragMoveEvent_accept_r(original_self as *mut ::drag_move_event::DragMoveEvent,
                                                r as *const ::qt_core::rect::Rect)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DragMoveEvent::ignore](../struct.DragMoveEvent.html#method.ignore) method.
  pub trait DragMoveEventIgnoreArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> ();
  }
  impl<'largs> DragMoveEventIgnoreArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> () {

      unsafe { ::ffi::qt_gui_c_QDragMoveEvent_ignore_no_args(original_self as *mut ::drag_move_event::DragMoveEvent) }
    }
  }
  impl<'largs> DragMoveEventIgnoreArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::drag_move_event::DragMoveEvent) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QDragMoveEvent_ignore_r(original_self as *mut ::drag_move_event::DragMoveEvent,
                                                r as *const ::qt_core::rect::Rect)
      }
    }
  }
}
