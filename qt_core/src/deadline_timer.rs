/// C++ type: <span style='color: green;'>```QDeadlineTimer```</span>
#[repr(C)]
pub struct DeadlineTimer([u8; ::type_sizes::QT_CORE_DEADLINE_TIMER_DEADLINE_TIMER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for DeadlineTimer {
  unsafe fn new_uninitialized() -> DeadlineTimer {
    DeadlineTimer(::std::mem::uninitialized())
  }
}

impl DeadlineTimer {
  /// C++ method: <span style='color: green;'>```static QDeadlineTimer QDeadlineTimer::addNSecs(QDeadlineTimer dt, qint64 nsecs)```</span>
  ///
  ///
  pub fn add_n_secs(dt: &::deadline_timer::DeadlineTimer, nsecs: i64) -> ::deadline_timer::DeadlineTimer {
    {
      let mut object: ::deadline_timer::DeadlineTimer =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDeadlineTimer_addNSecs_to_output(dt as *const ::deadline_timer::DeadlineTimer,
                                                           nsecs,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QDeadlineTimer::current```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn current(()) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```static QDeadlineTimer QDeadlineTimer::current()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn current(&::qt::TimerType) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```static QDeadlineTimer QDeadlineTimer::current(Qt::TimerType timerType = ?)```</span>
  ///
  ///
  pub fn current<Args>(args: Args) -> ::deadline_timer::DeadlineTimer
    where Args: overloading::DeadlineTimerCurrentArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```qint64 QDeadlineTimer::deadline() const```</span>
  ///
  ///
  pub fn deadline(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_deadline(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDeadlineTimer::deadlineNSecs() const```</span>
  ///
  ///
  pub fn deadline_n_secs(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_deadlineNSecs(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```bool QDeadlineTimer::hasExpired() const```</span>
  ///
  ///
  pub fn has_expired(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_hasExpired(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```bool QDeadlineTimer::isForever() const```</span>
  ///
  ///
  pub fn is_forever(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_isForever(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```QDeadlineTimer::QDeadlineTimer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::deadline_timer::ForeverConstant) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer(QDeadlineTimer::ForeverConstant arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::deadline_timer::ForeverConstant, &::qt::TimerType)) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer(QDeadlineTimer::ForeverConstant arg1, Qt::TimerType type_ = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt::TimerType) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer(Qt::TimerType type_ = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(i64) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer(qint64 msecs)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((i64, &::qt::TimerType)) -> ::deadline_timer::DeadlineTimer```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDeadlineTimer::QDeadlineTimer(qint64 msecs, Qt::TimerType type = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::deadline_timer::DeadlineTimer
    where Args: overloading::DeadlineTimerNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDeadlineTimer& QDeadlineTimer::operator+=(qint64 msecs)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, msecs: i64) -> &'l0 mut ::deadline_timer::DeadlineTimer {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QDeadlineTimer_operator_add_assign(self as *mut ::deadline_timer::DeadlineTimer, msecs)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDeadlineTimer& QDeadlineTimer::operator-=(qint64 msecs)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, msecs: i64) -> &'l0 mut ::deadline_timer::DeadlineTimer {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QDeadlineTimer_operator_sub_assign(self as *mut ::deadline_timer::DeadlineTimer, msecs)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```qint64 QDeadlineTimer::remainingTime() const```</span>
  ///
  ///
  pub fn remaining_time(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_remainingTime(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QDeadlineTimer::remainingTimeNSecs() const```</span>
  ///
  ///
  pub fn remaining_time_n_secs(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_remainingTimeNSecs(self as *const ::deadline_timer::DeadlineTimer) }
  }

  /// C++ method: <span style='color: green;'>```QDeadlineTimer::setDeadline```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_deadline(&mut self, i64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setDeadline(qint64 msecs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_deadline(&mut self, (i64, &::qt::TimerType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setDeadline(qint64 msecs, Qt::TimerType timerType = ?)```</span>
  ///
  ///
  pub fn set_deadline<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DeadlineTimerSetDeadlineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDeadlineTimer::setPreciseDeadline```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_precise_deadline(&mut self, i64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseDeadline(qint64 secs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_precise_deadline(&mut self, (i64, i64)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseDeadline(qint64 secs, qint64 nsecs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_precise_deadline(&mut self, (i64, i64, &::qt::TimerType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseDeadline(qint64 secs, qint64 nsecs = ?, Qt::TimerType type = ?)```</span>
  ///
  ///
  pub fn set_precise_deadline<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DeadlineTimerSetPreciseDeadlineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDeadlineTimer::setPreciseRemainingTime```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_precise_remaining_time(&mut self, i64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseRemainingTime(qint64 secs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_precise_remaining_time(&mut self, (i64, i64)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseRemainingTime(qint64 secs, qint64 nsecs = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_precise_remaining_time(&mut self, (i64, i64, &::qt::TimerType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setPreciseRemainingTime(qint64 secs, qint64 nsecs = ?, Qt::TimerType type = ?)```</span>
  ///
  ///
  pub fn set_precise_remaining_time<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DeadlineTimerSetPreciseRemainingTimeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDeadlineTimer::setRemainingTime```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_remaining_time(&mut self, i64) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setRemainingTime(qint64 msecs)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_remaining_time(&mut self, (i64, &::qt::TimerType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setRemainingTime(qint64 msecs, Qt::TimerType type = ?)```</span>
  ///
  ///
  pub fn set_remaining_time<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DeadlineTimerSetRemainingTimeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::setTimerType(Qt::TimerType type)```</span>
  ///
  ///
  pub fn set_timer_type(&mut self, type_: &::qt::TimerType) {
    unsafe {
      ::ffi::qt_core_c_QDeadlineTimer_setTimerType(self as *mut ::deadline_timer::DeadlineTimer,
                                                   type_ as *const ::qt::TimerType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QDeadlineTimer::swap(QDeadlineTimer& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::deadline_timer::DeadlineTimer) {
    unsafe {
      ::ffi::qt_core_c_QDeadlineTimer_swap(self as *mut ::deadline_timer::DeadlineTimer,
                                           other as *mut ::deadline_timer::DeadlineTimer)
    }
  }
}

impl Drop for ::deadline_timer::DeadlineTimer {
  /// C++ method: <span style='color: green;'>```[destructor] void QDeadlineTimer::~QDeadlineTimer()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QDeadlineTimer_destructor(self as *mut ::deadline_timer::DeadlineTimer) }
  }
}

/// C++ type: <span style='color: green;'>```QDeadlineTimer::ForeverConstant```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ForeverConstant {
  /// C++ enum variant: <span style='color: green;'>```Forever = 0```</span>
  Forever = 0,
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 1,
}

/// C++ method: <span style='color: green;'>```void swap(QDeadlineTimer& value1, QDeadlineTimer& value2)```</span>
///
///
pub fn swap(value1: &mut ::deadline_timer::DeadlineTimer, value2: &mut ::deadline_timer::DeadlineTimer) {
  unsafe {
    ::ffi::qt_core_c_QDeadlineTimer_G_swap(value1 as *mut ::deadline_timer::DeadlineTimer,
                                           value2 as *mut ::deadline_timer::DeadlineTimer)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DeadlineTimer::current](../struct.DeadlineTimer.html#method.current) method.
  pub trait DeadlineTimerCurrentArgs {
    fn exec(self) -> ::deadline_timer::DeadlineTimer;
  }
  impl DeadlineTimerCurrentArgs for () {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {

      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_current_to_output_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> DeadlineTimerCurrentArgs for &'a ::qt::TimerType {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let timer_type = self;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_current_to_output_timerType(timer_type as *const ::qt::TimerType,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DeadlineTimer::new](../struct.DeadlineTimer.html#method.new) method.
  pub trait DeadlineTimerNewArgs {
    fn exec(self) -> ::deadline_timer::DeadlineTimer;
  }
  impl DeadlineTimerNewArgs for ::deadline_timer::ForeverConstant {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let arg1 = self;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_arg1(arg1, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DeadlineTimerNewArgs for (::deadline_timer::ForeverConstant, &'a ::qt::TimerType) {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let arg1 = self.0;
      let type_ = self.1;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_arg1_type_(arg1, type_ as *const ::qt::TimerType, &mut object);
        }
        object
      }
    }
  }
  impl DeadlineTimerNewArgs for i64 {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let msecs = self;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_msecs(msecs, &mut object);
        }
        object
      }
    }
  }
  impl<'a> DeadlineTimerNewArgs for (i64, &'a ::qt::TimerType) {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let msecs = self.0;
      let type_ = self.1;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_msecs_type(msecs, type_ as *const ::qt::TimerType, &mut object);
        }
        object
      }
    }
  }
  impl DeadlineTimerNewArgs for () {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {

      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> DeadlineTimerNewArgs for &'a ::qt::TimerType {
    fn exec(self) -> ::deadline_timer::DeadlineTimer {
      let type_ = self;
      {
        let mut object: ::deadline_timer::DeadlineTimer =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QDeadlineTimer_constructor_type_(type_ as *const ::qt::TimerType, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [DeadlineTimer::set_deadline](../struct.DeadlineTimer.html#method.set_deadline) method.
  pub trait DeadlineTimerSetDeadlineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> ();
  }
  impl<'largs> DeadlineTimerSetDeadlineArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let msecs = self;
      unsafe {
        ::ffi::qt_core_c_QDeadlineTimer_setDeadline_msecs(original_self as *mut ::deadline_timer::DeadlineTimer, msecs)
      }
    }
  }
  impl<'largs> DeadlineTimerSetDeadlineArgs<'largs> for (i64, &'largs ::qt::TimerType) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let msecs = self.0;
      let timer_type = self.1;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setDeadline_msecs_timerType(original_self as *mut ::deadline_timer::DeadlineTimer, msecs, timer_type as *const ::qt::TimerType) }
    }
  }
  /// This trait represents a set of arguments accepted by [DeadlineTimer::set_precise_deadline](../struct.DeadlineTimer.html#method.set_precise_deadline) method.
  pub trait DeadlineTimerSetPreciseDeadlineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> ();
  }
  impl<'largs> DeadlineTimerSetPreciseDeadlineArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self;
      unsafe {
        ::ffi::qt_core_c_QDeadlineTimer_setPreciseDeadline_secs(original_self as *mut ::deadline_timer::DeadlineTimer,
                                                                secs)
      }
    }
  }
  impl<'largs> DeadlineTimerSetPreciseDeadlineArgs<'largs> for (i64, i64) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self.0;
      let nsecs = self.1;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs(original_self as *mut ::deadline_timer::DeadlineTimer, secs, nsecs) }
    }
  }
  impl<'largs> DeadlineTimerSetPreciseDeadlineArgs<'largs> for (i64, i64, &'largs ::qt::TimerType) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self.0;
      let nsecs = self.1;
      let type_ = self.2;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs_type(original_self as *mut ::deadline_timer::DeadlineTimer, secs, nsecs, type_ as *const ::qt::TimerType) }
    }
  }
  /// This trait represents a set of arguments accepted by [DeadlineTimer::set_precise_remaining_time](../struct.DeadlineTimer.html#method.set_precise_remaining_time) method.
  pub trait DeadlineTimerSetPreciseRemainingTimeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> ();
  }
  impl<'largs> DeadlineTimerSetPreciseRemainingTimeArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs(original_self as *mut ::deadline_timer::DeadlineTimer, secs) }
    }
  }
  impl<'largs> DeadlineTimerSetPreciseRemainingTimeArgs<'largs> for (i64, i64) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self.0;
      let nsecs = self.1;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs(original_self as *mut ::deadline_timer::DeadlineTimer, secs, nsecs) }
    }
  }
  impl<'largs> DeadlineTimerSetPreciseRemainingTimeArgs<'largs> for (i64, i64, &'largs ::qt::TimerType) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let secs = self.0;
      let nsecs = self.1;
      let type_ = self.2;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs_type(original_self as *mut ::deadline_timer::DeadlineTimer, secs, nsecs, type_ as *const ::qt::TimerType) }
    }
  }
  /// This trait represents a set of arguments accepted by [DeadlineTimer::set_remaining_time](../struct.DeadlineTimer.html#method.set_remaining_time) method.
  pub trait DeadlineTimerSetRemainingTimeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> ();
  }
  impl<'largs> DeadlineTimerSetRemainingTimeArgs<'largs> for i64 {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let msecs = self;
      unsafe {
        ::ffi::qt_core_c_QDeadlineTimer_setRemainingTime_msecs(original_self as *mut ::deadline_timer::DeadlineTimer,
                                                               msecs)
      }
    }
  }
  impl<'largs> DeadlineTimerSetRemainingTimeArgs<'largs> for (i64, &'largs ::qt::TimerType) {
    fn exec(self, original_self: &'largs mut ::deadline_timer::DeadlineTimer) -> () {
      let msecs = self.0;
      let type_ = self.1;
      unsafe { ::ffi::qt_core_c_QDeadlineTimer_setRemainingTime_msecs_type(original_self as *mut ::deadline_timer::DeadlineTimer, msecs, type_ as *const ::qt::TimerType) }
    }
  }
}
