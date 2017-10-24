/// C++ type: <span style='color: green;'>```QLockFile::LockError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LockError {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```LockFailedError = 1```</span>
  LockFailed = 1,
  /// C++ enum variant: <span style='color: green;'>```PermissionError = 2```</span>
  Permission = 2,
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 3```</span>
  Unknown = 3,
}

/// C++ type: <span style='color: green;'>```QLockFile```</span>
#[repr(C)]
pub struct LockFile([u8; ::type_sizes::QT_CORE_LOCK_FILE_LOCK_FILE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for LockFile {
  unsafe fn new_uninitialized() -> LockFile {
    LockFile(::std::mem::uninitialized())
  }
}

impl LockFile {
  /// C++ method: <span style='color: green;'>```QLockFile::LockError QLockFile::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::lock_file::LockError {
    unsafe { ::ffi::qt_core_c_QLockFile_error(self as *const ::lock_file::LockFile) }
  }

  /// C++ method: <span style='color: green;'>```bool QLockFile::getLockInfo(qint64* pid, QString* hostname, QString* appname) const```</span>
  ///
  ///
  pub unsafe fn get_lock_info(&self,
                              pid: *mut i64,
                              hostname: *mut ::string::String,
                              appname: *mut ::string::String)
                              -> bool {
    ::ffi::qt_core_c_QLockFile_getLockInfo(self as *const ::lock_file::LockFile, pid, hostname, appname)
  }

  /// C++ method: <span style='color: green;'>```bool QLockFile::isLocked() const```</span>
  ///
  ///
  pub fn is_locked(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLockFile_isLocked(self as *const ::lock_file::LockFile) }
  }

  /// C++ method: <span style='color: green;'>```bool QLockFile::lock()```</span>
  ///
  ///
  pub fn lock(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QLockFile_lock(self as *mut ::lock_file::LockFile) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QLockFile::QLockFile(const QString& fileName)```</span>
  ///
  ///
  pub fn new(file_name: &::string::String) -> ::lock_file::LockFile {
    {
      let mut object: ::lock_file::LockFile =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLockFile_constructor(file_name as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLockFile::removeStaleLockFile()```</span>
  ///
  ///
  pub fn remove_stale_lock_file(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QLockFile_removeStaleLockFile(self as *mut ::lock_file::LockFile) }
  }

  /// C++ method: <span style='color: green;'>```void QLockFile::setStaleLockTime(int arg1)```</span>
  ///
  ///
  pub fn set_stale_lock_time(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QLockFile_setStaleLockTime(self as *mut ::lock_file::LockFile, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QLockFile::staleLockTime() const```</span>
  ///
  ///
  pub fn stale_lock_time(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QLockFile_staleLockTime(self as *const ::lock_file::LockFile) }
  }

  /// C++ method: <span style='color: green;'>```QLockFile::tryLock```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn try_lock(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLockFile::tryLock()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn try_lock(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QLockFile::tryLock(int timeout = ?)```</span>
  ///
  ///
  pub fn try_lock<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::LockFileTryLockArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QLockFile::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QLockFile_unlock(self as *mut ::lock_file::LockFile) }
  }
}

impl Drop for ::lock_file::LockFile {
  /// C++ method: <span style='color: green;'>```[destructor] void QLockFile::~QLockFile()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLockFile_destructor(self as *mut ::lock_file::LockFile) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LockFile::try_lock](../struct.LockFile.html#method.try_lock) method.
  pub trait LockFileTryLockArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::lock_file::LockFile) -> bool;
  }
  impl<'largs> LockFileTryLockArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::lock_file::LockFile) -> bool {

      unsafe { ::ffi::qt_core_c_QLockFile_tryLock_no_args(original_self as *mut ::lock_file::LockFile) }
    }
  }
  impl<'largs> LockFileTryLockArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::lock_file::LockFile) -> bool {
      let timeout = self;
      unsafe { ::ffi::qt_core_c_QLockFile_tryLock_timeout(original_self as *mut ::lock_file::LockFile, timeout) }
    }
  }
}
