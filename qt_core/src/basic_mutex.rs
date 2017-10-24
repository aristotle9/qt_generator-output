/// C++ type: <span style='color: green;'>```QBasicMutex```</span>
#[repr(C)]
pub struct BasicMutex(u8);

impl BasicMutex {
  /// C++ method: <span style='color: green;'>```bool QBasicMutex::isRecursive() const```</span>
  ///
  ///
  pub fn is_recursive(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QBasicMutex_isRecursive_const(self as *const ::basic_mutex::BasicMutex) }
  }

  /// C++ method: <span style='color: green;'>```bool QBasicMutex::isRecursive()```</span>
  ///
  ///
  pub fn is_recursive_mut(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QBasicMutex_isRecursive(self as *mut ::basic_mutex::BasicMutex) }
  }

  /// C++ method: <span style='color: green;'>```void QBasicMutex::lock()```</span>
  ///
  ///
  pub fn lock(&mut self) {
    unsafe { ::ffi::qt_core_c_QBasicMutex_lock(self as *mut ::basic_mutex::BasicMutex) }
  }

  /// C++ method: <span style='color: green;'>```bool QBasicMutex::tryLock()```</span>
  ///
  ///
  pub fn try_lock0(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QBasicMutex_tryLock(self as *mut ::basic_mutex::BasicMutex) }
  }

  /// C++ method: <span style='color: green;'>```bool QBasicMutex::try_lock()```</span>
  ///
  ///
  pub fn try_lock1(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QBasicMutex_try_lock(self as *mut ::basic_mutex::BasicMutex) }
  }

  /// C++ method: <span style='color: green;'>```void QBasicMutex::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QBasicMutex_unlock(self as *mut ::basic_mutex::BasicMutex) }
  }
}

impl ::cpp_utils::CppDeletable for ::basic_mutex::BasicMutex {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QBasicMutex_delete
  }
}
