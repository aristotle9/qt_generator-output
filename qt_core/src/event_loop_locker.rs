/// C++ type: <span style='color: green;'>```QEventLoopLocker```</span>
#[repr(C)]
pub struct EventLoopLocker([u8; ::type_sizes::QT_CORE_EVENT_LOOP_LOCKER_EVENT_LOOP_LOCKER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for EventLoopLocker {
  unsafe fn new_uninitialized() -> EventLoopLocker {
    EventLoopLocker(::std::mem::uninitialized())
  }
}

impl EventLoopLocker {
  /// C++ method: <span style='color: green;'>```[constructor] void QEventLoopLocker::QEventLoopLocker()```</span>
  ///
  ///
  pub fn new() -> ::event_loop_locker::EventLoopLocker {
    {
      let mut object: ::event_loop_locker::EventLoopLocker =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QEventLoopLocker_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QEventLoopLocker::QEventLoopLocker```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::event_loop::EventLoop) -> ::event_loop_locker::EventLoopLocker```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEventLoopLocker::QEventLoopLocker(QEventLoop* loop)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::thread::Thread) -> ::event_loop_locker::EventLoopLocker```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEventLoopLocker::QEventLoopLocker(QThread* thread)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::event_loop_locker::EventLoopLocker
    where Args: overloading::EventLoopLockerNewUnsafeArgs
  {
    args.exec()
  }
}

impl Drop for ::event_loop_locker::EventLoopLocker {
  /// C++ method: <span style='color: green;'>```[destructor] void QEventLoopLocker::~QEventLoopLocker()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QEventLoopLocker_destructor(self as *mut ::event_loop_locker::EventLoopLocker) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [EventLoopLocker::new_unsafe](../struct.EventLoopLocker.html#method.new_unsafe) method.
  pub trait EventLoopLockerNewUnsafeArgs {
    unsafe fn exec(self) -> ::event_loop_locker::EventLoopLocker;
  }
  impl EventLoopLockerNewUnsafeArgs for *mut ::event_loop::EventLoop {
    unsafe fn exec(self) -> ::event_loop_locker::EventLoopLocker {
      let loop_ = self;
      {
        let mut object: ::event_loop_locker::EventLoopLocker =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QEventLoopLocker_constructor_loop(loop_, &mut object);
        object
      }
    }
  }
  impl EventLoopLockerNewUnsafeArgs for *mut ::thread::Thread {
    unsafe fn exec(self) -> ::event_loop_locker::EventLoopLocker {
      let thread = self;
      {
        let mut object: ::event_loop_locker::EventLoopLocker =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QEventLoopLocker_constructor_thread(thread, &mut object);
        object
      }
    }
  }
}
