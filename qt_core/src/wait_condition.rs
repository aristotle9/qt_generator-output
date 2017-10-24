/// C++ type: <span style='color: green;'>```QWaitCondition```</span>
#[repr(C)]
pub struct WaitCondition([u8; ::type_sizes::QT_CORE_WAIT_CONDITION_WAIT_CONDITION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for WaitCondition {
  unsafe fn new_uninitialized() -> WaitCondition {
    WaitCondition(::std::mem::uninitialized())
  }
}

impl WaitCondition {
  /// C++ method: <span style='color: green;'>```[constructor] void QWaitCondition::QWaitCondition()```</span>
  ///
  ///
  pub fn new() -> ::wait_condition::WaitCondition {
    {
      let mut object: ::wait_condition::WaitCondition =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QWaitCondition_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QWaitCondition::notify_all()```</span>
  ///
  ///
  pub fn notify_all(&mut self) {
    unsafe { ::ffi::qt_core_c_QWaitCondition_notify_all(self as *mut ::wait_condition::WaitCondition) }
  }

  /// C++ method: <span style='color: green;'>```void QWaitCondition::notify_one()```</span>
  ///
  ///
  pub fn notify_one(&mut self) {
    unsafe { ::ffi::qt_core_c_QWaitCondition_notify_one(self as *mut ::wait_condition::WaitCondition) }
  }

  /// C++ method: <span style='color: green;'>```QWaitCondition::wait```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait(&mut self, *mut ::mutex::Mutex) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWaitCondition::wait(QMutex* lockedMutex)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait(&mut self, (*mut ::mutex::Mutex, ::libc::c_ulong)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWaitCondition::wait(QMutex* lockedMutex, unsigned long time = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn wait(&mut self, *mut ::read_write_lock::ReadWriteLock) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWaitCondition::wait(QReadWriteLock* lockedReadWriteLock)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn wait(&mut self, (*mut ::read_write_lock::ReadWriteLock, ::libc::c_ulong)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QWaitCondition::wait(QReadWriteLock* lockedReadWriteLock, unsigned long time = ?)```</span>
  ///
  ///
  pub unsafe fn wait<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::WaitConditionWaitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QWaitCondition::wakeAll()```</span>
  ///
  ///
  pub fn wake_all(&mut self) {
    unsafe { ::ffi::qt_core_c_QWaitCondition_wakeAll(self as *mut ::wait_condition::WaitCondition) }
  }

  /// C++ method: <span style='color: green;'>```void QWaitCondition::wakeOne()```</span>
  ///
  ///
  pub fn wake_one(&mut self) {
    unsafe { ::ffi::qt_core_c_QWaitCondition_wakeOne(self as *mut ::wait_condition::WaitCondition) }
  }
}

impl Drop for ::wait_condition::WaitCondition {
  /// C++ method: <span style='color: green;'>```[destructor] void QWaitCondition::~QWaitCondition()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QWaitCondition_destructor(self as *mut ::wait_condition::WaitCondition) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [WaitCondition::wait](../struct.WaitCondition.html#method.wait) method.
  pub trait WaitConditionWaitArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::wait_condition::WaitCondition) -> bool;
  }
  impl<'largs> WaitConditionWaitArgs<'largs> for *mut ::mutex::Mutex {
    unsafe fn exec(self, original_self: &'largs mut ::wait_condition::WaitCondition) -> bool {
      let locked_mutex = self;
      ::ffi::qt_core_c_QWaitCondition_wait_lockedMutex(original_self as *mut ::wait_condition::WaitCondition,
                                                       locked_mutex)
    }
  }
  impl<'largs> WaitConditionWaitArgs<'largs> for (*mut ::mutex::Mutex, ::libc::c_ulong) {
    unsafe fn exec(self, original_self: &'largs mut ::wait_condition::WaitCondition) -> bool {
      let locked_mutex = self.0;
      let time = self.1;
      ::ffi::qt_core_c_QWaitCondition_wait_lockedMutex_time(original_self as *mut ::wait_condition::WaitCondition,
                                                            locked_mutex,
                                                            time)
    }
  }
  impl<'largs> WaitConditionWaitArgs<'largs> for *mut ::read_write_lock::ReadWriteLock {
    unsafe fn exec(self, original_self: &'largs mut ::wait_condition::WaitCondition) -> bool {
      let locked_read_write_lock = self;
      ::ffi::qt_core_c_QWaitCondition_wait_lockedReadWriteLock(original_self as *mut ::wait_condition::WaitCondition,
                                                               locked_read_write_lock)
    }
  }
  impl<'largs> WaitConditionWaitArgs<'largs> for (*mut ::read_write_lock::ReadWriteLock, ::libc::c_ulong) {
    unsafe fn exec(self, original_self: &'largs mut ::wait_condition::WaitCondition) -> bool {
      let locked_read_write_lock = self.0;
      let time = self.1;
      ::ffi::qt_core_c_QWaitCondition_wait_lockedReadWriteLock_time(original_self as *mut ::wait_condition::WaitCondition, locked_read_write_lock, time)
    }
  }
}
