/// C++ type: <span style='color: green;'>```QTimeLine::CurveShape```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CurveShape {
  /// C++ enum variant: <span style='color: green;'>```EaseInCurve = 0```</span>
  EaseIn = 0,
  /// C++ enum variant: <span style='color: green;'>```EaseOutCurve = 1```</span>
  EaseOut = 1,
  /// C++ enum variant: <span style='color: green;'>```EaseInOutCurve = 2```</span>
  EaseInOut = 2,
  /// C++ enum variant: <span style='color: green;'>```LinearCurve = 3```</span>
  Linear = 3,
  /// C++ enum variant: <span style='color: green;'>```SineCurve = 4```</span>
  Sine = 4,
  /// C++ enum variant: <span style='color: green;'>```CosineCurve = 5```</span>
  Cosine = 5,
}

/// C++ type: <span style='color: green;'>```QTimeLine::Direction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Direction {
  /// C++ enum variant: <span style='color: green;'>```Forward = 0```</span>
  Forward = 0,
  /// C++ enum variant: <span style='color: green;'>```Backward = 1```</span>
  Backward = 1,
}

/// C++ type: <span style='color: green;'>```QTimeLine::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```NotRunning = 0```</span>
  NotRunning = 0,
  /// C++ enum variant: <span style='color: green;'>```Paused = 1```</span>
  Paused = 1,
  /// C++ enum variant: <span style='color: green;'>```Running = 2```</span>
  Running = 2,
}

/// C++ type: <span style='color: green;'>```QTimeLine```</span>
#[repr(C)]
pub struct TimeLine(u8);

