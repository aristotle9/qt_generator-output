/// C++ type: <span style='color: green;'>```QBasicTimer```</span>
#[repr(C)]
pub struct BasicTimer(u8);

impl BasicTimer {
  /// C++ method: <span style='color: green;'>```bool QBasicTimer::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBasicTimer_isActive(self as *const ::basic_timer::BasicTimer) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QBasicTimer::QBasicTimer()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::basic_timer::BasicTimer> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QBasicTimer_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QBasicTimer::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, (::libc::c_int, *mut ::object::Object)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBasicTimer::start(int msec, QObject* obj)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, (::libc::c_int, &::qt::TimerType, *mut ::object::Object)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QBasicTimer::start(int msec, Qt::TimerType timerType, QObject* obj)```</span>
  ///
  ///
  pub unsafe fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::BasicTimerStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QBasicTimer::stop()```</span>
  ///
  ///
  pub fn stop(&mut self) {
    unsafe { ::ffi::qt_core_c_QBasicTimer_stop(self as *mut ::basic_timer::BasicTimer) }
  }

  /// C++ method: <span style='color: green;'>```int QBasicTimer::timerId() const```</span>
  ///
  ///
  pub fn timer_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QBasicTimer_timerId(self as *const ::basic_timer::BasicTimer) }
  }
}

impl ::cpp_utils::CppDeletable for ::basic_timer::BasicTimer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QBasicTimer_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [BasicTimer::start](../struct.BasicTimer.html#method.start) method.
  pub trait BasicTimerStartArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::basic_timer::BasicTimer) -> ();
  }
  impl<'largs> BasicTimerStartArgs<'largs> for (::libc::c_int, *mut ::object::Object) {
    unsafe fn exec(self, original_self: &'largs mut ::basic_timer::BasicTimer) -> () {
      let msec = self.0;
      let obj = self.1;
      ::ffi::qt_core_c_QBasicTimer_start_msec_obj(original_self as *mut ::basic_timer::BasicTimer, msec, obj)
    }
  }
  impl<'largs> BasicTimerStartArgs<'largs> for (::libc::c_int, &'largs ::qt::TimerType, *mut ::object::Object) {
    unsafe fn exec(self, original_self: &'largs mut ::basic_timer::BasicTimer) -> () {
      let msec = self.0;
      let timer_type = self.1;
      let obj = self.2;
      ::ffi::qt_core_c_QBasicTimer_start_msec_timerType_obj(original_self as *mut ::basic_timer::BasicTimer,
                                                            msec,
                                                            timer_type as *const ::qt::TimerType,
                                                            obj)
    }
  }
}
