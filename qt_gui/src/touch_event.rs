/// C++ type: <span style='color: green;'>```QTouchEvent```</span>
#[repr(C)]
pub struct TouchEvent(u8);

impl TouchEvent {
  /// C++ method: <span style='color: green;'>```QTouchDevice* QTouchEvent::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::touch_device::TouchDevice {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_device(self as *const ::touch_event::TouchEvent) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::setDevice(QTouchDevice* adevice)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, adevice: *mut ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QTouchEvent_setDevice(self as *mut ::touch_event::TouchEvent, adevice)
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::setTarget(QObject* atarget)```</span>
  ///
  ///
  pub unsafe fn set_target(&mut self, atarget: *mut ::qt_core::object::Object) {
    ::ffi::qt_gui_c_QTouchEvent_setTarget(self as *mut ::touch_event::TouchEvent, atarget)
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::setTouchPoints(const QList<QTouchEvent::TouchPoint>& atouchPoints)```</span>
  ///
  ///
  pub fn set_touch_points(&mut self, atouch_points: &::list::ListTouchEventTouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_setTouchPoints(self as *mut ::touch_event::TouchEvent,
                                                 atouch_points as *const ::list::ListTouchEventTouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::setWindow(QWindow* awindow)```</span>
  ///
  ///
  pub unsafe fn set_window(&mut self, awindow: *mut ::window::Window) {
    ::ffi::qt_gui_c_QTouchEvent_setWindow(self as *mut ::touch_event::TouchEvent, awindow)
  }

  /// C++ method: <span style='color: green;'>```QObject* QTouchEvent::target() const```</span>
  ///
  ///
  pub fn target(&self) -> *mut ::qt_core::object::Object {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_target(self as *const ::touch_event::TouchEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QList<QTouchEvent::TouchPoint>& QTouchEvent::touchPoints() const```</span>
  ///
  ///
  pub fn touch_points<'l0>(&'l0 self) -> &'l0 ::list::ListTouchEventTouchPoint {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_touchPoints(self as *const ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow* QTouchEvent::window() const```</span>
  ///
  ///
  pub fn window(&self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_window(self as *const ::touch_event::TouchEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::touch_event::TouchEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTouchEvent_delete
  }
}

/// C++ type: <span style='color: green;'>```QTouchEvent::TouchPoint```</span>
#[repr(C)]
pub struct TouchPoint([u8; ::type_sizes::QT_GUI_TOUCH_EVENT_TOUCH_POINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TouchPoint {
  unsafe fn new_uninitialized() -> TouchPoint {
    TouchPoint(::std::mem::uninitialized())
  }
}

impl TouchPoint {
  /// C++ method: <span style='color: green;'>```QSizeF QTouchEvent::TouchPoint::ellipseDiameters() const```</span>
  ///
  ///
  pub fn ellipse_diameters(&self) -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_ellipseDiameters_to_output(self as *const ::touch_event::TouchPoint,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFlags<QTouchEvent::TouchPoint::InfoFlag> QTouchEvent::TouchPoint::flags() const```</span>
  ///
  ///
  pub fn flags(&self) -> ::qt_core::flags::Flags<::touch_event::touch_point::InfoFlag> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_flags(self as *const ::touch_event::TouchPoint) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```int QTouchEvent::TouchPoint::id() const```</span>
  ///
  ///
  pub fn id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_id(self as *const ::touch_event::TouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::lastNormalizedPos() const```</span>
  ///
  ///
  pub fn last_normalized_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_lastNormalizedPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::lastPos() const```</span>
  ///
  ///
  pub fn last_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_lastPos_to_output(self as *const ::touch_event::TouchPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::lastScenePos() const```</span>
  ///
  ///
  pub fn last_scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_lastScenePos_to_output(self as *const ::touch_event::TouchPoint,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::lastScreenPos() const```</span>
  ///
  ///
  pub fn last_screen_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_lastScreenPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint::TouchPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::touch_event::TouchPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTouchEvent::TouchPoint::TouchPoint()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::touch_event::TouchPoint) -> ::touch_event::TouchPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTouchEvent::TouchPoint::TouchPoint(const QTouchEvent::TouchPoint& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::touch_event::TouchPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTouchEvent::TouchPoint::TouchPoint(int id = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::touch_event::TouchPoint
    where Args: overloading::TouchPointNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::normalizedPos() const```</span>
  ///
  ///
  pub fn normalized_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_normalizedPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QTouchEvent::TouchPoint::operator=(const QTouchEvent::TouchPoint& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::touch_event::TouchPoint)
                             -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_operator_assign(self as *mut ::touch_event::TouchPoint,
                                                             other as *const ::touch_event::TouchPoint)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::pos() const```</span>
  ///
  ///
  pub fn pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_pos_to_output(self as *const ::touch_event::TouchPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTouchEvent::TouchPoint::pressure() const```</span>
  ///
  ///
  pub fn pressure(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_pressure(self as *const ::touch_event::TouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF> QTouchEvent::TouchPoint::rawScreenPositions() const```</span>
  ///
  ///
  pub fn raw_screen_positions(&self) -> ::qt_core::vector::VectorPointF {
    {
      let mut object: ::qt_core::vector::VectorPointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_rawScreenPositions_to_output(self as *const ::touch_event::TouchPoint,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTouchEvent::TouchPoint::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_rect_to_output(self as *const ::touch_event::TouchPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTouchEvent::TouchPoint::rotation() const```</span>
  ///
  ///
  pub fn rotation(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_rotation(self as *const ::touch_event::TouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::scenePos() const```</span>
  ///
  ///
  pub fn scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_scenePos_to_output(self as *const ::touch_event::TouchPoint,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTouchEvent::TouchPoint::sceneRect() const```</span>
  ///
  ///
  pub fn scene_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_sceneRect_to_output(self as *const ::touch_event::TouchPoint,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::screenPos() const```</span>
  ///
  ///
  pub fn screen_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_screenPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTouchEvent::TouchPoint::screenRect() const```</span>
  ///
  ///
  pub fn screen_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_screenRect_to_output(self as *const ::touch_event::TouchPoint,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setEllipseDiameters(const QSizeF& dia)```</span>
  ///
  ///
  pub fn set_ellipse_diameters(&mut self, dia: &::qt_core::size_f::SizeF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setEllipseDiameters(self as *mut ::touch_event::TouchPoint,
                                                                 dia as *const ::qt_core::size_f::SizeF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setFlags(QFlags<QTouchEvent::TouchPoint::InfoFlag> flags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, flags: ::qt_core::flags::Flags<::touch_event::touch_point::InfoFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setFlags(self as *mut ::touch_event::TouchPoint,
                                                      flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setId(int id)```</span>
  ///
  ///
  pub fn set_id(&mut self, id: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setId(self as *mut ::touch_event::TouchPoint, id) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setLastNormalizedPos(const QPointF& lastNormalizedPos)```</span>
  ///
  ///
  pub fn set_last_normalized_pos(&mut self, last_normalized_pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setLastNormalizedPos(self as *mut ::touch_event::TouchPoint, last_normalized_pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setLastPos(const QPointF& lastPos)```</span>
  ///
  ///
  pub fn set_last_pos(&mut self, last_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setLastPos(self as *mut ::touch_event::TouchPoint,
                                                        last_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setLastScenePos(const QPointF& lastScenePos)```</span>
  ///
  ///
  pub fn set_last_scene_pos(&mut self, last_scene_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setLastScenePos(self as *mut ::touch_event::TouchPoint,
                                                             last_scene_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setLastScreenPos(const QPointF& lastScreenPos)```</span>
  ///
  ///
  pub fn set_last_screen_pos(&mut self, last_screen_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setLastScreenPos(self as *mut ::touch_event::TouchPoint,
                                                              last_screen_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setNormalizedPos(const QPointF& normalizedPos)```</span>
  ///
  ///
  pub fn set_normalized_pos(&mut self, normalized_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setNormalizedPos(self as *mut ::touch_event::TouchPoint,
                                                              normalized_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setPos(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_pos(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setPos(self as *mut ::touch_event::TouchPoint,
                                                    pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setPressure(double pressure)```</span>
  ///
  ///
  pub fn set_pressure(&mut self, pressure: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setPressure(self as *mut ::touch_event::TouchPoint, pressure) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setRawScreenPositions(const QVector<QPointF>& positions)```</span>
  ///
  ///
  pub fn set_raw_screen_positions(&mut self, positions: &::qt_core::vector::VectorPointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setRawScreenPositions(self as *mut ::touch_event::TouchPoint, positions as *const ::qt_core::vector::VectorPointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setRect(const QRectF& rect)```</span>
  ///
  ///
  pub fn set_rect(&mut self, rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setRect(self as *mut ::touch_event::TouchPoint,
                                                     rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setRotation(double angle)```</span>
  ///
  ///
  pub fn set_rotation(&mut self, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setRotation(self as *mut ::touch_event::TouchPoint, angle) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setScenePos(const QPointF& scenePos)```</span>
  ///
  ///
  pub fn set_scene_pos(&mut self, scene_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setScenePos(self as *mut ::touch_event::TouchPoint,
                                                         scene_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setSceneRect(const QRectF& sceneRect)```</span>
  ///
  ///
  pub fn set_scene_rect(&mut self, scene_rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setSceneRect(self as *mut ::touch_event::TouchPoint,
                                                          scene_rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setScreenPos(const QPointF& screenPos)```</span>
  ///
  ///
  pub fn set_screen_pos(&mut self, screen_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setScreenPos(self as *mut ::touch_event::TouchPoint,
                                                          screen_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setScreenRect(const QRectF& screenRect)```</span>
  ///
  ///
  pub fn set_screen_rect(&mut self, screen_rect: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setScreenRect(self as *mut ::touch_event::TouchPoint,
                                                           screen_rect as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setStartNormalizedPos(const QPointF& startNormalizedPos)```</span>
  ///
  ///
  pub fn set_start_normalized_pos(&mut self, start_normalized_pos: &::qt_core::point_f::PointF) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setStartNormalizedPos(self as *mut ::touch_event::TouchPoint, start_normalized_pos as *const ::qt_core::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setStartPos(const QPointF& startPos)```</span>
  ///
  ///
  pub fn set_start_pos(&mut self, start_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setStartPos(self as *mut ::touch_event::TouchPoint,
                                                         start_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setStartScenePos(const QPointF& startScenePos)```</span>
  ///
  ///
  pub fn set_start_scene_pos(&mut self, start_scene_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setStartScenePos(self as *mut ::touch_event::TouchPoint,
                                                              start_scene_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setStartScreenPos(const QPointF& startScreenPos)```</span>
  ///
  ///
  pub fn set_start_screen_pos(&mut self, start_screen_pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setStartScreenPos(self as *mut ::touch_event::TouchPoint,
                                                               start_screen_pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setUniqueId(qint64 uid)```</span>
  ///
  ///
  pub fn set_unique_id(&mut self, uid: i64) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setUniqueId(self as *mut ::touch_event::TouchPoint, uid) }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::setVelocity(const QVector2D& v)```</span>
  ///
  ///
  pub fn set_velocity(&mut self, v: &::vector_2d::Vector2D) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_setVelocity(self as *mut ::touch_event::TouchPoint,
                                                         v as *const ::vector_2d::Vector2D)
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::startNormalizedPos() const```</span>
  ///
  ///
  pub fn start_normalized_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_startNormalizedPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::startPos() const```</span>
  ///
  ///
  pub fn start_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_startPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::startScenePos() const```</span>
  ///
  ///
  pub fn start_scene_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_startScenePos_to_output(self as *const ::touch_event::TouchPoint,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTouchEvent::TouchPoint::startScreenPos() const```</span>
  ///
  ///
  pub fn start_screen_pos(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_startScreenPos_to_output(self as *const ::touch_event::TouchPoint,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTouchEvent::TouchPoint::swap(QTouchEvent::TouchPoint& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QTouchEvent_TouchPoint_swap(self as *mut ::touch_event::TouchPoint,
                                                  other as *mut ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```QPointingDeviceUniqueId QTouchEvent::TouchPoint::uniqueId() const```</span>
  ///
  ///
  pub fn unique_id(&self) -> ::pointing_device_unique_id::PointingDeviceUniqueId {
    {
      let mut object: ::pointing_device_unique_id::PointingDeviceUniqueId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTouchEvent_TouchPoint_uniqueId_to_output(self as *const ::touch_event::TouchPoint,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector2D QTouchEvent::TouchPoint::velocity() const```</span>
  ///
  ///
  pub fn velocity(&self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_velocity_as_ptr(self as *const ::touch_event::TouchPoint) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl Drop for ::touch_event::TouchPoint {
  /// C++ method: <span style='color: green;'>```[destructor] void QTouchEvent::TouchPoint::~TouchPoint()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTouchEvent_TouchPoint_destructor(self as *mut ::touch_event::TouchPoint) }
  }
}

impl ::cpp_utils::DynamicCast<::touch_event::TouchEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::touch_event::TouchEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTouchEvent_G_dynamic_cast_QTouchEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::touch_event::TouchEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_G_dynamic_cast_QTouchEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::touch_event::TouchEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QEvent_ptr(self as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QEvent_ptr(self as *const ::touch_event::TouchEvent as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::touch_event::TouchEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(self as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(self as *const ::touch_event::TouchEvent as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::touch_event::TouchEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::touch_event::TouchEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::touch_event::TouchEvent {
    let ffi_result = ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::touch_event::TouchEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::touch_event::TouchEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::touch_event::TouchEvent {
    let ffi_result = ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QTouchEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::touch_event::TouchEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(self as *const ::touch_event::TouchEvent as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::touch_event::TouchEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTouchEvent_G_static_cast_QInputEvent_ptr(self as *mut ::touch_event::TouchEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TouchPoint::new](../struct.TouchPoint.html#method.new) method.
  pub trait TouchPointNewArgs {
    fn exec(self) -> ::touch_event::TouchPoint;
  }
  impl TouchPointNewArgs for ::libc::c_int {
    fn exec(self) -> ::touch_event::TouchPoint {
      let id = self;
      {
        let mut object: ::touch_event::TouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTouchEvent_TouchPoint_constructor_id(id, &mut object);
        }
        object
      }
    }
  }
  impl TouchPointNewArgs for () {
    fn exec(self) -> ::touch_event::TouchPoint {

      {
        let mut object: ::touch_event::TouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTouchEvent_TouchPoint_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> TouchPointNewArgs for &'a ::touch_event::TouchPoint {
    fn exec(self) -> ::touch_event::TouchPoint {
      let other = self;
      {
        let mut object: ::touch_event::TouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTouchEvent_TouchPoint_constructor_other(other as *const ::touch_event::TouchPoint,
                                                                   &mut object);
        }
        object
      }
    }
  }
}

pub mod touch_point {
  /// C++ type: <span style='color: green;'>```QTouchEvent::TouchPoint::InfoFlag```</span>
  #[derive(Debug, PartialEq, Eq, Clone)]
  #[repr(C)]
  pub enum InfoFlag {
    /// C++ enum variant: <span style='color: green;'>```Pen = 1```</span>
    Pen = 1,
    /// C++ enum variant: <span style='color: green;'>```Token = 2```</span>
    Token = 2,
  }

  impl ::qt_core::flags::FlaggableEnum for InfoFlag {
    fn to_flag_value(self) -> ::libc::c_int {
      self as ::libc::c_int
    }
    fn enum_name() -> &'static str {
      "InfoFlag"
    }
  }

}