impl TimeLine {
  /// C++ method: <span style='color: green;'>```int QTimeLine::currentFrame() const```</span>
  ///
  ///
  pub fn current_frame(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_currentFrame(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::currentTime() const```</span>
  ///
  ///
  pub fn current_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_currentTime(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```double QTimeLine::currentValue() const```</span>
  ///
  ///
  pub fn current_value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QTimeLine_currentValue(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```QTimeLine::CurveShape QTimeLine::curveShape() const```</span>
  ///
  ///
  pub fn curve_shape(&self) -> ::time_line::CurveShape {
    unsafe { ::ffi::qt_core_c_QTimeLine_curveShape(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```QTimeLine::Direction QTimeLine::direction() const```</span>
  ///
  ///
  pub fn direction(&self) -> ::time_line::Direction {
    unsafe { ::ffi::qt_core_c_QTimeLine_direction(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::duration() const```</span>
  ///
  ///
  pub fn duration(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_duration(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```QEasingCurve QTimeLine::easingCurve() const```</span>
  ///
  ///
  pub fn easing_curve(&self) -> ::easing_curve::EasingCurve {
    {
      let mut object: ::easing_curve::EasingCurve =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QTimeLine_easingCurve_to_output(self as *const ::time_line::TimeLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::endFrame() const```</span>
  ///
  ///
  pub fn end_frame(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_endFrame(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::frameForTime(int msec) const```</span>
  ///
  ///
  pub fn frame_for_time(&self, msec: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_frameForTime(self as *const ::time_line::TimeLine, msec) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::loopCount() const```</span>
  ///
  ///
  pub fn loop_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_loopCount(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QTimeLine::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QTimeLine_metaObject(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```QTimeLine::QTimeLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::time_line::TimeLine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeLine::QTimeLine()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::cpp_utils::CppBox<::time_line::TimeLine>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeLine::QTimeLine(int duration = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::time_line::TimeLine>
    where Args: overloading::TimeLineNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QTimeLine::QTimeLine(int duration = ?, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(duration: ::libc::c_int,
                           parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::time_line::TimeLine> {
    let ffi_result = ::ffi::qt_core_c_QTimeLine_new_duration_parent(duration, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::resume()```</span>
  ///
  ///
  pub fn resume(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeLine_resume(self as *mut ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::setCurrentTime(int msec)```</span>
  ///
  ///
  pub fn set_current_time(&mut self, msec: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setCurrentTime(self as *mut ::time_line::TimeLine, msec) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setCurveShape(QTimeLine::CurveShape shape)```</span>
  ///
  ///
  pub fn set_curve_shape(&mut self, shape: ::time_line::CurveShape) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setCurveShape(self as *mut ::time_line::TimeLine, shape) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setDirection(QTimeLine::Direction direction)```</span>
  ///
  ///
  pub fn set_direction(&mut self, direction: ::time_line::Direction) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setDirection(self as *mut ::time_line::TimeLine, direction) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setDuration(int duration)```</span>
  ///
  ///
  pub fn set_duration(&mut self, duration: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setDuration(self as *mut ::time_line::TimeLine, duration) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setEasingCurve(const QEasingCurve& curve)```</span>
  ///
  ///
  pub fn set_easing_curve(&mut self, curve: &::easing_curve::EasingCurve) {
    unsafe {
      ::ffi::qt_core_c_QTimeLine_setEasingCurve(self as *mut ::time_line::TimeLine,
                                                curve as *const ::easing_curve::EasingCurve)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setEndFrame(int frame)```</span>
  ///
  ///
  pub fn set_end_frame(&mut self, frame: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setEndFrame(self as *mut ::time_line::TimeLine, frame) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setFrameRange(int startFrame, int endFrame)```</span>
  ///
  ///
  pub fn set_frame_range(&mut self, start_frame: ::libc::c_int, end_frame: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setFrameRange(self as *mut ::time_line::TimeLine, start_frame, end_frame) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setLoopCount(int count)```</span>
  ///
  ///
  pub fn set_loop_count(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setLoopCount(self as *mut ::time_line::TimeLine, count) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::setPaused(bool paused)```</span>
  ///
  ///
  pub fn set_paused(&mut self, paused: bool) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setPaused(self as *mut ::time_line::TimeLine, paused) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setStartFrame(int frame)```</span>
  ///
  ///
  pub fn set_start_frame(&mut self, frame: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setStartFrame(self as *mut ::time_line::TimeLine, frame) }
  }

  /// C++ method: <span style='color: green;'>```void QTimeLine::setUpdateInterval(int interval)```</span>
  ///
  ///
  pub fn set_update_interval(&mut self, interval: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QTimeLine_setUpdateInterval(self as *mut ::time_line::TimeLine, interval) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::start()```</span>
  ///
  ///
  pub fn start(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeLine_start(self as *mut ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::startFrame() const```</span>
  ///
  ///
  pub fn start_frame(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_startFrame(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```QTimeLine::State QTimeLine::state() const```</span>
  ///
  ///
  pub fn state(&self) -> ::time_line::State {
    unsafe { ::ffi::qt_core_c_QTimeLine_state(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeLine_stop(self as *mut ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QTimeLine::toggleDirection()```</span>
  ///
  ///
  pub fn toggle_direction(&mut self) {
    unsafe { ::ffi::qt_core_c_QTimeLine_toggleDirection(self as *mut ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```static QString QTimeLine::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTimeLine_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QTimeLine::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QTimeLine_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTimeLine::updateInterval() const```</span>
  ///
  ///
  pub fn update_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimeLine_updateInterval(self as *const ::time_line::TimeLine) }
  }

  /// C++ method: <span style='color: green;'>```virtual double QTimeLine::valueForTime(int msec) const```</span>
  ///
  ///
  pub fn value_for_time(&self, msec: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QTimeLine_valueForTime(self as *const ::time_line::TimeLine, msec) }
  }
}

impl ::cpp_utils::CppDeletable for ::time_line::TimeLine {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTimeLine_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TimeLine`.
  pub struct Signals<'a>(&'a ::time_line::TimeLine);
  /// Represents a built-in Qt signal `QTimeLine::frameChanged`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.signals().frame_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct FrameChanged<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for FrameChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2frameChanged(int)\0"
    }
  }
  impl<'a> ::connection::Signal for FrameChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeLine::valueChanged`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.signals().value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct ValueChanged<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for ValueChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2valueChanged(double)\0"
    }
  }
  impl<'a> ::connection::Signal for ValueChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeLine::finished`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct Finished<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for Finished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished()\0"
    }
  }
  impl<'a> ::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QTimeLine::objectNameChanged`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct ObjectNameChanged<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QTimeLine::stateChanged`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.signals().state_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct StateChanged<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for StateChanged<'a> {
    type Arguments = (&'static ::time_line::State,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stateChanged(QTimeLine::State)\0"
    }
  }
  impl<'a> ::connection::Signal for StateChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QTimeLine::frameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn frame_changed(&self) -> FrameChanged {
      FrameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeLine::valueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn value_changed(&self) -> ValueChanged {
      ValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeLine::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeLine::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QTimeLine::stateChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn state_changed(&self) -> StateChanged {
      StateChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TimeLine`.
  pub struct Slots<'a>(&'a ::time_line::TimeLine);
  /// Represents a built-in Qt slot `QTimeLine::setCurrentTime`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().set_current_time()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct SetCurrentTime<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for SetCurrentTime<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setCurrentTime(int)\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeLine::stop`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().stop()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct Stop<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for Stop<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stop()\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeLine::resume`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().resume()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct Resume<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for Resume<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1resume()\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeLine::toggleDirection`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().toggle_direction()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct ToggleDirection<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for ToggleDirection<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1toggleDirection()\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeLine::start`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct Start<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start()\0"
    }
  }
  /// Represents a built-in Qt slot `QTimeLine::setPaused`.
  ///
  /// An object of this type can be created from `TimeLine` with `object.slots().set_paused()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TimeLine` object.
  pub struct SetPaused<'a>(&'a ::time_line::TimeLine);
  impl<'a> ::connection::Receiver for SetPaused<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPaused(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QTimeLine::setCurrentTime`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_current_time(&self) -> SetCurrentTime {
      SetCurrentTime(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeLine::stop`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop(&self) -> Stop {
      Stop(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeLine::resume`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resume(&self) -> Resume {
      Resume(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeLine::toggleDirection`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn toggle_direction(&self) -> ToggleDirection {
      ToggleDirection(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeLine::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QTimeLine::setPaused`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_paused(&self) -> SetPaused {
      SetPaused(self.0)
    }
  }
  impl ::time_line::TimeLine {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::time_line::TimeLine> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::time_line::TimeLine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_G_dynamic_cast_QTimeLine_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::time_line::TimeLine> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_G_dynamic_cast_QTimeLine_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::time_line::TimeLine {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimeLine_G_static_cast_QObject_ptr(self as *mut ::time_line::TimeLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_G_static_cast_QObject_ptr(self as *const ::time_line::TimeLine as *mut ::time_line::TimeLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::time_line::TimeLine> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::time_line::TimeLine {
    let ffi_result = ::ffi::qt_core_c_QTimeLine_G_static_cast_QTimeLine_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::time_line::TimeLine {
    let ffi_result = ::ffi::qt_core_c_QTimeLine_G_static_cast_QTimeLine_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::time_line::TimeLine {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_G_static_cast_QObject_ptr(self as *const ::time_line::TimeLine as *mut ::time_line::TimeLine) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::time_line::TimeLine {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimeLine_G_static_cast_QObject_ptr(self as *mut ::time_line::TimeLine) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TimeLine::new](../struct.TimeLine.html#method.new) method.
  pub trait TimeLineNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::time_line::TimeLine>;
  }
  impl TimeLineNewArgs for ::libc::c_int {
    fn exec(self) -> ::cpp_utils::CppBox<::time_line::TimeLine> {
      let duration = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_new_duration(duration) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TimeLineNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::time_line::TimeLine> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QTimeLine_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
