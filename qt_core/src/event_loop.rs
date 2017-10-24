/// C++ type: <span style='color: green;'>```QEventLoop```</span>
#[repr(C)]
pub struct EventLoop(u8);

impl EventLoop {
  /// C++ method: <span style='color: green;'>```virtual bool QEventLoop::event(QEvent* event)```</span>
  ///
  ///
  pub unsafe fn event(&mut self, event: *mut ::event::Event) -> bool {
    ::ffi::qt_core_c_QEventLoop_event(self as *mut ::event_loop::EventLoop, event)
  }

  /// C++ method: <span style='color: green;'>```QEventLoop::exec```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exec(&mut self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QEventLoop::exec()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exec(&mut self, ::flags::Flags<::event_loop::ProcessEventsFlag>) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QEventLoop::exec(QFlags<QEventLoop::ProcessEventsFlag> flags = ?)```</span>
  ///
  ///
  pub fn exec<'largs, Args>(&'largs mut self, args: Args) -> ::libc::c_int
    where Args: overloading::EventLoopExecArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QEventLoop::exit```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn exit(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QEventLoop::exit()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn exit(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QEventLoop::exit(int returnCode = ?)```</span>
  ///
  ///
  pub fn exit<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::EventLoopExitArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QEventLoop::isRunning() const```</span>
  ///
  ///
  pub fn is_running(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QEventLoop_isRunning(self as *const ::event_loop::EventLoop) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QEventLoop::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QEventLoop_metaObject(self as *const ::event_loop::EventLoop) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QEventLoop::QEventLoop()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::event_loop::EventLoop> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventLoop_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QEventLoop::QEventLoop(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::event_loop::EventLoop> {
    let ffi_result = ::ffi::qt_core_c_QEventLoop_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QEventLoop::processEvents```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn process_events(&mut self, ()) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QEventLoop::processEvents()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn process_events(&mut self, ::flags::Flags<::event_loop::ProcessEventsFlag>) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QEventLoop::processEvents(QFlags<QEventLoop::ProcessEventsFlag> flags = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn process_events(&mut self, (::flags::Flags<::event_loop::ProcessEventsFlag>, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QEventLoop::processEvents(QFlags<QEventLoop::ProcessEventsFlag> flags, int maximumTime)```</span>
  ///
  ///
  pub fn process_events<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::EventLoopProcessEventsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QEventLoop::quit()```</span>
  ///
  ///
  pub fn quit(&mut self) {
    unsafe { ::ffi::qt_core_c_QEventLoop_quit(self as *mut ::event_loop::EventLoop) }
  }

  /// C++ method: <span style='color: green;'>```static QString QEventLoop::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QEventLoop_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QEventLoop::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QEventLoop_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QEventLoop::wakeUp()```</span>
  ///
  ///
  pub fn wake_up(&mut self) {
    unsafe { ::ffi::qt_core_c_QEventLoop_wakeUp(self as *mut ::event_loop::EventLoop) }
  }
}

impl ::cpp_utils::CppDeletable for ::event_loop::EventLoop {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QEventLoop_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `EventLoop`.
  pub struct Signals<'a>(&'a ::event_loop::EventLoop);
  /// Represents a built-in Qt signal `QEventLoop::objectNameChanged`.
  ///
  /// An object of this type can be created from `EventLoop` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EventLoop` object.
  pub struct ObjectNameChanged<'a>(&'a ::event_loop::EventLoop);
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
    /// Returns an object representing a built-in Qt signal `QEventLoop::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `EventLoop`.
  pub struct Slots<'a>(&'a ::event_loop::EventLoop);
  /// Represents a built-in Qt slot `QEventLoop::quit`.
  ///
  /// An object of this type can be created from `EventLoop` with `object.slots().quit()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `EventLoop` object.
  pub struct Quit<'a>(&'a ::event_loop::EventLoop);
  impl<'a> ::connection::Receiver for Quit<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1quit()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QEventLoop::quit`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn quit(&self) -> Quit {
      Quit(self.0)
    }
  }
  impl ::event_loop::EventLoop {
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

/// C++ type: <span style='color: green;'>```QEventLoop::ProcessEventsFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ProcessEventsFlag {
  /// C++ enum variant: <span style='color: green;'>```AllEvents = 0```</span>
  AllEvents = 0,
  /// C++ enum variant: <span style='color: green;'>```ExcludeUserInputEvents = 1```</span>
  ExcludeUserInputEvents = 1,
  /// C++ enum variant: <span style='color: green;'>```ExcludeSocketNotifiers = 2```</span>
  ExcludeSocketNotifiers = 2,
  /// C++ enum variant: <span style='color: green;'>```WaitForMoreEvents = 4```</span>
  WaitForMoreEvents = 4,
  /// C++ enum variant: <span style='color: green;'>```X11ExcludeTimers = 8```</span>
  X11ExcludeTimers = 8,
  /// C++ enum variant: <span style='color: green;'>```EventLoopExec = 32```</span>
  EventLoopExec = 32,
  /// C++ enum variant: <span style='color: green;'>```DialogExec = 64```</span>
  DialogExec = 64,
}

