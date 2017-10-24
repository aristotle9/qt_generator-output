/// C++ type: <span style='color: green;'>```QNativeGestureEvent```</span>
#[repr(C)]
pub struct NativeGestureEvent(u8);

impl NativeGestureEvent {
  /// C++ method: <span style='color: green;'>```const QPoint QNativeGestureEvent::globalPos() const```</span>
  ///
  ///
  pub fn global_pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QNativeGestureEvent_globalPos_to_output(self as *const ::native_gesture_event::NativeGestureEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QNativeGestureEvent::localPos() const```</span>
  ///
  ///
  pub fn local_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QNativeGestureEvent_localPos(self as *const ::native_gesture_event::NativeGestureEvent)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QNativeGestureEvent::QNativeGestureEvent(Qt::NativeGestureType type, const QPointF& localPos, const QPointF& windowPos, const QPointF& screenPos, double value, unsigned long sequenceId, quint64 intArgument)```</span>
  ///
  ///
  pub fn new(type_: &::qt_core::qt::NativeGestureType,
             local_pos: &::qt_core::point_f::PointF,
             window_pos: &::qt_core::point_f::PointF,
             screen_pos: &::qt_core::point_f::PointF,
             value: ::libc::c_double,
             sequence_id: ::libc::c_ulong,
             int_argument: u64)
             -> ::cpp_utils::CppBox<::native_gesture_event::NativeGestureEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QNativeGestureEvent_new(type_ as *const ::qt_core::qt::NativeGestureType,
                                              local_pos as *const ::qt_core::point_f::PointF,
                                              window_pos as *const ::qt_core::point_f::PointF,
                                              screen_pos as *const ::qt_core::point_f::PointF,
                                              value,
                                              sequence_id,
                                              int_argument)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QPoint QNativeGestureEvent::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QNativeGestureEvent_pos_to_output(self as *const ::native_gesture_event::NativeGestureEvent,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QNativeGestureEvent::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QNativeGestureEvent_screenPos(self as *const ::native_gesture_event::NativeGestureEvent)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QNativeGestureEvent::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_value(self as *const ::native_gesture_event::NativeGestureEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QPointF& QNativeGestureEvent::windowPos() const```</span>
  ///
  ///
  pub fn window_pos<'l0>(&'l0 self) -> &'l0 ::qt_core::point_f::PointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QNativeGestureEvent_windowPos(self as *const ::native_gesture_event::NativeGestureEvent)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::native_gesture_event::NativeGestureEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QNativeGestureEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::native_gesture_event::NativeGestureEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::native_gesture_event::NativeGestureEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_dynamic_cast_QNativeGestureEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::native_gesture_event::NativeGestureEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_dynamic_cast_QNativeGestureEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::native_gesture_event::NativeGestureEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QEvent_ptr(self as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QEvent_ptr(self as *const ::native_gesture_event::NativeGestureEvent as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::native_gesture_event::NativeGestureEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(self as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(self as *const ::native_gesture_event::NativeGestureEvent as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::native_gesture_event::NativeGestureEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::native_gesture_event::NativeGestureEvent {
    let ffi_result = ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::native_gesture_event::NativeGestureEvent {
    let ffi_result = ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::native_gesture_event::NativeGestureEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::native_gesture_event::NativeGestureEvent {
    let ffi_result = ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::native_gesture_event::NativeGestureEvent {
    let ffi_result = ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QNativeGestureEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::native_gesture_event::NativeGestureEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(self as *const ::native_gesture_event::NativeGestureEvent as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::native_gesture_event::NativeGestureEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QNativeGestureEvent_G_static_cast_QInputEvent_ptr(self as *mut ::native_gesture_event::NativeGestureEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
