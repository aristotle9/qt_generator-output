/// C++ type: <span style='color: green;'>```QFocusEvent```</span>
#[repr(C)]
pub struct FocusEvent(u8);

impl FocusEvent {
  /// C++ method: <span style='color: green;'>```bool QFocusEvent::gotFocus() const```</span>
  ///
  ///
  pub fn got_focus(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFocusEvent_gotFocus(self as *const ::focus_event::FocusEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool QFocusEvent::lostFocus() const```</span>
  ///
  ///
  pub fn lost_focus(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QFocusEvent_lostFocus(self as *const ::focus_event::FocusEvent) }
  }

  /// C++ method: <span style='color: green;'>```QFocusEvent::QFocusEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(::qt_core::event::Type) -> ::cpp_utils::CppBox<::focus_event::FocusEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFocusEvent::QFocusEvent(QEvent::Type type)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::qt_core::event::Type, &::qt_core::qt::FocusReason)) -> ::cpp_utils::CppBox<::focus_event::FocusEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFocusEvent::QFocusEvent(QEvent::Type type, Qt::FocusReason reason = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::focus_event::FocusEvent>
    where Args: overloading::FocusEventNewArgs
  {
    args.exec()
  }
}

impl ::cpp_utils::CppDeletable for ::focus_event::FocusEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QFocusEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::focus_event::FocusEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(self as *mut ::focus_event::FocusEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(self as *const ::focus_event::FocusEvent as *mut ::focus_event::FocusEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::focus_event::FocusEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::focus_event::FocusEvent {
    let ffi_result = ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QFocusEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::focus_event::FocusEvent {
    let ffi_result = ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QFocusEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::focus_event::FocusEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(self as *const ::focus_event::FocusEvent as *mut ::focus_event::FocusEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::focus_event::FocusEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QFocusEvent_G_static_cast_QEvent_ptr(self as *mut ::focus_event::FocusEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FocusEvent::new](../struct.FocusEvent.html#method.new) method.
  pub trait FocusEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::focus_event::FocusEvent>;
  }
  impl FocusEventNewArgs for ::qt_core::event::Type {
    fn exec(self) -> ::cpp_utils::CppBox<::focus_event::FocusEvent> {
      let type_ = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QFocusEvent_new_type(type_) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> FocusEventNewArgs for (::qt_core::event::Type, &'a ::qt_core::qt::FocusReason) {
    fn exec(self) -> ::cpp_utils::CppBox<::focus_event::FocusEvent> {
      let type_ = self.0;
      let reason = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QFocusEvent_new_type_reason(type_, reason as *const ::qt_core::qt::FocusReason) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
