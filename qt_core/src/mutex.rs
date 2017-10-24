/// C++ type: <span style='color: green;'>```QMutex```</span>
#[repr(C)]
pub struct Mutex(u8);

impl Mutex {
  /// C++ method: <span style='color: green;'>```bool QMutex::isRecursive() const```</span>
  ///
  ///
  pub fn is_recursive(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMutex_isRecursive(self as *const ::mutex::Mutex) }
  }

  /// C++ method: <span style='color: green;'>```void QMutex::lock()```</span>
  ///
  ///
  pub fn lock(&mut self) {
    unsafe { ::ffi::qt_core_c_QMutex_lock(self as *mut ::mutex::Mutex) }
  }

  /// C++ method: <span style='color: green;'>```QMutex::QMutex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::mutex::Mutex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMutex::QMutex()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::mutex::RecursionMode) -> ::cpp_utils::CppBox<::mutex::Mutex>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMutex::QMutex(QMutex::RecursionMode mode = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::mutex::Mutex>
    where Args: overloading::MutexNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMutex::tryLock```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn try_lock0(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMutex::tryLock()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn try_lock0(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMutex::tryLock(int timeout = ?)```</span>
  ///
  ///
  pub fn try_lock0<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::MutexTryLock0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMutex::try_lock()```</span>
  ///
  ///
  pub fn try_lock1(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QMutex_try_lock(self as *mut ::mutex::Mutex) }
  }

  /// C++ method: <span style='color: green;'>```void QMutex::unlock()```</span>
  ///
  ///
  pub fn unlock(&mut self) {
    unsafe { ::ffi::qt_core_c_QMutex_unlock(self as *mut ::mutex::Mutex) }
  }
}

impl ::cpp_utils::CppDeletable for ::mutex::Mutex {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QMutex_delete
  }
}

/// C++ type: <span style='color: green;'>```QMutex::RecursionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RecursionMode {
  /// C++ enum variant: <span style='color: green;'>```NonRecursive = 0```</span>
  NonRecursive = 0,
  /// C++ enum variant: <span style='color: green;'>```Recursive = 1```</span>
  Recursive = 1,
}

impl ::cpp_utils::StaticCast<::basic_mutex::BasicMutex> for ::mutex::Mutex {
  fn static_cast_mut(&mut self) -> &mut ::basic_mutex::BasicMutex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(self as *mut ::mutex::Mutex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::basic_mutex::BasicMutex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(self as *const ::mutex::Mutex as *mut ::mutex::Mutex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mutex::Mutex> for ::basic_mutex::BasicMutex {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mutex::Mutex {
    let ffi_result = ::ffi::qt_core_c_QMutex_G_static_cast_QMutex_ptr(self as *mut ::basic_mutex::BasicMutex);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mutex::Mutex {
    let ffi_result = ::ffi::qt_core_c_QMutex_G_static_cast_QMutex_ptr(self as *const ::basic_mutex::BasicMutex as *mut ::basic_mutex::BasicMutex);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mutex::Mutex {
  type Target = ::basic_mutex::BasicMutex;
  fn deref(&self) -> &::basic_mutex::BasicMutex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(self as *const ::mutex::Mutex as *mut ::mutex::Mutex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mutex::Mutex {
  fn deref_mut(&mut self) -> &mut ::basic_mutex::BasicMutex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMutex_G_static_cast_QBasicMutex_ptr(self as *mut ::mutex::Mutex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Mutex::new](../struct.Mutex.html#method.new) method.
  pub trait MutexNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::mutex::Mutex>;
  }
  impl MutexNewArgs for ::mutex::RecursionMode {
    fn exec(self) -> ::cpp_utils::CppBox<::mutex::Mutex> {
      let mode = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QMutex_new_mode(mode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl MutexNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::mutex::Mutex> {

      let ffi_result = unsafe { ::ffi::qt_core_c_QMutex_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Mutex::try_lock0](../struct.Mutex.html#method.try_lock0) method.
  pub trait MutexTryLock0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::mutex::Mutex) -> bool;
  }
  impl<'largs> MutexTryLock0Args<'largs> for () {
    fn exec(self, original_self: &'largs mut ::mutex::Mutex) -> bool {

      unsafe { ::ffi::qt_core_c_QMutex_tryLock_no_args(original_self as *mut ::mutex::Mutex) }
    }
  }
  impl<'largs> MutexTryLock0Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::mutex::Mutex) -> bool {
      let timeout = self;
      unsafe { ::ffi::qt_core_c_QMutex_tryLock_timeout(original_self as *mut ::mutex::Mutex, timeout) }
    }
  }
}
