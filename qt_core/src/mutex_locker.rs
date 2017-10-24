/// C++ type: <span style='color: green;'>```QMutexLocker```</span>
#[repr(C)]
pub struct MutexLocker([u8; ::type_sizes::QT_CORE_MUTEX_LOCKER_MUTEX_LOCKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MutexLocker {
  unsafe fn new_uninitialized() -> MutexLocker {
    MutexLocker(::std::mem::uninitialized())
  }
}

impl MutexLocker {
  /// C++ method: <span style='color: green;'>```QMutex* QMutexLocker::mutex() const```</span>
  ///
  ///
  pub fn mutex(&self) -> *mut ::mutex::Mutex {
    unsafe { ::ffi::qt_core_c_QMutexLocker_mutex(self as *const ::mutex_locker::MutexLocker) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMutexLocker::QMutexLocker(QBasicMutex* m)```</span>
  ///
  ///
  pub unsafe fn new(m: *mut ::basic_mutex::BasicMutex) -> ::mutex_locker::MutexLocker {
    {
      let mut object: ::mutex_locker::MutexLocker =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMutexLocker_constructor(m, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMutexLocker::relock()```</span>
  ///
  ///
  pub fn relock(&mut self) {
    unsafe { ::ffi::qt_core_c_QMutexLocker_relock(self as *mut ::mutex_locker::MutexLocker) }
  }

  /// C++ method: <span style='color: green;'>```void QMutexLocker::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QMutexLocker_unlock(self as *mut ::mutex_locker::MutexLocker) }
  }
}

impl Drop for ::mutex_locker::MutexLocker {
  /// C++ method: <span style='color: green;'>```[destructor] void QMutexLocker::~QMutexLocker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMutexLocker_destructor(self as *mut ::mutex_locker::MutexLocker) }
  }
}
