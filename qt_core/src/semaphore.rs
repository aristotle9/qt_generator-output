/// C++ type: <span style='color: green;'>```QSemaphore```</span>
#[repr(C)]
pub struct Semaphore([u8; ::type_sizes::QT_CORE_SEMAPHORE_SEMAPHORE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Semaphore {
  unsafe fn new_uninitialized() -> Semaphore {
    Semaphore(::std::mem::uninitialized())
  }
}

impl Semaphore {
  /// C++ method: <span style='color: green;'>```QSemaphore::acquire```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn acquire(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSemaphore::acquire()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn acquire(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSemaphore::acquire(int n = ?)```</span>
  ///
  ///
  pub fn acquire<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SemaphoreAcquireArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QSemaphore::available() const```</span>
  ///
  ///
  pub fn available(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSemaphore_available(self as *const ::semaphore::Semaphore) }
  }

  /// C++ method: <span style='color: green;'>```QSemaphore::QSemaphore```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::semaphore::Semaphore```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSemaphore::QSemaphore()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::semaphore::Semaphore```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSemaphore::QSemaphore(int n = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::semaphore::Semaphore
    where Args: overloading::SemaphoreNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSemaphore::release```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn release(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSemaphore::release()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn release(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSemaphore::release(int n = ?)```</span>
  ///
  ///
  pub fn release<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SemaphoreReleaseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSemaphore::tryAcquire```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn try_acquire(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSemaphore::tryAcquire()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn try_acquire(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSemaphore::tryAcquire(int n = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn try_acquire(&mut self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSemaphore::tryAcquire(int n, int timeout)```</span>
  ///
  ///
  pub fn try_acquire<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SemaphoreTryAcquireArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::semaphore::Semaphore {
  /// C++ method: <span style='color: green;'>```[destructor] void QSemaphore::~QSemaphore()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSemaphore_destructor(self as *mut ::semaphore::Semaphore) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Semaphore::acquire](../struct.Semaphore.html#method.acquire) method.
  pub trait SemaphoreAcquireArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> ();
  }
  impl<'largs> SemaphoreAcquireArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> () {
      let n = self;
      unsafe { ::ffi::qt_core_c_QSemaphore_acquire_n(original_self as *mut ::semaphore::Semaphore, n) }
    }
  }
  impl<'largs> SemaphoreAcquireArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> () {

      unsafe { ::ffi::qt_core_c_QSemaphore_acquire_no_args(original_self as *mut ::semaphore::Semaphore) }
    }
  }
  /// This trait represents a set of arguments accepted by [Semaphore::new](../struct.Semaphore.html#method.new) method.
  pub trait SemaphoreNewArgs {
    fn exec(self) -> ::semaphore::Semaphore;
  }
  impl SemaphoreNewArgs for ::libc::c_int {
    fn exec(self) -> ::semaphore::Semaphore {
      let n = self;
      {
        let mut object: ::semaphore::Semaphore =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSemaphore_constructor_n(n, &mut object);
        }
        object
      }
    }
  }
  impl SemaphoreNewArgs for () {
    fn exec(self) -> ::semaphore::Semaphore {

      {
        let mut object: ::semaphore::Semaphore =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSemaphore_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Semaphore::release](../struct.Semaphore.html#method.release) method.
  pub trait SemaphoreReleaseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> ();
  }
  impl<'largs> SemaphoreReleaseArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> () {
      let n = self;
      unsafe { ::ffi::qt_core_c_QSemaphore_release_n(original_self as *mut ::semaphore::Semaphore, n) }
    }
  }
  impl<'largs> SemaphoreReleaseArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> () {

      unsafe { ::ffi::qt_core_c_QSemaphore_release_no_args(original_self as *mut ::semaphore::Semaphore) }
    }
  }
  /// This trait represents a set of arguments accepted by [Semaphore::try_acquire](../struct.Semaphore.html#method.try_acquire) method.
  pub trait SemaphoreTryAcquireArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> bool;
  }
  impl<'largs> SemaphoreTryAcquireArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> bool {
      let n = self;
      unsafe { ::ffi::qt_core_c_QSemaphore_tryAcquire_n(original_self as *mut ::semaphore::Semaphore, n) }
    }
  }
  impl<'largs> SemaphoreTryAcquireArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> bool {
      let n = self.0;
      let timeout = self.1;
      unsafe {
        ::ffi::qt_core_c_QSemaphore_tryAcquire_n_timeout(original_self as *mut ::semaphore::Semaphore, n, timeout)
      }
    }
  }
  impl<'largs> SemaphoreTryAcquireArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::semaphore::Semaphore) -> bool {

      unsafe { ::ffi::qt_core_c_QSemaphore_tryAcquire_no_args(original_self as *mut ::semaphore::Semaphore) }
    }
  }
}
