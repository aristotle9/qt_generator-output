/// C++ type: <span style='color: green;'>```QPaintEvent```</span>
#[repr(C)]
pub struct PaintEvent(u8);

impl PaintEvent {
  /// C++ method: <span style='color: green;'>```QPaintEvent::QPaintEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::paint_event::PaintEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPaintEvent::QPaintEvent(const QRect& paintRect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::region::Region) -> ::cpp_utils::CppBox<::paint_event::PaintEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPaintEvent::QPaintEvent(const QRegion& paintRegion)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::paint_event::PaintEvent>
    where Args: overloading::PaintEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```const QRect& QPaintEvent::rect() const```</span>
  ///
  ///
  pub fn rect<'l0>(&'l0 self) -> &'l0 ::qt_core::rect::Rect {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_rect(self as *const ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRegion& QPaintEvent::region() const```</span>
  ///
  ///
  pub fn region<'l0>(&'l0 self) -> &'l0 ::region::Region {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_region(self as *const ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::paint_event::PaintEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPaintEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::paint_event::PaintEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(self as *mut ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(self as *const ::paint_event::PaintEvent as *mut ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::paint_event::PaintEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::paint_event::PaintEvent {
    let ffi_result = ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QPaintEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::paint_event::PaintEvent {
    let ffi_result = ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QPaintEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::paint_event::PaintEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(self as *const ::paint_event::PaintEvent as *mut ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::paint_event::PaintEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPaintEvent_G_static_cast_QEvent_ptr(self as *mut ::paint_event::PaintEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PaintEvent::new](../struct.PaintEvent.html#method.new) method.
  pub trait PaintEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::paint_event::PaintEvent>;
  }
  impl<'a> PaintEventNewArgs for &'a ::qt_core::rect::Rect {
    fn exec(self) -> ::cpp_utils::CppBox<::paint_event::PaintEvent> {
      let paint_rect = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_new_paintRect(paint_rect as *const ::qt_core::rect::Rect) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PaintEventNewArgs for &'a ::region::Region {
    fn exec(self) -> ::cpp_utils::CppBox<::paint_event::PaintEvent> {
      let paint_region = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPaintEvent_new_paintRegion(paint_region as *const ::region::Region) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
