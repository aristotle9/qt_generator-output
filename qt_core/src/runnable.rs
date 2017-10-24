/// C++ type: <span style='color: green;'>```QRunnable```</span>
#[repr(C)]
pub struct Runnable(u8);

impl Runnable {
  /// C++ method: <span style='color: green;'>```bool QRunnable::autoDelete() const```</span>
  ///
  ///
  pub fn auto_delete(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QRunnable_autoDelete(self as *const ::runnable::Runnable) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QRunnable::run()```</span>
  ///
  ///
  pub fn run(&mut self) {
    unsafe { ::ffi::qt_core_c_QRunnable_run(self as *mut ::runnable::Runnable) }
  }

  /// C++ method: <span style='color: green;'>```void QRunnable::setAutoDelete(bool _autoDelete)```</span>
  ///
  ///
  pub fn set_auto_delete(&mut self, auto_delete: bool) {
    unsafe { ::ffi::qt_core_c_QRunnable_setAutoDelete(self as *mut ::runnable::Runnable, auto_delete) }
  }
}

impl ::cpp_utils::CppDeletable for ::runnable::Runnable {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QRunnable_delete
  }
}