impl ::flags::FlaggableEnum for ProcessEventsFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ProcessEventsFlag"
  }
}

impl ::cpp_utils::DynamicCast<::event_loop::EventLoop> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::event_loop::EventLoop> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QEventLoop_G_dynamic_cast_QEventLoop_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::event_loop::EventLoop> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventLoop_G_dynamic_cast_QEventLoop_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::event_loop::EventLoop {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QEventLoop_G_static_cast_QObject_ptr(self as *mut ::event_loop::EventLoop) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventLoop_G_static_cast_QObject_ptr(self as *const ::event_loop::EventLoop as *mut ::event_loop::EventLoop) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::event_loop::EventLoop> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::event_loop::EventLoop {
    let ffi_result = ::ffi::qt_core_c_QEventLoop_G_static_cast_QEventLoop_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::event_loop::EventLoop {
    let ffi_result = ::ffi::qt_core_c_QEventLoop_G_static_cast_QEventLoop_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::event_loop::EventLoop {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QEventLoop_G_static_cast_QObject_ptr(self as *const ::event_loop::EventLoop as *mut ::event_loop::EventLoop) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::event_loop::EventLoop {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QEventLoop_G_static_cast_QObject_ptr(self as *mut ::event_loop::EventLoop) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [EventLoop::exec](../struct.EventLoop.html#method.exec) method.
  pub trait EventLoopExecArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> ::libc::c_int;
  }
  impl<'largs> EventLoopExecArgs<'largs> for ::flags::Flags<::event_loop::ProcessEventsFlag> {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> ::libc::c_int {
      let flags = self;
      unsafe {
        ::ffi::qt_core_c_QEventLoop_exec_flags(original_self as *mut ::event_loop::EventLoop,
                                               flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> EventLoopExecArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QEventLoop_exec_no_args(original_self as *mut ::event_loop::EventLoop) }
    }
  }
  /// This trait represents a set of arguments accepted by [EventLoop::exit](../struct.EventLoop.html#method.exit) method.
  pub trait EventLoopExitArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> ();
  }
  impl<'largs> EventLoopExitArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> () {

      unsafe { ::ffi::qt_core_c_QEventLoop_exit_no_args(original_self as *mut ::event_loop::EventLoop) }
    }
  }
  impl<'largs> EventLoopExitArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> () {
      let return_code = self;
      unsafe { ::ffi::qt_core_c_QEventLoop_exit_returnCode(original_self as *mut ::event_loop::EventLoop, return_code) }
    }
  }
  /// This trait represents a set of arguments accepted by [EventLoop::process_events](../struct.EventLoop.html#method.process_events) method.
  pub trait EventLoopProcessEventsArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> Self::ReturnType;
  }
  impl<'largs> EventLoopProcessEventsArgs<'largs> for ::flags::Flags<::event_loop::ProcessEventsFlag> {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> bool {
      let flags = self;
      unsafe {
        ::ffi::qt_core_c_QEventLoop_processEvents_flags(original_self as *mut ::event_loop::EventLoop,
                                                        flags.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> EventLoopProcessEventsArgs<'largs> for (::flags::Flags<::event_loop::ProcessEventsFlag>, ::libc::c_int) {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> () {
      let flags = self.0;
      let maximum_time = self.1;
      unsafe {
        ::ffi::qt_core_c_QEventLoop_processEvents_flags_maximumTime(original_self as *mut ::event_loop::EventLoop,
                                                                    flags.to_int() as ::libc::c_uint,
                                                                    maximum_time)
      }
    }
  }
  impl<'largs> EventLoopProcessEventsArgs<'largs> for () {
    type ReturnType = bool;
    fn exec(self, original_self: &'largs mut ::event_loop::EventLoop) -> bool {

      unsafe { ::ffi::qt_core_c_QEventLoop_processEvents_no_args(original_self as *mut ::event_loop::EventLoop) }
    }
  }
}
