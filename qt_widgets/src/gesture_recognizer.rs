/// C++ type: <span style='color: green;'>```QGestureRecognizer```</span>
#[repr(C)]
pub struct GestureRecognizer(u8);

impl GestureRecognizer {
  /// C++ method: <span style='color: green;'>```virtual QGesture* QGestureRecognizer::create(QObject* target)```</span>
  ///
  ///
  pub unsafe fn create(&mut self, target: *mut ::qt_core::object::Object) -> *mut ::gesture::Gesture {
    ::ffi::qt_widgets_c_QGestureRecognizer_create(self as *mut ::gesture_recognizer::GestureRecognizer, target)
  }

  /// C++ method: <span style='color: green;'>```pure virtual QFlags<QGestureRecognizer::ResultFlag> QGestureRecognizer::recognize(QGesture* state, QObject* watched, QEvent* event)```</span>
  ///
  ///
  pub unsafe fn recognize(&mut self,
                          state: *mut ::gesture::Gesture,
                          watched: *mut ::qt_core::object::Object,
                          event: *mut ::qt_core::event::Event)
                          -> ::qt_core::flags::Flags<::gesture_recognizer::ResultFlag> {
    let ffi_result =
      ::ffi::qt_widgets_c_QGestureRecognizer_recognize(self as *mut ::gesture_recognizer::GestureRecognizer,
                                                       state,
                                                       watched,
                                                       event);
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```virtual void QGestureRecognizer::reset(QGesture* state)```</span>
  ///
  ///
  pub unsafe fn reset(&mut self, state: *mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QGestureRecognizer_reset(self as *mut ::gesture_recognizer::GestureRecognizer, state)
  }

  /// C++ method: <span style='color: green;'>```static void QGestureRecognizer::unregisterRecognizer(Qt::GestureType type)```</span>
  ///
  ///
  pub fn unregister_recognizer(type_: &::qt_core::qt::GestureType) {
    unsafe { ::ffi::qt_widgets_c_QGestureRecognizer_unregisterRecognizer(type_ as *const ::qt_core::qt::GestureType) }
  }
}

impl ::cpp_utils::CppDeletable for ::gesture_recognizer::GestureRecognizer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGestureRecognizer_delete
  }
}

/// C++ type: <span style='color: green;'>```QGestureRecognizer::ResultFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ResultFlag {
  /// C++ enum variant: <span style='color: green;'>```Ignore = 1```</span>
  Ignore = 1,
  /// C++ enum variant: <span style='color: green;'>```MayBeGesture = 2```</span>
  MayBeGesture = 2,
  /// C++ enum variant: <span style='color: green;'>```TriggerGesture = 4```</span>
  TriggerGesture = 4,
  /// C++ enum variant: <span style='color: green;'>```FinishGesture = 8```</span>
  FinishGesture = 8,
  /// C++ enum variant: <span style='color: green;'>```CancelGesture = 16```</span>
  CancelGesture = 16,
  /// C++ enum variant: <span style='color: green;'>```ResultState_Mask = 255```</span>
  ResultStateMask = 255,
  /// C++ enum variant: <span style='color: green;'>```ConsumeEventHint = 256```</span>
  ConsumeEventHint = 256,
  /// C++ enum variant: <span style='color: green;'>```ResultHint_Mask = 65280```</span>
  ResultHintMask = 65280,
}

impl ::qt_core::flags::FlaggableEnum for ResultFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ResultFlag"
  }
}
