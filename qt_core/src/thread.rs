/// C++ type: <span style='color: green;'>```QThread::Priority```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Priority {
  /// C++ enum variant: <span style='color: green;'>```IdlePriority = 0```</span>
  Idle = 0,
  /// C++ enum variant: <span style='color: green;'>```LowestPriority = 1```</span>
  Lowest = 1,
  /// C++ enum variant: <span style='color: green;'>```LowPriority = 2```</span>
  Low = 2,
  /// C++ enum variant: <span style='color: green;'>```NormalPriority = 3```</span>
  Normal = 3,
  /// C++ enum variant: <span style='color: green;'>```HighPriority = 4```</span>
  High = 4,
  /// C++ enum variant: <span style='color: green;'>```HighestPriority = 5```</span>
  Highest = 5,
  /// C++ enum variant: <span style='color: green;'>```TimeCriticalPriority = 6```</span>
  TimeCritical = 6,
  /// C++ enum variant: <span style='color: green;'>```InheritPriority = 7```</span>
  Inherit = 7,
}

/// C++ type: <span style='color: green;'>```QThread```</span>
#[repr(C)]
pub struct Thread(u8);

impl Thread {
  /// C++ method: <span style='color: green;'>```static QThread* QThread::currentThread()```</span>
  ///
  ///
  pub fn current_thread() -> *mut ::thread::Thread {
    unsafe { ::ffi::qt_core_c_QThread_currentThread() }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QThread::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QThread_event(self as *mut ::thread::Thread, event)
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher* QThread::eventDispatcher() const```</span>
  ///
  ///
  pub fn event_dispatcher(&self) -> *mut ::abstract_event_dispatcher::AbstractEventDispatcher {
    unsafe { ::ffi::qt_core_c_QThread_eventDispatcher(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```QThread::exit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exit(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QThread::exit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exit(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QThread::exit(int retcode = ?)```</span>
  ///
  ///
  pub fn exit<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ThreadExitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static int QThread::idealThreadCount()```</span>
  ///
  ///
  pub fn ideal_thread_count() -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QThread_idealThreadCount() }
  }

  /// C++ method: <span style='color: green;'>```bool QThread::isFinished() const```</span>
  ///
  ///
  pub fn is_finished(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QThread_isFinished(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```bool QThread::isInterruptionRequested() const```</span>
  ///
  ///
  pub fn is_interruption_requested(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QThread_isInterruptionRequested(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```bool QThread::isRunning() const```</span>
  ///
  ///
  pub fn is_running(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QThread_isRunning(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```int QThread::loopLevel() const```</span>
  ///
  ///
  pub fn loop_level(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QThread_loopLevel(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QThread::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QThread_metaObject(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```static void QThread::msleep(unsigned long arg1)```</span>
  ///
  ///
  pub fn msleep(arg1: ::libc::c_ulong) {
    unsafe { ::ffi::qt_core_c_QThread_msleep(arg1) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QThread::QThread()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::thread::Thread> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThread_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QThread::QThread(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::thread::Thread> {
    let ffi_result = ::ffi::qt_core_c_QThread_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QThread::Priority QThread::priority() const```</span>
  ///
  ///
  pub fn priority(&self) -> ::thread::Priority {
    unsafe { ::ffi::qt_core_c_QThread_priority(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QThread::quit()```</span>
  ///
  ///
  pub fn quit(&mut self) {
    unsafe { ::ffi::qt_core_c_QThread_quit(self as *mut ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```void QThread::requestInterruption()```</span>
  ///
  ///
  pub fn request_interruption(&mut self) {
    unsafe { ::ffi::qt_core_c_QThread_requestInterruption(self as *mut ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```void QThread::setEventDispatcher(QAbstractEventDispatcher* eventDispatcher)```</span>
  ///
  ///
  pub unsafe fn set_event_dispatcher(&mut self,
                                     event_dispatcher: *mut ::abstract_event_dispatcher::AbstractEventDispatcher) {
    ::ffi::qt_core_c_QThread_setEventDispatcher(self as *mut ::thread::Thread, event_dispatcher)
  }

  /// C++ method: <span style='color: green;'>```void QThread::setPriority(QThread::Priority priority)```</span>
  ///
  ///
  pub fn set_priority(&mut self, priority: ::thread::Priority) {
    unsafe { ::ffi::qt_core_c_QThread_setPriority(self as *mut ::thread::Thread, priority) }
  }

  /// C++ method: <span style='color: green;'>```void QThread::setStackSize(unsigned int stackSize)```</span>
  ///
  ///
  pub fn set_stack_size(&mut self, stack_size: ::libc::c_uint) {
    unsafe { ::ffi::qt_core_c_QThread_setStackSize(self as *mut ::thread::Thread, stack_size) }
  }

  /// C++ method: <span style='color: green;'>```static void QThread::sleep(unsigned long arg1)```</span>
  ///
  ///
  pub fn sleep(arg1: ::libc::c_ulong) {
    unsafe { ::ffi::qt_core_c_QThread_sleep(arg1) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QThread::stackSize() const```</span>
  ///
  ///
  pub fn stack_size(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QThread_stackSize(self as *const ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```QThread::start```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QThread::start()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start(&mut self, ::thread::Priority) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QThread::start(QThread::Priority arg1 = ?)```</span>
  ///
  ///
  pub fn start<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ThreadStartArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QThread::terminate()```</span>
  ///
  ///
  pub fn terminate(&mut self) {
    unsafe { ::ffi::qt_core_c_QThread_terminate(self as *mut ::thread::Thread) }
  }

  /// C++ method: <span style='color: green;'>```static QString QThread::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QThread_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QThread::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QThread_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static void QThread::usleep(unsigned long arg1)```</span>
  ///
  ///
  pub fn usleep(arg1: ::libc::c_ulong) {
    unsafe { ::ffi::qt_core_c_QThread_usleep(arg1) }
  }

  /// C++ method: <span style='color: green;'>```QThread::wait```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn wait(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QThread::wait()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn wait(&mut self, ::libc::c_ulong) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QThread::wait(unsigned long time = ?)```</span>
  ///
  ///
  pub fn wait<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ThreadWaitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static void QThread::yieldCurrentThread()```</span>
  ///
  ///
  pub fn yield_current_thread() {
    unsafe { ::ffi::qt_core_c_QThread_yieldCurrentThread() }
  }
}

impl ::cpp_utils::CppDeletable for ::thread::Thread {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QThread_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Thread`.
  pub struct Signals<'a>(&'a ::thread::Thread);
  /// Represents a built-in Qt signal `QThread::started`.
  ///
  /// An object of this type can be created from `Thread` with `object.signals().started()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct Started<'a>(&'a ::thread::Thread);
  impl<'a> ::connection::Receiver for Started<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2started()\0"
    }
  }
  impl<'a> ::connection::Signal for Started<'a> {}
  /// Represents a built-in Qt signal `QThread::finished`.
  ///
  /// An object of this type can be created from `Thread` with `object.signals().finished()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct Finished<'a>(&'a ::thread::Thread);
  impl<'a> ::connection::Receiver for Finished<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2finished()\0"
    }
  }
  impl<'a> ::connection::Signal for Finished<'a> {}
  /// Represents a built-in Qt signal `QThread::objectNameChanged`.
  ///
  /// An object of this type can be created from `Thread` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct ObjectNameChanged<'a>(&'a ::thread::Thread);
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
    /// Returns an object representing a built-in Qt signal `QThread::started`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn started(&self) -> Started {
      Started(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QThread::finished`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn finished(&self) -> Finished {
      Finished(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QThread::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Thread`.
  pub struct Slots<'a>(&'a ::thread::Thread);
  /// Represents a built-in Qt slot `QThread::terminate`.
  ///
  /// An object of this type can be created from `Thread` with `object.slots().terminate()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct Terminate<'a>(&'a ::thread::Thread);
  impl<'a> ::connection::Receiver for Terminate<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1terminate()\0"
    }
  }
  /// Represents a built-in Qt slot `QThread::quit`.
  ///
  /// An object of this type can be created from `Thread` with `object.slots().quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct Quit<'a>(&'a ::thread::Thread);
  impl<'a> ::connection::Receiver for Quit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1quit()\0"
    }
  }
  /// Represents a built-in Qt slot `QThread::start`.
  ///
  /// An object of this type can be created from `Thread` with `object.slots().start()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Thread` object.
  pub struct Start<'a>(&'a ::thread::Thread);
  impl<'a> ::connection::Receiver for Start<'a> {
    type Arguments = (::thread::Priority,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1start(QThread::Priority)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QThread::terminate`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn terminate(&self) -> Terminate {
      Terminate(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QThread::quit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quit(&self) -> Quit {
      Quit(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QThread::start`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start(&self) -> Start {
      Start(self.0)
    }
  }
  impl ::thread::Thread {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::thread::Thread> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::thread::Thread> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThread_G_dynamic_cast_QThread_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::thread::Thread> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QThread_G_dynamic_cast_QThread_ptr(self as *const ::object::Object as *mut ::object::Object)
      };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::thread::Thread {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThread_G_static_cast_QObject_ptr(self as *mut ::thread::Thread) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QThread_G_static_cast_QObject_ptr(self as *const ::thread::Thread as *mut ::thread::Thread)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::thread::Thread> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::thread::Thread {
    let ffi_result = ::ffi::qt_core_c_QThread_G_static_cast_QThread_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::thread::Thread {
    let ffi_result =
      ::ffi::qt_core_c_QThread_G_static_cast_QThread_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::thread::Thread {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QThread_G_static_cast_QObject_ptr(self as *const ::thread::Thread as *mut ::thread::Thread)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::thread::Thread {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QThread_G_static_cast_QObject_ptr(self as *mut ::thread::Thread) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Thread::exit](../struct.Thread.html#method.exit) method.
  pub trait ThreadExitArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> ();
  }
  impl<'largs> ThreadExitArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> () {

      unsafe { ::ffi::qt_core_c_QThread_exit_no_args(original_self as *mut ::thread::Thread) }
    }
  }
  impl<'largs> ThreadExitArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> () {
      let retcode = self;
      unsafe { ::ffi::qt_core_c_QThread_exit_retcode(original_self as *mut ::thread::Thread, retcode) }
    }
  }
  /// This trait represents a set of arguments accepted by [Thread::start](../struct.Thread.html#method.start) method.
  pub trait ThreadStartArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> ();
  }
  impl<'largs> ThreadStartArgs<'largs> for ::thread::Priority {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_core_c_QThread_start_arg1(original_self as *mut ::thread::Thread, arg1) }
    }
  }
  impl<'largs> ThreadStartArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> () {

      unsafe { ::ffi::qt_core_c_QThread_start_no_args(original_self as *mut ::thread::Thread) }
    }
  }
  /// This trait represents a set of arguments accepted by [Thread::wait](../struct.Thread.html#method.wait) method.
  pub trait ThreadWaitArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> bool;
  }
  impl<'largs> ThreadWaitArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> bool {

      unsafe { ::ffi::qt_core_c_QThread_wait_no_args(original_self as *mut ::thread::Thread) }
    }
  }
  impl<'largs> ThreadWaitArgs<'largs> for ::libc::c_ulong {
    fn exec(self, original_self: &'largs mut ::thread::Thread) -> bool {
      let time = self;
      unsafe { ::ffi::qt_core_c_QThread_wait_time(original_self as *mut ::thread::Thread, time) }
    }
  }
}
