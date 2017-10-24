/// C++ type: <span style='color: green;'>```QReadWriteLock```</span>
#[repr(C)]
pub struct ReadWriteLock(u8);

impl ReadWriteLock {
  /// C++ method: <span style='color: green;'>```void QReadWriteLock::lockForRead()```</span>
  ///
  ///
  pub fn lock_for_read(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadWriteLock_lockForRead(self as *mut ::read_write_lock::ReadWriteLock) }
  }

  /// C++ method: <span style='color: green;'>```void QReadWriteLock::lockForWrite()```</span>
  ///
  ///
  pub fn lock_for_write(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadWriteLock_lockForWrite(self as *mut ::read_write_lock::ReadWriteLock) }
  }

  /// C++ method: <span style='color: green;'>```QReadWriteLock::QReadWriteLock```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QReadWriteLock::QReadWriteLock()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::read_write_lock::RecursionMode) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QReadWriteLock::QReadWriteLock(QReadWriteLock::RecursionMode recursionMode = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock>
    where Args: overloading::ReadWriteLockNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QReadWriteLock::tryLockForRead```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn try_lock_for_read(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QReadWriteLock::tryLockForRead()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn try_lock_for_read(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QReadWriteLock::tryLockForRead(int timeout)```</span>
  ///
  ///
  pub fn try_lock_for_read<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ReadWriteLockTryLockForReadArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QReadWriteLock::tryLockForWrite```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn try_lock_for_write(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QReadWriteLock::tryLockForWrite()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn try_lock_for_write(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QReadWriteLock::tryLockForWrite(int timeout)```</span>
  ///
  ///
  pub fn try_lock_for_write<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ReadWriteLockTryLockForWriteArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QReadWriteLock::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QReadWriteLock_unlock(self as *mut ::read_write_lock::ReadWriteLock) }
  }
}

impl ::cpp_utils::CppDeletable for ::read_write_lock::ReadWriteLock {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QReadWriteLock_delete
  }
}

/// C++ type: <span style='color: green;'>```QReadWriteLock::RecursionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RecursionMode {
  /// C++ enum variant: <span style='color: green;'>```NonRecursive = 0```</span>
  NonRecursive = 0,
  /// C++ enum variant: <span style='color: green;'>```Recursive = 1```</span>
  Recursive = 1,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ReadWriteLock::new](../struct.ReadWriteLock.html#method.new) method.
  pub trait ReadWriteLockNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock>;
  }
  impl ReadWriteLockNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QReadWriteLock_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ReadWriteLockNewArgs for ::read_write_lock::RecursionMode {
    fn exec(self) -> ::cpp_utils::CppBox<::read_write_lock::ReadWriteLock> {
      let recursion_mode = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QReadWriteLock_new_recursionMode(recursion_mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [ReadWriteLock::try_lock_for_read](../struct.ReadWriteLock.html#method.try_lock_for_read) method.
  pub trait ReadWriteLockTryLockForReadArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool;
  }
  impl<'largs> ReadWriteLockTryLockForReadArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool {

      unsafe {
        ::ffi::qt_core_c_QReadWriteLock_tryLockForRead_no_args(original_self as *mut ::read_write_lock::ReadWriteLock)
      }
    }
  }
  impl<'largs> ReadWriteLockTryLockForReadArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool {
      let timeout = self;
      unsafe {
        ::ffi::qt_core_c_QReadWriteLock_tryLockForRead_timeout(original_self as *mut ::read_write_lock::ReadWriteLock,
                                                               timeout)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ReadWriteLock::try_lock_for_write](../struct.ReadWriteLock.html#method.try_lock_for_write) method.
  pub trait ReadWriteLockTryLockForWriteArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool;
  }
  impl<'largs> ReadWriteLockTryLockForWriteArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool {

      unsafe {
        ::ffi::qt_core_c_QReadWriteLock_tryLockForWrite_no_args(original_self as *mut ::read_write_lock::ReadWriteLock)
      }
    }
  }
  impl<'largs> ReadWriteLockTryLockForWriteArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::read_write_lock::ReadWriteLock) -> bool {
      let timeout = self;
      unsafe {
        ::ffi::qt_core_c_QReadWriteLock_tryLockForWrite_timeout(original_self as *mut ::read_write_lock::ReadWriteLock, timeout)
      }
    }
  }
}
