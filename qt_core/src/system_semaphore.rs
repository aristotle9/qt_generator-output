/// C++ type: <span style='color: green;'>```QSystemSemaphore::AccessMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AccessMode {
  /// C++ enum variant: <span style='color: green;'>```Open = 0```</span>
  Open = 0,
  /// C++ enum variant: <span style='color: green;'>```Create = 1```</span>
  Create = 1,
}

/// C++ type: <span style='color: green;'>```QSystemSemaphore```</span>
#[repr(C)]
pub struct SystemSemaphore([u8; ::type_sizes::QT_CORE_SYSTEM_SEMAPHORE_SYSTEM_SEMAPHORE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SystemSemaphore {
  unsafe fn new_uninitialized() -> SystemSemaphore {
    SystemSemaphore(::std::mem::uninitialized())
  }
}

impl SystemSemaphore {
  /// C++ method: <span style='color: green;'>```bool QSystemSemaphore::acquire()```</span>
  ///
  ///
  pub fn acquire(&mut self) -> bool {
    unsafe { ::ffi::qt_core_c_QSystemSemaphore_acquire(self as *mut ::system_semaphore::SystemSemaphore) }
  }

  /// C++ method: <span style='color: green;'>```QSystemSemaphore::SystemSemaphoreError QSystemSemaphore::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::system_semaphore::SystemSemaphoreError {
    unsafe { ::ffi::qt_core_c_QSystemSemaphore_error(self as *const ::system_semaphore::SystemSemaphore) }
  }

  /// C++ method: <span style='color: green;'>```QString QSystemSemaphore::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSystemSemaphore_errorString_to_output(self as *const ::system_semaphore::SystemSemaphore,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QSystemSemaphore::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSystemSemaphore_key_to_output(self as *const ::system_semaphore::SystemSemaphore,
                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSystemSemaphore::QSystemSemaphore```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::system_semaphore::SystemSemaphore```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemSemaphore::QSystemSemaphore(const QString& key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::string::String, ::libc::c_int)) -> ::system_semaphore::SystemSemaphore```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemSemaphore::QSystemSemaphore(const QString& key, int initialValue = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::string::String, ::libc::c_int, ::system_semaphore::AccessMode)) -> ::system_semaphore::SystemSemaphore```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSystemSemaphore::QSystemSemaphore(const QString& key, int initialValue = ?, QSystemSemaphore::AccessMode mode = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::system_semaphore::SystemSemaphore
    where Args: overloading::SystemSemaphoreNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSystemSemaphore::release```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn release(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSystemSemaphore::release()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn release(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSystemSemaphore::release(int n = ?)```</span>
  ///
  ///
  pub fn release<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::SystemSemaphoreReleaseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSystemSemaphore::setKey```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_key(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSystemSemaphore::setKey(const QString& key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_key(&mut self, (&::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSystemSemaphore::setKey(const QString& key, int initialValue = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_key(&mut self, (&::string::String, ::libc::c_int, ::system_semaphore::AccessMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSystemSemaphore::setKey(const QString& key, int initialValue = ?, QSystemSemaphore::AccessMode mode = ?)```</span>
  ///
  ///
  pub fn set_key<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SystemSemaphoreSetKeyArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::system_semaphore::SystemSemaphore {
  /// C++ method: <span style='color: green;'>```[destructor] void QSystemSemaphore::~QSystemSemaphore()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSystemSemaphore_destructor(self as *mut ::system_semaphore::SystemSemaphore) }
  }
}

/// C++ type: <span style='color: green;'>```QSystemSemaphore::SystemSemaphoreError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SystemSemaphoreError {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  NoError = 0,
  /// C++ enum variant: <span style='color: green;'>```PermissionDenied = 1```</span>
  PermissionDenied = 1,
  /// C++ enum variant: <span style='color: green;'>```KeyError = 2```</span>
  KeyError = 2,
  /// C++ enum variant: <span style='color: green;'>```AlreadyExists = 3```</span>
  AlreadyExists = 3,
  /// C++ enum variant: <span style='color: green;'>```NotFound = 4```</span>
  NotFound = 4,
  /// C++ enum variant: <span style='color: green;'>```OutOfResources = 5```</span>
  OutOfResources = 5,
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 6```</span>
  UnknownError = 6,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SystemSemaphore::new](../struct.SystemSemaphore.html#method.new) method.
  pub trait SystemSemaphoreNewArgs {
    fn exec(self) -> ::system_semaphore::SystemSemaphore;
  }
  impl<'a> SystemSemaphoreNewArgs for &'a ::string::String {
    fn exec(self) -> ::system_semaphore::SystemSemaphore {
      let key = self;
      {
        let mut object: ::system_semaphore::SystemSemaphore =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSystemSemaphore_constructor_key(key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> SystemSemaphoreNewArgs for (&'a ::string::String, ::libc::c_int) {
    fn exec(self) -> ::system_semaphore::SystemSemaphore {
      let key = self.0;
      let initial_value = self.1;
      {
        let mut object: ::system_semaphore::SystemSemaphore =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSystemSemaphore_constructor_key_initialValue(key as *const ::string::String,
                                                                         initial_value,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> SystemSemaphoreNewArgs for (&'a ::string::String, ::libc::c_int, ::system_semaphore::AccessMode) {
    fn exec(self) -> ::system_semaphore::SystemSemaphore {
      let key = self.0;
      let initial_value = self.1;
      let mode = self.2;
      {
        let mut object: ::system_semaphore::SystemSemaphore =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QSystemSemaphore_constructor_key_initialValue_mode(key as *const ::string::String,
                                                                              initial_value,
                                                                              mode,
                                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SystemSemaphore::release](../struct.SystemSemaphore.html#method.release) method.
  pub trait SystemSemaphoreReleaseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> bool;
  }
  impl<'largs> SystemSemaphoreReleaseArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> bool {
      let n = self;
      unsafe {
        ::ffi::qt_core_c_QSystemSemaphore_release_n(original_self as *mut ::system_semaphore::SystemSemaphore, n)
      }
    }
  }
  impl<'largs> SystemSemaphoreReleaseArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> bool {

      unsafe {
        ::ffi::qt_core_c_QSystemSemaphore_release_no_args(original_self as *mut ::system_semaphore::SystemSemaphore)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SystemSemaphore::set_key](../struct.SystemSemaphore.html#method.set_key) method.
  pub trait SystemSemaphoreSetKeyArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> ();
  }
  impl<'largs> SystemSemaphoreSetKeyArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> () {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QSystemSemaphore_setKey_key(original_self as *mut ::system_semaphore::SystemSemaphore,
                                                     key as *const ::string::String)
      }
    }
  }
  impl<'largs> SystemSemaphoreSetKeyArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> () {
      let key = self.0;
      let initial_value = self.1;
      unsafe { ::ffi::qt_core_c_QSystemSemaphore_setKey_key_initialValue(original_self as *mut ::system_semaphore::SystemSemaphore, key as *const ::string::String, initial_value) }
    }
  }
  impl<'largs> SystemSemaphoreSetKeyArgs<'largs>
    for (&'largs ::string::String, ::libc::c_int, ::system_semaphore::AccessMode) {
    fn exec(self, original_self: &'largs mut ::system_semaphore::SystemSemaphore) -> () {
      let key = self.0;
      let initial_value = self.1;
      let mode = self.2;
      unsafe { ::ffi::qt_core_c_QSystemSemaphore_setKey_key_initialValue_mode(original_self as *mut ::system_semaphore::SystemSemaphore, key as *const ::string::String, initial_value, mode) }
    }
  }
}
