/// C++ type: <span style='color: green;'>```QElapsedTimer::ClockType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ClockType {
  /// C++ enum variant: <span style='color: green;'>```SystemTime = 0```</span>
  SystemTime = 0,
  /// C++ enum variant: <span style='color: green;'>```MonotonicClock = 1```</span>
  MonotonicClock = 1,
  /// C++ enum variant: <span style='color: green;'>```TickCounter = 2```</span>
  TickCounter = 2,
  /// C++ enum variant: <span style='color: green;'>```MachAbsoluteTime = 3```</span>
  MachAbsoluteTime = 3,
  /// C++ enum variant: <span style='color: green;'>```PerformanceCounter = 4```</span>
  PerformanceCounter = 4,
}

/// C++ type: <span style='color: green;'>```QElapsedTimer```</span>
#[repr(C)]
pub struct ElapsedTimer([u8; ::type_sizes::QT_CORE_ELAPSED_TIMER_ELAPSED_TIMER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ElapsedTimer {
  unsafe fn new_uninitialized() -> ElapsedTimer {
    ElapsedTimer(::std::mem::uninitialized())
  }
}

impl ElapsedTimer {
  /// C++ method: <span style='color: green;'>```static QElapsedTimer::ClockType QElapsedTimer::clockType()```</span>
  ///
  ///
  pub fn clock_type() -> ::elapsed_timer::ClockType {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_clockType() }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::elapsed() const```</span>
  ///
  ///
  pub fn elapsed(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_elapsed(self as *const ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```bool QElapsedTimer::hasExpired(qint64 timeout) const```</span>
  ///
  ///
  pub fn has_expired(&self, timeout: i64) -> bool {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_hasExpired(self as *const ::elapsed_timer::ElapsedTimer, timeout) }
  }

  /// C++ method: <span style='color: green;'>```void QElapsedTimer::invalidate()```</span>
  ///
  ///
  pub fn invalidate(&mut self) {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_invalidate(self as *mut ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```static bool QElapsedTimer::isMonotonic()```</span>
  ///
  ///
  pub fn is_monotonic() -> bool {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_isMonotonic() }
  }

  /// C++ method: <span style='color: green;'>```bool QElapsedTimer::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_isValid(self as *const ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::msecsSinceReference() const```</span>
  ///
  ///
  pub fn msecs_since_reference(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_msecsSinceReference(self as *const ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::msecsTo(const QElapsedTimer& other) const```</span>
  ///
  ///
  pub fn msecs_to(&self, other: &::elapsed_timer::ElapsedTimer) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QElapsedTimer_msecsTo(self as *const ::elapsed_timer::ElapsedTimer,
                                             other as *const ::elapsed_timer::ElapsedTimer)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QElapsedTimer::QElapsedTimer()```</span>
  ///
  ///
  pub fn new() -> ::elapsed_timer::ElapsedTimer {
    {
      let mut object: ::elapsed_timer::ElapsedTimer =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QElapsedTimer_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::nsecsElapsed() const```</span>
  ///
  ///
  pub fn nsecs_elapsed(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_nsecsElapsed(self as *const ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```bool QElapsedTimer::operator==(const QElapsedTimer& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::elapsed_timer::ElapsedTimer) -> bool {
    unsafe {
      ::ffi::qt_core_c_QElapsedTimer_operator_eq(self as *const ::elapsed_timer::ElapsedTimer,
                                                 other as *const ::elapsed_timer::ElapsedTimer)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QElapsedTimer::operator!=(const QElapsedTimer& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::elapsed_timer::ElapsedTimer) -> bool {
    unsafe {
      ::ffi::qt_core_c_QElapsedTimer_operator_neq(self as *const ::elapsed_timer::ElapsedTimer,
                                                  other as *const ::elapsed_timer::ElapsedTimer)
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::restart()```</span>
  ///
  ///
  pub fn restart(&mut self) -> i64 {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_restart(self as *mut ::elapsed_timer::ElapsedTimer) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QElapsedTimer::secsTo(const QElapsedTimer& other) const```</span>
  ///
  ///
  pub fn secs_to(&self, other: &::elapsed_timer::ElapsedTimer) -> i64 {
    unsafe {
      ::ffi::qt_core_c_QElapsedTimer_secsTo(self as *const ::elapsed_timer::ElapsedTimer,
                                            other as *const ::elapsed_timer::ElapsedTimer)
    }
  }

  /// C++ method: <span style='color: green;'>```void QElapsedTimer::start()```</span>
  ///
  ///
  pub fn start(&mut self) {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_start(self as *mut ::elapsed_timer::ElapsedTimer) }
  }
}

impl Drop for ::elapsed_timer::ElapsedTimer {
  /// C++ method: <span style='color: green;'>```[destructor] void QElapsedTimer::~QElapsedTimer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QElapsedTimer_destructor(self as *mut ::elapsed_timer::ElapsedTimer) }
  }
}
