/// C++ type: <span style='color: green;'>```QWriteLocker```</span>
#[repr(C)]
pub struct WriteLocker([u8; ::type_sizes::QT_CORE_WRITE_LOCKER_WRITE_LOCKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for WriteLocker {
  unsafe fn new_uninitialized() -> WriteLocker {
    WriteLocker(::std::mem::uninitialized())
  }
}

impl WriteLocker {
  /// C++ method: <span style='color: green;'>```[constructor] void QWriteLocker::QWriteLocker(QReadWriteLock* readWriteLock)```</span>
  ///
  ///
  pub unsafe fn new(read_write_lock: *mut ::read_write_lock::ReadWriteLock) -> ::write_locker::WriteLocker {
    {
      let mut object: ::write_locker::WriteLocker =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QWriteLocker_constructor(read_write_lock, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QReadWriteLock* QWriteLocker::readWriteLock() const```</span>
  ///
  ///
  pub fn read_write_lock(&self) -> *mut ::read_write_lock::ReadWriteLock {
    unsafe { ::ffi::qt_core_c_QWriteLocker_readWriteLock(self as *const ::write_locker::WriteLocker) }
  }

  /// C++ method: <span style='color: green;'>```void QWriteLocker::relock()```</span>
  ///
  ///
  pub fn relock(&mut self) {
    unsafe { ::ffi::qt_core_c_QWriteLocker_relock(self as *mut ::write_locker::WriteLocker) }
  }

  /// C++ method: <span style='color: green;'>```void QWriteLocker::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QWriteLocker_unlock(self as *mut ::write_locker::WriteLocker) }
  }
}

impl Drop for ::write_locker::WriteLocker {
  /// C++ method: <span style='color: green;'>```[destructor] void QWriteLocker::~QWriteLocker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QWriteLocker_destructor(self as *mut ::write_locker::WriteLocker) }
  }
}
