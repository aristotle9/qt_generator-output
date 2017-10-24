/// C++ type: <span style='color: green;'>```QContextMenuEvent```</span>
#[repr(C)]
pub struct ContextMenuEvent(u8);

impl ContextMenuEvent {
  /// C++ method: <span style='color: green;'>```const QPoint& QContextMenuEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QContextMenuEvent_globalPos(self as *const ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QContextMenuEvent::globalX() const```</span>
  ///
  ///
  pub fn global_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QContextMenuEvent_globalX(self as *const ::context_menu_event::ContextMenuEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QContextMenuEvent::globalY() const```</span>
  ///
  ///
  pub fn global_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QContextMenuEvent_globalY(self as *const ::context_menu_event::ContextMenuEvent) }
  }

  /// C++ method: <span style='color: green;'>```QContextMenuEvent::QContextMenuEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((::context_menu_event::Reason, &::qt_core::point::Point)) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QContextMenuEvent::QContextMenuEvent(QContextMenuEvent::Reason reason, const QPoint& pos)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::context_menu_event::Reason, &::qt_core::point::Point, &::qt_core::point::Point)) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QContextMenuEvent::QContextMenuEvent(QContextMenuEvent::Reason reason, const QPoint& pos, const QPoint& globalPos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent>
    where Args: overloading::ContextMenuEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QPoint& QContextMenuEvent::pos() const```</span>
  ///
  ///
  pub fn pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QContextMenuEvent_pos(self as *const ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QContextMenuEvent::Reason QContextMenuEvent::reason() const```</span>
  ///
  ///
  pub fn reason(&self) -> ::context_menu_event::Reason {
    unsafe { ::ffi::qt_gui_c_QContextMenuEvent_reason(self as *const ::context_menu_event::ContextMenuEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QContextMenuEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QContextMenuEvent_x(self as *const ::context_menu_event::ContextMenuEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QContextMenuEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QContextMenuEvent_y(self as *const ::context_menu_event::ContextMenuEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::context_menu_event::ContextMenuEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QContextMenuEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QContextMenuEvent::Reason```</span>
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

impl ::cpp_utils::DynamicCast<::context_menu_event::ContextMenuEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::context_menu_event::ContextMenuEvent> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QContextMenuEvent_G_dynamic_cast_QContextMenuEvent_ptr(self as *mut ::input_event::InputEvent)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::context_menu_event::ContextMenuEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_dynamic_cast_QContextMenuEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::context_menu_event::ContextMenuEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QEvent_ptr(self as *mut ::context_menu_event::ContextMenuEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QEvent_ptr(self as *const ::context_menu_event::ContextMenuEvent as *mut ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::context_menu_event::ContextMenuEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(self as *mut ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(self as *const ::context_menu_event::ContextMenuEvent as *mut ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::context_menu_event::ContextMenuEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::context_menu_event::ContextMenuEvent {
    let ffi_result = ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::context_menu_event::ContextMenuEvent {
    let ffi_result = ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::context_menu_event::ContextMenuEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::context_menu_event::ContextMenuEvent {
    let ffi_result = ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::context_menu_event::ContextMenuEvent {
    let ffi_result = ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QContextMenuEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::context_menu_event::ContextMenuEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(self as *const ::context_menu_event::ContextMenuEvent as *mut ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::context_menu_event::ContextMenuEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QContextMenuEvent_G_static_cast_QInputEvent_ptr(self as *mut ::context_menu_event::ContextMenuEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ContextMenuEvent::new](../struct.ContextMenuEvent.html#method.new) method.
  pub trait ContextMenuEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent>;
  }
  impl<'a> ContextMenuEventNewArgs for (::context_menu_event::Reason, &'a ::qt_core::point::Point) {
    fn exec(self) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent> {
      let reason = self.0;
      let pos = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QContextMenuEvent_new_reason_pos(reason, pos as *const ::qt_core::point::Point) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ContextMenuEventNewArgs
    for (::context_menu_event::Reason, &'a ::qt_core::point::Point, &'a ::qt_core::point::Point) {
    fn exec(self) -> ::cpp_utils::CppBox<::context_menu_event::ContextMenuEvent> {
      let reason = self.0;
      let pos = self.1;
      let global_pos = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QContextMenuEvent_new_reason_pos_globalPos(reason,
                                                                     pos as *const ::qt_core::point::Point,
                                                                     global_pos as *const ::qt_core::point::Point)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
