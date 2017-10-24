/// C++ type: <span style='color: green;'>```QFuture<void>```</span>
#[repr(C)]
pub struct FutureEmpty([u8; ::type_sizes::QT_CORE_FUTURE_FUTURE_EMPTY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for FutureEmpty {
  unsafe fn new_uninitialized() -> FutureEmpty {
    FutureEmpty(::std::mem::uninitialized())
  }
}

impl FutureEmpty {
  /// C++ method: <span style='color: green;'>```void QFuture<void>::cancel()```</span>
  ///
  ///
  pub fn cancel(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_cancel(self as *mut ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::isCanceled() const```</span>
  ///
  ///
  pub fn is_canceled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFuture_void_isCanceled(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::isFinished() const```</span>
  ///
  ///
  pub fn is_finished(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFuture_void_isFinished(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::isPaused() const```</span>
  ///
  ///
  pub fn is_paused(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFuture_void_isPaused(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::isRunning() const```</span>
  ///
  ///
  pub fn is_running(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFuture_void_isRunning(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::isStarted() const```</span>
  ///
  ///
  pub fn is_started(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QFuture_void_isStarted(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QFuture<void>::QFuture()```</span>
  ///
  ///
  pub fn new() -> ::future::FutureEmpty {
    {
      let mut object: ::future::FutureEmpty =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFuture_void_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::operator==(const QFuture<void>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::future::FutureEmpty) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFuture_void_operator_eq(self as *const ::future::FutureEmpty,
                                                other as *const ::future::FutureEmpty)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFuture<void>::operator!=(const QFuture<void>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::future::FutureEmpty) -> bool {
    unsafe {
      ::ffi::qt_core_c_QFuture_void_operator_neq(self as *const ::future::FutureEmpty,
                                                 other as *const ::future::FutureEmpty)
    }
  }

  /// C++ method: <span style='color: green;'>```void QFuture<void>::pause()```</span>
  ///
  ///
  pub fn pause(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_pause(self as *mut ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```int QFuture<void>::progressMaximum() const```</span>
  ///
  ///
  pub fn progress_maximum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QFuture_void_progressMaximum(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```int QFuture<void>::progressMinimum() const```</span>
  ///
  ///
  pub fn progress_minimum(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QFuture_void_progressMinimum(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```QString QFuture<void>::progressText() const```</span>
  ///
  ///
  pub fn progress_text(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QFuture_void_progressText_to_output(self as *const ::future::FutureEmpty, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFuture<void>::progressValue() const```</span>
  ///
  ///
  pub fn progress_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QFuture_void_progressValue(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```int QFuture<void>::resultCount() const```</span>
  ///
  ///
  pub fn result_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QFuture_void_resultCount(self as *const ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```void QFuture<void>::resume()```</span>
  ///
  ///
  pub fn resume(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_resume(self as *mut ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```void QFuture<void>::setPaused(bool paused)```</span>
  ///
  ///
  pub fn set_paused(&mut self, paused: bool) {
    unsafe { ::ffi::qt_core_c_QFuture_void_setPaused(self as *mut ::future::FutureEmpty, paused) }
  }

  /// C++ method: <span style='color: green;'>```void QFuture<void>::togglePaused()```</span>
  ///
  ///
  pub fn toggle_paused(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_togglePaused(self as *mut ::future::FutureEmpty) }
  }

  /// C++ method: <span style='color: green;'>```void QFuture<void>::waitForFinished()```</span>
  ///
  ///
  pub fn wait_for_finished(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_waitForFinished(self as *mut ::future::FutureEmpty) }
  }
}

impl Drop for ::future::FutureEmpty {
  /// C++ method: <span style='color: green;'>```[destructor] void QFuture<void>::~QFuture()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QFuture_void_destructor(self as *mut ::future::FutureEmpty) }
  }
}
