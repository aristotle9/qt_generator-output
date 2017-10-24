/// C++ type: <span style='color: green;'>```QThreadPool```</span>
#[repr(C)]
pub struct ThreadPool(u8);

impl ThreadPool {
  /// C++ method: <span style='color: green;'>```int QThreadPool::activeThreadCount() const```</span>
  ///
  ///
  pub fn active_thread_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QThreadPool_activeThreadCount(self as *const ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::cancel(QRunnable* runnable)```</span>
  ///
  ///
  pub unsafe fn cancel(&mut self, runnable: *mut ::runnable::Runnable) {
    ::ffi::qt_core_c_QThreadPool_cancel(self as *mut ::thread_pool::ThreadPool, runnable)
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QThreadPool_clear(self as *mut ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```int QThreadPool::expiryTimeout() const```</span>
  ///
  ///
  pub fn expiry_timeout(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QThreadPool_expiryTimeout(self as *const ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```static QThreadPool* QThreadPool::globalInstance()```</span>
  ///
  ///
  pub fn global_instance() -> *mut ::thread_pool::ThreadPool {
    unsafe { ::ffi::qt_core_c_QThreadPool_globalInstance() }
  }

  /// C++ method: <span style='color: green;'>```int QThreadPool::maxThreadCount() const```</span>
  ///
  ///
  pub fn max_thread_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QThreadPool_maxThreadCount(self as *const ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QThreadPool::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QThreadPool_metaObject(self as *const ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QThreadPool::QThreadPool()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::thread_pool::ThreadPool> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThreadPool_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QThreadPool::QThreadPool(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::thread_pool::ThreadPool> {
    let ffi_result = ::ffi::qt_core_c_QThreadPool_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::releaseThread()```</span>
  ///
  ///
  pub fn release_thread(&mut self) {
    unsafe { ::ffi::qt_core_c_QThreadPool_releaseThread(self as *mut ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::reserveThread()```</span>
  ///
  ///
  pub fn reserve_thread(&mut self) {
    unsafe { ::ffi::qt_core_c_QThreadPool_reserveThread(self as *mut ::thread_pool::ThreadPool) }
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::setExpiryTimeout(int expiryTimeout)```</span>
  ///
  ///
  pub fn set_expiry_timeout(&mut self, expiry_timeout: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QThreadPool_setExpiryTimeout(self as *mut ::thread_pool::ThreadPool, expiry_timeout) }
  }

  /// C++ method: <span style='color: green;'>```void QThreadPool::setMaxThreadCount(int maxThreadCount)```</span>
  ///
  ///
  pub fn set_max_thread_count(&mut self, max_thread_count: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QThreadPool_setMaxThreadCount(self as *mut ::thread_pool::ThreadPool, max_thread_count) }
  }

  /// C++ method: <span style='color: green;'>```QThreadPool::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, *mut ::runnable::Runnable) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QThreadPool::start(QRunnable* runnable)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, (*mut ::runnable::Runnable, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QThreadPool::start(QRunnable* runnable, int priority = ?)```</span>
  ///
  ///
  pub unsafe fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ThreadPoolStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QString QThreadPool::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QThreadPool_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QThreadPool::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QThreadPool_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QThreadPool::tryStart(QRunnable* runnable)```</span>
  ///
  ///
  pub unsafe fn try_start(&mut self, runnable: *mut ::runnable::Runnable) -> bool {
    ::ffi::qt_core_c_QThreadPool_tryStart(self as *mut ::thread_pool::ThreadPool, runnable)
  }

  /// C++ method: <span style='color: green;'>```bool QThreadPool::tryTake(QRunnable* runnable)```</span>
  ///
  ///
  pub unsafe fn try_take(&mut self, runnable: *mut ::runnable::Runnable) -> bool {
    ::ffi::qt_core_c_QThreadPool_tryTake(self as *mut ::thread_pool::ThreadPool, runnable)
  }

  /// C++ method: <span style='color: green;'>```QThreadPool::waitForDone```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait_for_done(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QThreadPool::waitForDone()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait_for_done(&mut self, ::libc::c_int) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QThreadPool::waitForDone(int msecs = ?)```</span>
  ///
  ///
  pub fn wait_for_done<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ThreadPoolWaitForDoneArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::thread_pool::ThreadPool {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QThreadPool_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ThreadPool`.
  pub struct Signals<'a>(&'a ::thread_pool::ThreadPool);
  /// Represents a built-in Qt signal `QThreadPool::objectNameChanged`.
  ///
  /// An object of this type can be created from `ThreadPool` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ThreadPool` object.
  pub struct ObjectNameChanged<'a>(&'a ::thread_pool::ThreadPool);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QThreadPool::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::thread_pool::ThreadPool {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::thread_pool::ThreadPool> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::thread_pool::ThreadPool> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QThreadPool_G_dynamic_cast_QThreadPool_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::thread_pool::ThreadPool> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThreadPool_G_dynamic_cast_QThreadPool_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::thread_pool::ThreadPool {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QThreadPool_G_static_cast_QObject_ptr(self as *mut ::thread_pool::ThreadPool) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThreadPool_G_static_cast_QObject_ptr(self as *const ::thread_pool::ThreadPool as *mut ::thread_pool::ThreadPool) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::thread_pool::ThreadPool> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::thread_pool::ThreadPool {
    let ffi_result = ::ffi::qt_core_c_QThreadPool_G_static_cast_QThreadPool_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::thread_pool::ThreadPool {
    let ffi_result = ::ffi::qt_core_c_QThreadPool_G_static_cast_QThreadPool_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::thread_pool::ThreadPool {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThreadPool_G_static_cast_QObject_ptr(self as *const ::thread_pool::ThreadPool as *mut ::thread_pool::ThreadPool) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::thread_pool::ThreadPool {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QThreadPool_G_static_cast_QObject_ptr(self as *mut ::thread_pool::ThreadPool) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ThreadPool::start](../struct.ThreadPool.html#method.start) method.
  pub trait ThreadPoolStartArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> ();
  }
  impl<'largs> ThreadPoolStartArgs<'largs> for *mut ::runnable::Runnable {
    unsafe fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> () {
      let runnable = self;
      ::ffi::qt_core_c_QThreadPool_start_runnable(original_self as *mut ::thread_pool::ThreadPool, runnable)
    }
  }
  impl<'largs> ThreadPoolStartArgs<'largs> for (*mut ::runnable::Runnable, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> () {
      let runnable = self.0;
      let priority = self.1;
      ::ffi::qt_core_c_QThreadPool_start_runnable_priority(original_self as *mut ::thread_pool::ThreadPool,
                                                           runnable,
                                                           priority)
    }
  }
  /// This trait represents a set of arguments accepted by [ThreadPool::wait_for_done](../struct.ThreadPool.html#method.wait_for_done) method.
  pub trait ThreadPoolWaitForDoneArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> bool;
  }
  impl<'largs> ThreadPoolWaitForDoneArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> bool {
      let msecs = self;
      unsafe { ::ffi::qt_core_c_QThreadPool_waitForDone_msecs(original_self as *mut ::thread_pool::ThreadPool, msecs) }
    }
  }
  impl<'largs> ThreadPoolWaitForDoneArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::thread_pool::ThreadPool) -> bool {

      unsafe { ::ffi::qt_core_c_QThreadPool_waitForDone_no_args(original_self as *mut ::thread_pool::ThreadPool) }
    }
  }
}
