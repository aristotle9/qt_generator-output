/// C++ type: <span style='color: green;'>```QGestureEvent```</span>
#[repr(C)]
pub struct GestureEvent(u8);

impl GestureEvent {
  /// C++ method: <span style='color: green;'>```void QGestureEvent::accept(Qt::GestureType arg1)```</span>
  ///
  ///
  pub fn accept(&mut self, arg1: &::qt_core::qt::GestureType) {
    unsafe {
      ::ffi::qt_widgets_c_QGestureEvent_accept_Qt_GestureType(self as *mut ::gesture_event::GestureEvent,
                                                              arg1 as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::accept(QGesture* arg1)```</span>
  ///
  ///
  pub unsafe fn accept_unsafe(&mut self, arg1: *mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QGestureEvent_accept_QGesture(self as *mut ::gesture_event::GestureEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*> QGestureEvent::activeGestures() const```</span>
  ///
  ///
  pub fn active_gestures(&self) -> ::list::ListGestureMutPtr {
    {
      let mut object: ::list::ListGestureMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGestureEvent_activeGestures_to_output(self as *const ::gesture_event::GestureEvent,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*> QGestureEvent::canceledGestures() const```</span>
  ///
  ///
  pub fn canceled_gestures(&self) -> ::list::ListGestureMutPtr {
    {
      let mut object: ::list::ListGestureMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGestureEvent_canceledGestures_to_output(self as *const ::gesture_event::GestureEvent,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGesture* QGestureEvent::gesture(Qt::GestureType type) const```</span>
  ///
  ///
  pub fn gesture(&self, type_: &::qt_core::qt::GestureType) -> *mut ::gesture::Gesture {
    unsafe {
      ::ffi::qt_widgets_c_QGestureEvent_gesture(self as *const ::gesture_event::GestureEvent,
                                                type_ as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*> QGestureEvent::gestures() const```</span>
  ///
  ///
  pub fn gestures(&self) -> ::list::ListGestureMutPtr {
    {
      let mut object: ::list::ListGestureMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGestureEvent_gestures_to_output(self as *const ::gesture_event::GestureEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::ignore(Qt::GestureType arg1)```</span>
  ///
  ///
  pub fn ignore(&mut self, arg1: &::qt_core::qt::GestureType) {
    unsafe {
      ::ffi::qt_widgets_c_QGestureEvent_ignore_Qt_GestureType(self as *mut ::gesture_event::GestureEvent,
                                                              arg1 as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::ignore(QGesture* arg1)```</span>
  ///
  ///
  pub unsafe fn ignore_unsafe(&mut self, arg1: *mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QGestureEvent_ignore_QGesture(self as *mut ::gesture_event::GestureEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```bool QGestureEvent::isAccepted(Qt::GestureType arg1) const```</span>
  ///
  ///
  pub fn is_accepted(&self, arg1: &::qt_core::qt::GestureType) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QGestureEvent_isAccepted_Qt_GestureType(self as *const ::gesture_event::GestureEvent,
                                                                  arg1 as *const ::qt_core::qt::GestureType)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QGestureEvent::isAccepted(QGesture* arg1) const```</span>
  ///
  ///
  pub unsafe fn is_accepted_unsafe(&self, arg1: *mut ::gesture::Gesture) -> bool {
    ::ffi::qt_widgets_c_QGestureEvent_isAccepted_QGesture(self as *const ::gesture_event::GestureEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```QPointF QGestureEvent::mapToGraphicsScene(const QPointF& gesturePoint) const```</span>
  ///
  ///
  pub fn map_to_graphics_scene(&self, gesture_point: &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QGestureEvent_mapToGraphicsScene_to_output(self as *const ::gesture_event::GestureEvent, gesture_point as *const ::qt_core::point_f::PointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QGestureEvent::QGestureEvent(const QList<QGesture*>& gestures)```</span>
  ///
  ///
  pub fn new(gestures: &::list::ListGestureMutPtr) -> ::cpp_utils::CppBox<::gesture_event::GestureEvent> {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGestureEvent_new(gestures as *const ::list::ListGestureMutPtr) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::setAccepted(Qt::GestureType arg1, bool arg2)```</span>
  ///
  ///
  pub fn set_accepted(&mut self, arg1: &::qt_core::qt::GestureType, arg2: bool) {
    unsafe {
      ::ffi::qt_widgets_c_QGestureEvent_setAccepted_Qt_GestureType_bool(self as *mut ::gesture_event::GestureEvent,
                                                                        arg1 as *const ::qt_core::qt::GestureType,
                                                                        arg2)
    }
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::setAccepted(QGesture* arg1, bool arg2)```</span>
  ///
  ///
  pub unsafe fn set_accepted_unsafe(&mut self, arg1: *mut ::gesture::Gesture, arg2: bool) {
    ::ffi::qt_widgets_c_QGestureEvent_setAccepted_QGesture_bool(self as *mut ::gesture_event::GestureEvent, arg1, arg2)
  }

  /// C++ method: <span style='color: green;'>```void QGestureEvent::setWidget(QWidget* widget)```</span>
  ///
  ///
  pub unsafe fn set_widget(&mut self, widget: *mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QGestureEvent_setWidget(self as *mut ::gesture_event::GestureEvent, widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* QGestureEvent::widget() const```</span>
  ///
  ///
  pub fn widget(&self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QGestureEvent_widget(self as *const ::gesture_event::GestureEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::gesture_event::GestureEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_widgets_c_QGestureEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::gesture_event::GestureEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(self as *mut ::gesture_event::GestureEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(self as *const ::gesture_event::GestureEvent as *mut ::gesture_event::GestureEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::gesture_event::GestureEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::gesture_event::GestureEvent {
    let ffi_result =
      ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QGestureEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::gesture_event::GestureEvent {
    let ffi_result = ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QGestureEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::gesture_event::GestureEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(self as *const ::gesture_event::GestureEvent as *mut ::gesture_event::GestureEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::gesture_event::GestureEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QGestureEvent_G_static_cast_QEvent_ptr(self as *mut ::gesture_event::GestureEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
