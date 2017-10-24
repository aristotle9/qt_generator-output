/// C++ type: <span style='color: green;'>```QReadLocker```</span>
#[repr(C)]
pub struct ReadLocker([u8; ::type_sizes::QT_CORE_READ_LOCKER_READ_LOCKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ReadLocker {
  unsafe fn new_uninitialized() -> ReadLocker {
    ReadLocker(::std::mem::uninitialized())
  }
}

impl ReadLocker {
  /// C++ method: <span style='color: green;'>```[constructor] void QReadLocker::QReadLocker(QReadWriteLock* readWriteLock)```</span>
  ///
  ///
  pub unsafe fn new(read_write_lock: *mut ::read_write_lock::ReadWriteLock) -> ::read_locker::ReadLocker {
    {
      let mut object: ::read_locker::ReadLocker =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QReadLocker_constructor(read_write_lock, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QReadWriteLock* QReadLocker::readWriteLock() const```</span>
  ///
  ///
  pub fn read_write_lock(&self) -> *mut ::read_write_lock::ReadWriteLock {
    unsafe { ::ffi::qt_core_c_QReadLocker_readWriteLock(self as *const ::read_locker::ReadLocker) }
  }

  /// C++ method: <span style='color: green;'>```void QReadLocker::relock()```</span>
  ///
  ///
  pub fn relock(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadLocker_relock(self as *mut ::read_locker::ReadLocker) }
  }

  /// C++ method: <span style='color: green;'>```void QReadLocker::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadLocker_unlock(self as *mut ::read_locker::ReadLocker) }
  }
}

impl Drop for ::read_locker::ReadLocker {
  /// C++ method: <span style='color: green;'>```[destructor] void QReadLocker::~QReadLocker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadLocker_destructor(self as *mut ::read_locker::ReadLocker) }
  }
}
